//! Expression engine crate.

use core::fmt;
use datastore::definition::{
    BooleanDefinition, ChoiceDefinition, FileDefinition, IntegerDefinition, NumberDefinition,
    StringDefinition,
};
use shareable_string::{ShareableString, SharedStringStore};

/// Processed data.
pub mod computed_data;
/// Evaluation engine.
pub mod evaluation;
/// Input data.
pub mod input_data;

pub use computed_data::*;
pub use evaluation::*;
pub use input_data::*;
use crate::expression::index::Index;

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
    /// Holds an integer value.
    Integer(IntegerDefinition),
    /// Holds numeric value.
    Number(NumberDefinition),
    /// Holds a string value.
    String(StringDefinition),
}

impl BasicDefinition {
    /// Returns a new `BasicDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            BasicDefinition::Boolean(boolean) => BasicDefinition::Boolean(boolean.launder(store)),
            BasicDefinition::Choice(choice) => BasicDefinition::Choice(choice.launder(store)),
            BasicDefinition::File(file) => BasicDefinition::File(file.launder(store)),
            BasicDefinition::Integer(integer) => BasicDefinition::Integer(integer.launder(store)),
            BasicDefinition::Number(number) => BasicDefinition::Number(number.launder(store)),
            BasicDefinition::String(string) => BasicDefinition::String(string.launder(store)),
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

/// An error produced while parsing or evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionError {
    /// The phase in which the error occurred (e.g. `"parse"` or `"evaluation"`).
    category: ExpressionCategory,
    /// The name of the expression, parameter, or function involved.
    /// A human-readable description of the error.
    message: ShareableString,
    /// The original expression that caused the error.
    original_expression: ShareableString,
    /// The indices in the original expression where the error occurred.
    marks: Vec<Index>,
}

impl ExpressionError {
    /// Creates a new `ExpressionError` with full context: the originating phase, a
    /// human-readable `message`, the `original_expression` text, and the `marks` (spans
    /// within that text) indicating where the error occurred.
    pub(crate) fn new(
        category: ExpressionCategory,
        message: impl Into<ShareableString>,
        original_expression: impl Into<ShareableString>,
        marks: Vec<Index>,
    ) -> Self {
        Self {
            category,
            message: message.into(),
            original_expression: original_expression.into(),
            marks,
        }
    }

    /// Creates a new `ExpressionError` with no source context.
    ///
    /// This is a convenience for error sites that do not (yet) know the original
    /// expression text or a precise span. The error can be enriched later with
    /// [`ExpressionError::with_context`].
    pub fn new_simple(category: ExpressionCategory, message: impl Into<ShareableString>) -> Self {
        Self {
            category,
            message: message.into(),
            original_expression: ShareableString::default(),
            marks: Vec::new(),
        }
    }

    /// Enriches this error with the original expression text and a span, if it does
    /// not already carry them.
    ///
    /// `original_expression` is set only when the error has no expression text yet,
    /// and `span` is recorded only when the error has no marks. This makes the call
    /// idempotent: the first (deepest) caller to supply context wins, so an error is
    /// associated with the most specific location available.
    pub(crate) fn with_context(mut self, source: impl Into<ShareableString>, span: &Index) -> Self {
        if self.original_expression.as_ref().is_empty() {
            self.original_expression = source.into();
        }
        if self.marks.is_empty() {
            self.marks.push(span.clone());
        }
        self
    }

    /// Returns a new `ExpressionError` with the message laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            category: self.category.clone(),
            message: store.launder(&self.message),
            original_expression: store.launder(&self.original_expression),
            marks: self.marks.clone(),
        }
    }

    /// Builds the underline string for the error: `~` characters under each marked
    /// span and spaces elsewhere. Returns `None` when there is no expression text or
    /// no mark falls within it.
    ///
    /// Indices are interpreted as character offsets (matching how the lexer produces
    /// them via `input.chars().enumerate()`).
    fn underline(&self) -> Option<String> {
        let chars: Vec<char> = self.original_expression.as_ref().chars().collect();
        let len = chars.len();
        if len == 0 || self.marks.is_empty() {
            return None;
        }

        let mut line = vec![' '; len];
        let mut any = false;
        for mark in &self.marks {
            let start = mark.start().min(len);
            let end = mark.end().min(len);
            if start < end {
                for i in start..end {
                    line[i] = '~';
                }
                any = true;
            }
        }

        if any {
            Some(line.into_iter().collect::<String>().trim_end().to_owned())
        } else {
            None
        }
    }
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}] {}\n{}",
            self.category, self.message, self.original_expression
        )?;
        if let Some(underline) = self.underline() {
            write!(f, "\n{}", underline)?;
        }
        writeln!(f)
    }
}

impl std::error::Error for ExpressionError {}
