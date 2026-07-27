use crate::expression::index::Index;
use crate::{ExpressionCategory, ExpressionError};

/// A simple lexer for tokenizing expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LexarToken {
    /// Represents an atomic value (e.g., a number or identifier).
    Atom((Index, String)),
    /// Represents an operator (e.g., `+`, `-`, `*`, `/`).
    Operator((Index, String)),
    /// Represents the end of the input.
    EndOfInput,
}

/// A simple lexer for tokenizing expressions.
///
/// Valid Tokens for Atoms:
/// - Alphanumeric characters (a-z, 0-9)
/// - Underscore (_)
///
/// Valid Tokens for Operators:
/// - Operators: +, -, *, /, (, ), \[, \], ==, <, >, <=, >=, !=, &&, ||, %, ^, !, ,
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Lexer {
    tokens: Vec<LexarToken>,
    source: String,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Result<Self, ExpressionError> {
        let mut lexer = Self {
            tokens: Vec::new(),
            source: input.to_string(),
        };
        lexer.tokenize(input)?;
        Ok(lexer)
    }

    /// Returns the original expression text that this lexer tokenized.
    pub(crate) fn source(&self) -> &str {
        &self.source
    }

    fn tokenize(&mut self, input: &str) -> Result<(), ExpressionError> {
        let mut chars = input.chars().enumerate().peekable();

        while let Some((i, c)) = chars.next() {
            // Skip whitespace characters
            if c.is_whitespace() {
                continue;
            }

            // Check for invalid characters (non-ASCII)
            if !c.is_ascii() {
                return Err(ExpressionError::new(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in expression: '{}'", c),
                    input,
                    vec![Index::new(i, 1)]
                ));
            }

            // Check for invalid tokens
            if !c.is_numeric() && !c.is_lowercase() && !"+_-*/()[]<>=!&|%^.,".contains(c) {
                return Err(ExpressionError::new(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in expression: '{}'", c),
                    input,
                    vec![Index::new(i, 1)]
                ));
            }

            if c == '.' {
                let mut s = String::new();
                s.push(c);
                let start = i;
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        s.push(chars.next().expect("peeked value must be present").1);
                    } else {
                        break;
                    }
                }

                if s == "." {
                    self.tokens
                        .push(LexarToken::Operator((Index::new(start, 1), s)));
                } else {
                    self.tokens
                        .push(LexarToken::Atom((Index::new(start, s.len()), s)));
                }
            } else if c.is_numeric() {
                let mut s = String::new();
                s.push(c);
                let start = i;
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        s.push(chars.next().expect("peeked value must be present").1);
                    } else {
                        break;
                    }
                }
                self.tokens
                    .push(LexarToken::Atom((Index::new(start, s.len()), s)));
            } else if c.is_alphanumeric() || c == '_' {
                let mut s = String::new();
                s.push(c);
                let start = i;
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '.' {
                        s.push(chars.next().expect("peeked value must be present").1);
                    } else {
                        break;
                    }
                }
                self.tokens
                    .push(LexarToken::Atom((Index::new(start, s.len()), s)));
            } else {
                let mut s = String::new();
                s.push(c);
                let start = i;
                if let Some(&(_, next_c)) = chars.peek() {
                    match (c, next_c) {
                        ('!', '=')
                        | ('&', '&')
                        | ('<', '=')
                        | ('=', '=')
                        | ('>', '=')
                        | ('|', '|') => {
                            s.push(chars.next().expect("peeked value must be present").1);
                        }
                        _ => {}
                    }
                }

                self.tokens
                    .push(LexarToken::Operator((Index::new(start, s.len()), s)));
            }
        }

        for token in self.tokens.iter() {
            match token {
                LexarToken::Atom((index, s)) => {
                    if s.starts_with("_") {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Lexer,
                            format!("Invalid string in expression: '{}'", s),
                            input,
                            vec![index.clone()],
                        ));
                    }

                    if s.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                        && s.matches('.').count() > 1
                    {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Lexer,
                            format!("Invalid number in expression: '{}'", s),
                            input,
                            vec![index.clone()],
                        ));
                    }
                }
                LexarToken::Operator((index, s)) => {
                    if s == "&" || s == "|" || s == "=" || s == "." {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Lexer,
                            format!("Invalid operator in expression: '{}'", s),
                            input,
                            vec![index.clone()],
                        ));
                    }
                }
                LexarToken::EndOfInput => unreachable!(),
            }
        }

        self.tokens.reverse();

        Ok(())
    }

    pub(crate) fn next(&mut self) -> LexarToken {
        self.tokens.pop().unwrap_or(LexarToken::EndOfInput)
    }

    pub(crate) fn peek(&mut self) -> LexarToken {
        self.tokens
            .last()
            .cloned()
            .unwrap_or(LexarToken::EndOfInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let input = "a + b * (c - d)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 1), "a".to_string())),
            LexarToken::Operator((Index::new(2, 1), "+".to_string())),
            LexarToken::Atom((Index::new(4, 1), "b".to_string())),
            LexarToken::Operator((Index::new(6, 1), "*".to_string())),
            LexarToken::Operator((Index::new(8, 1), "(".to_string())),
            LexarToken::Atom((Index::new(9, 1), "c".to_string())),
            LexarToken::Operator((Index::new(11, 1), "-".to_string())),
            LexarToken::Atom((Index::new(13, 1), "d".to_string())),
            LexarToken::Operator((Index::new(14, 1), ")".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn basic_test_no_spaces() {
        let input = "a+b*(c-d)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 1), "a".to_string())),
            LexarToken::Operator((Index::new(1, 1), "+".to_string())),
            LexarToken::Atom((Index::new(2, 1), "b".to_string())),
            LexarToken::Operator((Index::new(3, 1), "*".to_string())),
            LexarToken::Operator((Index::new(4, 1), "(".to_string())),
            LexarToken::Atom((Index::new(5, 1), "c".to_string())),
            LexarToken::Operator((Index::new(6, 1), "-".to_string())),
            LexarToken::Atom((Index::new(7, 1), "d".to_string())),
            LexarToken::Operator((Index::new(8, 1), ")".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_1() {
        let input = "g_test + p_apple * (v_one - v_two)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 6), "g_test".to_string())),
            LexarToken::Operator((Index::new(7, 1), "+".to_string())),
            LexarToken::Atom((Index::new(9, 7), "p_apple".to_string())),
            LexarToken::Operator((Index::new(17, 1), "*".to_string())),
            LexarToken::Operator((Index::new(19, 1), "(".to_string())),
            LexarToken::Atom((Index::new(20, 5), "v_one".to_string())),
            LexarToken::Operator((Index::new(26, 1), "-".to_string())),
            LexarToken::Atom((Index::new(28, 5), "v_two".to_string())),
            LexarToken::Operator((Index::new(33, 1), ")".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_2() {
        let input = "sin(p_angle)/(v_table[1][1]^2) + 43.5!";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 3), "sin".to_string())),
            LexarToken::Operator((Index::new(3, 1), "(".to_string())),
            LexarToken::Atom((Index::new(4, 7), "p_angle".to_string())),
            LexarToken::Operator((Index::new(11, 1), ")".to_string())),
            LexarToken::Operator((Index::new(12, 1), "/".to_string())),
            LexarToken::Operator((Index::new(13, 1), "(".to_string())),
            LexarToken::Atom((Index::new(14, 7), "v_table".to_string())),
            LexarToken::Operator((Index::new(21, 1), "[".to_string())),
            LexarToken::Atom((Index::new(22, 1), "1".to_string())),
            LexarToken::Operator((Index::new(23, 1), "]".to_string())),
            LexarToken::Operator((Index::new(24, 1), "[".to_string())),
            LexarToken::Atom((Index::new(25, 1), "1".to_string())),
            LexarToken::Operator((Index::new(26, 1), "]".to_string())),
            LexarToken::Operator((Index::new(27, 1), "^".to_string())),
            LexarToken::Atom((Index::new(28, 1), "2".to_string())),
            LexarToken::Operator((Index::new(29, 1), ")".to_string())),
            LexarToken::Operator((Index::new(31, 1), "+".to_string())),
            LexarToken::Atom((Index::new(33, 4), "43.5".to_string())),
            LexarToken::Operator((Index::new(37, 1), "!".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_3() {
        let input = "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 8), "p_value1".to_string())),
            LexarToken::Operator((Index::new(9, 2), ">=".to_string())),
            LexarToken::Atom((Index::new(12, 8), "p_value2".to_string())),
            LexarToken::Operator((Index::new(21, 2), "&&".to_string())),
            LexarToken::Atom((Index::new(24, 8), "p_value3".to_string())),
            LexarToken::Operator((Index::new(33, 2), "!=".to_string())),
            LexarToken::Atom((Index::new(36, 8), "p_value4".to_string())),
            LexarToken::Operator((Index::new(45, 2), "||".to_string())),
            LexarToken::Atom((Index::new(48, 8), "p_value1".to_string())),
            LexarToken::Operator((Index::new(57, 2), "<=".to_string())),
            LexarToken::Atom((Index::new(60, 8), "p_value2".to_string())),
            LexarToken::Operator((Index::new(69, 2), "||".to_string())),
            LexarToken::Atom((Index::new(72, 8), "p_value3".to_string())),
            LexarToken::Operator((Index::new(81, 2), "==".to_string())),
            LexarToken::Atom((Index::new(84, 8), "p_value4".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_4() {
        let input = "p_map[key1][item1] + p_map[key2][item2]";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 5), "p_map".to_string())),
            LexarToken::Operator((Index::new(5, 1), "[".to_string())),
            LexarToken::Atom((Index::new(6, 4), "key1".to_string())),
            LexarToken::Operator((Index::new(10, 1), "]".to_string())),
            LexarToken::Operator((Index::new(11, 1), "[".to_string())),
            LexarToken::Atom((Index::new(12, 5), "item1".to_string())),
            LexarToken::Operator((Index::new(17, 1), "]".to_string())),
            LexarToken::Operator((Index::new(19, 1), "+".to_string())),
            LexarToken::Atom((Index::new(21, 5), "p_map".to_string())),
            LexarToken::Operator((Index::new(26, 1), "[".to_string())),
            LexarToken::Atom((Index::new(27, 4), "key2".to_string())),
            LexarToken::Operator((Index::new(31, 1), "]".to_string())),
            LexarToken::Operator((Index::new(32, 1), "[".to_string())),
            LexarToken::Atom((Index::new(33, 5), "item2".to_string())),
            LexarToken::Operator((Index::new(38, 1), "]".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_5() {
        let input = "function(p_map[key1][item1], p_map[key2][item2])";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 8), "function".to_string())),
            LexarToken::Operator((Index::new(8, 1), "(".to_string())),
            LexarToken::Atom((Index::new(9, 5), "p_map".to_string())),
            LexarToken::Operator((Index::new(14, 1), "[".to_string())),
            LexarToken::Atom((Index::new(15, 4), "key1".to_string())),
            LexarToken::Operator((Index::new(19, 1), "]".to_string())),
            LexarToken::Operator((Index::new(20, 1), "[".to_string())),
            LexarToken::Atom((Index::new(21, 5), "item1".to_string())),
            LexarToken::Operator((Index::new(26, 1), "]".to_string())),
            LexarToken::Operator((Index::new(27, 1), ",".to_string())),
            LexarToken::Atom((Index::new(29, 5), "p_map".to_string())),
            LexarToken::Operator((Index::new(34, 1), "[".to_string())),
            LexarToken::Atom((Index::new(35, 4), "key2".to_string())),
            LexarToken::Operator((Index::new(39, 1), "]".to_string())),
            LexarToken::Operator((Index::new(40, 1), "[".to_string())),
            LexarToken::Atom((Index::new(41, 5), "item2".to_string())),
            LexarToken::Operator((Index::new(46, 1), "]".to_string())),
            LexarToken::Operator((Index::new(47, 1), ")".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_6() {
        let input = "2.0p_value1 + 5.0p_value2 * 6.0(.87p_value3 - 77p_value4) / p_value5";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom((Index::new(0, 3), "2.0".to_string())),
            LexarToken::Atom((Index::new(3, 8), "p_value1".to_string())),
            LexarToken::Operator((Index::new(12, 1), "+".to_string())),
            LexarToken::Atom((Index::new(14, 3), "5.0".to_string())),
            LexarToken::Atom((Index::new(17, 8), "p_value2".to_string())),
            LexarToken::Operator((Index::new(26, 1), "*".to_string())),
            LexarToken::Atom((Index::new(28, 3), "6.0".to_string())),
            LexarToken::Operator((Index::new(31, 1), "(".to_string())),
            LexarToken::Atom((Index::new(32, 3), ".87".to_string())),
            LexarToken::Atom((Index::new(35, 8), "p_value3".to_string())),
            LexarToken::Operator((Index::new(44, 1), "-".to_string())),
            LexarToken::Atom((Index::new(46, 2), "77".to_string())),
            LexarToken::Atom((Index::new(48, 8), "p_value4".to_string())),
            LexarToken::Operator((Index::new(56, 1), ")".to_string())),
            LexarToken::Operator((Index::new(58, 1), "/".to_string())),
            LexarToken::Atom((Index::new(60, 8), "p_value5".to_string())),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_peak_and_next_index() {
        let input = "a + b * (c - d)";
        let mut lexer = Lexer::new(input).unwrap();

        // Peek at the first token
        let token = lexer.peek();
        assert_eq!(token, LexarToken::Atom((Index::new(0, 1), "a".to_string())));

        // Consume the first token
        let token = lexer.next();
        assert_eq!(token, LexarToken::Atom((Index::new(0, 1), "a".to_string())));

        // Peek at the next token
        let token = lexer.peek();
        assert_eq!(
            token,
            LexarToken::Operator((Index::new(2, 1), "+".to_string()))
        );
    }

    #[test]
    fn test_invalid_characters_1() {
        for c in 0..=255 {
            let ch = c as u8 as char;
            if !ch.is_numeric()
                && !ch.is_lowercase()
                && !"+_-*/()[]<>=!&|%^.,".contains(ch)
                && !ch.is_whitespace()
            {
                let input = format!("a + b * (c - d) {} e", ch);
                let result = Lexer::new(&input);
                assert!(result.is_err());
                let error = result.err().unwrap();
                assert_eq!(error.category, ExpressionCategory::Lexer);
                assert_eq!(
                    error.message,
                    format!("Invalid character in expression: '{}'", ch)
                );
            }
        }
    }

    #[test]
    fn test_invalid_characters_2() {
        let input = "a + b * (c - d) \u{1F600} e"; // Includes a non-ASCII character (😀)
        let result = Lexer::new(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.category, ExpressionCategory::Lexer);
        assert_eq!(
            error.message,
            format!("Invalid character in expression: '{}'", '\u{1F600}')
        );
    }

    #[test]
    fn test_invalid_characters_3() {
        let input = "5..0";
        let result = Lexer::new(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.category, ExpressionCategory::Lexer);
        assert_eq!(error.message, "Invalid number in expression: '5..0'");
    }

    #[test]
    fn test_invalid_characters_4() {
        for c in "=&|.".chars() {
            let ch = c as u8 as char;
            let input = format!("a + b * (c - d) {} e", ch);
            let result = Lexer::new(&input);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.category, ExpressionCategory::Lexer);
            assert_eq!(
                error.message,
                format!("Invalid operator in expression: '{}'", ch)
            );
        }
    }

    #[test]
    fn test_invalid_characters_5() {
        for c in "_".chars() {
            let ch = c as u8 as char;
            let input = format!("a + b * (c - d) {} e", ch);
            let result = Lexer::new(&input);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.category, ExpressionCategory::Lexer);
            assert_eq!(
                error.message,
                format!("Invalid string in expression: '{}'", ch)
            );
        }
    }

    #[test]
    fn display_renders_underline_beneath_marked_span() {
        // A single invalid character produces a single `~` at its position.
        let error = Lexer::new("1 + @ * 2").unwrap_err();
        let rendered = error.to_string();
        assert_eq!(
            rendered,
            "[Lexer] Invalid character in expression: '@'\n1 + @ * 2\n    ~\n"
        );
    }

    #[test]
    fn display_renders_underline_across_a_multi_char_span() {
        // An invalid number spans the whole token, so the underline covers it.
        let error = Lexer::new("1.2.3 * 2").unwrap_err();
        let rendered = error.to_string();
        assert_eq!(
            rendered,
            "[Lexer] Invalid number in expression: '1.2.3'\n1.2.3 * 2\n~~~~~\n"
        );
    }
}
