//! Convenience re-exports for common types and macros.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use datastore::prelude::*;
//! ```

// Macros
pub use keys::{global_key, parameter_key, store_key, unit_key, variable_key};

// Core types
pub use errors::StoreError;
pub use keys::{
    global_key::{ConstGlobalKey, GlobalKey},
    parameter_key::{ConstParameterKey, ParameterKey},
    store_key::{ConstStoreKey, StoreKey},
    unit_key::{ConstUnitKey, UnitKey},
    variable_key::{ConstVariableKey, VariableKey},
};

// Definitions
pub use crate::definition::{
    BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition,
    GlobalObjectDefinition, GlobalObjectDefinitionBuilder, IntegerConstraint,
    IntegerConstraintEnum, IntegerDefinition, ItemDefinitionType, MapDefinition, MapItemDefinition,
    NumberConstraint, NumberConstraintEnum, NumberDefinition, NumberWithUnitsDefinition,
    ParameterObjectDefinition, ParameterObjectDefinitionBuilder, StringDefinition, TableDefinition,
    VariableObjectDefinition, VariableObjectDefinitionBuilder,
};

// Shareable strings
pub use shareable_string::{ShareableString, SharedStringStore, SharedStringTranslationMap};

// Frozen data
pub use crate::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, GlobalObjectFrozen, IntegerFrozen, ItemFrozen,
    MapEntryFrozen, MapFrozen, MapItemFrozen, NumberFrozen, ParameterObjectFrozen, StringFrozen,
    TableFrozen, VariableObjectFrozen,
};

// Editable data
pub use crate::editable::{
    BooleanEditable, ChoiceEditable, FileEditable, GlobalObjectEditable, IntegerEditable,
    ItemEditable, MapEditable, MapEntryEditable, MapItemEditable, NumberEditable,
    ParameterObjectEditable, StringEditable, TableEditable, VariableObjectEditable,
    editable_set_map_value, editable_set_value,
};
