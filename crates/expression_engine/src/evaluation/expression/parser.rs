use crate::expression::index::Index;
use crate::expression::lexer::{LexarToken, Lexer};
use crate::{ExpressionCategory, ExpressionError};
use std::fmt;

/// The result of parsing an expression: either a single atom (an identifier or literal) or a
/// compound expression consisting of an operator applied to one or more operands.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParserToken {
    /// An atomic value (e.g., a number or identifier), with the span it occupied in the
    /// original expression.
    Atom(String, Index),
    /// An operator applied to one or more operand expressions, with the span covering the
    /// whole application in the original expression.
    Operator(String, Vec<ParserToken>, Index),
}

impl ParserToken {
    /// Returns the span this token occupies in the original expression.
    pub(crate) fn span(&self) -> Index {
        match self {
            ParserToken::Atom(_, span) => *span,
            ParserToken::Operator(_, _, span) => *span,
        }
    }
}

impl fmt::Display for ParserToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserToken::Atom(i, _) => write!(f, "{}", i),
            ParserToken::Operator(head, rest, _) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

/// Returns the span of `token` in the original expression, or `None` for `EndOfInput`
/// (which has no position).
fn token_span(token: &LexarToken) -> Option<Index> {
    match token {
        LexarToken::Atom((index, _)) => Some(*index),
        LexarToken::Operator((index, _)) => Some(*index),
        LexarToken::EndOfInput => None,
    }
}

/// Parses the tokens produced by `lexer` into a fully parenthesized expression tree,
/// respecting operator precedence and associativity.
pub(crate) fn parse(lexer: &Lexer) -> Result<ParserToken, ExpressionError> {
    let mut lexer = lexer.clone();
    let result = expr_bp(&mut lexer, 0)?;

    match lexer.peek() {
        LexarToken::EndOfInput => Ok(result),
        t => Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!(
                "Invalid expression: expected end of input, found {}",
                describe_token(&t)
            ),
            lexer.source(),
            token_span(&t).map(|s| vec![s]).unwrap_or_default(),
        )),
    }
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<ParserToken, ExpressionError> {
    let mut lhs = match lexer.next() {
        LexarToken::Atom((index, it)) => ParserToken::Atom(it, index),
        LexarToken::Operator((_, op)) if op == "(" => {
            let lhs = expr_bp(lexer, 0)?;
            expect_operator(lexer, ")")?;
            lhs
        }
        LexarToken::Operator((index, op)) => {
            let ((), r_bp) = prefix_binding_power(&op, &index, lexer.source())?;
            let rhs = expr_bp(lexer, r_bp)?;
            let span = index.join(&rhs.span());
            ParserToken::Operator(op, vec![rhs], span)
        }
        t => {
            return Err(ExpressionError::new(
                ExpressionCategory::Parse,
                format!(
                    "Invalid expression: expected an atom or a prefix operator, found {}",
                    describe_token(&t)
                ),
                lexer.source(),
                token_span(&t).map(|s| vec![s]).unwrap_or_default(),
            ));
        }
    };

    loop {
        let (op_index, op) = match lexer.peek() {
            LexarToken::EndOfInput => break,
            LexarToken::Operator((index, op)) => (index, op),
            t => {
                return Err(ExpressionError::new(
                    ExpressionCategory::Parse,
                    format!(
                        "Invalid expression: expected an operator, found {}",
                        describe_token(&t)
                    ),
                    lexer.source(),
                    token_span(&t).map(|s| vec![s]).unwrap_or_default(),
                ));
            }
        };

        if let Some((l_bp, ())) = postfix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == "[" {
                let rhs = expr_bp(lexer, 0)?;
                expect_operator(lexer, "]")?;
                let span = lhs.span().join(&rhs.span());
                ParserToken::Operator(op, vec![lhs, rhs], span)
            } else if op == "(" {
                let (name, name_index) = match lhs {
                    ParserToken::Atom(name, name_index) => (name, name_index),
                    other => {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Parse,
                            format!(
                                "Invalid expression: function calls require a function name, found {}",
                                other
                            ),
                            lexer.source(),
                            vec![other.span()],
                        ));
                    }
                };
                let arguments = parse_call_arguments(lexer)?;
                let span = arguments
                    .last()
                    .map(|a| name_index.join(&a.span()))
                    .unwrap_or(name_index);
                ParserToken::Operator(name, arguments, span)
            } else {
                let span = op_index.join(&lhs.span());
                ParserToken::Operator(op, vec![lhs], span)
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp)?;
            let span = lhs.span().join(&rhs.span());
            lhs = ParserToken::Operator(op, vec![lhs, rhs], span);
            continue;
        }

        break;
    }

    Ok(lhs)
}

/// Parses a comma-separated list of call arguments, up to (but not including) the closing `)`.
fn parse_call_arguments(lexer: &mut Lexer) -> Result<Vec<ParserToken>, ExpressionError> {
    let mut arguments = Vec::new();

    if let LexarToken::Operator((_, op)) = lexer.peek() {
        if op == ")" {
            expect_operator(lexer, ")")?;
            return Ok(arguments);
        }
    }

    loop {
        arguments.push(expr_bp(lexer, 0)?);
        match lexer.peek() {
            LexarToken::Operator((_, comma)) if comma == "," => {
                lexer.next();
            }
            _ => break,
        }
    }

    expect_operator(lexer, ")")?;
    Ok(arguments)
}

/// Consumes the next token from `lexer`, returning an error if it isn't the expected operator.
fn expect_operator(lexer: &mut Lexer, expected: &str) -> Result<(), ExpressionError> {
    match lexer.next() {
        LexarToken::Operator((_, op)) if op == expected => Ok(()),
        t => Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!(
                "Invalid expression: expected operator '{}', found {}",
                expected,
                describe_token(&t)
            ),
            lexer.source(),
            token_span(&t).map(|s| vec![s]).unwrap_or_default(),
        )),
    }
}

/// Returns a human-readable description of `token`, suitable for use in error messages.
fn describe_token(token: &LexarToken) -> String {
    match token {
        LexarToken::Atom((_, s)) => format!("atom '{}'", s),
        LexarToken::Operator((_, s)) => format!("operator '{}'", s),
        LexarToken::EndOfInput => "end of input".to_string(),
    }
}

fn prefix_binding_power(
    op: &str,
    index: &Index,
    source: &str,
) -> Result<((), u8), ExpressionError> {
    match op {
        "+" | "-" => Ok(((), 19)),
        "!" => Ok(((), 19)),
        _ => Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!("Invalid prefix operator in expression: '{}'", op),
            source,
            vec![*index],
        )),
    }
}

fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
    let res = match op {
        "[" => (21, ()),
        "(" => (21, ()),
        _ => return None,
    };
    Some(res)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn expr(s: &str) -> Result<ParserToken, ExpressionError> {
        let lexer = Lexer::new(s)?;
        parse(&lexer)
    }

    #[test]
    fn basic_test() {
        let s = expr("1").unwrap();
        assert_eq!(s.to_string(), "1");

        let s = expr("1 + 2 * 3").unwrap();
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

        let s = expr("1.5 + 2.5 * 32").unwrap();
        assert_eq!(s.to_string(), "(+ 1.5 (* 2.5 32))");

        let s = expr("a + b * c * d + e").unwrap();
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

        let s = expr("--1 * 2").unwrap();
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

        let s = expr("!true").unwrap();
        assert_eq!(s.to_string(), "(! true)");

        let s = expr("!!true").unwrap();
        assert_eq!(s.to_string(), "(! (! true))");

        let s = expr("(((0)))").unwrap();
        assert_eq!(s.to_string(), "0");

        let s = expr("x[0][1]").unwrap();
        assert_eq!(s.to_string(), "([ ([ x 0) 1)");

        let s = expr("x[0+ 1][1]").unwrap();
        assert_eq!(s.to_string(), "([ ([ x (+ 0 1)) 1)");

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.to_string(), "(x 1 2 3)");
    }

    #[test]
    fn function_call_operators() {
        let s = expr("x()").unwrap();
        assert_eq!(s.to_string(), "(x)");

        let s = expr("x(1)").unwrap();
        assert_eq!(s.to_string(), "(x 1)");

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.to_string(), "(x 1 2 3)");

        // arguments can be arbitrary expressions
        let s = expr("f(a + b, c * d)").unwrap();
        assert_eq!(s.to_string(), "(f (+ a b) (* c d))");

        // calls can be nested, and combined with other operators
        let s = expr("f(g(a), h())").unwrap();
        assert_eq!(s.to_string(), "(f (g a) (h))");

        let s = expr("f(a) + g(b)").unwrap();
        assert_eq!(s.to_string(), "(+ (f a) (g b))");
    }

    #[test]
    fn comparison_operators() {
        let s = expr("a == b").unwrap();
        assert_eq!(s.to_string(), "(== a b)");

        let s = expr("a != b").unwrap();
        assert_eq!(s.to_string(), "(!= a b)");

        let s = expr("a < b").unwrap();
        assert_eq!(s.to_string(), "(< a b)");

        let s = expr("a <= b").unwrap();
        assert_eq!(s.to_string(), "(<= a b)");

        let s = expr("a > b").unwrap();
        assert_eq!(s.to_string(), "(> a b)");

        let s = expr("a >= b").unwrap();
        assert_eq!(s.to_string(), "(>= a b)");

        // comparisons are left-associative and lower precedence than arithmetic
        let s = expr("a + 1 >= b - 1").unwrap();
        assert_eq!(s.to_string(), "(>= (+ a 1) (- b 1))");

        let s = expr("a < b < c").unwrap();
        assert_eq!(s.to_string(), "(< (< a b) c)");
    }

    #[test]
    fn logical_operators() {
        let s = expr("a && b").unwrap();
        assert_eq!(s.to_string(), "(&& a b)");

        let s = expr("a || b").unwrap();
        assert_eq!(s.to_string(), "(|| a b)");

        // && binds tighter than ||
        let s = expr("a || b && c").unwrap();
        assert_eq!(s.to_string(), "(|| a (&& b c))");

        // comparisons bind tighter than && / ||
        let s = expr("a == b && c != d").unwrap();
        assert_eq!(s.to_string(), "(&& (== a b) (!= c d))");

        let s = expr(
            "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4",
        )
        .unwrap();
        assert_eq!(
            s.to_string(),
            "(|| (|| (&& (>= p_value1 p_value2) (!= p_value3 p_value4)) (<= p_value1 p_value2)) (== p_value3 p_value4))"
        );
    }

    #[test]
    fn modulo_and_power_operators() {
        let s = expr("a % b").unwrap();
        assert_eq!(s.to_string(), "(% a b)");

        // % has same precedence as * and /, higher than +
        let s = expr("a + b % c").unwrap();
        assert_eq!(s.to_string(), "(+ a (% b c))");

        let s = expr("a ^ b").unwrap();
        assert_eq!(s.to_string(), "(^ a b)");

        // ^ (power) binds tighter than *, /, %, and +, -
        let s = expr("a + b ^ c * d").unwrap();
        assert_eq!(s.to_string(), "(+ a (* (^ b c) d))");

        let s = expr("2 * 3 ^ 2").unwrap();
        assert_eq!(s.to_string(), "(* 2 (^ 3 2))");

        // ^ is right-associative
        let s = expr("a ^ b ^ c").unwrap();
        assert_eq!(s.to_string(), "(^ a (^ b c))");

        // ^ binds tighter than comparison and &&/||
        let s = expr("a == b ^ c && d").unwrap();
        assert_eq!(s.to_string(), "(&& (== a (^ b c)) d)");
    }

    /// Asserts that parsing `s` fails with an `ExpressionCategory::Parse` error whose message
    /// contains `expected_message`.
    fn assert_parse_error(s: &str, expected_message: &str) {
        let err = expr(s).unwrap_err();
        let err = err.to_string();
        assert!(
            err.starts_with("[Parse]"),
            "expected a Parse error for input {:?}, got: {}",
            s,
            err
        );
        assert!(
            err.contains(expected_message),
            "expected error message for input {:?} to contain {:?}, got: {}",
            s,
            expected_message,
            err
        );
    }

    #[test]
    fn missing_operand_at_start_of_expression() {
        // Empty input: an operand is expected but only EndOfInput is available.
        assert_parse_error(
            "",
            "expected an atom or a prefix operator, found end of input",
        );
    }

    #[test]
    fn missing_operand_after_infix_operator() {
        // After consuming `1` and `+`, the right-hand side is missing.
        assert_parse_error(
            "1+",
            "expected an atom or a prefix operator, found end of input",
        );
    }

    #[test]
    fn missing_operator_between_atoms() {
        // Two atoms in a row with no operator between them.
        assert_parse_error("1 2", "expected an operator, found atom '2'");
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
        assert_eq!(s.to_string(), "(! a)");

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

    #[test]
    fn parse_error_renders_underline_beneath_offending_token() {
        // The stray `2` is the offending token; the underline points at it.
        let err = expr("1 2").unwrap_err();
        assert_eq!(
            err.to_string(),
            "[Parse] Invalid expression: expected an operator, found atom '2'\n1 2\n  ~\n"
        );
    }
}
