//! Convenience re-exports for common types.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use expression_engine::prelude::*;
//! ```

// Errors
pub use crate::{ExpressionCategory, ExpressionError};

// Definitions
pub use crate::BasicDefinition;

// Engine
pub use crate::evaluation::engine::ExpressionEngine;
pub use crate::evaluation::expression::function_definition::{
    ArgumentCount, FunctionDefinition, FunctionDefinitions,
};

// Computed data
pub use crate::computed_data::{
    ComputedItem, ComputedTable, ComputedTableWithUnits, GlobalObjectComputedData,
    ParameterObjectComputedData, VariableObjectComputedData,
};

// Input data
pub use crate::input_data::{
    BasicInputData, GlobalObjectInputData, ObjectItemInputData, ParameterObjectInputData,
    TableInputData, VariableObjectInputData,
};
