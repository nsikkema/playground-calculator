//! This module contains the core components for evaluating expressions, including the lexer, parser, and evaluator.

/// The `expression` module is responsible for parsing and evaluating expressions within the expression engine,
/// providing the necessary structures and methods to facilitate the evaluation process.
pub mod ast;
/// The `evaluator` computes the final result based on the AST.
pub mod evaluator;
/// The `function_definition` module contains the definition of functions that can be invoked
/// from within expressions. It provides the necessary structures and methods to define and
/// manage functions, including their names, descriptions, and the logic for evaluating them
/// based on the provided arguments.
pub mod function_definition;
/// The `function_definitions_default` module provides a set of default function definitions that
/// can be used within the expression engine.
pub(crate) mod function_definitions_default;
/// The `globals_default` module provides a set of default global variables that can be used
/// within the expression engine.
pub(crate) mod globals_default;
/// The `requirements` module analyzes an expression to determine its dependencies on global variables,
/// parameters, and other resources. It provides the necessary structures and methods to identify
/// and manage these requirements, ensuring that the expression can be evaluated correctly
/// within the context of the expression engine.
pub(crate) mod requirements;
/// The `translations` module provides functionality for translating expressions into different languages.
pub mod translations;
