use crate::evaluation::expression::ast::lexer::{Lexer, LexerToken};
use crate::{ExpressionCategory, ExpressionError};
use message::span::{Span, SpanSet};
use shareable_string::ShareableString;
use std::fmt;

/// The result of parsing an expression: either a single identifier, a single numeric value,
/// or an operator applied to one or more operand expressions.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParserToken {
    /// An identifier (e.g., a variable or function name).
    Identifier(Span, String),
    /// A numeric value (e.g., `123`, `45.67`, `.89`, `0.001`, `1e10`, `1.5e-3`, `.5e+2`).
    Numeric(Span, String),
    /// An operator applied to one or more operand expressions.
    Operator(Span, String, Vec<ParserToken>),
    /// A quoted string literal (e.g., `"some/path.txt"`), holding the unquoted contents.
    Text(Span, String),
}

impl fmt::Display for ParserToken {
    #[hotpath::measure]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserToken::Identifier(i, value) | ParserToken::Numeric(i, value) => {
                write!(f, "{{{i}}}{value}")
            }
            ParserToken::Text(i, value) => write!(f, "{{{i}}}\"{value}\""),
            ParserToken::Operator(i, op, rest) => {
                write!(f, "({{{i}}}{op}")?;
                for s in rest {
                    write!(f, " {s}")?;
                }
                write!(f, ")")
            }
        }
    }
}

/// The result of parsing a complete expression into a Pratt-style token tree.
#[derive(Debug)]
pub(crate) struct Parser {
    /// The root token of the parsed expression tree.
    token: ParserToken,
    /// The original expression source text, retained for error reporting.
    source: ShareableString,
}

impl Parser {
    /// Creates a new `Parser` by parsing the expression from the given `Lexer`.
    #[hotpath::measure]
    pub(crate) fn new(lexer: &Lexer) -> Result<Parser, ExpressionError> {
        let mut lexer = lexer.clone();
        let source = lexer.source().clone();
        let result = Self::expr_bp(&mut lexer, 0)?;

        match lexer.peek() {
            LexerToken::EndOfInput => Ok(Parser {
                token: result,
                source,
            }),
            t @ (LexerToken::Identifier(..)
            | LexerToken::Numeric(..)
            | LexerToken::Operator(..)
            | LexerToken::Text(..)) => {
                let index_set = Self::token_index_set(&t);

                Err(ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    format!(
                        "Invalid expression: expected end of input, found {}",
                        Self::describe_token(&t),
                    ),
                    source,
                    index_set,
                ))
            }
        }
    }

    /// Returns a reference to the root token of the parsed expression tree.
    pub(crate) const fn get_token(&self) -> &ParserToken {
        &self.token
    }

    /// Returns a reference to the original source text used to build this parser.
    pub(crate) const fn get_source(&self) -> &ShareableString {
        &self.source
    }

    /// Recursive Pratt-expression parser.
    ///
    /// Parses tokens from `lexer` with a minimum binding power of `min_bp`,
    /// returning the root [`ParserToken`] for the sub-expression.
    ///
    /// # Errors
    ///
    /// Returns an error on unexpected tokens or empty input.
    #[hotpath::measure]
    fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<ParserToken, ExpressionError> {
        let mut lhs = match lexer.next() {
            LexerToken::Identifier(index, value) => ParserToken::Identifier(index, value),
            LexerToken::Text(index, value) => ParserToken::Text(index, value),
            LexerToken::Numeric(index, value) => ParserToken::Numeric(index, value),
            LexerToken::Operator(_index, op) if op == "(" => {
                let lhs = Self::expr_bp(lexer, 0)?;
                Self::expect_operator(lexer, ")")?;
                lhs
            }
            LexerToken::Operator(index, op) => {
                let ((), r_bp) = Self::prefix_binding_power(&op, index, lexer.source())?;
                let rhs = Self::expr_bp(lexer, r_bp)?;
                ParserToken::Operator(index, op, vec![rhs])
            }
            LexerToken::EndOfInput => {
                let index = Span::new(lexer.source().as_str().len(), 0);
                return Err(ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    format!(
                        "Invalid expression: expected an identifier, a number, or a prefix operator, found {}",
                        Self::describe_token(&LexerToken::EndOfInput)
                    ),
                    lexer.source(),
                    SpanSet::from_span(index),
                ));
            }
        };

        loop {
            let (op_index, op) = match lexer.peek() {
                LexerToken::EndOfInput => break,
                LexerToken::Operator(index, value) => (index, value),
                LexerToken::Identifier(index, value) => {
                    return Err(ExpressionError::new_complex(
                        ExpressionCategory::Parse,
                        format!(
                            "Invalid expression: expected an operator, found {}",
                            Self::describe_token(&LexerToken::Identifier(index, value))
                        ),
                        lexer.source(),
                        SpanSet::from_span(index),
                    ));
                }
                LexerToken::Numeric(index, value) => {
                    return Err(ExpressionError::new_complex(
                        ExpressionCategory::Parse,
                        format!(
                            "Invalid expression: expected an operator, found {}",
                            Self::describe_token(&LexerToken::Numeric(index, value))
                        ),
                        lexer.source(),
                        SpanSet::from_span(index),
                    ));
                }
                LexerToken::Text(index, value) => {
                    return Err(ExpressionError::new_complex(
                        ExpressionCategory::Parse,
                        format!(
                            "Invalid expression: expected an operator, found {}",
                            Self::describe_token(&LexerToken::Text(index, value))
                        ),
                        lexer.source(),
                        SpanSet::from_span(index),
                    ));
                }
            };

            if let Some((l_bp, ())) = Self::postfix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }
                lexer.next();

                lhs = if op == "[" {
                    let rhs = Self::expr_bp(lexer, 0)?;
                    Self::expect_operator(lexer, "]")?;
                    ParserToken::Operator(op_index, op, vec![lhs, rhs])
                } else if op == "(" {
                    let (name_index, name) = match lhs {
                        ParserToken::Identifier(index, name) => (index, name),
                        ParserToken::Numeric(_, name) => {
                            return Err(ExpressionError::new_complex(
                                ExpressionCategory::Parse,
                                format!(
                                    "Invalid expression: function calls require a function name, found number {name}"
                                ),
                                lexer.source(),
                                SpanSet::from_span(op_index),
                            ));
                        }
                        ParserToken::Text(index, name) => {
                            return Err(ExpressionError::new_complex(
                                ExpressionCategory::Parse,
                                format!(
                                    "Invalid expression: function calls require a function name, found \"{name}\""
                                ),
                                lexer.source(),
                                SpanSet::from_span(index),
                            ));
                        }
                        ParserToken::Operator(_, name, ..) => {
                            return Err(ExpressionError::new_complex(
                                ExpressionCategory::Parse,
                                format!(
                                    "Invalid expression: function calls require a function name, found expression starting with operator {name}"
                                ),
                                lexer.source(),
                                SpanSet::from_span(op_index),
                            ));
                        }
                    };
                    let arguments = Self::parse_call_arguments(lexer)?;
                    ParserToken::Operator(name_index, name, arguments)
                } else {
                    ParserToken::Operator(op_index, op, vec![lhs])
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }
                lexer.next();

                let rhs = Self::expr_bp(lexer, r_bp)?;
                lhs = ParserToken::Operator(op_index, op, vec![lhs, rhs]);
                continue;
            }

            break;
        }

        Ok(lhs)
    }

    /// Parses a comma-separated list of call arguments, up to (but not including) the closing `)`.
    #[hotpath::measure]
    fn parse_call_arguments(lexer: &mut Lexer) -> Result<Vec<ParserToken>, ExpressionError> {
        let mut arguments = Vec::new();
        if let LexerToken::Operator(_index, value) = lexer.peek() {
            if value == ")" {
                lexer.next();
                return Ok(arguments);
            }
        }

        loop {
            arguments.push(Self::expr_bp(lexer, 0)?);
            match lexer.peek() {
                LexerToken::Operator(_index, value) if value == "," => {
                    lexer.next();
                }
                LexerToken::Identifier(..)
                | LexerToken::Numeric(..)
                | LexerToken::Operator(..)
                | LexerToken::Text(..)
                | LexerToken::EndOfInput => break,
            }
        }

        Self::expect_operator(lexer, ")")?;
        Ok(arguments)
    }

    /// Consumes the next token from `lexer`, returning an error if it isn't the expected operator.
    #[hotpath::measure]
    fn expect_operator(lexer: &mut Lexer, expected: &str) -> Result<(), ExpressionError> {
        match lexer.next() {
            LexerToken::Operator(_index, value) if value == expected => Ok(()),
            t @ (LexerToken::Identifier(..)
            | LexerToken::Numeric(..)
            | LexerToken::Operator(..)
            | LexerToken::Text(..)
            | LexerToken::EndOfInput) => {
                let index_set = Self::token_index_set(&t);
                Err(ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    format!(
                        "Invalid expression: expected operator '{}', found {}",
                        expected,
                        Self::describe_token(&t)
                    ),
                    lexer.source(),
                    index_set,
                ))
            }
        }
    }

    /// Returns the set of source indices covered by `lexer_token`, or an empty set for
    /// `LexerToken::EndOfInput`, which doesn't correspond to any position in the source.
    #[hotpath::measure]
    fn token_index_set(lexer_token: &LexerToken) -> SpanSet {
        match lexer_token {
            LexerToken::Identifier(index, _)
            | LexerToken::Numeric(index, _)
            | LexerToken::Text(index, _)
            | LexerToken::Operator(index, _) => SpanSet::from_span(*index),
            LexerToken::EndOfInput => SpanSet::new(),
        }
    }

    /// Returns a human-readable description of `token`, suitable for use in error messages.
    #[hotpath::measure]
    fn describe_token(token: &LexerToken) -> String {
        match token {
            LexerToken::Identifier(_index, value) => format!("identifier '{value}'"),
            LexerToken::Numeric(_index, value) => format!("number '{value}'"),
            LexerToken::Text(_index, value) => format!("text \"{value}\""),
            LexerToken::Operator(_index, value) => format!("operator '{value}'"),
            LexerToken::EndOfInput => "end of input".to_string(),
        }
    }

    /// Returns the prefix binding power for the given operator, or an error if the operator
    /// is not a valid prefix operator.
    ///
    /// # Errors
    ///
    /// Returns an error if `op` is not a valid prefix operator.
    #[hotpath::measure]
    fn prefix_binding_power(
        op: &str,
        index: Span,
        source: &ShareableString,
    ) -> Result<((), u8), ExpressionError> {
        match op {
            "+" | "-" | "!" => Ok(((), 19)),
            _ => Err(ExpressionError::new_complex(
                ExpressionCategory::Parse,
                format!("Invalid prefix operator in expression: '{op}'"),
                source,
                SpanSet::from_span(index),
            )),
        }
    }

    /// Returns the postfix binding power for the given operator, or `None` if the operator
    /// is not a valid postfix operator.
    #[hotpath::measure]
    fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
        let res = match op {
            "[" | "(" => (21, ()),
            _ => return None,
        };
        Some(res)
    }

    /// Returns the left and right binding powers for the given infix operator, or `None` if
    /// the operator is not a valid infix operator.
    #[hotpath::measure]
    fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
        let res = match op {
            "=" => (2, 1),
            "||" => (5, 6),
            "&&" => (7, 8),
            "==" | "!=" => (11, 12),
            "<" | "<=" | ">" | ">=" => (13, 14),
            "+" | "-" => (15, 16),
            "*" | "/" | "%" => (17, 18),
            "^" => (20, 19),
            _ => return None,
        };
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expr(s: &str) -> Result<Parser, ExpressionError> {
        let lexer = Lexer::new(s)?;
        Parser::new(&lexer)
    }

    #[test]
    fn basic_test() {
        let s = expr("1").unwrap();
        assert_eq!(s.get_token().to_string(), "{0:1}1");

        let s = expr("1 + 2 * 3").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}+ {0:1}1 ({6:7}* {4:5}2 {8:9}3))"
        );

        let s = expr("1.5 + 2.5 * 32").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({4:5}+ {0:3}1.5 ({10:11}* {6:9}2.5 {12:14}32))"
        );

        let s = expr("a + b * c * d + e").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({14:15}+ ({2:3}+ {0:1}a ({10:11}* ({6:7}* {4:5}b {8:9}c) {12:13}d)) {16:17}e)"
        );

        let s = expr("--1 * 2").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({4:5}* ({0:1}- ({1:2}- {2:3}1)) {6:7}2)"
        );

        let s = expr("!true").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}! {1:5}true)");

        let s = expr("!!true").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}! ({1:2}! {2:6}true))");

        let s = expr("(((0)))").unwrap();
        assert_eq!(s.get_token().to_string(), "{3:4}0");

        let s = expr("x[0][1]").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({4:5}[ ({1:2}[ {0:1}x {2:3}0) {5:6}1)"
        );

        let s = expr("x[0+ 1][1]").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({7:8}[ ({1:2}[ {0:1}x ({3:4}+ {2:3}0 {5:6}1)) {8:9}1)"
        );

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x {2:3}1 {4:5}2 {6:7}3)");
    }

    #[test]
    fn function_call_operators() {
        let s = expr("x()").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x)");

        let s = expr("x(1)").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x {2:3}1)");

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x {2:3}1 {4:5}2 {6:7}3)");

        // arguments can be arbitrary expressions
        let s = expr("f(a + b, c * d)").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({0:1}f ({4:5}+ {2:3}a {6:7}b) ({11:12}* {9:10}c {13:14}d))"
        );

        // calls can be nested and combined with other operators
        let s = expr("f(g(a), h())").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({0:1}f ({2:3}g {4:5}a) ({8:9}h))"
        );

        let s = expr("f(a) + g(b)").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({5:6}+ ({0:1}f {2:3}a) ({7:8}g {9:10}b))"
        );
    }

    #[test]
    fn comparison_operators() {
        let s = expr("a == b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}== {0:1}a {5:6}b)");

        let s = expr("a != b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}!= {0:1}a {5:6}b)");

        let s = expr("a < b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}< {0:1}a {4:5}b)");

        let s = expr("a <= b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}<= {0:1}a {5:6}b)");

        let s = expr("a > b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}> {0:1}a {4:5}b)");

        let s = expr("a >= b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}>= {0:1}a {5:6}b)");

        // comparisons are left-associative and lower precedence than arithmetic
        let s = expr("a + 1 >= b - 1").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({6:8}>= ({2:3}+ {0:1}a {4:5}1) ({11:12}- {9:10}b {13:14}1))"
        );

        let s = expr("a < b < c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({6:7}< ({2:3}< {0:1}a {4:5}b) {8:9}c)"
        );
    }

    #[test]
    fn logical_operators() {
        let s = expr("a && b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}&& {0:1}a {5:6}b)");

        let s = expr("a || b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}|| {0:1}a {5:6}b)");

        // && binds tighter than ||
        let s = expr("a || b && c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:4}|| {0:1}a ({7:9}&& {5:6}b {10:11}c))"
        );

        // comparisons bind tighter than && / ||
        let s = expr("a == b && c != d").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({7:9}&& ({2:4}== {0:1}a {5:6}b) ({12:14}!= {10:11}c {15:16}d))"
        );

        let s = expr(
            "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4",
        )
            .unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({69:71}|| ({45:47}|| ({21:23}&& ({9:11}>= {0:8}p_value1 {12:20}p_value2) ({33:35}!= {24:32}p_value3 {36:44}p_value4)) ({57:59}<= {48:56}p_value1 {60:68}p_value2)) ({81:83}== {72:80}p_value3 {84:92}p_value4))"
        );
    }

    #[test]
    fn modulo_and_power_operators() {
        let s = expr("a % b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}% {0:1}a {4:5}b)");

        // % has same precedence as * and /, higher than +
        let s = expr("a + b % c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}+ {0:1}a ({6:7}% {4:5}b {8:9}c))"
        );

        let s = expr("a ^ b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}^ {0:1}a {4:5}b)");

        // ^ (power) binds tighter than *, /, %, and +, -
        let s = expr("a + b ^ c * d").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}+ {0:1}a ({10:11}* ({6:7}^ {4:5}b {8:9}c) {12:13}d))"
        );

        let s = expr("2 * 3 ^ 2").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}* {0:1}2 ({6:7}^ {4:5}3 {8:9}2))"
        );

        // ^ is right-associative
        let s = expr("a ^ b ^ c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}^ {0:1}a ({6:7}^ {4:5}b {8:9}c))"
        );

        // ^ binds tighter than comparison and &&/||
        let s = expr("a == b ^ c && d").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({11:13}&& ({2:4}== {0:1}a ({7:8}^ {5:6}b {9:10}c)) {14:15}d)"
        );
    }

    /// Asserts that parsing `s` fails with an `ExpressionCategory::Parse` error whose message
    /// contains `expected_message`.
    fn assert_parse_error(s: &str, expected_message: &str) {
        let err = expr(s).unwrap_err();
        let err = err.to_string();
        assert!(
            err.starts_with("[Parse]"),
            "expected a Parse error for input {s:?}, got: {err}"
        );
        assert!(
            err.contains(expected_message),
            "expected error message for input {s:?} to contain {expected_message:?}, got: {err}"
        );
    }

    #[test]
    fn missing_operand_at_start_of_expression() {
        // Empty input: an operand is expected but only EndOfInput is available.
        assert_parse_error(
            "",
            "expected an identifier, a number, or a prefix operator, found end of input",
        );
    }

    #[test]
    fn missing_operand_after_infix_operator() {
        // After consuming `1` and `+`, the right-hand side is missing.
        assert_parse_error(
            "1+",
            "expected an identifier, a number, or a prefix operator, found end of input",
        );
    }

    #[test]
    fn missing_operator_between_atoms() {
        // Two atoms in a row with no operator between them.
        assert_parse_error("1 2", "expected an operator, found number '2'");
    }

    #[test]
    fn unclosed_parenthesis() {
        // Missing closing `)`, so `expect_operator` finds EndOfInput instead.
        assert_parse_error("(1", "expected operator ')', found end of input");
    }

    #[test]
    fn mismatched_closing_bracket() {
        // Closing token doesn't match the expected `)`.
        assert_parse_error("(1]", "expected operator ')', found operator ']'");
    }

    #[test]
    fn bad_prefix_operator() {
        // `*`, `/`, and `^` are not valid prefix operators.
        assert_parse_error("*2", "Invalid prefix operator in expression: '*'");
        assert_parse_error("/2", "Invalid prefix operator in expression: '/'");
        assert_parse_error("^2", "Invalid prefix operator in expression: '^'");
    }

    #[test]
    fn not_operator_is_prefix_only() {
        // `!` is a valid prefix (logical not) operator.
        let s = expr("!a").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}! {1:2}a)");

        // `!` is not a valid postfix operator, so trailing `!` is leftover, invalid input.
        assert_parse_error(
            "a!",
            "Invalid expression: expected end of input, found operator '!'",
        );
        assert_parse_error(
            "9!",
            "Invalid expression: expected end of input, found operator '!'",
        );
    }
}
