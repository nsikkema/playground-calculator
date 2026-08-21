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

use core::fmt;
use datastore::definition::{
    BooleanDefinition, ChoiceDefinition, FileDefinition, FolderDefinition, IntegerDefinition,
    NumberDefinition, NumberWithUnitsDefinition, StringDefinition, UnitDefinition,
};
use shareable_string::{ShareableString, SharedStringStore};

/// Processed data.
pub mod computed_data;
/// Evaluation engine.
pub mod evaluation;
/// Input data.
pub mod input_data;
/// Convenience re-exports for common types.
pub mod prelude;

pub use computed_data::*;
pub use evaluation::*;
pub use input_data::*;
use message::span::{SpanSet, underline_string};

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

/// An enumeration of the different categories of expression errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionCategory {
    /// An error that occurred during the lexing phase of expression processing.
    Lexer,
    /// An error that occurred during the parsing phase of expression processing.
    Parse,
    /// An error that occurred during the evaluation phase of expression processing.
    Evaluation,
}

/// Additional, less-commonly-needed context for an `ExpressionError`. This is kept
/// out of `ExpressionError` itself (behind a `Box`) so that `Result<T, ExpressionError>`
/// stays small and cheap to return from functions.
#[derive(Debug, Clone, PartialEq, Default)]
struct ExpressionErrorContext {
    /// The original expression that caused the error.
    original_expression: ShareableString,
    /// The indices in the original expression where the error occurred.
    marks: SpanSet,
}

/// An error produced while parsing or evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionError {
    /// The phase in which the error occurred (e.g. `"parse"` or `"evaluation"`).
    category: ExpressionCategory,
    /// The name of the expression, parameter, or function involved.
    /// A human-readable description of the error.
    message: ShareableString,
    /// Additional context, boxed to keep `ExpressionError` small.
    context: Box<ExpressionErrorContext>,
}

impl ExpressionError {
    /// Creates a new `ExpressionError`.
    #[hotpath::measure]
    pub fn new(category: ExpressionCategory, message: impl Into<ShareableString>) -> Self {
        Self {
            category,
            message: message.into(),
            context: Box::default(),
        }
    }

    /// Creates a new `ExpressionError` with additional context, including the original expression and the indices where the error occurred.
    #[hotpath::measure]
    pub(crate) fn new_complex(
        category: ExpressionCategory,
        message: impl Into<ShareableString>,
        original_expression: impl Into<ShareableString>,
        marks: SpanSet,
    ) -> Self {
        Self {
            category,
            message: message.into(),
            context: Box::new(ExpressionErrorContext {
                original_expression: original_expression.into(),
                marks,
            }),
        }
    }

    /// Returns a new `ExpressionError` with the message laundered through the provided store.
    #[must_use]
    #[hotpath::measure]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            category: self.category.clone(),
            message: store.launder(&self.message),
            context: Box::new(ExpressionErrorContext {
                original_expression: store.launder(&self.context.original_expression),
                marks: self.context.marks.clone(),
            }),
        }
    }
}

impl fmt::Display for ExpressionError {
    #[hotpath::measure]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let underline_string = underline_string(
            self.context.original_expression.clone(),
            self.context.marks.clone(),
        );

        if underline_string.as_str().is_empty() {
            write!(f, "[{:?}] {}", self.category, self.message)
        } else {
            write!(
                f,
                "[{:?}] {}\n{}",
                self.category, self.message, underline_string
            )
        }
    }
}

impl std::error::Error for ExpressionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use message::span::Span;

    #[test]
    fn display_includes_underlined_expression_context() {
        let error = ExpressionError::new_complex(
            ExpressionCategory::Parse,
            "unexpected token",
            "1 + )",
            SpanSet::from_span(Span::new(4, 1)),
        );

        assert_eq!(error.to_string(), "[Parse] unexpected token\n1 + )\n    ~");
    }

    #[test]
    fn display_no_underlined_expression_context() {
        let error = ExpressionError::new_complex(
            ExpressionCategory::Parse,
            "unexpected token",
            "1 + )",
            SpanSet::new(),
        );

        assert_eq!(error.to_string(), "[Parse] unexpected token\n1 + )");
    }

    #[test]
    fn display_excludes_underlined_expression_context() {
        let error = ExpressionError::new(ExpressionCategory::Parse, "unexpected token");

        assert_eq!(error.to_string(), "[Parse] unexpected token");
    }
}
