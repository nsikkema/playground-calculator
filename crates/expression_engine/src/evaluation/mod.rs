//! The `evaluation` module serves as the entry point for the expression engine, providing
//! the necessary structures to facilitate the evaluation of expressions. It
//! includes the `engine` and `expression` submodules, which contain the core logic for
//! processing and evaluating expressions based on the defined syntax and semantics.

use crate::expression::ast::span::SpanSet;
use message::message::{Message, MessageCategory, MessageLevel};
use message::path::Path;
use shareable_string::ShareableString;
use std::collections::HashMap;

/// The `engine` module contains the core evaluation engine, which is responsible for managing
/// the evaluation of expressions. It provides the necessary structures and methods to facilitate
/// the evaluation process, including handling global computed data and orchestrating the
/// evaluation of various types of expressions.
pub mod engine;
/// The `expression` module contains the core components for parsing and evaluating expressions
/// within the expression engine. It includes the lexer, parser, and evaluator, which work
/// together to process and compute the results of expressions based on the defined syntax
/// and semantics.
pub mod expression;

/// Builds the underline string for the error: `~` characters under each marked
/// span and spaces elsewhere. Returns the original string if there is no expression text or
/// no mark falls within it.
#[hotpath::measure]
pub(crate) fn underline_string(source: ShareableString, marks: SpanSet) -> ShareableString {
    let chars: Vec<char> = source.as_ref().chars().collect();
    let len = chars.len();
    if len == 0 || marks.is_empty() {
        return source;
    }

    let mut line = vec![' '; len];
    let mut any = false;
    for mark in marks.iter() {
        let start = mark.start().min(len);
        let end = mark.end().min(len);
        if start < end {
            for i in line.iter_mut().take(end).skip(start) {
                *i = '~';
            }
            any = true;
        }
    }

    if any {
        format!(
            "{}\n{}",
            source,
            line.into_iter().collect::<String>().trim_end()
        )
        .into()
    } else {
        source
    }
}

/// Creates a new error message with the given parameters. If a source string and marks are provided,
/// the source string will be underlined according to the marks and included in the message.
#[hotpath::measure]
pub(crate) fn create_error_message(
    object_path: Path,
    item_path: Option<Path>,
    category: MessageCategory,
    key: ShareableString,
    params: HashMap<ShareableString, ShareableString>,
    source: Option<ShareableString>,
    marks: Option<SpanSet>,
) -> Message {
    let mut extra_detail = None;
    if let Some(source) = source {
        if let Some(marks) = marks {
            extra_detail = Some(underline_string(source.clone(), marks.clone()));
        }
    }

    Message::new_with_params(
        object_path,
        item_path,
        MessageLevel::Error,
        category,
        key,
        params,
        extra_detail,
    )
}
