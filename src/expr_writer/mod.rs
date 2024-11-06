pub mod expr_writer {
    use nu_ansi_term::{Color, Style};
    use reedline::StyledText;

    use crate::grammar::grammar::Expr;


    #[derive(Debug, Clone)]
    struct Column {
        width: usize,
        state: ColumnState,
    }

    #[derive(Debug, Clone)]
    enum ColumnState {
        Empty,
        Start,
        Open,
        End,
    }

    fn do_write_expr_tree(expr: &Expr, columns: Vec<Column>, active_column: usize) {
        const EDGE: &str = "└─";
        const PIPE: &str = "│ ";
        const BRANCH: &str = "├─";

        // Draw all of the columns before this one, for this line, making sure that the
        // width of the column is honoured.
        for column in columns.iter() {
            match column.state {
                ColumnState::Empty => print!("{:>width$} ", "", width = column.width),
                ColumnState::Start => print!("{:>width$} ", BRANCH, width = column.width),
                ColumnState::Open => print!("{:>width$} ", PIPE, width = column.width),
                ColumnState::End => print!("{:>width$} ", EDGE, width = column.width),
            }
        }

        match expr {
            Expr::Integer { value, variable } => {
                let mut styled_text = StyledText::new();
                styled_text.push((Style::new().fg(Color::Blue), value.to_string()));
                if let Some(variable) = variable {
                    styled_text.push((Style::new().fg(Color::Purple), format!(" ({})", variable.name)));
                }

                println!("{}", styled_text.render_simple());
            }
            Expr::Float { value, variable } => {
                let mut styled_text = StyledText::new();
                if value.is_nan() {
                    styled_text.push((Style::new().fg(Color::Blue), "???".to_string()));
                } else {
                    styled_text.push((Style::new().fg(Color::Blue), value.to_string()));
                }

                if let Some(variable) = variable {
                    styled_text.push((Style::new().fg(Color::Purple), format!(" ({})", variable.name)));
                }

                println!("{}", styled_text.render_simple());
            }
            Expr::UnboundVariable { name } => {
                let mut styled_text = StyledText::new();
                styled_text.push((Style::new().fg(Color::Red), format!("{} <- unbound variable", name.to_string())));
                println!("{}", styled_text.render_simple());
            }
            Expr::BinaryOperation { lhs, op, rhs, value } => {
                let mut styled_text = StyledText::new();

                if value.is_nan() {
                    styled_text.push((Style::new().fg(Color::Cyan), "???".to_string()));
                } else {
                    styled_text.push((Style::new().fg(Color::Cyan), value.to_string()));
                }

                styled_text.push((Style::new().fg(Color::White), " = ".to_string()));
                styled_text.push((Style::new().fg(Color::White), format!("{}", op)));

                println!("{}", styled_text.render_simple());
                let updated_columns = columns
                    .iter()
                    .map(|column| {
                        let updated_state = match column.state {
                            ColumnState::Empty => ColumnState::Empty,
                            ColumnState::Start => ColumnState::Open,
                            ColumnState::Open => ColumnState::Open,
                            ColumnState::End => ColumnState::Empty,
                        };
                        Column {
                            width: column.width,
                            state: updated_state,
                        }
                    })
                    .collect::<Vec<_>>();

                let mut left_columns = updated_columns.clone();
                left_columns.push(Column {
                    width: styled_text.raw_string().len() + 1,
                    state: ColumnState::Start,
                });

                let mut right_columns = updated_columns.clone();
                right_columns.push(Column {
                    width: styled_text.raw_string().len() + 1,
                    state: ColumnState::End,
                });

                do_write_expr_tree(lhs.as_ref(), left_columns, active_column + 1);

                do_write_expr_tree(rhs.as_ref(), right_columns, active_column + 1);
            }
            _ => unreachable!("Unexpected expression: {:?}", expr)
        }
    }

    pub fn write_expr_tree(expr: Expr) {
        let columns: Vec<Column> = vec![];
        do_write_expr_tree(&expr, columns, 0);
    }
}
