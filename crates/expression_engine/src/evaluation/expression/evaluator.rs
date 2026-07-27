use crate::BasicDefinition::{Boolean, Choice, File, Integer, Number, String};
use crate::evaluation::expression::function_definition::FunctionDefinitions;
use crate::expression::parser::parse;
use crate::expression::translator::{Expression, Literal, Operators, translate};
use crate::{
    BasicInputData, ComputedItem, ComputedTable, ExpressionCategory, ExpressionError,
    ObjectItemInputData, TableInputData,
};
use datastore::definition::{IntegerConstraint, NumberConstraint};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

fn lookup_variable(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    variable_name: &str,
) -> Result<ComputedItem, ExpressionError> {
    let key = ShareableString::from(variable_name);
    match computed_data.get(&key) {
        Some(computed_item) => Ok(computed_item.clone()),
        None => Err(ExpressionError::new_simple(
            ExpressionCategory::Evaluation,
            format!("Variable '{}' not found in computed data.", variable_name),
        )),
    }
}

fn evaluate_expression_impl(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    expression: Expression,
    source: &str,
) -> Result<ComputedItem, ExpressionError> {
    match expression {
        Expression::Literal(literal, _) => match literal {
            Literal::Integer(value) => Ok(ComputedItem::Integer(value)),
            Literal::Float(value) => Ok(ComputedItem::Float(value)),
            Literal::String(value) => Ok(lookup_variable(computed_data, &value)?),
            Literal::Boolean(value) => Ok(ComputedItem::Boolean(value)),
        },
        Expression::UnaryOperation { operator, operand, .. } => {
            let operand_value = evaluate_expression(computed_data, functions, *operand, source)?;
            match (operator, operand_value) {
                (Operators::Negate, ComputedItem::Float(value)) => Ok(ComputedItem::Float(-value)),
                (Operators::Negate, ComputedItem::Integer(value)) => {
                    Ok(ComputedItem::Integer(-value))
                }
                (Operators::Not, ComputedItem::Boolean(value)) => Ok(ComputedItem::Boolean(!value)),
                _ => Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    "Invalid unary operation.".to_string(),
                )),
            }
        }
        Expression::BinaryOperation {
            left,
            operator,
            right,
            ..
        } => {
            let left_value = evaluate_expression(computed_data, functions, *left, source)?;
            let right_value = evaluate_expression(computed_data, functions, *right, source)?;
            match (left_value, right_value) {
                (ComputedItem::Boolean(left_bool), ComputedItem::Boolean(right_bool)) => {
                    match operator {
                        Operators::And => Ok(ComputedItem::Boolean(left_bool && right_bool)),
                        Operators::Or => Ok(ComputedItem::Boolean(left_bool || right_bool)),
                        Operators::Equal => Ok(ComputedItem::Boolean(left_bool == right_bool)),
                        Operators::NotEqual => Ok(ComputedItem::Boolean(left_bool != right_bool)),
                        _ => Err(ExpressionError::new_simple(
                            ExpressionCategory::Evaluation,
                            format!("Unsupported operator for booleans: {:?}", operator),
                        )),
                    }
                }
                (ComputedItem::Boolean(_left_bool), ComputedItem::File(_right_file)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Boolean(_left_bool), ComputedItem::Float(_right_float)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Boolean(_left_bool), ComputedItem::Integer(_right_int)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Boolean(_left_bool), ComputedItem::String(_right_string)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Boolean(_left_bool), ComputedItem::Table(_right_table)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::File(_left_file), ComputedItem::Boolean(_right_bool)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::File(left_file), ComputedItem::File(right_file)) => match operator {
                    Operators::Equal => Ok(ComputedItem::Boolean(left_file == right_file)),
                    Operators::NotEqual => Ok(ComputedItem::Boolean(left_file != right_file)),
                    _ => Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for files: {:?}", operator),
                    )),
                },
                (ComputedItem::File(_left_file), ComputedItem::Float(_right_float)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::File(_left_file), ComputedItem::Integer(_right_int)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::File(_left_file), ComputedItem::String(_right_string)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::File(_left_file), ComputedItem::Table(_right_table)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Float(_left_float), ComputedItem::Boolean(_right_bool)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Float(_left_float), ComputedItem::File(_right_file)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Float(left_float), ComputedItem::Float(right_float)) => {
                    match operator {
                        Operators::Add => Ok(ComputedItem::Float(left_float + right_float)),
                        Operators::Subtract => Ok(ComputedItem::Float(left_float - right_float)),
                        Operators::Multiply => Ok(ComputedItem::Float(left_float * right_float)),
                        Operators::Divide => {
                            if right_float == 0.0 {
                                Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    "Division by zero.".to_string(),
                                ))
                            } else {
                                Ok(ComputedItem::Float(left_float / right_float))
                            }
                        }
                        Operators::Modulus => {
                            if right_float == 0.0 {
                                Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    "Modulus by zero.".to_string(),
                                ))
                            } else {
                                Ok(ComputedItem::Float(left_float % right_float))
                            }
                        }
                        Operators::Power => Ok(ComputedItem::Float(left_float.powf(right_float))),
                        Operators::LessThan => Ok(ComputedItem::Boolean(left_float < right_float)),
                        Operators::LessThanOrEqual => {
                            Ok(ComputedItem::Boolean(left_float <= right_float))
                        }
                        Operators::GreaterThan => {
                            Ok(ComputedItem::Boolean(left_float > right_float))
                        }
                        Operators::GreaterThanOrEqual => {
                            Ok(ComputedItem::Boolean(left_float >= right_float))
                        }
                        _ => Err(ExpressionError::new_simple(
                            ExpressionCategory::Evaluation,
                            format!("Unsupported operator for floats: {:?}", operator),
                        )),
                    }
                }
                (ComputedItem::Float(_left_float), ComputedItem::Integer(_right_int)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Float(_left_float), ComputedItem::String(_right_string)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Float(_left_float), ComputedItem::Table(_right_table)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Integer(_left_int), ComputedItem::Boolean(_right_bool)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Integer(_left_int), ComputedItem::File(_right_file)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Integer(_left_int), ComputedItem::Float(_right_float)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Integer(left_int), ComputedItem::Integer(right_int)) => {
                    match operator {
                        Operators::Add => Ok(ComputedItem::Integer(left_int + right_int)),
                        Operators::Subtract => Ok(ComputedItem::Integer(left_int - right_int)),
                        Operators::Multiply => Ok(ComputedItem::Integer(left_int * right_int)),
                        Operators::Divide => {
                            if right_int == 0 {
                                Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    "Division by zero.".to_string(),
                                ))
                            } else {
                                Ok(ComputedItem::Integer(left_int / right_int))
                            }
                        }
                        Operators::Modulus => {
                            if right_int == 0 {
                                Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    "Modulus by zero.".to_string(),
                                ))
                            } else {
                                Ok(ComputedItem::Integer(left_int % right_int))
                            }
                        }
                        Operators::Power => {
                            Ok(ComputedItem::Integer(left_int.pow(right_int as u32)))
                        }
                        Operators::Equal => Ok(ComputedItem::Boolean(left_int == right_int)),
                        Operators::NotEqual => Ok(ComputedItem::Boolean(left_int != right_int)),
                        Operators::LessThan => Ok(ComputedItem::Boolean(left_int < right_int)),
                        Operators::LessThanOrEqual => {
                            Ok(ComputedItem::Boolean(left_int <= right_int))
                        }
                        Operators::GreaterThan => Ok(ComputedItem::Boolean(left_int > right_int)),
                        Operators::GreaterThanOrEqual => {
                            Ok(ComputedItem::Boolean(left_int >= right_int))
                        }
                        _ => Err(ExpressionError::new_simple(
                            ExpressionCategory::Evaluation,
                            format!("Unsupported operator for integers: {:?}", operator),
                        )),
                    }
                }
                (ComputedItem::Integer(_left_int), ComputedItem::String(_right_string)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Integer(_left_int), ComputedItem::Table(_right_table)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::String(_left_string), ComputedItem::Boolean(_right_bool)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::String(_left_string), ComputedItem::File(_right_file)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::String(_left_string), ComputedItem::Float(_right_float)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::String(_left_string), ComputedItem::Integer(_right_int)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::String(left_string), ComputedItem::String(right_string)) => {
                    match operator {
                        Operators::Equal => Ok(ComputedItem::Boolean(left_string == right_string)),
                        Operators::NotEqual => {
                            Ok(ComputedItem::Boolean(left_string != right_string))
                        }
                        _ => Err(ExpressionError::new_simple(
                            ExpressionCategory::Evaluation,
                            format!("Unsupported operator for strings: {:?}", operator),
                        )),
                    }
                }
                (ComputedItem::String(_left_string), ComputedItem::Table(_right_table)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Table(_left_table), ComputedItem::Boolean(_right_bool)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Table(_left_table), ComputedItem::File(_right_file)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Table(_left_table), ComputedItem::Float(_right_float)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Table(_left_table), ComputedItem::Integer(_right_int)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Table(_left_table), ComputedItem::String(_right_string)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for mixed types: {:?}", operator),
                    ))
                }
                (ComputedItem::Table(_left_table), ComputedItem::Table(_right_table)) => {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Unsupported operator for tables: {:?}", operator),
                    ))
                }
            }
        }
        Expression::FunctionCall { name, arguments, .. } => {
            let definition = functions.get(&name).ok_or_else(|| {
                ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!("Function '{}' is not defined.", name),
                )
            })?;

            let mut evaluated_arguments = Vec::with_capacity(arguments.len());
            for argument in arguments {
                evaluated_arguments.push(evaluate_expression(
                    computed_data,
                    functions,
                    argument,
                    source,
                )?);
            }

            definition.call(&evaluated_arguments)
        }
        Expression::Index { name, index, .. } => {
            if index.len() != 2 && index.len() != 4 {
                return Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Indexing requires exactly 2 or 4 indices, got {}",
                        index.len()
                    ),
                ));
            }

            let mut indexes = Vec::new();
            for index_expression in index {
                // A bare identifier used as an index (e.g. the `col` in `t[0][col]`) is a
                // literal field name, not a reference to a variable, so it is not looked up.
                let index_value =
                    if let Expression::Literal(Literal::String(name), _) = &index_expression {
                        if let Ok(value) = lookup_variable(computed_data, name) {
                            value
                        } else {
                            ComputedItem::String(ShareableString::from(name.clone()))
                        }
                    } else {
                        evaluate_expression(computed_data, functions, index_expression, source)?
                    };
                indexes.push(index_value);
            }

            let map_lookup = format!("{}[{}][{}]", name.clone(), indexes[0], indexes[1]);
            let item = if let Ok(value) = lookup_variable(computed_data, &map_lookup) {
                indexes.drain(0..2);
                value
            } else {
                lookup_variable(computed_data, &name)?
            };

            if let ComputedItem::Table(table) = item {
                if indexes.is_empty() {
                    return Ok(ComputedItem::Table(table));
                }

                let index_1 = &indexes[0];
                let index_2 = &indexes[1];

                let row_index = match index_1 {
                    ComputedItem::Integer(i) => {
                        if *i < 0 || (*i as usize) >= table.row_count() {
                            return Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!(
                                    "Row index {} is out of bounds for a table with {} rows.",
                                    i,
                                    table.row_count()
                                ),
                            ));
                        }
                        *i as usize
                    }
                    _ => {
                        return Err(ExpressionError::new_simple(
                            ExpressionCategory::Evaluation,
                            format!("Expected an integer index for the table, got {:?}", index_1),
                        ));
                    }
                };

                return match index_2 {
                    ComputedItem::String(s) => {
                        if let Some(value) = table.get_cell_by_name(row_index, s) {
                            Ok(ComputedItem::Float(value))
                        } else {
                            Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!("Field '{}' not found in the table row.", s),
                            ))
                        }
                    }
                    ComputedItem::Integer(i) => {
                        if let Some(value) = table.get_cell(row_index, *i as usize) {
                            Ok(ComputedItem::Float(value))
                        } else {
                            Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!("Field '{}' not found in the table row.", i),
                            ))
                        }
                    }
                    other => Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!(
                            "Expected a string or integer index for the table field, got {:?}",
                            other
                        ),
                    )),
                };
            }

            Ok(item)
        }
    }
}

/// Evaluates `expression` against `computed_data`, attaching source context and the
/// expression's span to any error that does not already carry them.
///
/// Errors are enriched via [`ExpressionError::with_context`]: the deepest sub-expression whose
/// evaluation fails supplies the span (the first caller wins), so an error points at the most
/// specific location available.
fn evaluate_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    expression: Expression,
    source: &str,
) -> Result<ComputedItem, ExpressionError> {
    let span = expression.span();
    evaluate_expression_impl(computed_data, functions, expression, source)
        .map_err(|e| e.with_context(source, &span))
}

fn parse_str(s: &str) -> Result<Expression, ExpressionError> {
    let lexer = crate::expression::lexer::Lexer::new(s)?;
    let parser_token = parse(&lexer)?;
    translate(parser_token, s)
}

fn evaluate_basic_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    basic: BasicInputData,
) -> Result<ComputedItem, ExpressionError> {
    let data = basic.data();
    let source = data.as_ref();
    let expression = parse_str(source)?;
    let computed = evaluate_expression(computed_data, functions, expression, source)?;
    match basic.definition() {
        Boolean(_boolean_definition) => {
            // Validate that the computed value is a boolean
            if let ComputedItem::Boolean(_value) = &computed {
                Ok(computed)
            } else {
                Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Expected a boolean value for boolean definition, but got {:?}.",
                        computed
                    ),
                ))
            }
        }
        Choice(choice_definition) => {
            // Validate that the computed value is one of the allowed choices
            if let ComputedItem::String(value) = &computed {
                if choice_definition.contains(value) {
                    Ok(computed)
                } else {
                    Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("Value '{}' is not a valid choice.", value),
                    ))
                }
            } else {
                Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Expected a string value for choice, but got {:?}.",
                        computed
                    ),
                ))
            }
        }
        File(_file_definition) => {
            // Validate that the computed value is a file path
            if let ComputedItem::File(_path) = &computed {
                // Could add additional validation here (e.g., path exists, is readable)
                Ok(computed)
            } else {
                Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Expected a file path for file definition, but got {:?}.",
                        computed
                    ),
                ))
            }
        }
        Integer(integer_definition) => {
            // Validate that the computed value is an integer
            if let ComputedItem::Integer(value) = &computed {
                let constraint = integer_definition.constraint();
                match constraint {
                    IntegerConstraint::Min { min, inclusive } => {
                        if *value < min || (!inclusive && *value == min) {
                            return Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!(
                                    "Value {} is less than the minimum allowed value of {}.",
                                    value, min
                                ),
                            ));
                        }
                        Ok(computed)
                    }
                    IntegerConstraint::Max { max, inclusive } => {
                        if *value > max || (!inclusive && *value == max) {
                            return Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!(
                                    "Value {} is greater than the maximum allowed value of {}.",
                                    value, max
                                ),
                            ));
                        }
                        Ok(computed)
                    }
                    IntegerConstraint::Range {
                        min,
                        max,
                        min_inclusive,
                        max_inclusive,
                    } => {
                        if *value < min || (!min_inclusive && *value == min) {
                            return Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!(
                                    "Value {} is less than the minimum allowed value of {}.",
                                    value, min
                                ),
                            ));
                        }
                        if *value > max || (!max_inclusive && *value == max) {
                            return Err(ExpressionError::new_simple(
                                ExpressionCategory::Evaluation,
                                format!(
                                    "Value {} is greater than the maximum allowed value of {}.",
                                    value, max
                                ),
                            ));
                        }
                        Ok(computed)
                    }
                    IntegerConstraint::None => Ok(computed),
                }
            } else {
                Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Expected an integer value for integer definition, but got {:?}.",
                        computed
                    ),
                ))
            }
        }
        Number(number_definition) => {
            // Validate that the computed value is a number (integer or float)
            match &computed {
                ComputedItem::Float(value) => {
                    let constraint = number_definition.constraint();
                    match constraint {
                        NumberConstraint::Min { min, inclusive } => {
                            if (*value) < min || (!inclusive && (*value) == min) {
                                return Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    format!(
                                        "Value {} is less than the minimum allowed value of {}.",
                                        value, min
                                    ),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraint::Max { max, inclusive } => {
                            if (*value) > max || (!inclusive && (*value) == max) {
                                return Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    format!(
                                        "Value {} is greater than the maximum allowed value of {}.",
                                        value, max
                                    ),
                                ));
                            }
                            Ok(computed)
                        }
                        NumberConstraint::Range {
                            min,
                            max,
                            min_inclusive,
                            max_inclusive,
                        } => {
                            if (*value) < min || (!min_inclusive && (*value) == min) {
                                return Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    format!(
                                        "Value {} is less than the minimum allowed value of {}.",
                                        value, min
                                    ),
                                ));
                            }
                            if (*value) > max || (!max_inclusive && (*value) == max) {
                                return Err(ExpressionError::new_simple(
                                    ExpressionCategory::Evaluation,
                                    format!(
                                        "Value {} is greater than the maximum allowed value of {}.",
                                        value, max
                                    ),
                                ));
                            }

                            Ok(computed)
                        }
                        NumberConstraint::None => Ok(computed),
                    }
                }
                _ => Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Expected a numeric value for number definition, but got {:?}.",
                        computed
                    ),
                )),
            }
        }
        String(_string_definition) => {
            // Validate that the computed value is a string
            if let ComputedItem::String(_) = &computed {
                Ok(computed)
            } else {
                Err(ExpressionError::new_simple(
                    ExpressionCategory::Evaluation,
                    format!(
                        "Expected a string value for string definition, but got {:?}.",
                        computed
                    ),
                ))
            }
        }
    }
}

fn evaluate_table_expression(
    computed_data: &BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    table: TableInputData,
) -> Result<Vec<Vec<f64>>, Vec<ExpressionError>> {
    let definition = table.definition();

    let mut evaluated_rows = Vec::new();

    for row in table.data() {
        let mut evaluated_row = Vec::new();
        for (i, basic_data) in row.iter().enumerate() {
            let number_definition = definition
                .get_by_index(i)
                .expect("Column definition should exist for each key in the row.");

            let basic_input_data =
                BasicInputData::new(Number(number_definition.clone()), basic_data.clone());

            match evaluate_basic_expression(computed_data, functions, basic_input_data) {
                Ok(ComputedItem::Integer(value)) => {
                    evaluated_row.push(value as f64);
                }
                Ok(ComputedItem::Float(value)) => {
                    evaluated_row.push(value);
                }
                Ok(other) => {
                    return Err(vec![ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!(
                            "Expected a numeric value for table cell, but got {:?}.",
                            other
                        ),
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
pub(crate) fn evaluator(
    computed_data: BTreeMap<ShareableString, ComputedItem>,
    functions: &FunctionDefinitions,
    input_data: BTreeMap<ShareableString, ObjectItemInputData>,
) -> (
    BTreeMap<ShareableString, ComputedItem>,
    Vec<ExpressionError>,
) {
    let mut result = BTreeMap::new();
    let mut errors = Vec::new();

    for (key, data) in input_data {
        match data {
            ObjectItemInputData::Basic(basic_data) => {
                match evaluate_basic_expression(&computed_data, functions, basic_data) {
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
                match evaluate_table_expression(&computed_data, functions, table_data) {
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
        }
    }

    (result, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluation::expression::function_definition::FunctionDefinition;
    use datastore::definition::{BooleanDefinition, IntegerDefinition, NumberDefinition};
    use datastore::store_key;

    fn create_number_basic_input_data(value: &str) -> ObjectItemInputData {
        let definition = Number(NumberDefinition::new("Test Number"));
        let data = ShareableString::from(value.to_string());
        ObjectItemInputData::Basic(BasicInputData::new(definition, data))
    }

    fn check_number_float(computed_item: &ComputedItem, expected_value: f64) {
        match computed_item {
            ComputedItem::Float(value) => assert_eq!(*value, expected_value),
            _ => panic!("Expected a numeric computed item"),
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
    fn empty_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::new();
        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
        assert!(result.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn evaluation_error_marks_missing_variable_with_an_underline() {
        // A reference to an undefined variable produces an evaluation error whose
        // display renders the expression with a `~` underline beneath the variable.
        let computed_data = BTreeMap::new();
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("missing_var"))]);

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
        assert!(result.is_empty());
        assert_eq!(errors.len(), 1);

        let rendered = errors[0].to_string();
        assert!(rendered.starts_with("[Evaluation]"));
        assert!(rendered.contains("Variable 'missing_var' not found in computed data."));
        assert_eq!(
            rendered,
            "[Evaluation] Variable 'missing_var' not found in computed data.\n\
             missing_var\n\
             ~~~~~~~~~~~\n"
        );
    }

    #[test]
    fn boolean_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([("x".into(), create_boolean_basic_input_data("true"))]);

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_integer(&result["x"], 42);
    }

    #[test]
    fn integer_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data =
            BTreeMap::from([("x".into(), create_integer_basic_input_data("1 + 2 * 3"))]);

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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
    fn float_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([("x".into(), create_number_basic_input_data("42.0"))]);

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
        assert!(!result.is_empty());
        assert!(errors.is_empty());

        check_number_float(&result["x"], 42.0);
    }

    #[test]
    fn float_expression_test() {
        let computed_data = BTreeMap::new();
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("1.0 + 2.0 * 3.0"),
        )]);

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let expression = parse_str("t[5]").unwrap();
        assert!(
            evaluate_expression(&computed_data, &FunctionDefinitions::new(), expression, "t[5]")
                .is_err()
        );
    }

    #[test]
    fn field_access_via_bracket_indexing_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 3.5)]]),
        )]);

        let expression = parse_str("t[0][col]").unwrap();
        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            expression,
            "t[0][col]",
        )
        .unwrap();

        check_number_float(&result, 3.5);
    }

    #[test]
    fn field_access_missing_field_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 3.5)]]),
        )]);

        let expression = parse_str("t[0][missing]").unwrap();
        assert!(
            evaluate_expression(
                &computed_data,
                &FunctionDefinitions::new(),
                expression,
                "t[0][missing]"
            )
            .is_err()
        );
    }

    #[test]
    fn field_access_on_multi_row_table_test() {
        let computed_data = BTreeMap::from([(
            "t".into(),
            create_table_computed_item(vec![vec![("col", 1.0)], vec![("col", 2.0)]]),
        )]);

        let expression = parse_str("t[col]").unwrap();
        assert!(
            evaluate_expression(&computed_data, &FunctionDefinitions::new(), expression, "t[col]")
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let (result, errors) = evaluator(computed_data, &FunctionDefinitions::new(), input_data);
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

        let expression = parse_str("map[key][entry][1][col]").unwrap();
        let result = evaluate_expression(
            &computed_data,
            &FunctionDefinitions::new(),
            expression,
            "map[key][entry][1][col]",
        )
        .unwrap();

        check_number_float(&result, 9.0);
    }

    /// A helper that sums numeric arguments (Integer or Float) into a Float.
    fn sum_function(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
        let mut total = 0.0;
        for arg in args {
            match arg {
                ComputedItem::Float(v) => total += v,
                ComputedItem::Integer(v) => total += *v as f64,
                other => {
                    return Err(ExpressionError::new_simple(
                        ExpressionCategory::Evaluation,
                        format!("sum() expects numeric arguments, got {other:?}"),
                    ));
                }
            }
        }
        Ok(ComputedItem::Float(total))
    }

    /// A function with no arguments that always returns the float 42.
    fn constant_function(_args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
        Ok(ComputedItem::Float(42.0))
    }

    #[test]
    fn function_call_with_no_arguments_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("constant"),
            "returns 42",
            constant_function,
        ));
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("constant()"))]);

        let (result, errors) = evaluator(BTreeMap::new(), &functions, input_data);
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
            sum_function,
        ));
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("sum(1.0, 2.0, 3.5)"),
        )]);

        let (result, errors) = evaluator(BTreeMap::new(), &functions, input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 6.5);
    }

    #[test]
    fn function_call_with_variable_arguments_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            sum_function,
        ));
        let computed_data = BTreeMap::from([("a".into(), ComputedItem::Float(1.5))]);
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("sum(a, 2.0)"))]);

        let (result, errors) = evaluator(computed_data, &functions, input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 3.5);
    }

    #[test]
    fn nested_function_calls_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            sum_function,
        ));
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("sum(sum(1.0, 2.0), sum(3.0, 4.0))"),
        )]);

        let (result, errors) = evaluator(BTreeMap::new(), &functions, input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 10.0);
    }

    #[test]
    fn function_call_combined_with_operators_test() {
        let functions = FunctionDefinitions::new().with(FunctionDefinition::new(
            store_key!("sum"),
            "sums its arguments",
            sum_function,
        ));
        let input_data = BTreeMap::from([(
            "x".into(),
            create_number_basic_input_data("sum(1.0, 2.0) * 3.0 + 1.0"),
        )]);

        let (result, errors) = evaluator(BTreeMap::new(), &functions, input_data);
        assert!(errors.is_empty());
        check_number_float(&result["x"], 10.0);
    }

    #[test]
    fn undefined_function_call_returns_error_test() {
        let functions = FunctionDefinitions::new();
        let input_data =
            BTreeMap::from([("x".into(), create_number_basic_input_data("undefined()"))]);

        let (result, errors) = evaluator(BTreeMap::new(), &functions, input_data);
        assert!(result.is_empty());
        assert_eq!(errors.len(), 1);
        let message = errors[0].to_string();
        assert!(message.contains("[Evaluation]"));
        assert!(message.contains("Function 'undefined' is not defined."));
    }
}
