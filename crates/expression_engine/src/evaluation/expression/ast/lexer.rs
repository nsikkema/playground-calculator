use crate::{ExpressionCategory, ExpressionError};
use message::span::{Span, SpanSet};
use shareable_string::ShareableString;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

/// A simple lexer for tokenizing expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LexerToken {
    /// Represents an identifier (e.g., `variable_name`, `function1`, `my_var_2`).
    Identifier(Span, String),
    /// Represents a numeric value (e.g., `123`, `45.67`, `.89`, `0.001`, `1e10`, `1.5e-3`, `.5e+2`).
    Numeric(Span, String),
    /// Represents an operator (e.g., `+`, `-`, `*`, `/`).
    Operator(Span, String),
    /// Represents a quoted string literal (e.g., `"some/path.txt"`), holding the
    /// unquoted contents.
    Text(Span, String),
    /// Represents the end of the input.
    EndOfInput,
}

/// A simple lexer for tokenizing expressions.
///
/// Examples of valid identifiers:
/// - Identifiers: `variable_name`, `function1`, `my_var_2`
///
/// Examples of valid Numbers:
/// - Numbers: `123`, `45.67`, `.89`, `0.001`
/// - Numbers in scientific notation (e.g. `1e10`, `1.5e-3`, `.5e+2`)
///
/// Valid Tokens for Operators:
/// - Operators: +, -, *, /, (, ), \[, \], ==, <, >, <=, >=, !=, &&, ||, %, ^, !, ,
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Lexer {
    /// The tokens produced by lexing, stored in reverse order so `pop` yields the next token.
    tokens: Vec<LexerToken>,
    /// The original expression source text.
    source: ShareableString,
}

impl Lexer {
    /// Creates a new `Lexer` by tokenizing `input`.
    ///
    /// # Errors
    ///
    /// Returns an error if `input` contains invalid characters or unterminated string literals.
    #[hotpath::measure]
    pub(crate) fn new<S: Into<ShareableString>>(input: S) -> Result<Self, ExpressionError> {
        let input = input.into();
        let mut lexer = Self {
            tokens: Vec::new(),
            source: input.clone(),
        };
        lexer.tokenize(input.as_ref())?;
        Ok(lexer)
    }

    /// Returns the original expression text that this lexer tokenized.
    pub(crate) const fn source(&self) -> &ShareableString {
        &self.source
    }

    /// Tokenizes `input` and populates `self.tokens`.
    ///
    /// On success the token list is reversed so that [`Self::next`] can use `Vec::pop`.
    ///
    /// # Errors
    ///
    /// Returns an error if an invalid character or unterminated string literal is found.
    #[hotpath::measure]
    fn tokenize(&mut self, input: &str) -> Result<(), ExpressionError> {
        let mut chars = input.chars().enumerate().peekable();

        while let Some((index, c)) = chars.next() {
            // Skip whitespace characters
            if c.is_whitespace() {
                continue;
            }

            // Check for invalid characters (non-ASCII)
            if !c.is_ascii() {
                return Err(ExpressionError::new_complex(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in expression: '{c}'"),
                    input,
                    SpanSet::from_span(Span::new(index, 1)),
                ));
            }

            // Check for invalid tokens
            if !c.is_numeric() && !c.is_lowercase() && !"+_-*/()[]<>=!&|%^.,\"".contains(c) {
                return Err(ExpressionError::new_complex(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in expression: '{c}'"),
                    input,
                    SpanSet::from_span(Span::new(index, 1)),
                ));
            }

            if c == '"' {
                self.tokenize_text(&mut chars, index, input)?;
            } else if c == '.' {
                self.tokenize_dot_number(&mut chars, index);
            } else if c.is_numeric() {
                self.tokenize_number(&mut chars, index, c);
            } else if c.is_alphanumeric() || c == '_' {
                self.tokenize_identifier(&mut chars, index, c);
            } else {
                self.tokenize_operator(&mut chars, index, c);
            }
        }

        self.validate_tokens(input)?;
        self.tokens.reverse();

        Ok(())
    }

    /// Returns whether `c` is allowed inside a quoted string literal: ASCII letters, digits,
    /// underscore, dash, dot, `/`, and space (i.e., typical filesystem path characters).
    const fn is_valid_string_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ' ')
    }

    /// Tokenizes a quoted string literal starting just after the opening `"`.
    ///
    /// On success a [`LexerToken::Text`] containing the trimmed, unquoted contents is pushed.
    ///
    /// # Errors
    ///
    /// Returns an error if an invalid character is found inside the literal or if the closing
    /// `"` is never reached.
    #[hotpath::measure]
    fn tokenize_text(
        &mut self,
        chars: &mut Peekable<Enumerate<Chars<'_>>>,
        start: usize,
        input: &str,
    ) -> Result<(), ExpressionError> {
        let mut s = String::new();
        let mut closed = false;

        while let Some(&(idx, c)) = chars.peek() {
            if c == '"' {
                chars.next();
                closed = true;
                break;
            }

            if !Self::is_valid_string_char(c) {
                return Err(ExpressionError::new_complex(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in string literal: '{c}'"),
                    input,
                    SpanSet::from_span(Span::new(idx, 1)),
                ));
            }

            chars.next();
            s.push(c);
        }

        if !closed {
            return Err(ExpressionError::new_complex(
                ExpressionCategory::Lexer,
                "Unterminated string literal".to_string(),
                input,
                SpanSet::from_span(Span::new(start, s.len().saturating_add(1))),
            ));
        }

        // +2 accounts for the opening and closing quotes, which aren't part of `s`.
        let len = s.len().saturating_add(2);
        s = s.trim().to_string();

        self.tokens.push(LexerToken::Text(Span::new(start, len), s));
        Ok(())
    }

    /// Tokenizes a number that starts with a leading `.` (e.g. `.5`), or a
    /// bare `.` operator if no digits follow.
    #[hotpath::measure]
    fn tokenize_dot_number(&mut self, chars: &mut Peekable<Enumerate<Chars<'_>>>, start: usize) {
        let mut s = String::from(".");
        while let Some(&(_, c)) = chars.peek() {
            if c.is_numeric() || c == '.' {
                if let Some((_, next_char)) = chars.next() {
                    s.push(next_char);
                }
            } else {
                break;
            }
        }

        if s == "." {
            self.tokens
                .push(LexerToken::Operator(Span::new(start, 1), s));
            return;
        }

        Self::consume_exponent(chars, &mut s);
        let number_len = s.len();
        self.tokens
            .push(LexerToken::Numeric(Span::new(start, number_len), s));

        if let Some(&(_, '(')) = chars.peek() {
            self.tokens.push(LexerToken::Operator(
                Span::new(
                    start
                        .saturating_add(number_len)
                        .saturating_sub(1)
                        .max(start),
                    2,
                ),
                "*".to_string(),
            ));
        }
    }

    /// Tokenizes a number that starts with a digit (e.g. `123`, `45.67`).
    #[hotpath::measure]
    fn tokenize_number(
        &mut self,
        chars: &mut Peekable<Enumerate<Chars<'_>>>,
        start: usize,
        c: char,
    ) {
        let mut s = String::new();
        s.push(c);
        while let Some(&(_, c)) = chars.peek() {
            if c.is_numeric() || c == '.' {
                if let Some((_, next_char)) = chars.next() {
                    s.push(next_char);
                }
            } else {
                break;
            }
        }
        Self::consume_exponent(chars, &mut s);
        let number_len = s.len();
        self.tokens
            .push(LexerToken::Numeric(Span::new(start, number_len), s));

        if let Some(&(_, '(')) = chars.peek() {
            self.tokens.push(LexerToken::Operator(
                Span::new(
                    start
                        .saturating_add(number_len)
                        .saturating_sub(1)
                        .max(start),
                    2,
                ),
                "*".to_string(),
            ));
        }
    }

    /// Tokenizes an identifier (e.g. `variable_name`, `function1`).
    #[hotpath::measure]
    fn tokenize_identifier(
        &mut self,
        chars: &mut Peekable<Enumerate<Chars<'_>>>,
        start: usize,
        c: char,
    ) {
        let mut s = String::new();
        s.push(c);
        while let Some(&(_, c)) = chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                if let Some((_, next_char)) = chars.next() {
                    s.push(next_char);
                }
            } else {
                break;
            }
        }
        let len = s.len();
        self.tokens
            .push(LexerToken::Identifier(Span::new(start, len), s));
    }

    /// Tokenizes an operator, including two-character operators such as
    /// `==`, `<=`, `>=`, `!=`, `&&`, and `||`.
    #[hotpath::measure]
    fn tokenize_operator(
        &mut self,
        chars: &mut Peekable<Enumerate<Chars<'_>>>,
        start: usize,
        c: char,
    ) {
        let mut s = String::new();
        s.push(c);
        if let Some(&(_, next_c)) = chars.peek() {
            match (c, next_c) {
                ('!' | '<' | '=' | '>', '=') | ('&', '&') | ('|', '|') => {
                    if let Some((_, next_char)) = chars.next() {
                        s.push(next_char);
                    }
                }
                _ => {}
            }
        }

        let len = s.len();
        self.tokens
            .push(LexerToken::Operator(Span::new(start, len), s));
    }

    /// Validates the tokens collected so far, returning an error if any `LexerToken` is invalid.
    /// This includes cases where an identifier starts with `_`, a number has multiple decimal points,
    /// or a standalone operator is malformed (e.g., `&`, `|`, `=`, or `.`).
    #[hotpath::measure]
    fn validate_tokens(&self, input: &str) -> Result<(), ExpressionError> {
        for token in &self.tokens {
            match token {
                LexerToken::Identifier(index, s) => {
                    if s.starts_with('_') {
                        return Err(ExpressionError::new_complex(
                            ExpressionCategory::Lexer,
                            format!("Invalid string in expression: '{s}'"),
                            input,
                            SpanSet::from_span(Span::new(index.start(), s.len())),
                        ));
                    }
                }
                LexerToken::Numeric(index, s) => {
                    if s.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                        && s.matches('.').count() > 1
                    {
                        return Err(ExpressionError::new_complex(
                            ExpressionCategory::Lexer,
                            format!("Invalid number in expression: '{s}'"),
                            input,
                            SpanSet::from_span(Span::new(index.start(), s.len())),
                        ));
                    }
                }
                LexerToken::Operator(index, s) => {
                    if s == "&" || s == "|" || s == "=" || s == "." {
                        return Err(ExpressionError::new_complex(
                            ExpressionCategory::Lexer,
                            format!("Invalid operator in expression: '{s}'"),
                            input,
                            SpanSet::from_span(Span::new(index.start(), s.len())),
                        ));
                    }
                }
                // String literal contents are already validated character-by-character while
                // they're being tokenized in `tokenize_text`. `EndOfInput` is only ever
                // synthesized on-demand by `next()`/`peek()` when `self.tokens` is empty; it is
                // never pushed into `self.tokens`.
                LexerToken::Text(_, _) | LexerToken::EndOfInput => {}
            }
        }

        Ok(())
    }

    /// Attempts to consume a scientific notation exponent suffix (e.g. `e10`, `e+10`, `e-10`)
    /// from `chars` and append it to `s`. If the characters following the current position
    /// don't form a valid exponent (i.e. `e` optionally followed by a sign and at least one
    /// digit), `chars` and `s` are left untouched.
    #[hotpath::measure]
    fn consume_exponent(chars: &mut Peekable<Enumerate<Chars<'_>>>, s: &mut String) {
        let mut lookahead = chars.clone();
        let mut exponent = String::new();

        match lookahead.peek() {
            Some(&(_, 'e')) => {
                if let Some((_, next_char)) = lookahead.next() {
                    exponent.push(next_char);
                }
            }
            _ => return,
        }

        if let Some(&(_, sign)) = lookahead.peek() {
            if sign == '+' || sign == '-' {
                if let Some((_, next_char)) = lookahead.next() {
                    exponent.push(next_char);
                }
            }
        }

        let mut has_digit = false;
        while let Some(&(_, d)) = lookahead.peek() {
            if d.is_numeric() {
                if let Some((_, next_char)) = lookahead.next() {
                    exponent.push(next_char);
                }
                has_digit = true;
            } else {
                break;
            }
        }

        if has_digit {
            s.push_str(&exponent);
            *chars = lookahead;
        }
    }

    /// Removes and returns the next token from the front of the token stream.
    ///
    /// Returns [`LexerToken::EndOfInput`] once all tokens have been consumed.
    #[hotpath::measure]
    pub(crate) fn next(&mut self) -> LexerToken {
        self.tokens.pop().unwrap_or(LexerToken::EndOfInput)
    }

    /// Returns a clone of the next token without consuming it.
    ///
    /// Returns [`LexerToken::EndOfInput`] once all tokens have been consumed.
    #[hotpath::measure]
    pub(crate) fn peek(&mut self) -> LexerToken {
        self.tokens
            .last()
            .cloned()
            .unwrap_or(LexerToken::EndOfInput)
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
            LexerToken::Identifier(Span::new(0, 1), "a".to_string()),
            LexerToken::Operator(Span::new(2, 1), "+".to_string()),
            LexerToken::Identifier(Span::new(4, 1), "b".to_string()),
            LexerToken::Operator(Span::new(6, 1), "*".to_string()),
            LexerToken::Operator(Span::new(8, 1), "(".to_string()),
            LexerToken::Identifier(Span::new(9, 1), "c".to_string()),
            LexerToken::Operator(Span::new(11, 1), "-".to_string()),
            LexerToken::Identifier(Span::new(13, 1), "d".to_string()),
            LexerToken::Operator(Span::new(14, 1), ")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn basic_test_no_spaces() {
        let input = "a+b*(c-d)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Identifier(Span::new(0, 1), "a".to_string()),
            LexerToken::Operator(Span::new(1, 1), "+".to_string()),
            LexerToken::Identifier(Span::new(2, 1), "b".to_string()),
            LexerToken::Operator(Span::new(3, 1), "*".to_string()),
            LexerToken::Operator(Span::new(4, 1), "(".to_string()),
            LexerToken::Identifier(Span::new(5, 1), "c".to_string()),
            LexerToken::Operator(Span::new(6, 1), "-".to_string()),
            LexerToken::Identifier(Span::new(7, 1), "d".to_string()),
            LexerToken::Operator(Span::new(8, 1), ")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_1() {
        let input = "g_test + p_apple * (v_one - v_two)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Identifier(Span::new(0, 6), "g_test".to_string()),
            LexerToken::Operator(Span::new(7, 1), "+".to_string()),
            LexerToken::Identifier(Span::new(9, 7), "p_apple".to_string()),
            LexerToken::Operator(Span::new(17, 1), "*".to_string()),
            LexerToken::Operator(Span::new(19, 1), "(".to_string()),
            LexerToken::Identifier(Span::new(20, 5), "v_one".to_string()),
            LexerToken::Operator(Span::new(26, 1), "-".to_string()),
            LexerToken::Identifier(Span::new(28, 5), "v_two".to_string()),
            LexerToken::Operator(Span::new(33, 1), ")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_2() {
        let input = "sin(p_angle)/(v_table[1][1]^2) + 43.5!";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Identifier(Span::new(0, 3), "sin".to_string()),
            LexerToken::Operator(Span::new(3, 1), "(".to_string()),
            LexerToken::Identifier(Span::new(4, 7), "p_angle".to_string()),
            LexerToken::Operator(Span::new(11, 1), ")".to_string()),
            LexerToken::Operator(Span::new(12, 1), "/".to_string()),
            LexerToken::Operator(Span::new(13, 1), "(".to_string()),
            LexerToken::Identifier(Span::new(14, 7), "v_table".to_string()),
            LexerToken::Operator(Span::new(21, 1), "[".to_string()),
            LexerToken::Numeric(Span::new(22, 1), "1".to_string()),
            LexerToken::Operator(Span::new(23, 1), "]".to_string()),
            LexerToken::Operator(Span::new(24, 1), "[".to_string()),
            LexerToken::Numeric(Span::new(25, 1), "1".to_string()),
            LexerToken::Operator(Span::new(26, 1), "]".to_string()),
            LexerToken::Operator(Span::new(27, 1), "^".to_string()),
            LexerToken::Numeric(Span::new(28, 1), "2".to_string()),
            LexerToken::Operator(Span::new(29, 1), ")".to_string()),
            LexerToken::Operator(Span::new(31, 1), "+".to_string()),
            LexerToken::Numeric(Span::new(33, 4), "43.5".to_string()),
            LexerToken::Operator(Span::new(37, 1), "!".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_3() {
        let input = "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Identifier(Span::new(0, 8), "p_value1".to_string()),
            LexerToken::Operator(Span::new(9, 2), ">=".to_string()),
            LexerToken::Identifier(Span::new(12, 8), "p_value2".to_string()),
            LexerToken::Operator(Span::new(21, 2), "&&".to_string()),
            LexerToken::Identifier(Span::new(24, 8), "p_value3".to_string()),
            LexerToken::Operator(Span::new(33, 2), "!=".to_string()),
            LexerToken::Identifier(Span::new(36, 8), "p_value4".to_string()),
            LexerToken::Operator(Span::new(45, 2), "||".to_string()),
            LexerToken::Identifier(Span::new(48, 8), "p_value1".to_string()),
            LexerToken::Operator(Span::new(57, 2), "<=".to_string()),
            LexerToken::Identifier(Span::new(60, 8), "p_value2".to_string()),
            LexerToken::Operator(Span::new(69, 2), "||".to_string()),
            LexerToken::Identifier(Span::new(72, 8), "p_value3".to_string()),
            LexerToken::Operator(Span::new(81, 2), "==".to_string()),
            LexerToken::Identifier(Span::new(84, 8), "p_value4".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_4() {
        let input = "p_map[key1][item1] + p_map[key2][item2]";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Identifier(Span::new(0, 5), "p_map".to_string()),
            LexerToken::Operator(Span::new(5, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(6, 4), "key1".to_string()),
            LexerToken::Operator(Span::new(10, 1), "]".to_string()),
            LexerToken::Operator(Span::new(11, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(12, 5), "item1".to_string()),
            LexerToken::Operator(Span::new(17, 1), "]".to_string()),
            LexerToken::Operator(Span::new(19, 1), "+".to_string()),
            LexerToken::Identifier(Span::new(21, 5), "p_map".to_string()),
            LexerToken::Operator(Span::new(26, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(27, 4), "key2".to_string()),
            LexerToken::Operator(Span::new(31, 1), "]".to_string()),
            LexerToken::Operator(Span::new(32, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(33, 5), "item2".to_string()),
            LexerToken::Operator(Span::new(38, 1), "]".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_5() {
        let input = "function(p_map[key1][item1], p_map[key2][item2])";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Identifier(Span::new(0, 8), "function".to_string()),
            LexerToken::Operator(Span::new(8, 1), "(".to_string()),
            LexerToken::Identifier(Span::new(9, 5), "p_map".to_string()),
            LexerToken::Operator(Span::new(14, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(15, 4), "key1".to_string()),
            LexerToken::Operator(Span::new(19, 1), "]".to_string()),
            LexerToken::Operator(Span::new(20, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(21, 5), "item1".to_string()),
            LexerToken::Operator(Span::new(26, 1), "]".to_string()),
            LexerToken::Operator(Span::new(27, 1), ",".to_string()),
            LexerToken::Identifier(Span::new(29, 5), "p_map".to_string()),
            LexerToken::Operator(Span::new(34, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(35, 4), "key2".to_string()),
            LexerToken::Operator(Span::new(39, 1), "]".to_string()),
            LexerToken::Operator(Span::new(40, 1), "[".to_string()),
            LexerToken::Identifier(Span::new(41, 5), "item2".to_string()),
            LexerToken::Operator(Span::new(46, 1), "]".to_string()),
            LexerToken::Operator(Span::new(47, 1), ")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_6() {
        let input = "2.0p_value1 + 5.0p_value2 * 6.0(.87p_value3 - 77p_value4) / p_value5";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Numeric(Span::new(0, 3), "2.0".to_string()),
            LexerToken::Identifier(Span::new(3, 8), "p_value1".to_string()),
            LexerToken::Operator(Span::new(12, 1), "+".to_string()),
            LexerToken::Numeric(Span::new(14, 3), "5.0".to_string()),
            LexerToken::Identifier(Span::new(17, 8), "p_value2".to_string()),
            LexerToken::Operator(Span::new(26, 1), "*".to_string()),
            LexerToken::Numeric(Span::new(28, 3), "6.0".to_string()),
            LexerToken::Operator(Span::new(30, 2), "*".to_string()),
            LexerToken::Operator(Span::new(31, 1), "(".to_string()),
            LexerToken::Numeric(Span::new(32, 3), ".87".to_string()),
            LexerToken::Identifier(Span::new(35, 8), "p_value3".to_string()),
            LexerToken::Operator(Span::new(44, 1), "-".to_string()),
            LexerToken::Numeric(Span::new(46, 2), "77".to_string()),
            LexerToken::Identifier(Span::new(48, 8), "p_value4".to_string()),
            LexerToken::Operator(Span::new(56, 1), ")".to_string()),
            LexerToken::Operator(Span::new(58, 1), "/".to_string()),
            LexerToken::Identifier(Span::new(60, 8), "p_value5".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_implicit_multiplication_before_parenthesis() {
        let input = "5(.2(3 + 2))";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Numeric(Span::new(0, 1), "5".to_string()),
            LexerToken::Operator(Span::new(0, 2), "*".to_string()),
            LexerToken::Operator(Span::new(1, 1), "(".to_string()),
            LexerToken::Numeric(Span::new(2, 2), ".2".to_string()),
            LexerToken::Operator(Span::new(3, 2), "*".to_string()),
            LexerToken::Operator(Span::new(4, 1), "(".to_string()),
            LexerToken::Numeric(Span::new(5, 1), "3".to_string()),
            LexerToken::Operator(Span::new(7, 1), "+".to_string()),
            LexerToken::Numeric(Span::new(9, 1), "2".to_string()),
            LexerToken::Operator(Span::new(10, 1), ")".to_string()),
            LexerToken::Operator(Span::new(11, 1), ")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_scientific_notation() {
        let input = "1e10 + 1.5e-3 - .5e+2 * 6.022e23";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Numeric(Span::new(0, 4), "1e10".to_string()),
            LexerToken::Operator(Span::new(5, 1), "+".to_string()),
            LexerToken::Numeric(Span::new(7, 6), "1.5e-3".to_string()),
            LexerToken::Operator(Span::new(14, 1), "-".to_string()),
            LexerToken::Numeric(Span::new(16, 5), ".5e+2".to_string()),
            LexerToken::Operator(Span::new(22, 1), "*".to_string()),
            LexerToken::Numeric(Span::new(24, 8), "6.022e23".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_scientific_notation_without_digits_falls_back_to_atom() {
        let input = "1e + 1e_value";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexerToken::Numeric(Span::new(0, 1), "1".to_string()),
            LexerToken::Identifier(Span::new(1, 1), "e".to_string()),
            LexerToken::Operator(Span::new(3, 1), "+".to_string()),
            LexerToken::Numeric(Span::new(5, 1), "1".to_string()),
            LexerToken::Identifier(Span::new(6, 7), "e_value".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn test_peak_and_next_index() {
        let input = "a + b * (c - d)";
        let mut lexer = Lexer::new(input).unwrap();

        // Peek at the first token
        let token = lexer.peek();
        assert_eq!(
            token,
            LexerToken::Identifier(Span::new(0, 1), "a".to_string())
        );

        // Consume the first token
        let token = lexer.next();
        assert_eq!(
            token,
            LexerToken::Identifier(Span::new(0, 1), "a".to_string())
        );

        // Peek at the next token
        let token = lexer.peek();
        assert_eq!(
            token,
            LexerToken::Operator(Span::new(2, 1), "+".to_string())
        );
    }

    #[test]
    fn test_invalid_characters_1() {
        for c in 0..=255 {
            let ch = c as u8 as char;
            if !ch.is_numeric()
                && !ch.is_lowercase()
                && !"+_-*/()[]<>=!&|%^.,\"".contains(ch)
                && !ch.is_whitespace()
            {
                let input = format!("a + b * (c - d) {ch} e");
                let result = Lexer::new(&input);
                assert!(result.is_err());
                let error = result.err().unwrap();
                assert_eq!(error.category, ExpressionCategory::Lexer);
                assert_eq!(
                    error.message,
                    format!("Invalid character in expression: '{ch}'")
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
            let input = format!("a + b * (c - d) {ch} e");
            let result = Lexer::new(&input);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.category, ExpressionCategory::Lexer);
            assert_eq!(
                error.message,
                format!("Invalid operator in expression: '{ch}'")
            );
        }
    }

    #[test]
    fn test_invalid_characters_5() {
        for c in "_".chars() {
            let ch = c as u8 as char;
            let input = format!("a + b * (c - d) {ch} e");
            let result = Lexer::new(&input);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.category, ExpressionCategory::Lexer);
            assert_eq!(
                error.message,
                format!("Invalid string in expression: '{ch}'")
            );
        }
    }

    #[test]
    fn tokenizes_a_simple_string_literal() {
        let input = "\"hello\"";
        let mut lexer = Lexer::new(input).unwrap();
        assert_eq!(
            lexer.next(),
            LexerToken::Text(Span::new(0, 7), "hello".to_string())
        );
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn tokenizes_a_path_like_string_literal() {
        let input = "\"Some/Path-1.txt\"";
        let mut lexer = Lexer::new(input).unwrap();
        assert_eq!(
            lexer.next(),
            LexerToken::Text(Span::new(0, 17), "Some/Path-1.txt".to_string())
        );
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn tokenizes_an_empty_string_literal() {
        let input = "\"\"";
        let mut lexer = Lexer::new(input).unwrap();
        assert_eq!(
            lexer.next(),
            LexerToken::Text(Span::new(0, 2), String::new())
        );
    }

    #[test]
    fn string_literal_used_within_a_larger_expression() {
        let input = "a + \"b\"";
        let mut lexer = Lexer::new(input).unwrap();
        assert_eq!(
            lexer.next(),
            LexerToken::Identifier(Span::new(0, 1), "a".to_string())
        );
        assert_eq!(
            lexer.next(),
            LexerToken::Operator(Span::new(2, 1), "+".to_string())
        );
        assert_eq!(
            lexer.next(),
            LexerToken::Text(Span::new(4, 3), "b".to_string())
        );
        assert_eq!(lexer.next(), LexerToken::EndOfInput);
    }

    #[test]
    fn unterminated_string_literal_returns_an_error() {
        let input = "\"unterminated";
        let result = Lexer::new(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.category, ExpressionCategory::Lexer);
        assert_eq!(error.message, "Unterminated string literal");
    }

    #[test]
    fn string_literal_rejects_non_ascii_character() {
        let input = "\"caf\u{e9}\"";
        let result = Lexer::new(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.category, ExpressionCategory::Lexer);
        assert_eq!(
            error.message,
            format!("Invalid character in string literal: '{}'", '\u{e9}')
        );
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
