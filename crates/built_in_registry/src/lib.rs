//! Definitions and lookup tables for the application's built-in components.

/// Registry of built-in component definitions.
pub mod components;
/// Component definitions.
pub mod definitions;
/// Lookup tables generated from the available component definitions.
pub mod registry;

pub use components::*;
pub use definitions::*;
