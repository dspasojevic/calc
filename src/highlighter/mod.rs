pub mod highlighter {
    use nu_ansi_term::{Color, Style};
    use pest::iterators::Pair;
    use reedline::{Highlighter, StyledText};

    use crate::grammar::grammar::{parse_partial_term, Rule};
    pub struct ArithmeticHighlighter;

    impl ArithmeticHighlighter {
        pub fn highlight_pair(&self, pair: Pair<Rule>, styled_text: &mut StyledText) {
            if pair.clone().into_inner().count() == 0 {
                // This is a leaf node, highlight it
                let span = pair.as_span();

                // Match the rule and apply colors based on the smallest matched rule
                let styled = match pair.as_rule() {
                    Rule::integer | Rule::float => Style::new().fg(Color::Blue),
                    Rule::multiply => Style::new().fg(Color::Green),
                    Rule::divide => Style::new().fg(Color::Green),
                    Rule::add => Style::new().fg(Color::Green),
                    Rule::modulo => Style::new().fg(Color::Green),
                    Rule::power => Style::new().fg(Color::Green),
                    Rule::identifier => Style::new().fg(Color::Yellow),
                    Rule::variable => Style::new().fg(Color::Purple),
                    Rule::comment => Style::new().fg(Color::LightGray),
                    Rule::WHITESPACE => Style::new().fg(Color::White),
                    _ => Style::new().fg(Color::White),
                };

                // Style the range.
                styled_text.style_range(span.start(), span.end(), styled);
            } else {
                // Recursively process inner pairs (smaller parts of the expression)
                for inner_pair in pair.into_inner() {
                    self.highlight_pair(inner_pair, styled_text);
                }
            }
        }
    }

    impl Highlighter for ArithmeticHighlighter {
        fn highlight(&self, line: &str, _cursor_pos: usize) -> StyledText {
            let mut styled_text = StyledText::new();

            styled_text.push((Style::new().fg(Color::Default), line.to_string()));

            match parse_partial_term(line) {
                Ok(pairs) => {
                    for pair in pairs {
                        self.highlight_pair(pair, &mut styled_text);
                    }
                }
                Err(_) => {
                    // Fallback: style the entire line as error (red)
                    styled_text.push((Style::new().fg(Color::Red), line.to_string()));
                }
            }

            styled_text
        }
    }
}
