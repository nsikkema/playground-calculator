//! This module contains the core components for evaluating expressions, including the lexer, parser, and evaluator.

/// The `evaluator` computes the final result based on the AST.
pub mod evaluator;
/// The `function_definition` module contains the definition of functions that can be invoked
/// from within expressions. It provides the necessary structures and methods to define and
/// manage functions, including their names, descriptions, and the logic for evaluating them
/// based on the provided arguments.
pub mod function_definition;
/// The `function_definition` module contains the definition of functions that can be invoked
/// from within expressions. It provides the necessary structures and methods to define and
/// manage functions, including their names, descriptions, and the logic for evaluating them
/// based on the provided arguments.
pub(crate) mod function_definitions_default;
/// The `index` module provides functionality for indexing and retrieving expressions, enabling efficient access and management of expressions within the evaluation engine.
pub(crate) mod index;
/// The `lexer` is responsible for tokenizing the input expression.
pub mod lexer;
/// The `precedence_parser` implements a parser that respects operator precedence and associativity.
pub mod parser;
/// The 'translator' implements a conversion from the AST to a more efficient representation for evaluation, optimizing the evaluation process.
pub mod translator;
