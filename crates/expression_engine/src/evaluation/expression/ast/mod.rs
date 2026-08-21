/// The `ast_helper` module provides utility functions for working with the AST, including converting strings to expressions.
pub(crate) mod ast_helper;
/// The `lexer` is responsible for tokenizing the input expression.
pub(crate) mod lexer;
/// The `precedence_parser` implements a parser that respects operator precedence and associativity.
pub(crate) mod parser;
/// The 'translator' implements a conversion from the AST to a more efficient representation for evaluation, optimizing the evaluation process.
pub(crate) mod translator;
