pub mod grammar {
    use std::collections::HashMap;

    use pest_derive::Parser;
    use pest::iterators::Pairs;
    use pest::pratt_parser::PrattParser;
    use pest::Parser;

    lazy_static::lazy_static! {
        static ref PRATT_PARSER: PrattParser<Rule> = {
            use pest::pratt_parser::{Assoc::*, Op};
            use Rule::*;

            PrattParser::new()
                .op(Op::infix(add, Left) | Op::infix(subtract, Left))
                .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
                .op(Op::infix(modulo, Left))
                .op(Op::infix(power, Left))
                .op(Op::prefix(unary_minus))
                .op(Op::infix(assignment, Left))
        };
    }

    #[derive(Parser)]
    #[grammar = "grammar/peg/calc.pest"]
    pub struct CalculatorParser;

    #[derive(Debug, Clone)]
    pub struct Variable {
        pub name: String,
        pub expr: Box<Expr>,
    }

    #[derive(Debug, Clone)]
    pub enum Expr {
        Integer {
            value: i32,
            variable: Option<Variable>,
        },
        Float {
            value: f64,
            variable: Option<Variable>,
        },
        UnboundVariable {
            name: String,
        },
        BinaryOperation {
            lhs: Box<Expr>,
            op: BinaryOperator,
            rhs: Box<Expr>,
            value: f64,
        },
        UnaryOperation {
            op: UnaryOperator,
            expr: Box<Expr>,
            value: f64,
        },
        Assignment {
            identifier: String,
            expr: Box<Expr>,
        },
    }

    impl From<Expr> for f64 {
        fn from(val: Expr) -> Self {
            match val {
                Expr::Integer { value, .. } => value as f64,
                Expr::Float { value, .. } => value,
                Expr::BinaryOperation { value, .. } => value,
                Expr::UnaryOperation { value, .. } => value,
                Expr::UnboundVariable { .. } => f64::NAN,
                Expr::Assignment { .. } => unreachable!("Can't unwrap assignment"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum UnaryOperator {
        Minus,
    }

    #[derive(Debug, Clone)]
    pub enum BinaryOperator {
        Add,
        Subtract,
        Multiply,
        Divide,
        Modulo,
        Power,
    }

    impl std::fmt::Display for BinaryOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                BinaryOperator::Add => write!(f, "+"),
                BinaryOperator::Subtract => write!(f, "-"),
                BinaryOperator::Multiply => write!(f, "*"),
                BinaryOperator::Divide => write!(f, "/"),
                BinaryOperator::Modulo => write!(f, "%"),
                BinaryOperator::Power => write!(f, "^"),
            }
        }
    }

    pub fn parse_equation(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
        CalculatorParser::parse(Rule::equation, input)
    }

    pub fn parse_partial_term(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
        CalculatorParser::parse(Rule::partial_term, input)
    }

    pub fn eval(pairs: Pairs<Rule>, variables: &HashMap<String, Expr>) -> Expr {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::integer => Expr::Integer {
                    value: primary.as_str().parse::<i32>().unwrap(),
                    variable: None,
                },
                Rule::float => Expr::Float {
                    value: primary.as_str().parse::<f64>().unwrap(),
                    variable: None,
                },
                Rule::variable => {
                    let variable_name = primary.as_str().to_string();
                    if let Some(expr) = variables.get(&variable_name) {
                        let value: f64 = expr.clone().into();
                        Expr::Float {
                            value,
                            variable: Some(Variable {
                                name: variable_name,
                                expr: Box::new(expr.clone()),
                            }),
                        }
                    } else {
                        Expr::UnboundVariable { name: variable_name }
                    }
                }
                Rule::expr => eval(primary.into_inner(), variables),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            })
            .map_prefix(|op, expr| {
                let expr_value: f64 = expr.clone().into();
                match op.as_rule() {
                    Rule::unary_minus => Expr::UnaryOperation {
                        op: UnaryOperator::Minus,
                        expr: Box::new(expr),
                        value: -expr_value,
                    },
                    rule => unreachable!("Expr::parse expected prefix, found {:?}", rule),
                }
            })
            .map_infix(|lhs, op, rhs| {
                let lhs_value: f64 = lhs.clone().into();
                let rhs_value: f64 = rhs.clone().into();
                let (op, value) = match op.as_rule() {
                    Rule::add => (BinaryOperator::Add, lhs_value + rhs_value),
                    Rule::subtract => (BinaryOperator::Subtract, lhs_value - rhs_value),
                    Rule::multiply => (BinaryOperator::Multiply, lhs_value * rhs_value),
                    Rule::divide => (BinaryOperator::Divide, lhs_value / rhs_value),
                    Rule::modulo => (BinaryOperator::Modulo, lhs_value % rhs_value),
                    Rule::power => (BinaryOperator::Power, lhs_value.powf(rhs_value)),
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                };
                Expr::BinaryOperation {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                    value,
                }
            })
            .parse(pairs)
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_precedence() {
            let input = "1 + 4 * 3";
            let mut pairs = parse_equation(input).unwrap();
            println!("{:?}", pairs);
            let result = eval(pairs.next().unwrap().into_inner(), &HashMap::new());
            let value: f64 = result.into();
            assert_eq!(value, 13.0);
        }

        #[test]
        fn test_bracket_precedence() {
            let input = "(1 + 4) * 3";
            let mut pairs = parse_equation(input).unwrap();
            println!("{:?}", pairs);
            let result = eval(pairs.next().unwrap().into_inner(), &HashMap::new());
            let value: f64 = result.into();
            assert_eq!(value, 15.0);
        }

        #[test]
        fn test_unary_minus() {
            let input = "1 + 4 * -3";
            let mut pairs = parse_equation(input).unwrap();
            println!("{:?}", pairs);
            let result = eval(pairs.next().unwrap().into_inner(), &HashMap::new());
            let value: f64 = result.into();
            assert_eq!(value, -11.0);
        }

        #[test]
        fn test_variables() {
            let input = "a + b";
            let mut pairs = parse_equation(input).unwrap();
            println!("{:?}", pairs);
            let result = eval(
                pairs.next().unwrap().into_inner(),
                &HashMap::from([
                    (
                        "a".to_string(),
                        Expr::Float {
                            value: 10f64,
                            variable: None,
                        },
                    ),
                    (
                        "b".to_string(),
                        Expr::Float {
                            value: 20f64,
                            variable: None,
                        },
                    ),
                ]),
            );
            let value: f64 = result.into();
            assert_eq!(value, 30.0);
        }

        #[test]
        fn test_unbound_variables() {
            let input = "a + (b * c)";
            let mut pairs = parse_equation(input).unwrap();
            println!("{:?}", pairs);
            let result = eval(
                pairs.next().unwrap().into_inner(),
                &HashMap::from([
                    (
                        "a".to_string(),
                        Expr::Float {
                            value: 10f64,
                            variable: None,
                        },
                    ),
                    (
                        "b".to_string(),
                        Expr::Float {
                            value: 20f64,
                            variable: None,
                        },
                    ),
                ]),
            );
            let value: f64 = result.into();
            assert!(value.is_nan())
        }
    }
}
