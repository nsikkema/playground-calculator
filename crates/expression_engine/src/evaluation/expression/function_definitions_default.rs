use crate::evaluation::expression::function_definition::{FunctionDefinition, FunctionDefinitions};
use crate::{ComputedItem, ExpressionError};
use datastore::store_key;

fn sin(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    if args.len() != 1 {
        return Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "sin function requires exactly 1 argument".to_string(),
        ));
    }

    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.sin())),
        _ => Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "sin function argument must be a number".to_string(),
        )),
    }
}

fn cos(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    if args.len() != 1 {
        return Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "cos function requires exactly 1 argument".to_string(),
        ));
    }

    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.cos())),
        _ => Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "cos function argument must be a number".to_string(),
        )),
    }
}

fn tan(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    if args.len() != 1 {
        return Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "tan function requires exactly 1 argument".to_string(),
        ));
    }

    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.tan())),
        _ => Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "tan function argument must be a number".to_string(),
        )),
    }
}

fn arcsin(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    if args.len() != 1 {
        return Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "arcsin function requires exactly 1 argument".to_string(),
        ));
    }

    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.asin())),
        _ => Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "arcsin function argument must be a number".to_string(),
        )),
    }
}

fn arccos(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    if args.len() != 1 {
        return Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "arccos function requires exactly 1 argument".to_string(),
        ));
    }

    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.acos())),
        _ => Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "arccos function argument must be a number".to_string(),
        )),
    }
}

fn arctan(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    if args.len() != 1 {
        return Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "arctan function requires exactly 1 argument".to_string(),
        ));
    }

    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.atan())),
        _ => Err(ExpressionError::new_simple(
            crate::ExpressionCategory::Evaluation,
            "arctan function argument must be a number".to_string(),
        )),
    }
}

/// Returns a `FunctionDefinitions` containing the default mathematical functions.
pub(crate) fn get_default_function_definitions() -> FunctionDefinitions {
    FunctionDefinitions::new()
        .with(FunctionDefinition::new(
            store_key!("sin"),
            "sine function",
            sin,
        ))
        .with(FunctionDefinition::new(
            store_key!("cos"),
            "cosine function",
            cos,
        ))
        .with(FunctionDefinition::new(
            store_key!("tan"),
            "tangent function",
            tan,
        ))
        .with(FunctionDefinition::new(
            store_key!("arcsin"),
            "inverse sine function",
            arcsin,
        ))
        .with(FunctionDefinition::new(
            store_key!("arccos"),
            "inverse cosine function",
            arccos,
        ))
        .with(FunctionDefinition::new(
            store_key!("arctan"),
            "inverse tangent function",
            arctan,
        ))
        .with(FunctionDefinition::new(
            store_key!("arccos"),
            "inverse cosine function",
            arccos,
        ))
        .with(FunctionDefinition::new(
            store_key!("arctan"),
            "inverse tangent function",
            arctan,
        ))
}
