use crate::evaluation::expression::ast::parser::{Parser, ParserToken};
use crate::{ExpressionCategory, ExpressionError};
use message::span::{Span, SpanSet};
use shareable_string::ShareableString;
use std::fmt;

/// A concrete value that appears literally in the source expression.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    /// An integer literal (e.g. `42`), which is a constant numeric value rather than a
    /// variable/field name to be looked up.
    Integer(i64),
    /// A floating-point literal (e.g. `3.14`), which is a constant numeric value rather than a
    /// variable/field name to be looked up.
    Float(f64),
    /// A bare string literal, which is a variable/field name to be looked up.
    Identifier(String),
    /// A quoted string literal (e.g. `"some/path.txt"`), which is a constant text value rather
    /// than a variable/field name to be looked up.
    Text(String),
    /// A boolean literal (`true` or `false`).
    Boolean(bool),
}

impl fmt::Display for Literal {
    #[hotpath::measure]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(value) => write!(f, "{value}"),
            Literal::Float(value) => write!(f, "{value}"),
            Literal::Identifier(value) => write!(f, "{value}"),
            Literal::Text(value) => write!(f, "\"{value}\""),
            Literal::Boolean(value) => write!(f, "{value}"),
        }
    }
}

/// Binary and unary operators supported in expressions.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Operators {
    /// Addition (`+`).
    Add,
    /// Subtraction (`-`).
    Subtract,
    /// Multiplication (`*`).
    Multiply,
    /// Division (`/`).
    Divide,
    /// Remainder / modulus (`%`).
    Modulus,
    /// Exponentiation (`^`).
    Power,
    /// Arithmetic negation (unary `-`).
    Negate,
    /// Equality comparison (`==`).
    Equal,
    /// Inequality comparison (`!=`).
    NotEqual,
    /// Strictly-less-than comparison (`<`).
    LessThan,
    /// Less-than-or-equal comparison (`<=`).
    LessThanOrEqual,
    /// Strictly-greater-than comparison (`>`).
    GreaterThan,
    /// Greater-than-or-equal comparison (`>=`).
    GreaterThanOrEqual,
    /// Logical AND (`&&`).
    And,
    /// Logical OR (`||`).
    Or,
    /// Logical NOT (`!`).
    Not,
}

impl fmt::Display for Operators {
    #[hotpath::measure]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operators::Add => "+",
            Operators::Subtract | Operators::Negate => "-",
            Operators::Multiply => "*",
            Operators::Divide => "/",
            Operators::Modulus => "%",
            Operators::Power => "^",
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
        write!(f, "{symbol}")
    }
}

/// A fully typed abstract syntax tree node produced by translating a [`ParserToken`] tree.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    /// A single literal value together with its source span.
    Literal(Span, Literal),
    /// An infix binary operation.
    BinaryOperation {
        /// The span covering the entire binary expression.
        span: Span,
        /// The span that covers only the operator token.
        operator_span: Span,
        /// The left-hand operand.
        left: Box<Expression>,
        /// The binary operator.
        operator: Operators,
        /// The right-hand operand.
        right: Box<Expression>,
    },
    /// A prefix unary operation.
    UnaryOperation {
        /// The span covering the entire unary expression.
        span: Span,
        /// The unary operator.
        operator: Operators,
        /// The operand being operated on.
        operand: Box<Expression>,
    },
    /// A function call with zero or more arguments.
    FunctionCall {
        /// The span covering the entire function-call expression.
        span: Span,
        /// The name of the function being called.
        name: String,
        /// The evaluated argument expressions.
        arguments: Vec<Expression>,
    },
    /// A subscript-index expression (e.g. `table[0][col]`).
    Index {
        /// The span covering the entire index expression.
        span: Span,
        /// The name of the variable being indexed.
        name: String,
        /// The sequence of index sub-expressions.
        index: Vec<Expression>,
    },
}

impl fmt::Display for Expression {
    #[hotpath::measure]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(_, literal) => write!(f, "{literal}"),
            Expression::BinaryOperation {
                span: _,
                operator_span: _,
                left,
                operator,
                right,
            } => write!(f, "({left} {operator} {right})"),
            Expression::UnaryOperation {
                span: _,
                operator,
                operand,
            } => {
                write!(f, "({operator}{operand})")
            }
            Expression::FunctionCall {
                span: _,
                name,
                arguments,
            } => {
                let args = arguments
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{name}({args})")
            }
            Expression::Index {
                span: _,
                name,
                index,
            } => {
                write!(f, "{name}")?;
                for idx in index {
                    write!(f, "[{idx}]")?;
                }
                Ok(())
            }
        }
    }
}

/// Returns the span associated with the given expression.
pub(crate) const fn expression_span(expression: &Expression) -> Span {
    match expression {
        Expression::Literal(span, _)
        | Expression::BinaryOperation { span, .. }
        | Expression::UnaryOperation { span, .. }
        | Expression::FunctionCall { span, .. }
        | Expression::Index { span, .. } => *span,
    }
}

/// Converts a [`Parser`]'s token tree into a fully typed [`Expression`] AST.
#[derive(Debug)]
pub(crate) struct Translator {
    /// The root of the translated expression tree.
    expression: Expression,
}

impl Translator {
    /// Translates the given `Parser`'s token tree into an `Expression` tree.
    #[hotpath::measure]
    pub(crate) fn new(parser: &Parser) -> Result<Self, ExpressionError> {
        let parser_token = parser.get_token().clone();
        let source = parser.get_source().clone();

        Self::translate_token(parser_token, &source).map(|expression| Self { expression })
    }

    /// Returns a reference to the root [`Expression`] of this translator.
    pub(crate) const fn expression(&self) -> &Expression {
        &self.expression
    }

    /// Translates a binary `ParserToken::Operator` into a `BinaryOperation` expression.
    #[hotpath::measure]
    fn translate_binary(
        span: Span,
        operands: &[ParserToken],
        operator: Operators,
        source: &ShareableString,
    ) -> Result<Expression, ExpressionError> {
        let left = Self::translate_token(
            operands.first().cloned().ok_or_else(|| {
                ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    "Binary operator is missing its left operand.".to_string(),
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?,
            source,
        )?;
        let right = Self::translate_token(
            operands.get(1).cloned().ok_or_else(|| {
                ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    "Binary operator is missing its right operand.".to_string(),
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?,
            source,
        )?;
        let operator_span = span;
        let combined_span = span
            .join(&expression_span(&left))
            .join(&expression_span(&right));

        Ok(Expression::BinaryOperation {
            span: combined_span,
            operator_span,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    /// Translates a unary `ParserToken::Operator` into a `UnaryOperation` expression.
    #[hotpath::measure]
    fn translate_unary(
        span: Span,
        operands: &[ParserToken],
        operator: Operators,
        source: &ShareableString,
    ) -> Result<Expression, ExpressionError> {
        let operand = Self::translate_token(
            operands.first().cloned().ok_or_else(|| {
                ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    "Unary operator is missing its operand.".to_string(),
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?,
            source,
        )?;
        let combined_span = span.join(&expression_span(&operand));

        Ok(Expression::UnaryOperation {
            span: combined_span,
            operator,
            operand: Box::new(operand),
        })
    }

    /// Translates a `ParserToken::Operator("[", ...)` into an `Index` expression.
    ///
    /// When the collection being indexed is itself an `Index` expression (i.e., this is a
    /// chained index such as `arr[0][1]`), the new index is appended to the existing `Index`'s
    /// vector of indices rather than wrapping it in another `Index` expression.
    #[hotpath::measure]
    fn translate_index(
        span: Span,
        operands: &[ParserToken],
        source: &ShareableString,
    ) -> Result<Expression, ExpressionError> {
        let target = Self::translate_token(
            operands.first().cloned().ok_or_else(|| {
                ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    "Index operator is missing its target.".to_string(),
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?,
            source,
        )?;
        let new_index = Self::translate_token(
            operands.get(1).cloned().ok_or_else(|| {
                ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    "Index operator is missing its index.".to_string(),
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?,
            source,
        )?;
        let combined_span = span
            .join(&expression_span(&target))
            .join(&expression_span(&new_index));

        match target {
            Expression::Index {
                span: _,
                name,
                mut index,
            } => {
                index.push(new_index);
                Ok(Expression::Index {
                    span: combined_span,
                    name,
                    index,
                })
            }
            other @ (Expression::Literal(..)
            | Expression::BinaryOperation { .. }
            | Expression::UnaryOperation { .. }
            | Expression::FunctionCall { .. }) => Ok(Expression::Index {
                span: combined_span,
                name: other.to_string(),
                index: vec![new_index],
            }),
        }
    }

    /// Translates a `ParserToken::Operator` whose head is a function name (rather than a known
    /// operator symbol) into a `FunctionCall` expression.
    #[hotpath::measure]
    fn translate_call(
        span: Span,
        name: String,
        arguments: Vec<ParserToken>,
        source: &ShareableString,
    ) -> Result<Expression, ExpressionError> {
        let arguments = arguments
            .into_iter()
            .map(|argument| Self::translate_token(argument, source))
            .collect::<Result<Vec<_>, _>>()?;
        let combined_span = arguments
            .iter()
            .fold(span, |acc, argument| acc.join(&expression_span(argument)));
        // Extend the span by one to account for the closing `)`, which isn't captured by any
        // operand's span but is still part of the call's textual representation.
        let combined_span = combined_span.join(&Span::new(combined_span.end(), 1));

        Ok(Expression::FunctionCall {
            span: combined_span,
            name,
            arguments,
        })
    }

    /// Returns whether `name` looks like a function/variable name (i.e., what the lexer would
    /// have produced as an `Atom`), as opposed to an operator symbol such as `+` or `!`.
    #[hotpath::measure]
    fn is_function_name(name: &str) -> bool {
        name.chars()
            .next()
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
    }

    /// Translates a `ParserToken::Atom` into a `Literal` expression.
    #[hotpath::measure]
    fn translate_atom(span: Span, value: String) -> Expression {
        if let Ok(boolean) = value.parse::<bool>() {
            return Expression::Literal(span, Literal::Boolean(boolean));
        }

        Expression::Literal(span, Literal::Identifier(value))
    }

    /// Translates a `ParserToken::Numeric` into either an integer or floating-point `Literal`
    /// expression, depending on whether the numeric value can be parsed as an integer or a float.
    #[hotpath::measure]
    fn translate_numeric(span: Span, value: &str) -> Result<Expression, ExpressionError> {
        if let Ok(integer) = value.parse::<i64>() {
            return Ok(Expression::Literal(span, Literal::Integer(integer)));
        }

        if let Ok(float) = value.parse::<f64>() {
            return Ok(Expression::Literal(span, Literal::Float(float)));
        }

        Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!("Invalid numeric literal: {value}"),
        ))
    }

    /// Recursively translates a single [`ParserToken`] into an [`Expression`] node.
    ///
    /// # Errors
    ///
    /// Returns an error if the token tree cannot be mapped to a valid expression
    /// (e.g., an unsupported operator arity or an invalid numeric literal).
    #[hotpath::measure]
    fn translate_token(
        parser_token: ParserToken,
        source: &ShareableString,
    ) -> Result<Expression, ExpressionError> {
        match parser_token {
            ParserToken::Identifier(span, value) => Ok(Self::translate_atom(span, value)),
            ParserToken::Numeric(span, value) => Self::translate_numeric(span, value.as_str()),
            ParserToken::Text(span, value) => Ok(Expression::Literal(span, Literal::Text(value))),
            ParserToken::Operator(span, op, operands) => match (op.as_str(), operands.len()) {
                ("+", 1) => Self::translate_token(
                    operands.first().cloned().ok_or_else(|| {
                        ExpressionError::new_complex(
                            ExpressionCategory::Parse,
                            "Unary '+' operator is missing its operand.".to_string(),
                            source.clone(),
                            SpanSet::from_span(span),
                        )
                    })?,
                    source,
                ),
                ("-", 1) => Self::translate_unary(span, &operands, Operators::Negate, source),
                ("!", 1) => Self::translate_unary(span, &operands, Operators::Not, source),
                ("+", 2) => Self::translate_binary(span, &operands, Operators::Add, source),
                ("-", 2) => Self::translate_binary(span, &operands, Operators::Subtract, source),
                ("*", 2) => Self::translate_binary(span, &operands, Operators::Multiply, source),
                ("/", 2) => Self::translate_binary(span, &operands, Operators::Divide, source),
                ("%", 2) => Self::translate_binary(span, &operands, Operators::Modulus, source),
                ("^", 2) => Self::translate_binary(span, &operands, Operators::Power, source),
                ("==", 2) => Self::translate_binary(span, &operands, Operators::Equal, source),
                ("!=", 2) => Self::translate_binary(span, &operands, Operators::NotEqual, source),
                ("<", 2) => Self::translate_binary(span, &operands, Operators::LessThan, source),
                ("<=", 2) => {
                    Self::translate_binary(span, &operands, Operators::LessThanOrEqual, source)
                }
                (">", 2) => Self::translate_binary(span, &operands, Operators::GreaterThan, source),
                (">=", 2) => {
                    Self::translate_binary(span, &operands, Operators::GreaterThanOrEqual, source)
                }
                ("&&", 2) => Self::translate_binary(span, &operands, Operators::And, source),
                ("||", 2) => Self::translate_binary(span, &operands, Operators::Or, source),
                ("[", 2) => Self::translate_index(span, &operands, source),
                _ if Self::is_function_name(op.as_str()) => {
                    Self::translate_call(span, op, operands, source)
                }
                _ => Err(ExpressionError::new(
                    ExpressionCategory::Parse,
                    format!("Unsupported operator: {op}"),
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluation::expression::ast::lexer::Lexer;
    use crate::evaluation::expression::ast::parser::Parser;

    fn translate_str(s: &str) -> Result<Expression, ExpressionError> {
        let lexer = Lexer::new(s)?;
        let parser = Parser::new(&lexer)?;
        Translator::new(&parser).map(|translator| translator.expression().clone())
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
                Span::new(0, 0),
                op.to_string(),
                vec![
                    ParserToken::Identifier(Span::new(0, 0), "a".to_string()),
                    ParserToken::Identifier(Span::new(0, 0), "b".to_string()),
                ],
            );
            let err = Translator::translate_token(token, &ShareableString::from(""))
                .unwrap_err()
                .to_string();
            assert!(err.starts_with("[Parse]"));
            assert!(err.contains(&format!("Unsupported operator: {op}")));
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
        // field access is now expressed as a second level of bracket indexing and can be
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
            Expression::Literal(Span::new(0, 2), Literal::Integer(42))
        );
        assert_eq!(translate_str("42").unwrap().to_string(), "42");
    }

    #[test]
    fn translates_float_literals() {
        assert_eq!(
            translate_str("2.5").unwrap(),
            Expression::Literal(Span::new(0, 3), Literal::Float(2.5))
        );
        assert_eq!(translate_str("2.5").unwrap().to_string(), "2.5");

        assert_eq!(
            translate_str(".87").unwrap(),
            Expression::Literal(Span::new(0, 3), Literal::Float(0.87))
        );
    }

    #[test]
    fn translates_quoted_string_literals() {
        assert_eq!(
            translate_str("\"hello/world.txt\"").unwrap(),
            Expression::Literal(
                Span::new(0, 17),
                Literal::Text("hello/world.txt".to_string())
            )
        );
        assert_eq!(translate_str("\"hello\"").unwrap().to_string(), "\"hello\"");
    }

    #[test]
    fn quoted_string_literals_are_not_treated_as_variable_names() {
        // Unlike a bare identifier (which is treated as a variable name), a quoted string is always a
        // literal `Text` value, never a `String` used for variable lookup.
        assert_eq!(
            translate_str("\"v_foo\"").unwrap(),
            Expression::Literal(Span::new(0, 7), Literal::Text("v_foo".to_string()))
        );
        assert_eq!(
            translate_str("v_foo").unwrap(),
            Expression::Literal(Span::new(0, 5), Literal::Identifier("v_foo".to_string()))
        );
    }

    #[test]
    fn translates_expressions_mixing_quoted_strings_and_operators() {
        assert_eq!(
            translate_str("\"a\" + \"b\"").unwrap().to_string(),
            "(\"a\" + \"b\")"
        );
    }

    #[test]
    fn translates_scientific_notation_literals() {
        assert_eq!(
            translate_str("1e10").unwrap(),
            Expression::Literal(Span::new(0, 4), Literal::Float(1e10))
        );

        assert_eq!(
            translate_str("1.5e-3").unwrap(),
            Expression::Literal(Span::new(0, 6), Literal::Float(1.5e-3))
        );

        assert_eq!(
            translate_str(".5e+2").unwrap(),
            Expression::Literal(Span::new(0, 5), Literal::Float(0.5e2))
        );

        assert_eq!(
            translate_str("6.022e23").unwrap(),
            Expression::Literal(Span::new(0, 8), Literal::Float(6.022e23))
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

    #[test]
    fn function_call_span_includes_closing_parenthesis() {
        let expr = translate_str("sin(1.0)").unwrap();
        let span = expression_span(&expr);
        assert_eq!(span.start(), 0);
        assert_eq!(span.end(), 8);
    }

    #[test]
    fn binary_operation_operator_span() {
        let expr = translate_str("5.0*sin(3.)+1.0+1").unwrap();
        match expr {
            Expression::BinaryOperation {
                span,
                operator_span,
                ..
            } => {
                assert_eq!(span.start(), 0);
                assert_eq!(span.end(), 17);
                assert_eq!(operator_span.start(), 15);
                assert_eq!(operator_span.end(), 16);
            }
            other => panic!("expected BinaryOperation, got {other:?}"),
        }
    }
}
