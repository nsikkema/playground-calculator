use datastore::prelude::*;
use expression_engine::prelude::*;
use shareable_string::ShareableString;

fn message_text(message: &Message) -> &str {
    message
        .translate_data()
        .message_params()
        .get("message")
        .expect("expression messages include their text")
        .as_str()
}

#[test]
fn no_missing_requirements_when_everything_is_present() {
    // Why: Test that no missing requirements are reported when all referenced globals, parameters, and functions are provided.
    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Globals")
            .with(
                global_key!("g_a"),
                IntegerDefinition::new_with_default("a global operand", "10"),
            )
            .finish(),
    );
    let global_data = GlobalObjectInputData::new(&global_frozen);

    let parameter_frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Parameters")
            .with(
                parameter_key!("p_b"),
                IntegerDefinition::new_with_default("a parameter operand", "5"),
            )
            .finish(),
    );
    let parameter_data = ParameterObjectInputData::new(&parameter_frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .evaluate_globals(&global_data)
        .expect("globals should evaluate");

    let expression = ShareableString::from("sqrt(g_a) + p_b");

    let result =
        engine.check_missing_requirements(&Some(parameter_data), &None, &None, &expression);

    assert!(result.is_ok());
}

#[test]
fn reports_missing_global() {
    // Why: Test that a global item referenced by an expression but not supplied is reported as missing.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("g_missing + 1");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing global should be reported");

    assert_eq!(errors.len(), 1);
    assert!(
        message_text(&errors[0]).contains("Missing required global: g_missing"),
        "unexpected error message: {:?}",
        errors[0]
    );
}

#[test]
fn reports_missing_parameter_when_none_supplied() {
    // Why: Test that a parameter referenced by an expression is reported as missing when no parameters are supplied.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("p_missing * 2");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing parameter should be reported");

    assert_eq!(errors.len(), 1);
    assert!(
        message_text(&errors[0]).contains("Missing required parameter: p_missing"),
        "unexpected error message: {:?}",
        errors[0]
    );
}

#[test]
fn reports_missing_variable_when_none_supplied() {
    // Why: Test that a variable referenced by an expression is reported as missing when no variables are supplied.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("v_missing - 1");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing variable should be reported");

    assert_eq!(errors.len(), 1);
    assert!(
        message_text(&errors[0]).contains("Missing required variable: v_missing"),
        "unexpected error message: {:?}",
        errors[0]
    );
}

#[test]
fn reports_missing_function() {
    // Why: Test that a function call to an unregistered function is reported as a missing requirement.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("not_a_real_function(1, 2)");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing function should be reported");

    assert_eq!(errors.len(), 1);
    assert!(
        message_text(&errors[0]).contains("Missing required function: not_a_real_function"),
        "unexpected error message: {:?}",
        errors[0]
    );
}

#[test]
fn reports_all_missing_requirement_kinds_at_once() {
    // Why: Test that all four kinds of missing requirements (global, parameter, variable, function) are reported together in a single expression.
    let engine = ExpressionEngine::new();
    let expression =
        ShareableString::from("g_missing + p_missing + v_missing + not_a_real_function(1)");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("all missing requirement kinds should be reported");

    let messages: Vec<String> = errors.iter().map(message_text).collect();
    assert!(
        messages
            .iter()
            .any(|m| m.contains("Missing required global: g_missing"))
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("Missing required parameter: p_missing"))
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("Missing required variable: v_missing"))
    );
    assert!(
        messages
            .iter()
            .any(|m| m.contains("Missing required function: not_a_real_function"))
    );
}

#[test]
fn provided_parameters_variables_and_globals_satisfy_requirements() {
    // Why: Test that supplying globals, parameters, and variables together satisfies all requirements of an expression referencing them.
    let engine = ExpressionEngine::new();

    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Globals")
            .with(
                global_key!("g_a"),
                IntegerDefinition::new_with_default("a global operand", "1"),
            )
            .finish(),
    );
    let parameter_frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Parameters")
            .with(
                parameter_key!("p_b"),
                IntegerDefinition::new_with_default("a parameter operand", "2"),
            )
            .finish(),
    );
    let variable_frozen = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Variables")
            .with(
                variable_key!("v_c"),
                IntegerDefinition::new_with_default("a variable operand", "3"),
            )
            .finish(),
    );

    let globals = Some(GlobalObjectInputData::new(&global_frozen));
    let parameters = Some(ParameterObjectInputData::new(&parameter_frozen));
    let variables = Some(VariableObjectInputData::new(&variable_frozen));

    let expression = ShareableString::from("g_a + p_b + v_c");

    let result = engine.check_missing_requirements(&parameters, &variables, &globals, &expression);

    assert!(result.is_ok());
}

#[test]
fn invalid_expression_syntax_returns_an_error() {
    // Why: Test that a syntactically invalid expression returns an error rather than reporting missing requirements.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("1 +");

    let result = engine.check_missing_requirements(&None, &None, &None, &expression);

    assert!(result.is_err());
}
