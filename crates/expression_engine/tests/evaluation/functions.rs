use datastore::definition::{
    GlobalObjectDefinition, IntegerDefinition, NumberDefinition, ParameterObjectDefinition,
};
use datastore::frozen::{GlobalObjectFrozen, ParameterObjectFrozen};
use datastore::{global_key, parameter_key, store_key};
use expression_engine::engine::ExpressionEngine;
use expression_engine::expression::function_definition::FunctionDefinition;
use expression_engine::{ComputedItem, GlobalObjectInputData, ParameterObjectInputData};

/// A function that sums its integer arguments.
fn add_integers(args: &[ComputedItem]) -> Result<ComputedItem, expression_engine::ExpressionError> {
    let mut total: i64 = 0;
    for arg in args {
        match arg {
            ComputedItem::Integer(value) => total += value,
            other => {
                return Err(expression_engine::ExpressionError::new_simple(
                    expression_engine::ExpressionCategory::Evaluation,
                    format!("add() expects integer arguments, got {other:?}"),
                ));
            }
        }
    }
    Ok(ComputedItem::Integer(total))
}

/// A function that multiplies two float arguments.
fn multiply_floats(
    args: &[ComputedItem],
) -> Result<ComputedItem, expression_engine::ExpressionError> {
    match args {
        [ComputedItem::Float(a), ComputedItem::Float(b)] => Ok(ComputedItem::Float(a * b)),
        _ => Err(expression_engine::ExpressionError::new_simple(
            expression_engine::ExpressionCategory::Evaluation,
            "multiply() expects exactly two float arguments".to_string(),
        )),
    }
}

fn build_parameter(definition: ParameterObjectDefinition) -> ParameterObjectInputData {
    ParameterObjectInputData::new(ParameterObjectFrozen::new(definition))
}

#[test]
fn registered_function_is_invoked_during_evaluation() {
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("A number parameter", "add(2, 3)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    let number = output.get("p_result").expect("p_result should be computed");
    match number {
        ComputedItem::Integer(value) => assert_eq!(*value, 5),
        other => panic!("expected integer 5, got {other:?}"),
    }
}

#[test]
fn registered_function_can_reference_variables() {
    // Parameters are evaluated against the engine's globals, so set up a global
    // variable that the function can reference alongside a literal argument.
    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Globals")
            .with(
                global_key!("g_a"),
                IntegerDefinition::new_with_default("a global operand", "10"),
            )
            .finish(),
    );

    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_sum"),
            IntegerDefinition::new_with_default("sum of operands", "add(g_a, 32)"),
        )
        .finish();

    let global_data = GlobalObjectInputData::new(global_frozen);
    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            add_integers,
        ))
        .expect("function should register");
    engine
        .evaluate_globals(global_data)
        .expect("globals should evaluate");

    let output = engine
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    match output.get("p_sum").expect("p_sum should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 42),
        other => panic!("expected integer 42, got {other:?}"),
    }
}

#[test]
fn registered_function_combines_with_other_operators() {
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "add(2, 3) * 4 - 1"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 19),
        other => panic!("expected integer 19, got {other:?}"),
    }
}

#[test]
fn nested_registered_function_calls_evaluate_correctly() {
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "add(add(1, 2), add(3, 4))"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 10),
        other => panic!("expected integer 10, got {other:?}"),
    }
}

#[test]
fn float_returning_function_works_with_number_definition() {
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            NumberDefinition::new_with_default("a number parameter", "multiply(2.5, 4.0)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("multiply"),
            "multiplies two floats",
            multiply_floats,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Float(value) => {
            let expected: f64 = 10.0;
            assert!(
                (value - expected).abs() < f64::EPSILON,
                "expected {expected}, got {value}"
            );
        }
        other => panic!("expected float 10.0, got {other:?}"),
    }
}

#[test]
fn calling_an_unregistered_function_returns_an_error() {
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "missing(1, 2)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let engine = ExpressionEngine::new();
    let error = engine
        .evaluate_parameters(data)
        .expect_err("evaluation should fail for an undefined function");

    let message = error
        .first()
        .expect("at least one error should be reported")
        .to_string();
    assert!(message.contains("[Evaluation]"));
    assert!(message.contains("Function 'missing' is not defined."));
}
