/// Definitions for boolean-based data structures.
pub mod definition_boolean;
/// Definitions for choice-based data structures.
pub mod definition_choice;
/// Common definitions used across multiple data structures.
pub mod definition_common;
/// Definitions for file data structures.
pub mod definition_file;
/// Definitions for integer-based data structures.
pub mod definition_integer;
/// Definitions for parameter within objects or containers.
pub mod definition_item;
/// Definitions for map-based data structures.
pub mod definition_map;
/// Definitions for number-based data structures.
pub mod definition_number;
/// Definitions for number-based data structures with units.
pub mod definition_number_with_units;
/// Definitions for object-based data structures.
pub mod definition_object_global;
/// Definitions for object parameter configurations within data structures.
pub mod definition_object_parameter;
/// Definitions for object variable configurations within data structures.
pub mod definition_object_variable;
/// Definitions for string-based data structures.
pub mod definition_string;
/// Definitions for table-based data structures.
pub mod definition_table;

pub use definition_boolean::*;
pub use definition_choice::*;
pub use definition_common::*;
pub use definition_file::*;
pub use definition_integer::*;
pub use definition_item::*;
pub use definition_map::*;
pub use definition_number::*;
pub use definition_number_with_units::*;
pub use definition_object_global::*;
pub use definition_object_parameter::*;
pub use definition_object_variable::*;
pub use definition_string::*;
pub use definition_table::*;
