use crate::definition::NumberWithUnitsDefinition;
use crate::editable::NumberWithUnitsEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents number data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberWithUnitsFrozen {
    /// Definition metadata for this number value.
    definition: NumberWithUnitsDefinition,
    /// Current numeric value as a string.
    value: ShareableString,
    /// Current units for this number data.
    units: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl NumberWithUnitsFrozen {
    /// Creates a new `NumberWithUnitsFrozen` instance.
    #[must_use]
    pub fn new(definition: NumberWithUnitsDefinition) -> Self {
        let value = definition.default_value();
        let units = definition.preferred_units().string_id().into();

        let mut s = Self {
            definition,
            value,
            units,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `NumberWithUnitsFrozen` instance with a specified value.
    #[must_use]
    pub fn new_with_value(
        definition: NumberWithUnitsDefinition,
        value: ShareableString,
        units: ShareableString,
    ) -> Self {
        let mut s = Self {
            definition,
            value,
            units,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `NumberWithUnitsFrozen` instance from a given `NumberEditable` value.
    #[must_use]
    pub fn new_from_editable(basic: &NumberWithUnitsEditable) -> Self {
        let definition = basic.definition().clone();
        let value = basic.value();
        let units = basic.units();
        let mut s = Self {
            definition,
            value,
            units,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `NumberWithUnitsFrozen` instance into a `NumberWithUnitsEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> NumberWithUnitsEditable {
        NumberWithUnitsEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of the current value.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Number");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
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

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&NumberWithUnitsFrozen> for NumberWithUnitsFrozen {
    fn eq(&self, other: &&NumberWithUnitsFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<NumberWithUnitsFrozen> for &NumberWithUnitsFrozen {
    fn eq(&self, other: &NumberWithUnitsFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for NumberWithUnitsFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Number - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
