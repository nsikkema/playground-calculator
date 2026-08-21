//! This module contains the `message` and `path` modules, which provide functionality for working with messages and paths.

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

/// The `message` module contains the `Message` struct, which represents a message that can be sent or received.
pub mod message;
/// The `path` module contains the `Path` struct, which represents a path to an object or item.
pub mod path;
/// The `span` module provides structures and methods for managing ranges of indices, which can be used to represent spans of text or other sequential data.
pub mod span;
