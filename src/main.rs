mod expr_writer;
mod highlighter;
mod grammar;

use clap::Parser as ClapParser;
use highlighter::highlighter::ArithmeticHighlighter;
use std::collections::{HashMap, HashSet};
use grammar::grammar::{eval, parse_equation, Expr, Rule};
use expr_writer::expr_writer::write_expr_tree;

use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

#[derive(Default, ClapParser, Debug)]
struct Arguments {
    expression: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    let mut variables = HashMap::new();
    if let Some(expression) = args.expression {
        handle_input(expression, &mut variables, None);
    } else {
        let mut line_editor = Reedline::create().with_highlighter(Box::new(ArithmeticHighlighter));
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic("> ".to_string()),
            right_prompt: DefaultPromptSegment::Empty,
        };

        loop {
            let sig = line_editor.read_line(&prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    handle_input(buffer, &mut variables, None);
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("\nAborted!");
                    break;
                }
                x => {
                    println!("Event: {:?}", x);
                }
            }
        }
    }
}

fn unbound_variables(expr: &Expr) -> HashSet<String> {
    let mut unbound = HashSet::new();
    match expr {
        Expr::BinaryOperation { lhs, rhs, .. } => {
            unbound.extend(unbound_variables(lhs));
            unbound.extend(unbound_variables(rhs));
        }
        Expr::UnaryOperation { expr, .. } => {
            unbound.extend(unbound_variables(expr));
        }
        Expr::UnboundVariable { name } => {
            unbound.insert(name.clone());
        }
        _ => {}
    }
    unbound
}

fn handle_input(buffer: String, variables: &mut HashMap<String, Expr>, last_expr: Option<Expr>) {
    match parse_equation(&buffer) {
        Ok(mut pairs) => {
            if let Some(pair) = pairs.next() {
                match pair.as_rule() {
                    Rule::assignment => {
                        // If the first pair is an assignment, evaluate it and store the variable
                        let mut inner_pairs = pair.into_inner();
                        let variable = inner_pairs.next().unwrap().as_str().to_string();
                        let expr = eval(inner_pairs.next().unwrap().into_inner(), variables);

                        let unbound = unbound_variables(&expr);
                        if unbound.len() > 0 {
                            println!("Unbound variables: {:?}", unbound);
                        } else {
                            variables.insert(variable.clone(), expr.clone());
                        }

                        write_expr_tree(expr);
                    }
                    Rule::command => {
                        let command = pair.into_inner().next().unwrap().as_str();
                        match command {
                            "state" => {
                                for (key, value) in variables.iter() {
                                    println!("{} = {:?}", key, value);
                                }
                            }
                            "reset" => {
                                variables.clear();
                            }
                            "debug" => {
                                println!("{:?}", last_expr);
                            }
                            _ => println!("Unknown command: {}", command),
                        }
                    }
                    _ => {
                        let expr = eval(pair.into_inner(), variables);
                        let unbound = unbound_variables(&expr);
                        if unbound.len() > 0 {
                            println!("Unbound variables: {:?}", unbound);
                        }
                        write_expr_tree(expr);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", buffer);
            // Draw a ^ in the location of the error
            let location = match e.location {
                pest::error::InputLocation::Pos(col) => col,
                pest::error::InputLocation::Span((start, _)) => start,
            };
            let output = format!("{:>width$}^", "", width = location);
            println!("{}", output);

            println!("Parse failed: {:?}", e);
        }
    }
}
