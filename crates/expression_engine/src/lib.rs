//! Expression engine crate.

// Test code favors clarity and brevity over the strictness we require of library code:
// panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float comparisons
// are idiomatic and expected in tests.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::float_cmp,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::unreadable_literal,
        clippy::unnecessary_wraps,
        clippy::similar_names,
        clippy::arithmetic_side_effects,
        clippy::wildcard_enum_match_arm
    )
)]

use datastore::definition::{
    BooleanDefinition, ChoiceDefinition, FileDefinition, FolderDefinition, IntegerDefinition,
    NumberDefinition, NumberWithUnitsDefinition, StringDefinition, UnitDefinition,
};
pub use message::message::Message;
use message::{
    message::{MessageCategory, MessageLevel},
    path::Path,
};
use shareable_string::{ShareableString, SharedStringStore, TranslateMessage};
use std::collections::HashMap;

/// Processed data.
pub mod computed_data;
/// Evaluation engine.
pub mod evaluation;
/// Input data.
pub mod input_data;
/// Convenience re-exports for common types.
pub mod prelude;

pub use computed_data::*;
use evaluation::expression::ast::span::SpanSet;
pub use evaluation::*;
pub use input_data::*;

/// A definition for one of the basic (non-composite) data types supported by the
/// expression engine.
#[derive(Debug, Clone, PartialEq)]
pub enum BasicDefinition {
    /// Holds a boolean value.
    Boolean(BooleanDefinition),
    /// Holds a value from a fixed set of choices.
    Choice(ChoiceDefinition),
    /// Holds a file reference.
    File(FileDefinition),
    /// Holds a folder reference.
    Folder(FolderDefinition),
    /// Holds an integer value.
    Integer(IntegerDefinition),
    /// Holds numeric value.
    Number(NumberDefinition),
    /// Holds a numeric value with associated units.
    NumberWithUnits(NumberWithUnitsDefinition),
    /// Holds a string value.
    String(StringDefinition),
    /// Holds a unit value.
    Unit(UnitDefinition),
}

impl BasicDefinition {
    /// Returns a new `BasicDefinition` with strings laundered through the provided store.
    #[must_use]
    #[hotpath::measure]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            BasicDefinition::Boolean(boolean) => BasicDefinition::Boolean(boolean.launder(store)),
            BasicDefinition::Choice(choice) => BasicDefinition::Choice(choice.launder(store)),
            BasicDefinition::File(file) => BasicDefinition::File(file.launder(store)),
            BasicDefinition::Folder(folder) => BasicDefinition::Folder(folder.launder(store)),
            BasicDefinition::Integer(integer) => BasicDefinition::Integer(integer.launder(store)),
            BasicDefinition::Number(number) => BasicDefinition::Number(number.launder(store)),
            BasicDefinition::NumberWithUnits(number) => {
                BasicDefinition::NumberWithUnits(number.launder(store))
            }
            BasicDefinition::String(string) => BasicDefinition::String(string.launder(store)),
            BasicDefinition::Unit(unit) => BasicDefinition::Unit(unit.launder(store)),
        }
    }
}

#[hotpath::measure]
pub(crate) fn new_expression_message(
    category: ExpressionCategory,
    key: &'static str,
    params: HashMap<ShareableString, ShareableString>,
) -> Message {
    expression_message_with_context(category, key, params, None, None)
}

#[hotpath::measure]
pub(crate) fn expression_message_with_context(
    category: ExpressionCategory,
    key: &'static str,
    params: HashMap<ShareableString, ShareableString>,
    source: Option<ShareableString>,
    marks: Option<SpanSet>,
) -> Message {
    Message::new(
        Path::new("expression_engine"),
        None,
        MessageLevel::Error,
        match category {
            ExpressionCategory::Lexer | ExpressionCategory::Parse => {
                MessageCategory::ExpressionParsing
            }
            ExpressionCategory::Evaluation => MessageCategory::ExpressionEvaluation,
        },
        TranslateMessage::new(key.into(), params),
        source
            .zip(marks)
            .map(|(source, marks)| evaluation::underline_string(source, marks)),
    )
}
