use crate::BasicDefinition::{
    Boolean, Choice, File, Folder, Integer, Number, NumberWithUnits, String, Unit,
};
use crate::evaluation::expression::ast::span::{Span, SpanSet};
use crate::evaluation::expression::ast::translator::{
    Expression, Literal, Operators, expression_span,
};
use crate::evaluation::expression::function_definition::{ArgumentCount, FunctionDefinitions};
use crate::expression::ast::ast_helper::string_to_expression;
use crate::input_data::input_basic_with_units::BasicInputWithUnitsData;
use crate::{
    BasicInputData, ComputedItem, ComputedTable, ComputedTableWithUnits, ExpressionCategory,
    Message, ObjectItemInputData, TableInputData, TableWithUnitsInputData,
};
use datastore::definition::{IntegerConstraintEnum, NumberConstraintEnum};
use shareable_string::ShareableString;
use std::collections::BTreeMap;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use units::{UnitId, conversion::convert};

/// Rejects floating-point values that cannot be safely represented in computed output.
#[hotpath::measure]
fn finite_float(value: f64, source: &ShareableString, span: Span) -> Result<f64, Message> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_floating_point_not_finite",
            [],
            source.clone(),
            SpanSet::from_span(span),
        ))
    }
}

/// Rejects computed values containing non-finite floating-point data.
#[hotpath::measure]
fn ensure_finite_computed_item(
    item: ComputedItem,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    match &item {
        ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. } => {
            finite_float(*value, source, span)?;
        }
        ComputedItem::Table(table) => {
            for row in table.rows() {
                for value in row {
                    finite_float(*value, source, span)?;
                }
            }
        }
        ComputedItem::TableWithUnits(table) => {
            for row in table.rows() {
                for value in row {
                    finite_float(*value, source, span)?;
                }
            }
        }
        ComputedItem::Boolean(_)
        | ComputedItem::Integer(_)
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Unit(_) => {}
    }

    Ok(item)
}

/// Looks up `variable_name` in `computed_data`, returning its value or an evaluation error.
#[hotpath::measure]
fn lookup_variable(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    variable_name: &str,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    let key = ShareableString::from(variable_name);
    match computed_data.get(&key) {
        Some(computed_item) => ensure_finite_computed_item(computed_item.clone(), source, span),
        None => Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_variable_not_found",
            [("variable", variable_name)],
            source.clone(),
            SpanSet::from_span(span),
        )),
    }
}

/// Creates a computed float with unit metadata only when the unit is concrete.
#[hotpath::measure]
fn computed_float(value: f64, unit: UnitId) -> ComputedItem {
    if unit == UnitId::None {
        ComputedItem::Float(value)
    } else {
        ComputedItem::FloatWithUnit { value, unit }
    }
}

/// Returns whether an expression is a float literal, optionally prefixed with a single negation.
#[hotpath::measure]
fn is_signed_float_literal(expression: &Expression) -> bool {
    match expression {
        Expression::Literal(_, Literal::Float(_)) => true,
        Expression::UnaryOperation {
            operator: Operators::Negate,
            operand,
            ..
        } => matches!(operand.as_ref(), Expression::Literal(_, Literal::Float(_))),
        Expression::Literal(..)
        | Expression::BinaryOperation { .. }
        | Expression::UnaryOperation { .. }
        | Expression::FunctionCall { .. }
        | Expression::Index { .. } => false,
    }
}

/// Applies a unary `operator` to `operand_value`, returning the result or an error.
#[hotpath::measure]
fn evaluate_unary_operation(
    operator: Operators,
    operand_value: ComputedItem,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    match (operator, operand_value) {
        (
            Operators::Negate,
            ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, unit: _ },
        ) => Ok(ComputedItem::Float(finite_float(
            value.neg(),
            source,
            span,
        )?)),
        (Operators::Negate, ComputedItem::Integer(value)) => Ok(ComputedItem::Integer(
            value.checked_mul(-1).ok_or_else(|| {
                crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_integer_overflow",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?,
        )),
        (Operators::Not, ComputedItem::Boolean(value)) => Ok(ComputedItem::Boolean(!value)),
        _ => Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_invalid_unary_operation",
            [],
            source.clone(),
            SpanSet::from_span(span),
        )),
    }
}

/// Applies a binary `operator` to two boolean operands.
#[hotpath::measure]
fn evaluate_boolean_binary_operation(
    operator: &Operators,
    left_value: bool,
    right_value: bool,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    match operator {
        Operators::And => Ok(ComputedItem::Boolean(left_value && right_value)),
        Operators::Or => Ok(ComputedItem::Boolean(left_value || right_value)),
        Operators::Equal => Ok(ComputedItem::Boolean(left_value == right_value)),
        Operators::NotEqual => Ok(ComputedItem::Boolean(left_value != right_value)),
        Operators::Add
        | Operators::Subtract
        | Operators::Multiply
        | Operators::Divide
        | Operators::Modulus
        | Operators::Power
        | Operators::Negate
        | Operators::LessThan
        | Operators::LessThanOrEqual
        | Operators::GreaterThan
        | Operators::GreaterThanOrEqual
        | Operators::Not => Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_unsupported_operator",
            [("operator", operator), ("type", "boolean")],
            source.clone(),
            SpanSet::from_span(span),
        )),
    }
}

/// Applies a binary `operator` to two `f64` operands.
#[hotpath::measure]
fn evaluate_float_binary_operation(
    operator: &Operators,
    left_value: f64,
    right_value: f64,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    match operator {
        Operators::Add => Ok(ComputedItem::Float(finite_float(
            left_value.add(right_value),
            source,
            span,
        )?)),
        Operators::Subtract => Ok(ComputedItem::Float(finite_float(
            left_value.sub(right_value),
            source,
            span,
        )?)),
        Operators::Multiply => Ok(ComputedItem::Float(finite_float(
            left_value.mul(right_value),
            source,
            span,
        )?)),
        Operators::Divide => {
            if right_value == 0.0 {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_division_by_zero",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            } else {
                Ok(ComputedItem::Float(finite_float(
                    left_value.div(right_value),
                    source,
                    span,
                )?))
            }
        }
        Operators::Modulus => {
            if right_value == 0.0 {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_modulus_by_zero",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            } else {
                Ok(ComputedItem::Float(finite_float(
                    left_value.rem(right_value),
                    source,
                    span,
                )?))
            }
        }
        Operators::Power => Ok(ComputedItem::Float(finite_float(
            left_value.powf(right_value),
            source,
            span,
        )?)),
        Operators::LessThan => Ok(ComputedItem::Boolean(left_value < right_value)),
        Operators::LessThanOrEqual => Ok(ComputedItem::Boolean(left_value <= right_value)),
        Operators::GreaterThan => Ok(ComputedItem::Boolean(left_value > right_value)),
        Operators::GreaterThanOrEqual => Ok(ComputedItem::Boolean(left_value >= right_value)),
        Operators::Negate
        | Operators::Equal
        | Operators::NotEqual
        | Operators::And
        | Operators::Or
        | Operators::Not => Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_unsupported_operator",
            [("operator", operator), ("type", "float")],
            source.clone(),
            SpanSet::from_span(span),
        )),
    }
}

/// Applies a binary `operator` to two `i64` operands, using checked arithmetic to detect overflow.
#[hotpath::measure]
fn evaluate_integer_binary_operation(
    operator: &Operators,
    left_value: i64,
    right_value: i64,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    match operator {
        Operators::Add => {
            let checked_add = left_value.checked_add(right_value);
            match checked_add {
                Some(result) => Ok(ComputedItem::Integer(result)),
                None => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_integer_overflow",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                )),
            }
        }
        Operators::Subtract => {
            let checked_sub = left_value.checked_sub(right_value);
            match checked_sub {
                Some(result) => Ok(ComputedItem::Integer(result)),
                None => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_integer_overflow",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                )),
            }
        }
        Operators::Multiply => {
            let checked_mul = left_value.checked_mul(right_value);
            match checked_mul {
                Some(result) => Ok(ComputedItem::Integer(result)),
                None => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_integer_overflow",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                )),
            }
        }
        Operators::Divide => {
            let checked_div = left_value.checked_div(right_value);

            match checked_div {
                Some(result) => Ok(ComputedItem::Integer(result)),
                None => {
                    if right_value == 0 {
                        Err(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_division_by_zero",
                            [],
                            source.clone(),
                            SpanSet::from_span(span),
                        ))
                    } else {
                        Err(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_integer_overflow",
                            [],
                            source.clone(),
                            SpanSet::from_span(span),
                        ))
                    }
                }
            }
        }
        Operators::Modulus => {
            let checked_mod = left_value.checked_rem(right_value);
            match checked_mod {
                Some(result) => Ok(ComputedItem::Integer(result)),
                None => {
                    if right_value == 0 {
                        Err(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_modulus_by_zero",
                            [],
                            source.clone(),
                            SpanSet::from_span(span),
                        ))
                    } else {
                        Err(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_integer_overflow",
                            [],
                            source.clone(),
                            SpanSet::from_span(span),
                        ))
                    }
                }
            }
        }
        Operators::Power => {
            let exponent = u32::try_from(right_value).map_err(|_| {
                crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_invalid_integer_exponent",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?;
            left_value.checked_pow(exponent).map_or_else(
                || {
                    Err(crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_integer_overflow",
                        [],
                        source.clone(),
                        SpanSet::from_span(span),
                    ))
                },
                |value| Ok(ComputedItem::Integer(value)),
            )
        }
        Operators::Equal => Ok(ComputedItem::Boolean(left_value == right_value)),
        Operators::NotEqual => Ok(ComputedItem::Boolean(left_value != right_value)),
        Operators::LessThan => Ok(ComputedItem::Boolean(left_value < right_value)),
        Operators::LessThanOrEqual => Ok(ComputedItem::Boolean(left_value <= right_value)),
        Operators::GreaterThan => Ok(ComputedItem::Boolean(left_value > right_value)),
        Operators::GreaterThanOrEqual => Ok(ComputedItem::Boolean(left_value >= right_value)),
        Operators::Negate | Operators::Not | Operators::And | Operators::Or => {
            Err(crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_unsupported_operator",
                [("operator", operator), ("type", "integer")],
                source.clone(),
                SpanSet::from_span(span),
            ))
        }
    }
}

/// Applies a binary `operator` to two string operands (equality and inequality only).
#[hotpath::measure]
fn evaluate_string_binary_operation(
    operator: &Operators,
    left_value: &ShareableString,
    right_value: &ShareableString,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    match operator {
        Operators::Equal => Ok(ComputedItem::Boolean(left_value == right_value)),
        Operators::NotEqual => Ok(ComputedItem::Boolean(left_value != right_value)),
        Operators::Negate
        | Operators::And
        | Operators::Or
        | Operators::Not
        | Operators::LessThan
        | Operators::LessThanOrEqual
        | Operators::GreaterThan
        | Operators::GreaterThanOrEqual
        | Operators::Add
        | Operators::Subtract
        | Operators::Multiply
        | Operators::Divide
        | Operators::Modulus
        | Operators::Power => Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_unsupported_operator",
            [("operator", operator), ("type", "string")],
            source.clone(),
            SpanSet::from_span(span),
        )),
    }
}

/// Looks up `function_name` in `functions`, evaluates each argument, validates the argument count,
/// and calls the function, returning its result.
#[hotpath::measure]
fn evaluate_function_call_operation(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    function_name: &str,
    arguments: Vec<Expression>,
    functions: &FunctionDefinitions,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    let definition = functions.get(function_name).ok_or_else(|| {
        crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_function_not_defined",
            [("function", function_name)],
            source.clone(),
            SpanSet::from_span(span),
        )
    })?;

    let mut evaluated_arguments = Vec::with_capacity(arguments.len());
    for argument in arguments {
        evaluated_arguments.push(evaluate_expression(
            computed_data,
            functions,
            source,
            argument,
        )?);
    }

    match definition.parameter_constraints() {
        ArgumentCount::Exact { count } => {
            if evaluated_arguments.len() != *count {
                return Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_function_wrong_argument_count_exact",
                    [
                        ("function", function_name),
                        ("expected", count),
                        ("actual", evaluated_arguments.len())
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ));
            }
        }
        ArgumentCount::Min { min } => {
            if evaluated_arguments.len() < *min {
                return Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_function_wrong_argument_count_minimum",
                    [
                        ("function", function_name),
                        ("minimum", min),
                        ("actual", evaluated_arguments.len())
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ));
            }
        }
        ArgumentCount::Max { max } => {
            if evaluated_arguments.len() > *max {
                return Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_function_wrong_argument_count_maximum",
                    [
                        ("function", function_name),
                        ("maximum", max),
                        ("actual", evaluated_arguments.len())
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ));
            }
        }
        ArgumentCount::Range { min, max } => {
            if evaluated_arguments.len() < *min || evaluated_arguments.len() > *max {
                return Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_function_wrong_argument_count_range",
                    [
                        ("function", function_name),
                        ("minimum", min),
                        ("maximum", max),
                        ("actual", evaluated_arguments.len())
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ));
            }
        }
        ArgumentCount::Unbounded => {}
    }

    ensure_finite_computed_item(definition.call(&evaluated_arguments)?, source, span)
}

/// Evaluates a subscript-index expression (e.g. `table[0][col]`), returning the referenced cell value.
#[hotpath::measure]
fn evaluate_index_operation(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    name: &str,
    index: Vec<Expression>,
    functions: &FunctionDefinitions,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    if index.len() != 2 && index.len() != 4 {
        return Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_invalid_index_count",
            [("actual", index.len())],
            source.clone(),
            SpanSet::from_span(span),
        ));
    }

    let mut indexes = Vec::new();
    for index_expression in index {
        // A bare identifier used as an index (e.g., the `col` in `t[0][col]`) is a
        // literal field name, not a reference to a variable, so it is not looked up.
        let index_value =
            if let Expression::Literal(lit_span, Literal::Identifier(name)) = &index_expression {
                if let Ok(value) = lookup_variable(computed_data, name, source, *lit_span) {
                    value
                } else {
                    ComputedItem::String(ShareableString::from(name.clone()))
                }
            } else {
                evaluate_expression(computed_data, functions, source, index_expression)?
            };
        indexes.push(index_value);
    }

    let index_1 = indexes.first().ok_or_else(|| {
        crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_missing_first_index",
            [],
            source.clone(),
            SpanSet::from_span(span),
        )
    })?;
    let index_2 = indexes.get(1).ok_or_else(|| {
        crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_missing_second_index",
            [],
            source.clone(),
            SpanSet::from_span(span),
        )
    })?;

    let map_lookup = format!("{name}[{index_1}][{index_2}]");
    let item = if let Ok(value) = lookup_variable(computed_data, &map_lookup, source, span) {
        indexes.drain(0..2);
        value
    } else {
        lookup_variable(computed_data, name, source, span)?
    };

    if matches!(
        &item,
        ComputedItem::Table(_) | ComputedItem::TableWithUnits(_)
    ) {
        if indexes.is_empty() {
            return Ok(item);
        }

        let (table, table_with_units) = match &item {
            ComputedItem::Table(table) => (table, None),
            ComputedItem::TableWithUnits(table) => (table.as_table(), Some(table)),
            ComputedItem::Boolean(_)
            | ComputedItem::Integer(_)
            | ComputedItem::Float(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::String(_)
            | ComputedItem::Identifier(_)
            | ComputedItem::Path(_)
            | ComputedItem::Unit(_) => {
                return Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_table_for_indexing",
                    [],
                    source.clone(),
                    SpanSet::from_span(span),
                ));
            }
        };

        let index_1 = indexes.first().ok_or_else(|| {
            crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_missing_first_index",
                [],
                source.clone(),
                SpanSet::from_span(span),
            )
        })?;
        let index_2 = indexes.get(1).ok_or_else(|| {
            crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_missing_second_index",
                [],
                source.clone(),
                SpanSet::from_span(span),
            )
        })?;

        let row_index = match index_1 {
            ComputedItem::Integer(i) => {
                let size = usize::try_from(*i)
                    .ok()
                    .filter(|size| *size < table.row_count());

                size.ok_or_else(|| {
                    crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_table_row_index_out_of_bounds",
                        [("index", i), ("count", table.row_count())],
                        source.clone(),
                        SpanSet::from_span(span),
                    )
                })?
            }
            ComputedItem::Boolean(_)
            | ComputedItem::Float(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::String(_)
            | ComputedItem::Identifier(_)
            | ComputedItem::Path(_)
            | ComputedItem::Table(_)
            | ComputedItem::TableWithUnits(_)
            | ComputedItem::Unit(_) => {
                return Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_table_row_index",
                    [("actual", format!("{index_1:?}"))],
                    source.clone(),
                    SpanSet::from_span(span),
                ));
            }
        };

        return match index_2 {
            ComputedItem::Identifier(s) | ComputedItem::String(s) => {
                let value = table.get_cell_by_name(row_index, s).ok_or_else(|| {
                    crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_table_field_not_found",
                        [("field", s)],
                        source.clone(),
                        SpanSet::from_span(span),
                    )
                })?;
                let unit = table_with_units
                    .and_then(|table| table.get_unit_by_name(s.clone()))
                    .unwrap_or(UnitId::None);
                Ok(computed_float(value, unit))
            }
            ComputedItem::Integer(i) => {
                let size = usize::try_from(*i)
                    .ok()
                    .filter(|size| *size < table.column_count());

                let size = size.ok_or_else(|| {
                    crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_table_column_index_out_of_bounds",
                        [("index", i), ("count", table.column_count())],
                        source.clone(),
                        SpanSet::from_span(span),
                    )
                })?;

                let value = table.get_cell(row_index, size).ok_or_else(|| {
                    crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_table_field_not_found",
                        [("field", i)],
                        source.clone(),
                        SpanSet::from_span(span),
                    )
                })?;
                let unit = table_with_units
                    .and_then(|table| table.get_unit(size))
                    .unwrap_or(UnitId::None);
                Ok(computed_float(value, unit))
            }
            other @ (ComputedItem::Boolean(_)
            | ComputedItem::Float(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::Path(_)
            | ComputedItem::Table(_)
            | ComputedItem::TableWithUnits(_)
            | ComputedItem::Unit(_)) => Err(crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_expected_table_field_index",
                [("actual", format!("{other:?}"))],
                source.clone(),
                SpanSet::from_span(span),
            )),
        };
    }

    Ok(item)
}

/// Recursively evaluates an [`Expression`] node against `computed_data` and returns the result.
#[hotpath::measure]
fn evaluate_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    source: &ShareableString,
    expression: Expression,
) -> Result<ComputedItem, Message> {
    match expression {
        Expression::Literal(span, literal) => match literal {
            Literal::Integer(value) => Ok(ComputedItem::Integer(value)),
            Literal::Float(value) => Ok(ComputedItem::Float(finite_float(value, source, span)?)),
            Literal::Identifier(value) => Ok(lookup_variable(computed_data, &value, source, span)?),
            Literal::Text(value) => Ok(ComputedItem::String(ShareableString::from(value))),
            Literal::Boolean(value) => Ok(ComputedItem::Boolean(value)),
        },
        Expression::UnaryOperation {
            span,
            operator,
            operand,
        } => {
            let operand_value = evaluate_expression(computed_data, functions, source, *operand)?;
            evaluate_unary_operation(operator, operand_value, source, span)
        }
        Expression::BinaryOperation {
            span: _span,
            operator_span,
            left,
            operator,
            right,
        } => {
            let left_value = evaluate_expression(computed_data, functions, source, *left)?;
            let right_value = evaluate_expression(computed_data, functions, source, *right)?;
            match (left_value, right_value) {
                (ComputedItem::Boolean(left_bool), ComputedItem::Boolean(right_bool)) => {
                    evaluate_boolean_binary_operation(
                        &operator,
                        left_bool,
                        right_bool,
                        source,
                        operator_span,
                    )
                }
                (ComputedItem::Path(left_file), ComputedItem::Path(right_file)) => match operator {
                    Operators::Equal => Ok(ComputedItem::Boolean(left_file == right_file)),
                    Operators::NotEqual => Ok(ComputedItem::Boolean(left_file != right_file)),
                    Operators::Add
                    | Operators::Subtract
                    | Operators::Multiply
                    | Operators::Divide
                    | Operators::Modulus
                    | Operators::Power
                    | Operators::Negate
                    | Operators::LessThan
                    | Operators::LessThanOrEqual
                    | Operators::GreaterThan
                    | Operators::GreaterThanOrEqual
                    | Operators::And
                    | Operators::Or
                    | Operators::Not => Err(crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_unsupported_operator",
                        [("operator", operator), ("type", "file")],
                        source.clone(),
                        SpanSet::from_span(operator_span),
                    )),
                },
                (
                    ComputedItem::Float(left_float)
                    | ComputedItem::FloatWithUnit {
                        value: left_float,
                        unit: _,
                    },
                    ComputedItem::Float(right_float)
                    | ComputedItem::FloatWithUnit {
                        value: right_float,
                        unit: _,
                    },
                ) => evaluate_float_binary_operation(
                    &operator,
                    left_float,
                    right_float,
                    source,
                    operator_span,
                ),
                (ComputedItem::Integer(left_int), ComputedItem::Integer(right_int)) => {
                    evaluate_integer_binary_operation(
                        &operator,
                        left_int,
                        right_int,
                        source,
                        operator_span,
                    )
                }
                (
                    ComputedItem::String(left_string) | ComputedItem::Identifier(left_string),
                    ComputedItem::String(right_string) | ComputedItem::Identifier(right_string),
                ) => evaluate_string_binary_operation(
                    &operator,
                    &left_string,
                    &right_string,
                    source,
                    operator_span,
                ),
                (ComputedItem::Unit(left_unit), ComputedItem::Unit(right_unit)) => match operator {
                    Operators::Equal => Ok(ComputedItem::Boolean(left_unit == right_unit)),
                    Operators::NotEqual => Ok(ComputedItem::Boolean(left_unit != right_unit)),
                    Operators::Add
                    | Operators::Subtract
                    | Operators::Multiply
                    | Operators::Divide
                    | Operators::Modulus
                    | Operators::Power
                    | Operators::Negate
                    | Operators::LessThan
                    | Operators::LessThanOrEqual
                    | Operators::GreaterThan
                    | Operators::GreaterThanOrEqual
                    | Operators::And
                    | Operators::Or
                    | Operators::Not => Err(crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_unsupported_operator",
                        [("operator", operator), ("type", "unit")],
                        source.clone(),
                        SpanSet::from_span(operator_span),
                    )),
                },
                (
                    ComputedItem::Table(_) | ComputedItem::TableWithUnits(_),
                    ComputedItem::Table(_) | ComputedItem::TableWithUnits(_),
                ) => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_unsupported_operator",
                    [("operator", operator), ("type", "table")],
                    source.clone(),
                    SpanSet::from_span(operator_span),
                )),

                (
                    ComputedItem::Boolean(_),
                    ComputedItem::Path(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { value: _, unit: _ }
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Integer(_)
                    | ComputedItem::String(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_),
                )
                | (
                    ComputedItem::Path(_),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { value: _, unit: _ }
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Integer(_)
                    | ComputedItem::String(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_),
                )
                | (
                    ComputedItem::Float(_) | ComputedItem::FloatWithUnit { value: _, unit: _ },
                    ComputedItem::Boolean(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Integer(_)
                    | ComputedItem::String(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_),
                )
                | (
                    ComputedItem::Identifier(_) | ComputedItem::String(_),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { value: _, unit: _ }
                    | ComputedItem::Integer(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_),
                )
                | (
                    ComputedItem::Integer(_),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { value: _, unit: _ }
                    | ComputedItem::Identifier(_)
                    | ComputedItem::String(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_),
                )
                | (
                    ComputedItem::Table(_) | ComputedItem::TableWithUnits(_),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { value: _, unit: _ }
                    | ComputedItem::Integer(_)
                    | ComputedItem::Identifier(_)
                    | ComputedItem::String(_)
                    | ComputedItem::Unit(_),
                )
                | (
                    ComputedItem::Unit(_),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { value: _, unit: _ }
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Integer(_)
                    | ComputedItem::String(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_),
                ) => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_unsupported_operator",
                    [("operator", operator), ("type", "mixed type")],
                    source.clone(),
                    SpanSet::from_span(operator_span),
                )),
            }
        }
        Expression::FunctionCall {
            span,
            name,
            arguments,
        } => evaluate_function_call_operation(
            computed_data,
            name.as_str(),
            arguments,
            functions,
            source,
            span,
        ),
        Expression::Index { span, name, index } => {
            evaluate_index_operation(computed_data, name.as_str(), index, functions, source, span)
        }
    }
}

/// Validates a bare-identifier choice value (e.g. `option_1`) directly against the choice
/// definition's list of valid choices, without treating it as a variable reference.
#[hotpath::measure]
fn evaluate_bare_identifier_choice(
    choice_definition: &datastore::definition::ChoiceDefinition,
    name: &str,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    let value = ShareableString::from(name);
    if choice_definition.contains(value.clone()) {
        Ok(ComputedItem::Identifier(value))
    } else {
        Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_invalid_choice",
            [("value", value)],
            source.clone(),
            SpanSet::from_span(span),
        ))
    }
}

/// Validates that a computed unit value belongs to a unit definition's family.
#[hotpath::measure]
fn validate_unit_value(
    unit_definition: &datastore::definition::UnitDefinition,
    computed: &ComputedItem,
    source: &ShareableString,
    span: Span,
) -> Result<ComputedItem, Message> {
    let unit = match computed {
        ComputedItem::Unit(unit) => *unit,
        ComputedItem::String(value) | ComputedItem::Identifier(value) => {
            UnitId::from_unit_id_str(value.as_str()).ok_or_else(|| {
                crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_invalid_unit_id",
                    [("value", value)],
                    source.clone(),
                    SpanSet::from_span(span),
                )
            })?
        }
        ComputedItem::Boolean(_)
        | ComputedItem::Integer(_)
        | ComputedItem::Float(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_) => {
            return Err(crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_expected_unit_value",
                [("actual", format!("{computed:?}"))],
                source.clone(),
                SpanSet::from_span(span),
            ));
        }
    };

    if unit_definition.unit_family().unit_ids().contains(&unit) {
        Ok(ComputedItem::Unit(unit))
    } else {
        Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_invalid_unit_for_family",
            [
                ("value", unit.string_id()),
                ("family", unit_definition.unit_family().description())
            ],
            source.clone(),
            SpanSet::from_span(span),
        ))
    }
}

/// Evaluates the expression stored in a single [`BasicInputData`] item.
#[hotpath::measure]
fn evaluate_basic_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    basic: &BasicInputData,
) -> Result<ComputedItem, Message> {
    let source = basic.data();
    let expression = string_to_expression(source)?;

    let span = expression_span(&expression);

    // Choice and unit values may be written as bare identifiers rather than variable references.
    if let Expression::Literal(_, Literal::Identifier(name)) = &expression {
        match basic.definition() {
            Choice(choice_definition) => {
                return evaluate_bare_identifier_choice(choice_definition, name, source, span);
            }
            Unit(unit_definition) => {
                let computed = ComputedItem::Identifier(name.clone().into());
                return validate_unit_value(unit_definition, &computed, source, span);
            }
            Boolean(_) | File(_) | Folder(_) | Integer(_) | Number(_) | NumberWithUnits(_)
            | String(_) => {}
        }
    }

    let computed = evaluate_expression(computed_data, functions, source, expression)?;
    match basic.definition() {
        Boolean(_boolean_definition) => {
            // Validate that the computed value is a boolean
            if let ComputedItem::Boolean(_value) = &computed {
                Ok(computed)
            } else {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [("expected", "boolean"), ("actual", format!("{computed:?}"))],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            }
        }
        Choice(choice_definition) => {
            // Validate that the computed value is one of the allowed choices
            if let ComputedItem::String(value) | ComputedItem::Identifier(value) = &computed {
                if choice_definition.contains(value) {
                    Ok(ComputedItem::Identifier(value.clone()))
                } else {
                    Err(crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_invalid_choice",
                        [("value", value)],
                        source.clone(),
                        SpanSet::from_span(span),
                    ))
                }
            } else {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [
                        ("expected", "choice string"),
                        ("actual", format!("{computed:?}"))
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            }
        }
        File(_file_definition) => {
            // Validate that the computed value is a file path
            if let ComputedItem::Path(_path) = &computed {
                // Could add additional validation here (e.g., path exists, is readable)
                Ok(computed)
            } else if let ComputedItem::String(value) = &computed {
                Ok(ComputedItem::Path(value.clone()))
            } else {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [
                        ("expected", "file path"),
                        ("actual", format!("{computed:?}"))
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            }
        }
        Folder(_folder_definition) => {
            // Validate that the computed value is a folder path
            if let ComputedItem::Path(_path) = &computed {
                // Could add additional validation here (e.g., path exists, is a directory)
                Ok(computed)
            } else if let ComputedItem::String(value) = &computed {
                Ok(ComputedItem::Path(value.clone()))
            } else {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [
                        ("expected", "folder path"),
                        ("actual", format!("{computed:?}"))
                    ],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            }
        }
        Integer(integer_definition) => {
            // Validate that the computed value is an integer
            if let ComputedItem::Integer(value) = &computed {
                let constraint = integer_definition.constraint();
                match constraint {
                    IntegerConstraintEnum::Min { min, inclusive } => {
                        if *value < min || (!inclusive && *value == min) {
                            return Err(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_value_below_minimum",
                                [("value", value), ("minimum", min)],
                                source.clone(),
                                SpanSet::from_span(span),
                            ));
                        }
                        Ok(computed)
                    }
                    IntegerConstraintEnum::Max { max, inclusive } => {
                        if *value > max || (!inclusive && *value == max) {
                            return Err(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_value_above_maximum",
                                [("value", value), ("maximum", max)],
                                source.clone(),
                                SpanSet::from_span(span),
                            ));
                        }
                        Ok(computed)
                    }
                    IntegerConstraintEnum::Range {
                        min,
                        max,
                        min_inclusive,
                        max_inclusive,
                    } => {
                        if *value < min || (!min_inclusive && *value == min) {
                            return Err(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_value_below_minimum",
                                [("value", value), ("minimum", min)],
                                source.clone(),
                                SpanSet::from_span(span),
                            ));
                        }
                        if *value > max || (!max_inclusive && *value == max) {
                            return Err(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_value_above_maximum",
                                [("value", value), ("maximum", max)],
                                source.clone(),
                                SpanSet::from_span(span),
                            ));
                        }
                        Ok(computed)
                    }
                    IntegerConstraintEnum::None => Ok(computed),
                }
            } else {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [("expected", "integer"), ("actual", format!("{computed:?}"))],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            }
        }
        Number(number_definition) => {
            // Validate that the computed value is a number (integer or float)
            match &computed {
                ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. } => {
                    let constraint = number_definition.constraint();
                    match constraint {
                        NumberConstraintEnum::Min { min, inclusive } => {
                            if (*value) < min || (!inclusive && (*value) <= min) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_below_minimum",
                                    [("value", value), ("minimum", min)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraintEnum::Max { max, inclusive } => {
                            if (*value) > max || (!inclusive && (*value) >= max) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_above_maximum",
                                    [("value", value), ("maximum", max)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraintEnum::Range {
                            min,
                            max,
                            min_inclusive,
                            max_inclusive,
                        } => {
                            if (*value) < min || (!min_inclusive && (*value) <= min) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_below_minimum",
                                    [("value", value), ("minimum", min)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            if (*value) > max || (!max_inclusive && (*value) >= max) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_above_maximum",
                                    [("value", value), ("maximum", max)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }

                            Ok(computed)
                        }
                        NumberConstraintEnum::None => Ok(computed),
                    }
                }
                ComputedItem::Boolean(_)
                | ComputedItem::Integer(_)
                | ComputedItem::String(_)
                | ComputedItem::Identifier(_)
                | ComputedItem::Path(_)
                | ComputedItem::Table(_)
                | ComputedItem::TableWithUnits(_)
                | ComputedItem::Unit(_) => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [("expected", "number"), ("actual", format!("{computed:?}"))],
                    source.clone(),
                    SpanSet::from_span(span),
                )),
            }
        }
        NumberWithUnits(number_definition) => {
            // Validate that the computed value is a number (integer or float)
            match &computed {
                ComputedItem::Float(value) => {
                    if number_definition.preferred_units() != UnitId::None {
                        return Err(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_expected_definition_value",
                            [
                                ("expected", "number with units"),
                                ("actual", format!("{computed:?}"))
                            ],
                            source.clone(),
                            SpanSet::from_span(span),
                        ));
                    }

                    let constraint = number_definition.constraint();
                    match constraint {
                        NumberConstraintEnum::Min { min, inclusive } => {
                            if (*value) < min || (!inclusive && (*value) <= min) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_below_minimum",
                                    [("value", value), ("minimum", min)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraintEnum::Max { max, inclusive } => {
                            if (*value) > max || (!inclusive && (*value) >= max) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_above_maximum",
                                    [("value", value), ("maximum", max)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraintEnum::Range {
                            min,
                            max,
                            min_inclusive,
                            max_inclusive,
                        } => {
                            if (*value) < min || (!min_inclusive && (*value) <= min) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_below_minimum",
                                    [("value", value), ("minimum", min)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            if (*value) > max || (!max_inclusive && (*value) >= max) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_above_maximum",
                                    [("value", value), ("maximum", max)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }

                            Ok(computed)
                        }
                        NumberConstraintEnum::None => Ok(computed),
                    }
                }
                ComputedItem::FloatWithUnit { value, unit } => {
                    if number_definition.preferred_units().family_id() != unit.family_id() {
                        return Err(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_expected_definition_value",
                            [
                                ("expected", "number with units"),
                                ("actual", format!("{computed:?}"))
                            ],
                            source.clone(),
                            SpanSet::from_span(span),
                        ));
                    }

                    let converted_value =
                        convert(*value, *unit, number_definition.preferred_units()).map_err(
                            |error| {
                                crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_unit_conversion_failed",
                                    [("error", error)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                )
                            },
                        )?;

                    let computed = ComputedItem::FloatWithUnit {
                        value: converted_value,
                        unit: number_definition.preferred_units(),
                    };

                    let constraint = number_definition.constraint();
                    match constraint {
                        NumberConstraintEnum::Min { min, inclusive } => {
                            if (converted_value) < min || (!inclusive && (converted_value) <= min) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_below_minimum",
                                    [("value", converted_value), ("minimum", min)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraintEnum::Max { max, inclusive } => {
                            if (converted_value) > max || (!inclusive && (converted_value) >= max) {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_above_maximum",
                                    [("value", converted_value), ("maximum", max)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraintEnum::Range {
                            min,
                            max,
                            min_inclusive,
                            max_inclusive,
                        } => {
                            if (converted_value) < min
                                || (!min_inclusive && (converted_value) <= min)
                            {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_below_minimum",
                                    [("value", converted_value), ("minimum", min)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }
                            if (converted_value) > max
                                || (!max_inclusive && (converted_value) >= max)
                            {
                                return Err(crate::expression_message!(
                                    ExpressionCategory::Evaluation,
                                    "expression_engine_evaluation_value_above_maximum",
                                    [("value", converted_value), ("maximum", max)],
                                    source.clone(),
                                    SpanSet::from_span(span),
                                ));
                            }

                            Ok(computed)
                        }
                        NumberConstraintEnum::None => Ok(computed),
                    }
                }
                ComputedItem::Boolean(_)
                | ComputedItem::Integer(_)
                | ComputedItem::String(_)
                | ComputedItem::Identifier(_)
                | ComputedItem::Path(_)
                | ComputedItem::Table(_)
                | ComputedItem::TableWithUnits(_)
                | ComputedItem::Unit(_) => Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [("expected", "number"), ("actual", format!("{computed:?}"))],
                    source.clone(),
                    SpanSet::from_span(span),
                )),
            }
        }
        String(_string_definition) => {
            // Validate that the computed value is a string
            if let ComputedItem::String(_) = &computed {
                Ok(computed)
            } else if let ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. } =
                &computed
            {
                Ok(ComputedItem::String(value.to_string().into()))
            } else if let ComputedItem::Integer(value) = &computed {
                Ok(ComputedItem::String(value.to_string().into()))
            } else {
                Err(crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_definition_value",
                    [("expected", "string"), ("actual", format!("{computed:?}"))],
                    source.clone(),
                    SpanSet::from_span(span),
                ))
            }
        }
        Unit(unit_definition) => validate_unit_value(unit_definition, &computed, source, span),
    }
}

/// Evaluates a number expression and converts float results to the definition's preferred units.
#[hotpath::measure]
fn evaluate_number_with_units_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    basic: &BasicInputWithUnitsData,
) -> Result<ComputedItem, Message> {
    let source = basic.data();
    let expression = string_to_expression(source)?;
    let span = expression_span(&expression);
    let is_float_literal = is_signed_float_literal(&expression);
    let is_identifier_reference =
        matches!(expression, Expression::Literal(_, Literal::Identifier(_)));
    let computed = evaluate_expression(computed_data, functions, source, expression)?;

    let NumberWithUnits(number_definition) = basic.definition() else {
        return Err(crate::expression_message!(
            ExpressionCategory::Evaluation,
            "expression_engine_evaluation_expected_number_with_units_definition",
            [],
            source.clone(),
            SpanSet::from_span(span),
        ));
    };

    let (value, source_unit) = match computed {
        ComputedItem::Float(value) => {
            let unit = if is_float_literal {
                UnitId::from_unit_id_str(basic.units().as_str()).ok_or_else(|| {
                    crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_unknown_unit",
                        [("unit", basic.units())],
                        source.clone(),
                        SpanSet::from_span(span),
                    )
                })?
            } else {
                UnitId::None
            };
            (value, unit)
        }
        ComputedItem::FloatWithUnit { value, unit } => (value, unit),
        ComputedItem::Boolean(_)
        | ComputedItem::Integer(_)
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => {
            return Err(crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_expected_definition_value",
                [("expected", "number"), ("actual", format!("{computed:?}"))],
                source.clone(),
                SpanSet::from_span(span),
            ));
        }
    };

    if source_unit == UnitId::None && !is_float_literal && !is_identifier_reference {
        return Ok(ComputedItem::Float(value));
    }

    let value =
        convert(value, source_unit, number_definition.preferred_units()).map_err(|error| {
            crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_unit_conversion_failed",
                [("error", error)],
                source.clone(),
                SpanSet::from_span(span),
            )
        })?;

    Ok(computed_float(value, number_definition.preferred_units()))
}

/// Evaluates all cells in a [`TableInputData`] and returns the resulting rows of `f64` values.
#[hotpath::measure]
fn evaluate_table_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    table: &TableInputData,
) -> Result<Vec<Vec<f64>>, Vec<Message>> {
    let parameter = table.parameter();
    if !parameter.as_str().is_empty() {
        let parameter_source = ShareableString::from(parameter.as_str().to_string());
        let parameter_span = Span::new(0, parameter.as_str().chars().count());
        let referenced = lookup_variable(
            computed_data,
            parameter.as_str(),
            &parameter_source,
            parameter_span,
        )
        .map_err(|e| vec![e])?;
        let referenced = match referenced {
            ComputedItem::TableWithUnits(referenced_table) => {
                ComputedItem::Table(referenced_table.into_table())
            }
            other @ (ComputedItem::Boolean(_)
            | ComputedItem::Integer(_)
            | ComputedItem::Float(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::String(_)
            | ComputedItem::Identifier(_)
            | ComputedItem::Path(_)
            | ComputedItem::Table(_)
            | ComputedItem::Unit(_)) => other,
        };
        return match referenced {
            ComputedItem::Table(referenced_table) => {
                let referenced_table = &referenced_table;
                let table_definition = table.definition();
                if table_definition.count() != referenced_table.keys().len() {
                    return Err(vec![crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_table_column_count_mismatch",
                        [
                            ("parameter", parameter),
                            ("actual", referenced_table.keys().len()),
                            ("expected", table.definition().count())
                        ],
                        parameter_source,
                        SpanSet::from_span(parameter_span),
                    )]);
                }

                let mut errors = Vec::new();

                let mut converted_rows = Vec::with_capacity(referenced_table.row_count());
                for row in referenced_table.rows() {
                    let mut converted_row = Vec::with_capacity(row.len());
                    for (j, data) in row.iter().enumerate() {
                        let Some(column_definition) = table_definition.get_by_index(j) else {
                            errors.push(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_table_missing_column_definition",
                                [("parameter", parameter), ("index", j)],
                                parameter_source.clone(),
                                SpanSet::from_span(parameter_span),
                            ));
                            continue;
                        };
                        let data = *data;
                        match column_definition.constraint() {
                            NumberConstraintEnum::Min { min, inclusive } => {
                                if data < min || (!inclusive && data <= min) {
                                    errors.push(crate::expression_message!(
                                        ExpressionCategory::Evaluation,
                                        "expression_engine_evaluation_table_value_below_minimum",
                                        [
                                            ("value", data),
                                            ("column", column_definition.description()),
                                            ("minimum", min)
                                        ],
                                        parameter_source.clone(),
                                        SpanSet::from_span(parameter_span),
                                    ));
                                }
                            }
                            NumberConstraintEnum::Max { max, inclusive } => {
                                if data > max || (!inclusive && data >= max) {
                                    errors.push(crate::expression_message!(
                                        ExpressionCategory::Evaluation,
                                        "expression_engine_evaluation_table_value_above_maximum",
                                        [
                                            ("value", data),
                                            ("column", column_definition.description()),
                                            ("maximum", max)
                                        ],
                                        parameter_source.clone(),
                                        SpanSet::from_span(parameter_span),
                                    ));
                                }
                            }
                            NumberConstraintEnum::Range {
                                min,
                                max,
                                min_inclusive,
                                max_inclusive,
                            } => {
                                if data < min || (!min_inclusive && data <= min) {
                                    errors.push(crate::expression_message!(
                                        ExpressionCategory::Evaluation,
                                        "expression_engine_evaluation_table_value_below_minimum",
                                        [
                                            ("value", data),
                                            ("column", column_definition.description()),
                                            ("minimum", min)
                                        ],
                                        parameter_source.clone(),
                                        SpanSet::from_span(parameter_span),
                                    ));
                                }
                                if data > max || (!max_inclusive && data >= max) {
                                    errors.push(crate::expression_message!(
                                        ExpressionCategory::Evaluation,
                                        "expression_engine_evaluation_table_value_above_maximum",
                                        [
                                            ("value", data),
                                            ("column", column_definition.description()),
                                            ("maximum", max)
                                        ],
                                        parameter_source.clone(),
                                        SpanSet::from_span(parameter_span),
                                    ));
                                }
                            }
                            NumberConstraintEnum::None => {}
                        }
                        converted_row.push(data);
                    }
                    converted_rows.push(converted_row);
                }

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(converted_rows)
            }
            other @ (ComputedItem::Boolean(_)
            | ComputedItem::Integer(_)
            | ComputedItem::Float(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::String(_)
            | ComputedItem::Identifier(_)
            | ComputedItem::Path(_)
            | ComputedItem::TableWithUnits(_)
            | ComputedItem::Unit(_)) => Err(vec![crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_expected_table_parameter",
                [("parameter", parameter), ("actual", format!("{other:?}"))],
                parameter_source,
                SpanSet::from_span(parameter_span),
            )]),
        };
    }

    let definition = table.definition();

    let mut evaluated_rows = Vec::new();

    for row in table.data() {
        let mut evaluated_row = Vec::new();
        for (i, basic_data) in row.iter().enumerate() {
            let Some(number_definition) = definition.get_by_index(i) else {
                let cell_source = ShareableString::from(basic_data.as_str().to_string());
                let cell_span = Span::new(0, basic_data.as_str().chars().count());
                return Err(vec![crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_table_cell_missing_column_definition",
                    [("index", i)],
                    cell_source,
                    SpanSet::from_span(cell_span),
                )]);
            };

            let basic_input_data =
                BasicInputData::new(Number(number_definition.clone()), basic_data.clone());

            match evaluate_basic_expression(computed_data, functions, &basic_input_data) {
                Ok(ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. }) => {
                    evaluated_row.push(value);
                }
                Ok(other) => {
                    let cell_source = ShareableString::from(basic_data.as_str().to_string());
                    let cell_span = Span::new(0, basic_data.as_str().chars().count());
                    return Err(vec![crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_expected_table_cell_number",
                        [("actual", format!("{other:?}"))],
                        cell_source,
                        SpanSet::from_span(cell_span),
                    )]);
                }
                Err(e) => {
                    return Err(vec![e]);
                }
            }
        }
        evaluated_rows.push(evaluated_row);
    }
    Ok(evaluated_rows)
}

/// Evaluates all cells in a [`TableWithUnitsInputData`] and returns the resulting rows of `f64` values.
#[hotpath::measure]
fn evaluate_table_with_units_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    table: &TableWithUnitsInputData,
) -> Result<Vec<Vec<f64>>, Vec<Message>> {
    let parameter = table.parameter();
    if !parameter.as_str().is_empty() {
        let parameter_source = ShareableString::from(parameter.as_str().to_string());
        let parameter_span = Span::new(0, parameter.as_str().chars().count());
        let referenced = lookup_variable(
            computed_data,
            parameter.as_str(),
            &parameter_source,
            parameter_span,
        )
        .map_err(|e| vec![e])?;

        let (referenced_table, source_units) = match referenced {
            ComputedItem::Table(referenced_table) => (referenced_table, None),
            ComputedItem::TableWithUnits(referenced_table) => {
                let (referenced_table, units) = referenced_table.into_table_and_units();
                (referenced_table, Some(units))
            }
            other @ (ComputedItem::Boolean(_)
            | ComputedItem::Integer(_)
            | ComputedItem::Float(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::String(_)
            | ComputedItem::Identifier(_)
            | ComputedItem::Path(_)
            | ComputedItem::Unit(_)) => {
                return Err(vec![crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_expected_table_parameter",
                    [("parameter", parameter), ("actual", format!("{other:?}"))],
                    parameter_source,
                    SpanSet::from_span(parameter_span),
                )]);
            }
        };

        let table_definition = table.definition();
        if table_definition.count() != referenced_table.keys().len() {
            return Err(vec![crate::expression_message!(
                ExpressionCategory::Evaluation,
                "expression_engine_evaluation_table_column_count_mismatch",
                [
                    ("parameter", parameter),
                    ("actual", referenced_table.keys().len()),
                    ("expected", table.definition().count())
                ],
                parameter_source,
                SpanSet::from_span(parameter_span),
            )]);
        }
        if let Some(units) = &source_units {
            if units.len() != referenced_table.column_count() {
                return Err(vec![crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_table_unit_count_mismatch",
                    [
                        ("parameter", parameter),
                        ("actual", units.len()),
                        ("expected", referenced_table.column_count())
                    ],
                    parameter_source,
                    SpanSet::from_span(parameter_span),
                )]);
            }
        }
        let mut errors = Vec::new();

        let mut converted_rows = Vec::with_capacity(referenced_table.row_count());
        for row in referenced_table.rows() {
            let mut converted_row = Vec::with_capacity(row.len());
            for (j, data) in row.iter().enumerate() {
                let Some(column_definition) = table_definition.get_by_index(j) else {
                    errors.push(crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_table_missing_column_definition",
                        [("parameter", parameter), ("index", j)],
                        parameter_source.clone(),
                        SpanSet::from_span(parameter_span),
                    ));
                    continue;
                };
                let source_unit = source_units
                    .as_ref()
                    .and_then(|units| units.get(j))
                    .copied()
                    .unwrap_or(UnitId::None);
                let data = match convert(*data, source_unit, column_definition.preferred_units()) {
                    Ok(value) => value,
                    Err(error) => {
                        errors.push(crate::expression_message!(
                            ExpressionCategory::Evaluation,
                            "expression_engine_evaluation_table_unit_conversion_failed",
                            [
                                ("parameter", parameter),
                                ("column", column_definition.description()),
                                ("source_unit", source_unit.string_id()),
                                (
                                    "target_unit",
                                    column_definition.preferred_units().string_id()
                                ),
                                ("error", error)
                            ],
                            parameter_source.clone(),
                            SpanSet::from_span(parameter_span),
                        ));
                        continue;
                    }
                };
                match column_definition.constraint() {
                    NumberConstraintEnum::Min { min, inclusive } => {
                        if data < min || (!inclusive && data <= min) {
                            errors.push(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_table_value_below_minimum",
                                [
                                    ("value", data),
                                    ("column", column_definition.description()),
                                    ("minimum", min)
                                ],
                                parameter_source.clone(),
                                SpanSet::from_span(parameter_span),
                            ));
                        }
                    }
                    NumberConstraintEnum::Max { max, inclusive } => {
                        if data > max || (!inclusive && data >= max) {
                            errors.push(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_table_value_above_maximum",
                                [
                                    ("value", data),
                                    ("column", column_definition.description()),
                                    ("maximum", max)
                                ],
                                parameter_source.clone(),
                                SpanSet::from_span(parameter_span),
                            ));
                        }
                    }
                    NumberConstraintEnum::Range {
                        min,
                        max,
                        min_inclusive,
                        max_inclusive,
                    } => {
                        if data < min || (!min_inclusive && data <= min) {
                            errors.push(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_table_value_below_minimum",
                                [
                                    ("value", data),
                                    ("column", column_definition.description()),
                                    ("minimum", min)
                                ],
                                parameter_source.clone(),
                                SpanSet::from_span(parameter_span),
                            ));
                        }
                        if data > max || (!max_inclusive && data >= max) {
                            errors.push(crate::expression_message!(
                                ExpressionCategory::Evaluation,
                                "expression_engine_evaluation_table_value_above_maximum",
                                [
                                    ("value", data),
                                    ("column", column_definition.description()),
                                    ("maximum", max)
                                ],
                                parameter_source.clone(),
                                SpanSet::from_span(parameter_span),
                            ));
                        }
                    }
                    NumberConstraintEnum::None => {}
                }
                converted_row.push(data);
            }
            converted_rows.push(converted_row);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        return Ok(converted_rows);
    }

    let definition = table.definition();

    let mut evaluated_rows = Vec::new();

    for row in table.data() {
        let mut evaluated_row = Vec::new();
        let units = table.units();

        for (i, basic_data) in row.iter().enumerate() {
            let Some(number_definition) = definition.get_by_index(i) else {
                let cell_source = ShareableString::from(basic_data.as_str().to_string());
                let cell_span = Span::new(0, basic_data.as_str().chars().count());
                return Err(vec![crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_table_cell_missing_column_definition",
                    [("index", i)],
                    cell_source,
                    SpanSet::from_span(cell_span),
                )]);
            };
            let Some(unit) = units.get(i) else {
                let cell_source = ShareableString::from(basic_data.as_str().to_string());
                let cell_span = Span::new(0, basic_data.as_str().chars().count());
                return Err(vec![crate::expression_message!(
                    ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_table_cell_missing_unit",
                    [("index", i)],
                    cell_source,
                    SpanSet::from_span(cell_span),
                )]);
            };

            let basic_input_data = BasicInputWithUnitsData::new(
                NumberWithUnits(number_definition.clone()),
                basic_data.clone(),
                unit.clone(),
            );

            match evaluate_number_with_units_expression(computed_data, functions, &basic_input_data)
            {
                Ok(ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, unit: _ }) => {
                    evaluated_row.push(value);
                }
                Ok(other) => {
                    let cell_source = ShareableString::from(basic_data.as_str().to_string());
                    let cell_span = Span::new(0, basic_data.as_str().chars().count());
                    return Err(vec![crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_expected_table_cell_number",
                        [("actual", format!("{other:?}"))],
                        cell_source,
                        SpanSet::from_span(cell_span),
                    )]);
                }
                Err(e) => {
                    return Err(vec![e]);
                }
            }
        }
        evaluated_rows.push(evaluated_row);
    }
    Ok(evaluated_rows)
}

/// Evaluates the given input data against the provided computed data, returning a new set of computed data
/// along with any errors encountered during evaluation.
#[hotpath::measure]
pub(crate) fn evaluator(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    input_data: &BTreeMap<ShareableString, ObjectItemInputData>,
) -> (BTreeMap<ShareableString, ComputedItem>, Vec<Message>) {
    let mut result = BTreeMap::new();
    let mut errors = Vec::new();

    for (key, data) in input_data {
        match data {
            ObjectItemInputData::Basic(basic_data) => {
                match evaluate_basic_expression(computed_data, functions, basic_data) {
                    Ok(computed_item) => {
                        result.insert(key.clone(), computed_item);
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            ObjectItemInputData::BasicWithUnits(basic_with_units_data) => {
                match evaluate_number_with_units_expression(
                    computed_data,
                    functions,
                    basic_with_units_data,
                ) {
                    Ok(computed_item) => {
                        result.insert(key.clone(), computed_item);
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            ObjectItemInputData::Table(table_data) => {
                // For table data, we need to evaluate the expression for each row.
                let keys = table_data
                    .definition()
                    .keys()
                    .map(ShareableString::from)
                    .collect();
                match evaluate_table_expression(computed_data, functions, table_data) {
                    Ok(evaluated_table) => {
                        result.insert(
                            key.clone(),
                            ComputedItem::Table(ComputedTable::new(keys, evaluated_table)),
                        );
                    }
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            }
            ObjectItemInputData::TableWithUnits(table_data) => {
                // For table data, we need to evaluate the expression for each row.
                let keys = table_data
                    .definition()
                    .keys()
                    .map(ShareableString::from)
                    .collect::<Vec<_>>();
                let units: Vec<UnitId> = table_data
                    .definition()
                    .iter()
                    .map(|(_, definition)| definition.preferred_units())
                    .collect();
                match evaluate_table_with_units_expression(computed_data, functions, table_data) {
                    Ok(evaluated_table) => {
                        let computed_table = if units.iter().all(|unit| *unit == UnitId::None) {
                            ComputedItem::Table(ComputedTable::new(keys, evaluated_table))
                        } else {
                            ComputedItem::TableWithUnits(ComputedTableWithUnits::new(
                                keys,
                                units,
                                evaluated_table,
                            ))
                        };
                        result.insert(key.clone(), computed_table);
                    }
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            }
        }
    }

    (result, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use datastore::prelude::*;
    use std::ops::AddAssign;

    fn message_text(message: &Message) -> std::string::String {
        message
            .translated_message(
                &crate::evaluation::expression::translations::get_error_message_translations(),
                "en",
            )
            .expect("expression messages should be translated")
            .to_string()
    }

    fn assert_non_finite_expression_is_rejected(expression: &str, functions: &FunctionDefinitions) {
        let source = ShareableString::from(expression);
        let expression = string_to_expression(&source).expect("expression should parse");
        let result = evaluate_expression(&BTreeMap::new(), functions, &source, expression);

        assert!(
            result.is_err_and(|error| message_text(&error).contains("must be finite")),
            "{source} should reject a non-finite result"
        );
    }

    #[test]
    fn rejects_non_finite_float_results() {
        let functions =
            crate::evaluation::expression::function_definitions_default::default_function_definitions();

        for expression in [
            "1e308 * 1e308",
            "(-1.0) ^ 0.5",
            "sqrt(-1.0)",
            "log(0.0)",
            "exp(1000.0)",
        ] {
            assert_non_finite_expression_is_rejected(expression, &functions);
        }
    }

    #[test]
    fn rejects_non_finite_computed_values_and_custom_function_results() {
        let source = ShareableString::from("value");
        let expression = string_to_expression(&source).expect("expression should parse");
        let computed_data = BTreeMap::from([(
            ShareableString::from("value"),
            ComputedItem::Float(f64::NAN),
        )]);
        assert!(
            evaluate_expression(
                &computed_data,
                &FunctionDefinitions::new(),
                &source,
                expression
            )
            .is_err()
        );

        let source = ShareableString::from("not_finite()");
        let expression = string_to_expression(&source).expect("expression should parse");
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("not_finite"),
            "returns an invalid float",
            ArgumentCount::Exact { count: 0 },
            |_| Ok(ComputedItem::Float(f64::NAN)),
        ));
        assert!(evaluate_expression(&BTreeMap::new(), &functions, &source, expression).is_err());
    }

    fn create_number_basic_input_data(value: &str) -> ObjectItemInputData {
        let definition = Number(NumberDefinition::new("Test Number"));
        let data = ShareableString::from(value.to_string());
        ObjectItemInputData::Basic(BasicInputData::new(definition, data))
    }

    fn create_number_with_units_input_data(
        value: &str,
        units: &str,
        preferred_units: UnitId,
    ) -> ObjectItemInputData {
        let definition = NumberWithUnits(NumberWithUnitsDefinition::new(
            "Test Number With Units",
            preferred_units,
        ));
        ObjectItemInputData::BasicWithUnits(BasicInputWithUnitsData::new(
            definition,
            value.into(),
            units.into(),
        ))
    }

    fn check_number_float(computed_item: &ComputedItem, expected_value: f64) {
        match computed_item {
            ComputedItem::Float(value) => assert_eq!(*value, expected_value),
            ComputedItem::Boolean(_)
            | ComputedItem::Integer(_)
            | ComputedItem::FloatWithUnit { .. }
            | ComputedItem::String(_)
            | ComputedItem::Identifier(_)
            | ComputedItem::Path(_)
            | ComputedItem::Table(_)
            | ComputedItem::TableWithUnits(_)
            | ComputedItem::Unit(_) => panic!("Expected a numeric computed item"),
        }
    }

    fn create_integer_basic_input_data(value: &str) -> ObjectItemInputData {
        let definition = Integer(IntegerDefinition::new("Test Integer"));
        let data = ShareableString::from(value.to_string());
        ObjectItemInputData::Basic(BasicInputData::new(definition, data))
    }

    fn check_number_integer(computed_item: &ComputedItem, expected_value: i64) {
        match computed_item {
            ComputedItem::Integer(value) => assert_eq!(*value, expected_value),
            _ => panic!("Expected a numeric computed item"),
        }
    }

    fn create_boolean_basic_input_data(value: &str) -> ObjectItemInputData {
        let definition = Boolean(BooleanDefinition::new("Test Boolean"));
        let data = ShareableString::from(value.to_string());
        ObjectItemInputData::Basic(BasicInputData::new(definition, data))
    }

    fn check_boolean(computed_item: &ComputedItem, expected_value: bool) {
        match computed_item {
            ComputedItem::Boolean(value) => assert_eq!(*value, expected_value),
            _ => panic!("Expected a boolean computed item"),
        }
    }

    #[test]
    fn unit_definition_evaluates_to_a_typed_unit() {
        let definition = UnitDefinition::new("Length", units::UnitFamilyId::Length);
        let functions = FunctionDefinitions::new();
        let computed_data = BTreeMap::new();

        let bare_unit = BasicInputData::new(
            BasicDefinition::Unit(definition.clone()),
            "u_length_meter".into(),
        );
        assert_eq!(
            evaluate_basic_expression(&computed_data, &functions, &bare_unit),
            Ok(ComputedItem::Unit(UnitId::Length_Meter))
        );

        let quoted_unit = BasicInputData::new(
            BasicDefinition::Unit(definition),
            "\"u_length_foot\"".into(),
        );
        assert_eq!(
            evaluate_basic_expression(&computed_data, &functions, &quoted_unit),
            Ok(ComputedItem::Unit(UnitId::Length_Foot))
        );
    }

    #[test]
    fn unit_definition_rejects_a_unit_from_another_family() {
        let definition = UnitDefinition::new("Length", units::UnitFamilyId::Length);
        let source = ShareableString::from("selected_unit");

        let result = validate_unit_value(
            &definition,
            &ComputedItem::Unit(UnitId::Time_Second),
            &source,
            Span::new(0, source.as_str().chars().count()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::new();
        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(result.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn boolean_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([("x".into(), create_boolean_basic_input_data("true"))]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_boolean(&result["x"], true);
    }

    #[test]
    fn boolean_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([(
            "x".into(),
            create_boolean_basic_input_data("true && false || true"),
        )]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_boolean(&result["x"], true);
    }

    #[test]
    fn multiple_boolean_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([
            ("a".into(), create_boolean_basic_input_data("true && false")),
            ("b".into(), create_boolean_basic_input_data("true || false")),
            ("c".into(), create_boolean_basic_input_data("!true")),
            ("d".into(), create_boolean_basic_input_data("!false")),
            ("e".into(), create_boolean_basic_input_data("true == false")),
            ("f".into(), create_boolean_basic_input_data("true != false")),
        ]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_boolean(&result["a"], false);
        check_boolean(&result["b"], true);
        check_boolean(&result["c"], false);
        check_boolean(&result["d"], true);
        check_boolean(&result["e"], false);
        check_boolean(&result["f"], true);
    }

    #[test]
    fn integer_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([("x".into(), create_integer_basic_input_data("42"))]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_integer(&result["x"], 42);
    }

    #[test]
    fn integer_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data =
            BTreeMap::from([("x".into(), create_integer_basic_input_data("1 + 2 * 3"))]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_integer(&result["x"], 7);
    }

    #[test]
    fn multiple_integer_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([
            ("a".into(), create_integer_basic_input_data("1 + 2")),
            ("b".into(), create_integer_basic_input_data("1 - 2")),
            ("c".into(), create_integer_basic_input_data("1 * 2")),
            ("d".into(), create_integer_basic_input_data("1 / 2")),
            ("e".into(), create_integer_basic_input_data("1 % 2")),
            ("f".into(), create_integer_basic_input_data("1 ^ 2")),
            ("h".into(), create_integer_basic_input_data("-1 + 2")),
            (
                "g".into(),
                create_integer_basic_input_data("1 + 2 * 3 - 4 / 5 ^ 6"),
            ),
            ("i".into(), create_integer_basic_input_data("-(1 + 2)")),
            ("j".into(), create_integer_basic_input_data("--1")),
        ]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_integer(&result["a"], 3);
        check_number_integer(&result["b"], -1);
        check_number_integer(&result["c"], 2);
        check_number_integer(&result["d"], 0);
        check_number_integer(&result["e"], 1);
        check_number_integer(&result["f"], 1);
        check_number_integer(&result["g"], 7);
        check_number_integer(&result["h"], 1);
        check_number_integer(&result["i"], -3);
        check_number_integer(&result["j"], 1);
    }

    #[test]
    fn integer_power_overflow_returns_an_error() {
        let input_data = BTreeMap::from([("x".into(), create_integer_basic_input_data("2 ^ 63"))]);

        let (result, errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);

        assert!(result.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(message_text(&errors[0]).contains("Integer overflow."));
    }

    #[test]
    fn float_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([("x".into(), create_number_basic_input_data("42.0"))]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_float(&result["x"], 42.0);
    }

    #[test]
    fn number_with_units_converts_float_to_preferred_units() {
        let input_data = BTreeMap::from([(
            "distance".into(),
            create_number_with_units_input_data("1.0", "u_length_meter", UnitId::Length_Foot),
        )]);

        let (result, errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);

        assert!(errors.is_empty());
        match result.get("distance") {
            Some(ComputedItem::FloatWithUnit { value, unit }) => {
                assert!((*value - 3.280_839_895_013_123).abs() < f64::EPSILON);
                assert_eq!(*unit, UnitId::Length_Foot);
            }
            other => panic!("expected converted float with unit, got {other:?}"),
        }
    }

    #[test]
    fn number_with_units_converts_negative_float_literal_to_preferred_units() {
        let input_data = BTreeMap::from([(
            "temperature".into(),
            create_number_with_units_input_data(
                "-1.0",
                "u_temperature_celsius",
                UnitId::Temperature_Fahrenheit,
            ),
        )]);

        let (result, errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);

        assert!(errors.is_empty());
        assert_eq!(
            result.get("temperature"),
            Some(&ComputedItem::FloatWithUnit {
                value: 30.2,
                unit: UnitId::Temperature_Fahrenheit,
            })
        );
    }

    #[test]
    fn numeric_operation_drops_units() {
        let input_data = BTreeMap::from([(
            "distance".into(),
            create_number_with_units_input_data(
                "1.0 + 1.0",
                "u_length_meter",
                UnitId::Length_Meter,
            ),
        )]);
        let computed_data = BTreeMap::from([
            (
                "left".into(),
                ComputedItem::FloatWithUnit {
                    value: 2.0,
                    unit: UnitId::Length_Meter,
                },
            ),
            (
                "right".into(),
                ComputedItem::FloatWithUnit {
                    value: 4.0,
                    unit: UnitId::Time_Second,
                },
            ),
        ]);
        let operation_input = BTreeMap::from([(
            "speed".into(),
            create_number_basic_input_data("left / right"),
        )]);

        let (distance, distance_errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);
        assert!(distance_errors.is_empty());
        assert_eq!(distance.get("distance"), Some(&ComputedItem::Float(2.0)));

        let (result, errors) = evaluator(
            &computed_data,
            &FunctionDefinitions::new(),
            &operation_input,
        );
        assert!(errors.is_empty());
        assert_eq!(result.get("speed"), Some(&ComputedItem::Float(0.5)));
    }

    #[test]
    fn unitless_reference_cannot_be_converted_to_a_unit() {
        let computed_data = BTreeMap::from([("unitless".into(), ComputedItem::Float(2.0))]);
        let input_data = BTreeMap::from([(
            "distance".into(),
            create_number_with_units_input_data("unitless", "u_length_meter", UnitId::Length_Meter),
        )]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);

        assert!(result.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(message_text(&errors[0]).contains("Cannot convert a unitless value to a unit"));
    }

    #[test]
    fn float_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("1.0 + 2.0 * 3.0"),
        )]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_float(&result["x"], 7.0);
    }

    #[test]
    fn multiple_float_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([
            ("a".into(), create_number_basic_input_data("1.0 + 2.0")),
            ("b".into(), create_number_basic_input_data("1.0 - 2.0")),
            ("c".into(), create_number_basic_input_data("1.0 * 2.0")),
            ("d".into(), create_number_basic_input_data("1.0 / 2.0")),
            ("e".into(), create_number_basic_input_data("1.0 % 2.0")),
            ("f".into(), create_number_basic_input_data("1.0 ^ 2.0")),
            (
                "g".into(),
                create_number_basic_input_data("1.0 + 2.0 * 3.0 - 4.0 / 5.0 ^ 6.0"),
            ),
            ("h".into(), create_number_basic_input_data("-1.0 + 2.0")),
            ("i".into(), create_number_basic_input_data("-(1.0 + 2.0)")),
            ("j".into(), create_number_basic_input_data("--1.0")),
        ]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_float(&result["a"], 3.0);
        check_number_float(&result["b"], -1.0);
        check_number_float(&result["c"], 2.0);
        check_number_float(&result["d"], 0.5);
        check_number_float(&result["e"], 1.0);
        check_number_float(&result["f"], 1.0);
        check_number_float(&result["g"], 6.999744);
        check_number_float(&result["h"], 1.0);
        check_number_float(&result["i"], -3.0);
        check_number_float(&result["j"], 1.0);
    }

    fn create_table_computed_item(rows: Vec<Vec<(&str, f64)>>) -> ComputedItem {
        let keys = rows
            .first()
            .map(|row| {
                row.iter()
                    .map(|(key, _)| ShareableString::from(*key))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let table_rows = rows
            .into_iter()
            .map(|row| row.into_iter().map(|(_, value)| value).collect())
            .collect();
        ComputedItem::Table(ComputedTable::new(keys, table_rows))
    }

    #[test]
    fn index_out_of_bounds_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 1.0)]]),
        )]);

        let source = ShareableString::from("t[5]");
        let expression = string_to_expression(&source).unwrap();
        assert!(
            evaluate_expression(
                &computed_data,
                &FunctionDefinitions::new(),
                &source,
                expression
            )
            .is_err()
        );
    }

    #[test]
    fn field_access_via_bracket_indexing_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 3.5)]]),
        )]);
        let source = ShareableString::from("t[0][col]");
        let expression = string_to_expression(&source).unwrap();

        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            &source,
            expression,
        )
        .unwrap();

        check_number_float(&result, 3.5);
    }

    #[test]
    fn field_access_on_table_with_units_preserves_column_unit() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            ComputedItem::TableWithUnits(ComputedTableWithUnits::new(
                vec!["length".into()],
                vec![UnitId::Length_Meter],
                vec![vec![3.5]],
            )),
        )]);
        let source = ShareableString::from("t[0][length]");
        let expression = string_to_expression(&source).unwrap();

        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            &source,
            expression,
        )
        .unwrap();

        assert_eq!(
            result,
            ComputedItem::FloatWithUnit {
                value: 3.5,
                unit: UnitId::Length_Meter,
            }
        );
    }

    #[test]
    fn field_access_missing_field_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 3.5)]]),
        )]);

        let source = ShareableString::from("t[0][missing]");
        let expression = string_to_expression(&source).unwrap();
        assert!(
            evaluate_expression(
                &computed_data,
                &FunctionDefinitions::new(),
                &source,
                expression
            )
            .is_err()
        );
    }

    #[test]
    fn quoted_string_literal_evaluates_to_its_contents_without_variable_lookup() {
        // Unlike a bare atom, a quoted string is never looked up in `computed_data`, even if a
        // variable with a matching name exists.
        let computed_data = BTreeMap::from([("hello".into(), ComputedItem::Integer(1))]);
        let source = ShareableString::from("\"hello\"");
        let expression = string_to_expression(&source).unwrap();

        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            &source,
            expression,
        )
        .unwrap();

        assert_eq!(result, ComputedItem::String(ShareableString::from("hello")));
    }

    #[test]
    fn quoted_string_used_as_field_index_is_a_literal_field_name() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 3.5)]]),
        )]);
        let source = ShareableString::from("t[0][\"col\"]");
        let expression = string_to_expression(&source).unwrap();

        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            &source,
            expression,
        )
        .unwrap();

        check_number_float(&result, 3.5);
    }

    #[test]
    fn field_access_on_multi_row_table_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 1.0)], vec![("col", 2.0)]]),
        )]);

        let source = ShareableString::from("t[col]");
        let expression = string_to_expression(&source).unwrap();
        assert!(
            evaluate_expression(
                &computed_data,
                &FunctionDefinitions::new(),
                &source,
                expression
            )
            .is_err()
        );
    }

    #[test]
    fn field_access_and_index_via_evaluator_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 1.0)], vec![("col", 9.0)]]),
        )]);
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("t[1][col]"))]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(errors.is_empty());

        check_number_float(&result["x"], 9.0);
    }

    #[test]
    fn index_index_via_evaluator_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 1.0)], vec![("col", 9.0)]]),
        )]);
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("t[1][col]"))]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(errors.is_empty());

        check_number_float(&result["x"], 9.0);
    }

    #[test]
    fn map_key_entry_index_index_test() {
        // A map-style variable stored under the key "map[key][entry]" holding a table.
        // The expression `map[key][entry][row][col]` should first resolve the map key,
        // then index into the resulting table using the remaining two indices.
        let computed_data = BTreeMap::from([(
            "map[key][entry]".into(),
            create_table_computed_item(vec![vec![("col", 1.0)], vec![("col", 9.0)]]),
        )]);

        let source = ShareableString::from("map[key][entry][1][col]");
        let expression = string_to_expression(&source).unwrap();
        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            &source,
            expression,
        )
        .unwrap();

        check_number_float(&result, 9.0);
    }

    /// A helper that sums numeric arguments (Integer or Float) into a Float.
    fn sum_function(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
        let mut total = 0.0;
        for arg in args {
            match arg {
                ComputedItem::Float(v) => total.add_assign(v),
                ComputedItem::Integer(v) => total.add_assign(*v as f64),
                other => {
                    return Err(crate::expression_message!(
                        ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_sum_requires_numeric_arguments",
                        [("actual", format!("{other:?}"))],
                    ));
                }
            }
        }
        Ok(ComputedItem::Float(total))
    }

    /// A function with no arguments that always returns the float value of `42.0`.
    fn constant_function(_args: &[ComputedItem]) -> Result<ComputedItem, Message> {
        Ok(ComputedItem::Float(42.0))
    }

    #[test]
    fn function_call_with_no_arguments_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("constant"),
            "returns 42",
            ArgumentCount::Unbounded,
            constant_function,
        ));
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("constant()"))]);

        let (result, errors) = evaluator(&BTreeMap::new(), &functions, &input_data);
        assert!(errors.is_empty());
        match result.get("x") {
            Some(ComputedItem::Float(v)) => assert_eq!(*v, 42.0),
            other => panic!("expected float 42.0, got {other:?}"),
        }
    }

    #[test]
    fn function_call_with_arguments_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            sum_function,
        ));
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("sum(1.0, 2.0, 3.5)"),
        )]);

        let (result, errors) = evaluator(&BTreeMap::new(), &functions, &input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 6.5);
    }

    #[test]
    fn function_call_with_variable_arguments_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            sum_function,
        ));
        let computed_data = BTreeMap::from([("a".into(), ComputedItem::Float(1.5))]);
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("sum(a, 2.0)"))]);

        let (result, errors) = evaluator(&computed_data, &functions, &input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 3.5);
    }

    #[test]
    fn nested_function_calls_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            sum_function,
        ));
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("sum(sum(1.0, 2.0), sum(3.0, 4.0))"),
        )]);

        let (result, errors) = evaluator(&BTreeMap::new(), &functions, &input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 10.0);
    }

    #[test]
    fn function_call_combined_with_operators_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            sum_function,
        ));
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("sum(1.0, 2.0) * 3.0 + 1.0"),
        )]);

        let (result, errors) = evaluator(&BTreeMap::new(), &functions, &input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 10.0);
    }

    #[test]
    fn undefined_function_call_returns_error_test() {
        let functions = FunctionDefinitions::new();
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("undefined()"))]);

        let (result, errors) = evaluator(&BTreeMap::new(), &functions, &input_data);
        assert!(result.is_empty());
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].category(),
            message::message::MessageCategory::ExpressionEvaluation
        );
        assert!(message_text(&errors[0]).contains("Function 'undefined' is not defined."));
    }

    use datastore::definition::TableDefinition;

    #[test]
    fn table_parameter_test() {
        let computed_data = BTreeMap::from([(
            "table".into(),
            ComputedItem::Table(ComputedTable::new(
                vec!["a".into(), "b".into()],
                vec![vec![5.0, 6.0], vec![7.0, 8.0]],
            )),
        )]);

        let table_definition = TableDefinition::new(
            "Test Table",
            vec![
                (store_key!("col1"), NumberDefinition::new("")),
                (store_key!("col2"), NumberDefinition::new("")),
            ],
        );
        let table_data = vec![
            vec![ShareableString::from("1.0"), ShareableString::from("2.0")],
            vec![ShareableString::from("3.0"), ShareableString::from("4.0")],
        ];
        let table_input_data = ObjectItemInputData::Table(TableInputData::new(
            table_definition,
            "table".into(),
            table_data,
        ));

        let input_data = BTreeMap::from([("table".into(), table_input_data)]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert!(errors.is_empty());

        if let ComputedItem::Table(computed_table) = &result["table"] {
            assert_eq!(computed_table.rows().len(), 2);
            assert_eq!(computed_table.rows()[0], vec![5.0, 6.0]);
            assert_eq!(computed_table.rows()[1], vec![7.0, 8.0]);
        } else {
            panic!("Expected a computed table");
        }
    }

    #[test]
    fn table_with_units_parameter_converts_values_to_target_units() {
        let computed_data = BTreeMap::from([(
            "source".into(),
            ComputedItem::TableWithUnits(ComputedTableWithUnits::new(
                vec!["length".into()],
                vec![UnitId::Length_Centimeter],
                vec![vec![100.0]],
            )),
        )]);
        let table_definition = TableWithUnitsDefinition::new(
            "Target Table",
            vec![(
                store_key!("length"),
                NumberWithUnitsDefinition::new("length", UnitId::Length_Meter),
            )],
        );
        let table_input_data = ObjectItemInputData::TableWithUnits(TableWithUnitsInputData::new(
            table_definition,
            "source".into(),
            vec![],
            vec![],
        ));
        let input_data = BTreeMap::from([("target".into(), table_input_data)]);

        let (result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);

        assert!(errors.is_empty());
        let ComputedItem::TableWithUnits(table) = &result["target"] else {
            panic!("Expected a computed table with units");
        };
        assert_eq!(table.units(), &[UnitId::Length_Meter]);
        assert_eq!(table.rows(), &[vec![1.0]]);
    }

    #[test]
    fn table_with_only_none_units_produces_unitless_computed_table() {
        let table_definition = TableWithUnitsDefinition::new(
            "Unitless Table",
            vec![(
                store_key!("value"),
                NumberWithUnitsDefinition::new("value", UnitId::None),
            )],
        );
        let table_input_data = ObjectItemInputData::TableWithUnits(TableWithUnitsInputData::new(
            table_definition,
            "".into(),
            vec![UnitId::None.string_id().into()],
            vec![vec![ShareableString::from("1.0")]],
        ));
        let input_data = BTreeMap::from([("table".into(), table_input_data)]);

        let (result, errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);

        assert!(errors.is_empty());
        let ComputedItem::Table(table) = &result["table"] else {
            panic!("Expected a unitless computed table");
        };
        assert_eq!(table.rows(), &[vec![1.0]]);
    }

    #[test]
    fn table_with_units_evaluates_cells_in_preferred_units() {
        let table_definition = TableWithUnitsDefinition::new(
            "Length Table",
            vec![(
                store_key!("length"),
                NumberWithUnitsDefinition::new("length", UnitId::Length_Meter),
            )],
        );
        let table_input_data = ObjectItemInputData::TableWithUnits(TableWithUnitsInputData::new(
            table_definition,
            "".into(),
            vec![UnitId::Length_Meter.string_id().into()],
            vec![vec![ShareableString::from("1.0")]],
        ));
        let input_data = BTreeMap::from([("table".into(), table_input_data)]);

        let (result, errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);

        assert!(errors.is_empty());
        let ComputedItem::TableWithUnits(table) = &result["table"] else {
            panic!("Expected a computed table with units");
        };
        assert_eq!(table.units(), &[UnitId::Length_Meter]);
        assert_eq!(table.rows(), &[vec![1.0]]);
    }

    #[test]
    fn table_parameter_not_found_test() {
        // Why: A parameter that names a non-existent variable produces an error.
        let table_definition = TableDefinition::new(
            "Test Table",
            vec![(store_key!("col1"), NumberDefinition::new(""))],
        );
        let table_input_data = ObjectItemInputData::Table(TableInputData::new(
            table_definition,
            "missing".into(),
            vec![vec![ShareableString::from("1.0")]],
        ));

        let input_data = BTreeMap::from([("t".into(), table_input_data)]);
        let (_result, errors) =
            evaluator(&BTreeMap::new(), &FunctionDefinitions::new(), &input_data);
        assert_eq!(errors.len(), 1);
        assert!(message_text(&errors[0]).contains("missing"));
    }

    #[test]
    fn table_parameter_not_a_table_test() {
        // Why: A parameter that references a non-table computed item produces an error.
        let computed_data = BTreeMap::from([("not_a_table".into(), ComputedItem::Float(1.0))]);
        let table_definition = TableDefinition::new(
            "Test Table",
            vec![(store_key!("col1"), NumberDefinition::new(""))],
        );
        let table_input_data = ObjectItemInputData::Table(TableInputData::new(
            table_definition,
            "not_a_table".into(),
            vec![vec![ShareableString::from("1.0")]],
        ));

        let input_data = BTreeMap::from([("t".into(), table_input_data)]);
        let (_result, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert_eq!(errors.len(), 1);
        assert!(message_text(&errors[0]).contains("not_a_table"));
    }

    #[test]
    fn table_parameter_mismatched_size_test() {
        let computed_data = BTreeMap::from([(
            "table".into(),
            ComputedItem::Table(ComputedTable::new(
                vec!["a".into(), "b".into(), "c".into()],
                vec![vec![5.0, 6.0, 7.0], vec![8.0, 9.0, 10.0]],
            )),
        )]);

        let table_definition = TableDefinition::new(
            "Test Table",
            vec![
                (store_key!("col1"), NumberDefinition::new("")),
                (store_key!("col2"), NumberDefinition::new("")),
            ],
        );
        let table_data = vec![
            vec![ShareableString::from("1.0"), ShareableString::from("2.0")],
            vec![ShareableString::from("3.0"), ShareableString::from("4.0")],
        ];
        let table_input_data = ObjectItemInputData::Table(TableInputData::new(
            table_definition,
            "table".into(),
            table_data,
        ));

        let input_data = BTreeMap::from([("table".into(), table_input_data)]);

        let (_, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert_eq!(errors.len(), 1);
        assert!(message_text(&errors[0]).contains(
            "Parameter 'table' references a table with 3 columns, but the current table expects 2 columns."
        ));
    }

    #[test]
    fn table_parameter_constraint_test() {
        let computed_data = BTreeMap::from([(
            "table".into(),
            ComputedItem::Table(ComputedTable::new(
                vec!["a".into(), "b".into()],
                vec![vec![5.0, 6.1], vec![8.2, 9.0]],
            )),
        )]);

        let table_definition = TableDefinition::new(
            "Test Table",
            vec![
                (
                    store_key!("col1"),
                    NumberDefinition::new_with_constraint(
                        "column 1",
                        NumberConstraint::max(5.0, true),
                    ),
                ),
                (
                    store_key!("col2"),
                    NumberDefinition::new_with_constraint(
                        "column 2",
                        NumberConstraint::min(10.0, true),
                    ),
                ),
            ],
        );
        let table_data = vec![
            vec![ShareableString::from("1.0"), ShareableString::from("2.0")],
            vec![ShareableString::from("3.0"), ShareableString::from("4.0")],
        ];
        let table_input_data = ObjectItemInputData::Table(TableInputData::new(
            table_definition,
            "table".into(),
            table_data,
        ));

        let input_data = BTreeMap::from([("table".into(), table_input_data)]);

        let (_, errors) = evaluator(&computed_data, &FunctionDefinitions::new(), &input_data);
        assert_eq!(errors.len(), 3);
        assert!(message_text(&errors[0]).contains(
            "Value 6.1 in column 'column 2' is less than the minimum allowed value of 10."
        ));
        assert!(message_text(&errors[1]).contains(
            "Value 8.2 in column 'column 1' is greater than the maximum allowed value of 5."
        ));
        assert!(message_text(&errors[2]).contains(
            "Value 9 in column 'column 2' is less than the minimum allowed value of 10."
        ));
    }
}
