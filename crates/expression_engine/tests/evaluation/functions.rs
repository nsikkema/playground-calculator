use datastore::prelude::*;
use expression_engine::prelude::*;
use message::message::{Message, MessageCategory, MessageLevel};
use message::path::Path;
use shareable_string::TranslateMessage;
use std::ops::{Mul, Sub};

fn evaluation_message(actual: impl Into<ShareableString>) -> Message {
    Message::new(
        Path::new("expression_engine"),
        None,
        MessageLevel::Error,
        MessageCategory::ExpressionEvaluation,
        TranslateMessage::new(
            "expression_engine_evaluation_custom_function_failed".into(),
            [("actual".into(), actual.into())].into_iter().collect(),
        ),
        None,
    )
}

/// A function that sums its integer arguments.
fn add_integers(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let mut total: i64 = 0;
    for arg in args {
        match arg {
            ComputedItem::Integer(value) => total += value,
            other => {
                return Err(evaluation_message(format!(
                    "add() expects integer arguments, got {other:?}"
                )));
            }
        }
    }
    Ok(ComputedItem::Integer(total))
}

/// A function that multiplies two float arguments.
fn multiply_floats(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match args {
        [ComputedItem::Float(a), ComputedItem::Float(b)] => Ok(ComputedItem::Float(a.mul(b))),
        _ => Err(evaluation_message(
            "multiply() expects exactly two float arguments",
        )),
    }
}

fn build_parameter(definition: ParameterObjectDefinition) -> ParameterObjectInputData {
    ParameterObjectInputData::new(&ParameterObjectFrozen::new(definition))
}

#[test]
fn registered_function_is_invoked_during_evaluation() {
    // Why: Test that a registered custom function is invoked and its result used during evaluation.
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
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_result").expect("p_result should be computed");
    match number {
        ComputedItem::Integer(value) => assert_eq!(*value, 5),
        other => panic!("expected integer 5, got {other:?}"),
    }
}

#[test]
fn registered_function_can_reference_variables() {
    // Why: Test that a registered function can accept a global variable as one of its arguments.

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

    let global_data = GlobalObjectInputData::new(&global_frozen);
    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");
    engine
        .evaluate_globals(&global_data)
        .expect("globals should evaluate");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_sum").expect("p_sum should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 42),
        other => panic!("expected integer 42, got {other:?}"),
    }
}

#[test]
fn registered_function_combines_with_other_operators() {
    // Why: Test that a function call result can be combined with arithmetic operators in the same expression.
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
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 19),
        other => panic!("expected integer 19, got {other:?}"),
    }
}

#[test]
fn nested_registered_function_calls_evaluate_correctly() {
    // Why: Test that nested calls to a registered function are evaluated correctly.
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
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 10),
        other => panic!("expected integer 10, got {other:?}"),
    }
}

#[test]
fn float_returning_function_works_with_number_definition() {
    // Why: Test that a registered function returning a float value is compatible with a NumberDefinition parameter.
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
            ArgumentCount::Exact { count: 2 },
            multiply_floats,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Float(value) => {
            let expected: f64 = 10.0;
            assert!(
                value.sub(expected).abs() < f64::EPSILON,
                "expected {expected}, got {value}"
            );
        }
        other => panic!("expected float 10.0, got {other:?}"),
    }
}

#[test]
fn calling_an_unregistered_function_returns_an_error() {
    // Why: Test that calling a function that has not been registered produces a clear evaluation error.
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "missing(1, 2)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let engine = ExpressionEngine::new();
    let error = engine
        .evaluate_parameters(&data)
        .expect_err("evaluation should fail for an undefined function");

    let message = error
        .first()
        .expect("at least one error should be reported")
        .translate_data()
        .message_params()
        .get("function")
        .expect("undefined-function messages include the function name");
    assert!(
        message
            .as_str()
            .contains("Function 'missing' is not defined.")
    );
}
