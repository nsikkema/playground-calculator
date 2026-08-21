use crate::Message;
use crate::evaluation::expression::ast::lexer::Lexer;
use crate::evaluation::expression::ast::parser::Parser;
use crate::evaluation::expression::ast::translator::{Expression, Translator};
use message::path::Path;
use shareable_string::ShareableString;

/// Helper function to convert a `ShareableString` into an `Expression`.
///
/// This function takes a `ShareableString` as input, tokenizes it using the `Lexer`,
/// parses the tokens into an AST using the `Parser`, and then translates the AST into
/// an `Expression` using the `Translator`.
#[hotpath::measure]
pub(crate) fn string_to_expression(input: &ShareableString) -> Result<Expression, Message> {
    let lexer = Lexer::new(Path::new("expression_engine"), Path::new(""), input)?;
    let parser = Parser::new(&lexer)?;
    let translator = Translator::new(&parser)?;
    Ok(translator.expression().clone())
}
