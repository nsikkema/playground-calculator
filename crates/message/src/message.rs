use crate::path::Path;
use shareable_string::{ShareableString, SharedStringTranslationMap, TranslateMessage};
use std::collections::HashMap;
use std::fmt;

/// An enumeration representing the level of message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageLevel {
    /// A message that is for debugging purposes.
    Debug,
    /// A message that is informational in nature.
    Info,
    /// A message that is a warning.
    Warning,
    /// A message that is an error.
    Error,
}

impl fmt::Display for MessageLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MessageLevel::Debug => "Debug",
            MessageLevel::Info => "Info",
            MessageLevel::Warning => "Warning",
            MessageLevel::Error => "Error",
        };
        f.write_str(s)
    }
}

/// A message that is associated with a specific category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageCategory {
    /// A message related to expression parsing.
    ExpressionParsing,
    /// A message related to expression evaluation.
    ExpressionEvaluation,
}

impl fmt::Display for MessageCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MessageCategory::ExpressionParsing => "Expression Parsing",
            MessageCategory::ExpressionEvaluation => "Expression Evaluation",
        };
        f.write_str(s)
    }
}

/// A message with detailed information, including its path, level, category, and content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// The path of the object associated with this message.
    object_path: Box<Path>,
    /// The path of the item associated with this message.
    item_path: Option<Box<Path>>,
    /// The severity level of this message.
    level: MessageLevel,
    /// The category of this message.
    category: MessageCategory,
    /// The key/params for the message.
    translate_data: Box<TranslateMessage>,
    /// The translated message, if available.
    extra_detail: Option<ShareableString>,
}

impl Message {
    /// Creates a new `Message` with the specified details.
    #[hotpath::measure]
    pub fn new(
        object_path: Path,
        item_path: Option<Path>,
        level: MessageLevel,
        category: MessageCategory,
        translate_data: TranslateMessage,
        extra_detail: Option<ShareableString>,
    ) -> Self {
        Self {
            object_path: Box::new(object_path),
            item_path: item_path.map(Box::new),
            level,
            category,
            translate_data: Box::new(translate_data),
            extra_detail,
        }
    }

    /// Creates a new `Message` with the specified details, using a message key and parameters.
    #[hotpath::measure]
    #[must_use]
    pub fn new_with_params(
        object_path: Path,
        item_path: Option<Path>,
        level: MessageLevel,
        category: MessageCategory,
        message_key: ShareableString,
        message_params: HashMap<ShareableString, ShareableString>,
        extra_detail: Option<ShareableString>,
    ) -> Self {
        let translate_data = TranslateMessage::new(message_key, message_params);
        Self::new(
            object_path,
            item_path,
            level,
            category,
            translate_data,
            extra_detail,
        )
    }

    /// Returns the path of the object associated with this message.
    #[must_use]
    pub const fn object_path(&self) -> &Path {
        &self.object_path
    }

    /// Returns the path of the item associated with this message.
    #[must_use]
    pub const fn item_path(&self) -> Option<&Path> {
        match &self.item_path {
            Some(path) => Some(path),
            None => None,
        }
    }

    /// Returns the level of this message.
    #[must_use]
    pub const fn level(&self) -> MessageLevel {
        self.level
    }

    /// Returns the category of this message.
    #[must_use]
    pub const fn category(&self) -> MessageCategory {
        self.category
    }

    /// Returns the translation message associated with this message.
    #[must_use]
    pub fn launder(&self, store: &shareable_string::SharedStringStore) -> Self {
        Self {
            object_path: Box::new(self.object_path.launder(store)),
            item_path: self
                .item_path
                .as_deref()
                .map(|path| Box::new(path.launder(store))),
            level: self.level,
            category: self.category,
            translate_data: Box::new(self.translate_data.launder(store)),
            extra_detail: self.extra_detail.as_ref().map(|d| store.launder(d)),
        }
    }

    /// Translates the message using the provided translation map and language.
    #[must_use]
    pub fn translated_message(
        &self,
        translation_map: &SharedStringTranslationMap,
        lang: &str,
    ) -> Option<ShareableString> {
        self.translate_data.translate(translation_map, lang)
    }

    /// Returns the translation message associated with this message.
    #[must_use]
    pub const fn translate_data(&self) -> &TranslateMessage {
        &self.translate_data
    }

    /// Returns the extra detail associated with this message, if available.
    #[must_use]
    pub const fn extra_detail(&self) -> Option<&ShareableString> {
        self.extra_detail.as_ref()
    }

    /// Determines if this message should be displayed based on the provided minimum level.
    #[must_use]
    pub const fn should_display(&self, min_level: MessageLevel) -> bool {
        matches!(
            (self.level, min_level),
            (
                MessageLevel::Debug | MessageLevel::Info | MessageLevel::Warning,
                MessageLevel::Debug
            ) | (
                MessageLevel::Info | MessageLevel::Warning,
                MessageLevel::Info
            ) | (MessageLevel::Warning, MessageLevel::Warning)
                | (
                    MessageLevel::Error,
                    MessageLevel::Debug
                        | MessageLevel::Info
                        | MessageLevel::Warning
                        | MessageLevel::Error
                )
        )
    }
}
