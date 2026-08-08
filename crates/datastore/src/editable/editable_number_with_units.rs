use crate::definition::NumberWithUnitsDefinition;
use crate::frozen::NumberWithUnitsFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents number with units data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberWithUnitsEditable {
    /// Definition metadata for this number value.
    definition: NumberWithUnitsDefinition,
    /// Current value for this number data, stored as a `ShareableString`.
    value: ShareableString,
    /// Current units for this number data, stored as a `ShareableString`.
    units: ShareableString,
}

impl NumberWithUnitsEditable {
    /// Creates a new `NumberWithUnitsEditable` instance from a given `NumberWithUnitsFrozen` value.
    #[must_use]
    pub fn new(frozen_number: &NumberWithUnitsFrozen) -> Self {
        Self {
            definition: frozen_number.definition().clone(),
            value: frozen_number.value(),
            units: frozen_number.units(),
        }
    }

    /// Converts the current `NumberWithUnitsEditable` instance into a `NumberWithUnitsFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> NumberWithUnitsFrozen {
        NumberWithUnitsFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns the units as a `ShareableString`.
    #[must_use]
    pub fn units(&self) -> ShareableString {
        self.units.clone()
    }

    /// Returns a reference to the number definition.
    #[must_use]
    pub const fn definition(&self) -> &NumberWithUnitsDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }

    /// Sets the units and updates the hash.
    pub fn set_units<S: Into<ShareableString>>(&mut self, units: S) {
        self.units = units.into();
    }
}

impl PartialEq<&NumberWithUnitsEditable> for NumberWithUnitsEditable {
    fn eq(&self, other: &&NumberWithUnitsEditable) -> bool {
        self == *other
    }
}

impl PartialEq<NumberWithUnitsEditable> for &NumberWithUnitsEditable {
    fn eq(&self, other: &NumberWithUnitsEditable) -> bool {
        *self == other
    }
}

impl TreePrint for NumberWithUnitsEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) NumberWithUnits - \"{}\" \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
            self.units,
        )
    }
}
