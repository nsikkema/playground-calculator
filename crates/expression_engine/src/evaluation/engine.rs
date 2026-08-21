use crate::evaluation::expression::evaluator::evaluator;
use crate::evaluation::expression::function_definition::{FunctionDefinition, FunctionDefinitions};
use crate::evaluation::expression::function_definitions_default::default_function_definitions;
use crate::evaluation::expression::globals_default::default_globals;
use crate::expression::ast::ast_helper::string_to_expression;
use crate::expression::requirements::MissingRequirements;
use crate::{
    GlobalObjectComputedData, GlobalObjectInputData, Message, ParameterObjectComputedData,
    ParameterObjectInputData, VariableObjectComputedData, VariableObjectInputData,
};
use shareable_string::ShareableString;
use std::collections::HashSet;

/// The `Engine` struct represents the core evaluation engine for processing expressions. It is designed to handle various types of expressions and provide a framework for evaluating them efficiently.
/// The engine can be extended with additional features and optimizations as needed.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionEngine {
    /// The pre-evaluated global computed data shared across all evaluations.
    globals: GlobalObjectComputedData,
    /// The registered callable functions available during expression evaluation.
    functions: FunctionDefinitions,
}

impl Default for ExpressionEngine {
    #[hotpath::measure]
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionEngine {
    /// Creates a new instance of the `Engine`.
    #[must_use]
    #[hotpath::measure]
    pub fn new() -> Self {
        Self {
            globals: default_globals(),
            functions: default_function_definitions(),
        }
    }

    /// Registers a callable function that can be invoked from within an expression
    /// using the syntax `name(arg, ...)`.
    ///
    /// Registering a function with a name that already exists replaces the previous
    /// definition.
    ///
    /// # Errors
    ///
    /// Returns an error if `func`'s name is empty or only whitespace.
    #[hotpath::measure]
    pub fn register_function(&mut self, func: FunctionDefinition) -> Result<(), Message> {
        if func.name().as_str().trim().is_empty() {
            return Err(crate::expression_message!(
                crate::ExpressionCategory::Evaluation,
                "expression_engine_evaluation_function_name_empty",
                [],
            ));
        }

        self.functions.insert(func);
        Ok(())
    }

    /// Evaluates the provided global input data and updates the engine's state with the computed results.
    ///
    /// # Errors
    ///
    /// Returns the list of evaluation errors encountered while evaluating `globals`.
    #[hotpath::measure]
    pub fn evaluate_globals(
        &mut self,
        globals: &GlobalObjectInputData,
    ) -> Result<(), Vec<Message>> {
        let (computed_data, errors) =
            evaluator(default_globals().data(), &self.functions, globals.data());

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut data = default_globals().data().clone();
        data.extend(computed_data);
        self.globals = GlobalObjectComputedData::new(data);

        Ok(())
    }

    /// Evaluates the provided parameters against the engine's current global state and returns the computed results.
    ///
    /// # Errors
    ///
    /// Returns the list of evaluation errors encountered while evaluating `parameters`.
    #[hotpath::measure]
    pub fn evaluate_parameters(
        &self,
        parameters: &ParameterObjectInputData,
    ) -> Result<ParameterObjectComputedData, Vec<Message>> {
        let (computed_data, errors) =
            evaluator(self.globals.data(), &self.functions, parameters.data());

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ParameterObjectComputedData::new(computed_data))
    }

    /// Evaluates the provided variables against the engine's current global state and returns the computed results.
    ///
    /// # Errors
    ///
    /// Returns the list of evaluation errors encountered while evaluating `variables`.
    #[hotpath::measure]
    pub fn evaluate_variables(
        &self,
        parameters: &ParameterObjectComputedData,
        variables: &VariableObjectInputData,
    ) -> Result<VariableObjectComputedData, Vec<Message>> {
        let mut data = self.globals.data().clone();
        data.extend(parameters.data().clone());

        let (computed_data, errors) = evaluator(&data, &self.functions, variables.data());

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(VariableObjectComputedData::new(computed_data))
    }

    /// Extends the engine's global state with the provided parameters, variables, and global input data.
    /// This method evaluates the provided data and updates the engine's state accordingly.
    ///
    /// # Errors
    ///
    /// Returns the list of evaluation errors encountered while evaluating `globals`.
    #[hotpath::measure]
    pub fn extend_globals(
        &mut self,
        parameters: &ParameterObjectComputedData,
        variables: &VariableObjectComputedData,
        globals: &GlobalObjectInputData,
    ) -> Result<(), Vec<Message>> {
        let mut data = self.globals.data().clone();
        data.extend(parameters.data().clone());
        data.extend(variables.data().clone());

        let (computed_data, errors) = evaluator(&data, &self.functions, globals.data());

        if !errors.is_empty() {
            return Err(errors);
        }

        self.globals
            .extend(GlobalObjectComputedData::new(computed_data));
        Ok(())
    }

    /// Evaluates the provided child parameters against the engine's current global state, parameters, and variables.
    /// Returns the computed results for the child parameters.
    ///
    /// # Errors
    ///
    /// Returns the list of evaluation errors encountered while evaluating `child_parameters`.
    #[hotpath::measure]
    pub fn evaluate_child_parameters(
        &self,
        parameters: &ParameterObjectComputedData,
        variables: &VariableObjectComputedData,
        child_parameters: &ParameterObjectInputData,
    ) -> Result<ParameterObjectComputedData, Vec<Message>> {
        let mut data = self.globals.data().clone();
        data.extend(parameters.data().clone());
        data.extend(variables.data().clone());

        let (computed_data, errors) = evaluator(&data, &self.functions, child_parameters.data());

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ParameterObjectComputedData::new(computed_data))
    }

    /// Checks for missing requirements in the provided expression based on the current state of the engine.
    /// It verifies if all required globals, parameters, variables, and functions are present.
    ///
    /// # Errors
    ///
    /// Returns an error if any required global, parameter, variable, or function is missing.
    #[hotpath::measure]
    pub fn check_missing_requirements(
        &self,
        parameters: &Option<ParameterObjectInputData>,
        variables: &Option<VariableObjectInputData>,
        new_globals: &Option<GlobalObjectInputData>,
        expression: &ShareableString,
    ) -> Result<(), Vec<Message>> {
        let mut item_keys: HashSet<ShareableString> = self.globals.data().keys().cloned().collect();

        if let Some(parameters) = parameters {
            item_keys.extend(parameters.data().keys().cloned());
        }
        if let Some(variables) = variables {
            item_keys.extend(variables.data().keys().cloned());
        }
        if let Some(globals) = new_globals {
            item_keys.extend(globals.data().keys().cloned());
        }

        let function_keys: HashSet<ShareableString> = self.functions.keys().cloned().collect();

        let expression = string_to_expression(expression).map_err(|err| vec![err])?;

        let missing_requirements =
            MissingRequirements::new(&expression, &item_keys, &function_keys);

        if !missing_requirements.missing_requirements_exist() {
            return Ok(());
        }

        let mut errors = Vec::new();

        if missing_requirements.missing_globals() {
            for global in missing_requirements.globals() {
                errors.push(crate::expression_message!(
                    crate::ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_missing_required_global",
                    [("global", global)],
                ));
            }
        }

        if missing_requirements.missing_parameters() {
            for parameter in missing_requirements.parameters() {
                errors.push(crate::expression_message!(
                    crate::ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_missing_required_parameter",
                    [("parameter", parameter)],
                ));
            }
        }

        if missing_requirements.missing_variables() {
            for variable in missing_requirements.variables() {
                errors.push(crate::expression_message!(
                    crate::ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_missing_required_variable",
                    [("variable", variable)],
                ));
            }
        }

        if missing_requirements.missing_functions() {
            for function in missing_requirements.functions() {
                errors.push(crate::expression_message!(
                    crate::ExpressionCategory::Evaluation,
                    "expression_engine_evaluation_missing_required_function",
                    [("function", function)],
                ));
            }
        }

        Err(errors)
    }

    /// Returns a reference to the global computed data of the engine.
    #[must_use]
    pub const fn globals(&self) -> &GlobalObjectComputedData {
        &self.globals
    }

    /// Returns a reference to the registered function definitions of the engine.
    #[must_use]
    pub const fn functions(&self) -> &FunctionDefinitions {
        &self.functions
    }
}
