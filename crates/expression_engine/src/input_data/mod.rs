/// Basic input data.
pub mod input_basic;
/// Basic input data with an associated unit.
pub mod input_basic_with_units;
/// Input object data.
pub mod input_object;
/// Input table data.
pub mod input_table;
/// Input table data with units.
pub mod input_table_with_units;

pub use input_basic::*;
pub use input_basic_with_units::*;
pub use input_object::*;
pub use input_table::*;
pub use input_table_with_units::*;
