use crate::definition::{NumberConstraint, NumberConstraintEnum};
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use units::UnitId;

/// Definition for a number-based parameter with units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberWithUnitsDefinition {
    /// Human-readable description of this number parameter.
    description: ShareableString,
    /// Optional constraint (min, max, range, or none) applied to the value.
    constraint: NumberConstraint,
    /// Units associated with this number parameter.
    preferred_units: UnitId,
    /// Default value for this number parameter.
    default_value: ShareableString,
}

impl NumberWithUnitsDefinition {
    /// Creates a new number-based `NumberWithUnitsDefinition`.
    pub fn new<S1: Into<ShareableString>>(description: S1, preferred_units: UnitId) -> Self {
        Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            preferred_units,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new number-based `NumberWithUnitsDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
        preferred_units: UnitId,
    ) -> Self {
        Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            preferred_units,
            default_value: default_value.into(),
        }
    }

    /// Creates a new number-based `NumberWithUnitsDefinition` with a constraint.
    pub fn new_with_constraint<S: Into<ShareableString>>(
        description: S,
        constraint: NumberConstraint,
        preferred_units: UnitId,
    ) -> Self {
        Self {
            description: description.into(),
            constraint,
            preferred_units,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new number-based `NumberWithUnitsDefinition` with a default value.
    pub fn new_with_constraint_and_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        constraint: NumberConstraint,
        default_value: S2,
        preferred_units: UnitId,
    ) -> Self {
        Self {
            description: description.into(),
            constraint,
            preferred_units,
            default_value: default_value.into(),
        }
    }

    /// Returns the constraint.
    #[must_use]
    pub fn constraint(&self) -> NumberConstraintEnum {
        self.constraint.constraint_enum.clone()
    }

    /// Returns a reference to the constraint.
    #[must_use]
    pub const fn constraint_ref(&self) -> &NumberConstraintEnum {
        &self.constraint.constraint_enum
    }

    /// Returns a new `NumberWithUnitsDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            constraint: self.constraint.clone(),
            default_value: store.launder(&self.default_value),
            preferred_units: self.preferred_units,
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }

    /// Returns the preferred units of the parameter.
    #[must_use]
    pub const fn preferred_units(&self) -> UnitId {
        self.preferred_units
    }

    /// Returns a reference to the preferred units.
    #[must_use]
    pub const fn preferred_units_ref(&self) -> &UnitId {
        &self.preferred_units
    }

    /// Returns the keys of the units in the preferred units' family.
    #[must_use]
    pub fn unit_keys(&self) -> Vec<ShareableString> {
        self.preferred_units
            .family_id()
            .unit_ids()
            .iter()
            .map(|id| id.string_id().into())
            .collect()
    }

    /// Returns the descriptions of the units in the preferred units' family.
    #[must_use]
    pub fn unit_descriptions(&self) -> Vec<ShareableString> {
        self.preferred_units
            .family_id()
            .unit_ids()
            .iter()
            .map(|id| id.description().into())
            .collect()
    }
}

impl PartialEq<&NumberWithUnitsDefinition> for NumberWithUnitsDefinition {
    fn eq(&self, other: &&NumberWithUnitsDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<NumberWithUnitsDefinition> for &NumberWithUnitsDefinition {
    fn eq(&self, other: &NumberWithUnitsDefinition) -> bool {
        *self == other
    }
}

/// Formats an `f64` for display, appending `.0` when the value has no
/// fractional part and is not in scientific notation.
fn format_number_value(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let mut formatted = value.to_string();
    if !formatted.contains('.') && !formatted.contains('e') && !formatted.contains('E') {
        formatted.push_str(".0");
    }

    formatted
}

impl TreePrint for NumberWithUnitsDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let constraint_str = match &self.constraint.constraint_enum {
            NumberConstraintEnum::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    format_number_value(*min),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraintEnum::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    format_number_value(*max),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                let min_type = if *min_inclusive {
                    "inclusive"
                } else {
                    "exclusive"
                };
                let max_type = if *max_inclusive {
                    "inclusive"
                } else {
                    "exclusive"
                };
                format!(
                    " [Range({}, {}, {}, {})]",
                    format_number_value(*min),
                    format_number_value(*max),
                    min_type,
                    max_type
                )
            }
            NumberConstraintEnum::None => String::new(),
        };

        writeln!(
            f,
            "{}{}{} ({}) Number - unit: {} - default: \"{}\"{}",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.preferred_units.description(),
            self.default_value,
            constraint_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::format_number_value;

    #[test]
    fn format_number_value_keeps_fractional_values() {
        assert_eq!(format_number_value(1.52), "1.52");
    }

    #[test]
    fn format_number_value_adds_single_decimal_for_integers() {
        assert_eq!(format_number_value(1.0), "1.0");
        assert_eq!(format_number_value(42.0), "42.0");
    }
}
