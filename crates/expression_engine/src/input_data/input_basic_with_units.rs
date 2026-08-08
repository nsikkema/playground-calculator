use crate::BasicDefinition;
use shareable_string::{ShareableString, SharedStringStore};

/// Represents basic input data in the system.
///
/// The `BasicInputData` struct is used to encapsulate
/// the definition of a basic input data item.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicInputWithUnitsData {
    /// The definition describing valid values for this input item.
    definition: BasicDefinition,
    /// The raw string value provided by the user.
    data: ShareableString,
    /// The raw string value for the units provided by the user.
    units: ShareableString,
}

impl BasicInputWithUnitsData {
    /// Creates a new `BasicInputWithUnitsData` with the given `definition`, raw `data`, and `units`.
    pub(crate) const fn new(
        definition: BasicDefinition,
        data: ShareableString,
        units: ShareableString,
    ) -> Self {
        Self {
            definition,
            data,
            units,
        }
    }

    /// Returns a reference to the definition of the basic input data.
    #[must_use]
    pub const fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns a reference to the data of the basic input data.
    #[must_use]
    pub const fn data(&self) -> &ShareableString {
        &self.data
    }

    /// Returns a reference to the units of the basic input data.
    #[must_use]
    pub const fn units(&self) -> &ShareableString {
        &self.units
    }

    /// Returns a new `BasicInputWithUnitsData` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            definition: self.definition.launder(store),
            data: store.launder(&self.data),
            units: store.launder(&self.units),
        }
    }
}
