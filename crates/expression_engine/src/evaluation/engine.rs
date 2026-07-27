use crate::evaluation::expression::function_definition::{FunctionDefinition, FunctionDefinitions};
use crate::expression::evaluator::evaluator;
use crate::expression::function_definitions_default::get_default_function_definitions;
use crate::{
    ExpressionError, GlobalObjectComputedData, GlobalObjectInputData, ParameterObjectComputedData,
    ParameterObjectInputData, VariableObjectComputedData, VariableObjectInputData,
};
use std::collections::BTreeMap;

/// The `Engine` struct represents the core evaluation engine for processing expressions. It is designed to handle various types of expressions and provide a framework for evaluating them efficiently.
/// The engine can be extended with additional features and optimizations as needed.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionEngine {
    globals: GlobalObjectComputedData,
    functions: FunctionDefinitions,
}

impl Default for ExpressionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionEngine {
    /// Creates a new instance of the `Engine`.
    pub fn new() -> Self {
        Self {
            globals: GlobalObjectComputedData::new(BTreeMap::new()),
            functions: get_default_function_definitions(),
        }
    }

    /// Registers a callable function that can be invoked from within an expression
    /// using the syntax `name(arg, ...)`.
    ///
    /// Registering a function with a name that already exists replaces the previous
    /// definition.
    pub fn register_function(&mut self, func: FunctionDefinition) -> Result<(), ExpressionError> {
        if func.name().as_str().trim().is_empty() {
            return Err(ExpressionError::new_simple(
                crate::ExpressionCategory::Evaluation,
                "Function name must not be empty.".to_string(),
            ));
        }

        self.functions.insert(func);
        Ok(())
    }

    /// Evaluates the provided global input data and updates the engine's state with the computed results.
    pub fn evaluate_globals(
        &mut self,
        globals: GlobalObjectInputData,
    ) -> Result<(), Vec<ExpressionError>> {
        let (computed_data, errors) =
            evaluator(BTreeMap::new(), &self.functions, globals.data().clone());

        if !errors.is_empty() {
            return Err(errors);
        }

        self.globals = GlobalObjectComputedData::new(computed_data);

        Ok(())
    }

    /// Evaluates the provided parameters against the engine's current global state and returns the computed results.
    pub fn evaluate_parameters(
        &self,
        parameters: ParameterObjectInputData,
    ) -> Result<ParameterObjectComputedData, Vec<ExpressionError>> {
        let (computed_data, errors) = evaluator(
            self.globals.data().clone(),
            &self.functions,
            parameters.data().clone(),
        );

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ParameterObjectComputedData::new(computed_data))
    }

    /// Evaluates the provided variables against the engine's current global state and returns the computed results.
    pub fn evaluate_variables(
        &self,
        parameters: ParameterObjectComputedData,
        variables: VariableObjectInputData,
    ) -> Result<VariableObjectComputedData, Vec<ExpressionError>> {
        let mut data = self.globals.data().clone();
        data.extend(parameters.data().clone());

        let (computed_data, errors) = evaluator(data, &self.functions, variables.data().clone());

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(VariableObjectComputedData::new(computed_data))
    }

    /// Extends the engine's global state with the provided parameters, variables, and global input data.
    /// This method evaluates the provided data and updates the engine's state accordingly.
    pub fn extend_globals(
        &mut self,
        parameters: ParameterObjectComputedData,
        variables: VariableObjectComputedData,
        globals: GlobalObjectInputData,
    ) -> Result<(), Vec<ExpressionError>> {
        let mut data = self.globals.data().clone();
        data.extend(parameters.data().clone());
        data.extend(variables.data().clone());

        let (computed_data, errors) = evaluator(data, &self.functions, globals.data().clone());

        if !errors.is_empty() {
            return Err(errors);
        }

        self.globals
            .extend(GlobalObjectComputedData::new(computed_data));
        Ok(())
    }

    /// Evaluates the provided child parameters against the engine's current global state, parameters, and variables.
    /// Returns the computed results for the child parameters.
    pub fn evaluate_child_parameters(
        &self,
        parameters: ParameterObjectComputedData,
        variables: VariableObjectComputedData,
        child_parameters: ParameterObjectInputData,
    ) -> Result<ParameterObjectComputedData, Vec<ExpressionError>> {
        let mut data = self.globals.data().clone();
        data.extend(parameters.data().clone());
        data.extend(variables.data().clone());

        let (computed_data, errors) = evaluator(
            data.clone(),
            &self.functions,
            child_parameters.data().clone(),
        );

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ParameterObjectComputedData::new(computed_data))
    }

    /// Returns a reference to the global computed data of the engine.
    pub fn globals(&self) -> &GlobalObjectComputedData {
        &self.globals
    }

    /// Returns a reference to the registered function definitions of the engine.
    pub fn functions(&self) -> &FunctionDefinitions {
        &self.functions
    }
}
