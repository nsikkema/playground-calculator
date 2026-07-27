use crate::expression::index::Index;
use crate::expression::parser::ParserToken;
use crate::{ExpressionCategory, ExpressionError};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(value) => write!(f, "{}", value),
            Literal::Float(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Boolean(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    Negate,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
}

impl fmt::Display for Operators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operators::Add => "+",
            Operators::Subtract => "-",
            Operators::Multiply => "*",
            Operators::Divide => "/",
            Operators::Modulus => "%",
            Operators::Power => "^",
            Operators::Negate => "-",
            Operators::Equal => "==",
            Operators::NotEqual => "!=",
            Operators::LessThan => "<",
            Operators::LessThanOrEqual => "<=",
            Operators::GreaterThan => ">",
            Operators::GreaterThanOrEqual => ">=",
            Operators::And => "&&",
            Operators::Or => "||",
            Operators::Not => "!",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Literal(Literal, Index),
    BinaryOperation {
        left: Box<Expression>,
        operator: Operators,
        right: Box<Expression>,
        span: Index,
    },
    UnaryOperation {
        operator: Operators,
        operand: Box<Expression>,
        span: Index,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
        span: Index,
    },
    Index {
        name: String,
        index: Vec<Expression>,
        span: Index,
    },
}

impl Expression {
    /// Returns the span this expression occupies in the original source text.
    pub(crate) fn span(&self) -> Index {
        match self {
            Expression::Literal(_, span) => *span,
            Expression::BinaryOperation { span, .. } => *span,
            Expression::UnaryOperation { span, .. } => *span,
            Expression::FunctionCall { span, .. } => *span,
            Expression::Index { span, .. } => *span,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal, _) => write!(f, "{}", literal),
            Expression::BinaryOperation {
                left, operator, right, ..
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::UnaryOperation { operator, operand, .. } => {
                write!(f, "({}{})", operator, operand)
            }
            Expression::FunctionCall { name, arguments, .. } => {
                let args = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}({})", name, args)
            }
            Expression::Index { name, index, .. } => {
                write!(f, "{}", name)?;
                for idx in index {
                    write!(f, "[{}]", idx)?;
                }
                Ok(())
            }
        }
    }
}

/// Translates a binary `ParserToken::Operator` into a `BinaryOperation` expression.
fn translate_binary(
    operands: &[ParserToken],
    operator: Operators,
    source: &str,
    span: Index,
) -> Result<Expression, ExpressionError> {
    Ok(Expression::BinaryOperation {
        left: Box::new(translate(operands[0].clone(), source)?),
        operator,
        right: Box::new(translate(operands[1].clone(), source)?),
        span,
    })
}

/// Translates a unary `ParserToken::Operator` into a `UnaryOperation` expression.
fn translate_unary(
    operands: &[ParserToken],
    operator: Operators,
    source: &str,
    span: Index,
) -> Result<Expression, ExpressionError> {
    Ok(Expression::UnaryOperation {
        operator,
        operand: Box::new(translate(operands[0].clone(), source)?),
        span,
    })
}

/// Translates a `ParserToken::Operator("[", ...)` into an `Index` expression.
///
/// When the collection being indexed is itself an `Index` expression (i.e. this is a chained
/// index such as `arr[0][1]`), the new index is appended to the existing `Index`'s vector of
/// indices rather than wrapping it in another `Index` expression.
fn translate_index(
    operands: &[ParserToken],
    source: &str,
    span: Index,
) -> Result<Expression, ExpressionError> {
    let target = translate(operands[0].clone(), source)?;
    let new_index = translate(operands[1].clone(), source)?;

    match target {
        Expression::Index { name, mut index, .. } => {
            index.push(new_index);
            Ok(Expression::Index { name, index, span })
        }
        other => Ok(Expression::Index {
            name: other.to_string(),
            index: vec![new_index],
            span,
        }),
    }
}

/// Translates a `ParserToken::Operator` whose head is a function name (rather than a known
/// operator symbol) into a `FunctionCall` expression.
fn translate_call(
    name: String,
    arguments: Vec<ParserToken>,
    source: &str,
    span: Index,
) -> Result<Expression, ExpressionError> {
    Ok(Expression::FunctionCall {
        name,
        arguments: arguments
            .into_iter()
            .map(|a| translate(a, source))
            .collect::<Result<Vec<_>, _>>()?,
        span,
    })
}

/// Returns whether `name` looks like a function/variable name (i.e. what the lexer would have
/// produced as an `Atom`), as opposed to an operator symbol such as `+` or `!`.
fn is_function_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_')
}

/// Returns whether `value` looks like a numeric literal (i.e. what the lexer would have
/// produced as an `Atom` starting with a digit or a `.`), as opposed to a variable/function
/// name.
fn is_numeric_literal(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|c| c.is_numeric() || c == '.')
}

/// Translates a `ParserToken::Atom` into either a numeric `Literal` expression (when the atom
/// looks like a number) or a `Variable` expression (otherwise).
fn translate_atom(value: String, span: Index, source: &str) -> Result<Expression, ExpressionError> {
    if !is_numeric_literal(&value) {
        if let Ok(boolean) = value.parse::<bool>() {
            return Ok(Expression::Literal(Literal::Boolean(boolean), span));
        }

        return Ok(Expression::Literal(Literal::String(value), span));
    }

    if let Ok(integer) = value.parse::<i64>() {
        return Ok(Expression::Literal(Literal::Integer(integer), span));
    }

    if let Ok(float) = value.parse::<f64>() {
        return Ok(Expression::Literal(Literal::Float(float), span));
    }

    Err(ExpressionError::new(
        ExpressionCategory::Parse,
        format!("Invalid numeric literal: {}", value),
        source,
        vec![span],
    ))
}

pub(crate) fn translate(
    parser_token: ParserToken,
    source: &str,
) -> Result<Expression, ExpressionError> {
    match parser_token {
        ParserToken::Atom(value, span) => translate_atom(value, span, source),
        ParserToken::Operator(op, operands, span) => match (op.as_str(), operands.len()) {
            ("+", 1) => translate(operands[0].clone(), source),
            ("-", 1) => translate_unary(&operands, Operators::Negate, source, span),
            ("!", 1) => translate_unary(&operands, Operators::Not, source, span),
            ("+", 2) => translate_binary(&operands, Operators::Add, source, span),
            ("-", 2) => translate_binary(&operands, Operators::Subtract, source, span),
            ("*", 2) => translate_binary(&operands, Operators::Multiply, source, span),
            ("/", 2) => translate_binary(&operands, Operators::Divide, source, span),
            ("%", 2) => translate_binary(&operands, Operators::Modulus, source, span),
            ("^", 2) => translate_binary(&operands, Operators::Power, source, span),
            ("==", 2) => translate_binary(&operands, Operators::Equal, source, span),
            ("!=", 2) => translate_binary(&operands, Operators::NotEqual, source, span),
            ("<", 2) => translate_binary(&operands, Operators::LessThan, source, span),
            ("<=", 2) => translate_binary(&operands, Operators::LessThanOrEqual, source, span),
            (">", 2) => translate_binary(&operands, Operators::GreaterThan, source, span),
            (">=", 2) => translate_binary(&operands, Operators::GreaterThanOrEqual, source, span),
            ("&&", 2) => translate_binary(&operands, Operators::And, source, span),
            ("||", 2) => translate_binary(&operands, Operators::Or, source, span),
            ("[", 2) => translate_index(&operands, source, span),
            _ if is_function_name(op.as_str()) => translate_call(op, operands, source, span),
            _ => Err(ExpressionError::new(
                ExpressionCategory::Parse,
                format!("Unsupported operator: {}", op),
                source,
                vec![span],
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::parser::parse;

    fn translate_str(s: &str) -> Result<Expression, ExpressionError> {
        let lexer = crate::expression::lexer::Lexer::new(s)?;
        let parser_token = parse(&lexer)?;
        translate(parser_token, s)
    }

    #[test]
    fn translates_arithmetic_operators() {
        assert_eq!(translate_str("a + b").unwrap().to_string(), "(a + b)");
        assert_eq!(translate_str("a - b").unwrap().to_string(), "(a - b)");
        assert_eq!(translate_str("a * b").unwrap().to_string(), "(a * b)");
        assert_eq!(translate_str("a / b").unwrap().to_string(), "(a / b)");
        assert_eq!(translate_str("a % b").unwrap().to_string(), "(a % b)");
        assert_eq!(translate_str("a ^ b").unwrap().to_string(), "(a ^ b)");
    }

    #[test]
    fn translates_unary_operators() {
        assert_eq!(translate_str("-a").unwrap().to_string(), "(-a)");
        assert_eq!(translate_str("+a").unwrap().to_string(), "a");
        assert_eq!(translate_str("--a").unwrap().to_string(), "(-(-a))");
    }

    #[test]
    fn translates_comparison_operators() {
        assert_eq!(translate_str("a == b").unwrap().to_string(), "(a == b)");
        assert_eq!(translate_str("a != b").unwrap().to_string(), "(a != b)");
        assert_eq!(translate_str("a < b").unwrap().to_string(), "(a < b)");
        assert_eq!(translate_str("a <= b").unwrap().to_string(), "(a <= b)");
        assert_eq!(translate_str("a > b").unwrap().to_string(), "(a > b)");
        assert_eq!(translate_str("a >= b").unwrap().to_string(), "(a >= b)");
    }

    #[test]
    fn translates_logical_operators() {
        assert_eq!(translate_str("a && b").unwrap().to_string(), "(a && b)");
        assert_eq!(translate_str("a || b").unwrap().to_string(), "(a || b)");
        assert_eq!(
            translate_str("a == b && c != d").unwrap().to_string(),
            "((a == b) && (c != d))"
        );
    }

    #[test]
    fn translates_not_operator() {
        assert_eq!(translate_str("!a").unwrap().to_string(), "(!a)");
        assert_eq!(translate_str("!!a").unwrap().to_string(), "(!(!a))");
        assert_eq!(translate_str("!a && b").unwrap().to_string(), "((!a) && b)");
    }

    #[test]
    fn unsupported_operator_returns_error() {
        for op in &["=", "&", "|"] {
            let token = ParserToken::Operator(
                op.to_string(),
                vec![
                    ParserToken::Atom("a".to_string(), Index::new(0, 1)),
                    ParserToken::Atom("b".to_string(), Index::new(0, 1)),
                ],
                Index::new(0, 1),
            );
            let err = translate(token, "").unwrap_err().to_string();
            assert!(err.starts_with("[Parse]"));
            assert!(err.contains(&format!("Unsupported operator: {}", op)));
        }
    }

    #[test]
    fn translates_function_calls() {
        assert_eq!(translate_str("f()").unwrap().to_string(), "f()");
        assert_eq!(translate_str("f(a)").unwrap().to_string(), "f(a)");
        assert_eq!(
            translate_str("f(a, b, c)").unwrap().to_string(),
            "f(a, b, c)"
        );
        assert_eq!(
            translate_str("f(a + 1, b * 2)").unwrap().to_string(),
            "f((a + 1), (b * 2))"
        );
    }

    #[test]
    fn translates_nested_function_calls() {
        assert_eq!(
            translate_str("f(g(a), h())").unwrap().to_string(),
            "f(g(a), h())"
        );
    }

    #[test]
    fn translates_array_indexing() {
        assert_eq!(translate_str("arr[0]").unwrap().to_string(), "arr[0]");
        assert_eq!(
            translate_str("arr[i + 1]").unwrap().to_string(),
            "arr[(i + 1)]"
        );

        // indexing can be chained.
        assert_eq!(translate_str("arr[0][1]").unwrap().to_string(), "arr[0][1]");
    }

    #[test]
    fn dot_operator_is_no_longer_supported() {
        // the `.` operator has been removed; field access must now go through bracket
        // indexing (e.g. `p_map[key1][item1]`) instead of `p_map[key1].item1`.
        let err = translate_str("a . b").unwrap_err().to_string();
        assert!(err.starts_with("[Lexer]"));
        assert!(err.contains("Invalid operator in expression: '.'"));
    }

    #[test]
    fn translates_field_access_via_bracket_indexing() {
        // field access is now expressed as a second level of bracket indexing, and can be
        // chained just like array/table indexing.
        assert_eq!(
            translate_str("p_map[key1][item1]").unwrap().to_string(),
            "p_map[key1][item1]"
        );
    }

    #[test]
    fn translates_integer_literals() {
        assert_eq!(
            translate_str("42").unwrap(),
            Expression::Literal(Literal::Integer(42), Index::new(0, 2))
        );
        assert_eq!(translate_str("42").unwrap().to_string(), "42");
    }

    #[test]
    fn translates_float_literals() {
        assert_eq!(
            translate_str("2.5").unwrap(),
            Expression::Literal(Literal::Float(2.5), Index::new(0, 3))
        );
        assert_eq!(translate_str("2.5").unwrap().to_string(), "2.5");

        assert_eq!(
            translate_str(".87").unwrap(),
            Expression::Literal(Literal::Float(0.87), Index::new(0, 3))
        );
    }

    #[test]
    fn translates_expressions_mixing_literals_and_variables() {
        assert_eq!(translate_str("a + 1").unwrap().to_string(), "(a + 1)");
        assert_eq!(
            translate_str("f(1, b, 2.5)").unwrap().to_string(),
            "f(1, b, 2.5)"
        );
    }
}
