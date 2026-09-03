//! Shared component primitives.
//!
//! This crate contains metadata types used by component definitions and
//! registries. [`PortKind`] describes how a component port exchanges data,
//! while [`Rotation`] represents its orientation in quarter-turn increments.

/// Port types.
pub mod ports;
/// Quarter-turn rotation values.
pub mod rotation;

pub use ports::*;
pub use rotation::*;
