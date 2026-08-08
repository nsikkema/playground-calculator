use serde::{Deserialize, Serialize};

/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumberConstraintEnum {
    /// Minimum value constraint.
    Min {
        /// Minimum value of the constraint.
        min: f64,
        /// Whether the minimum value is inclusive.
        inclusive: bool,
    },
    /// Maximum value constraint.
    Max {
        /// Maximum value of the constraint.
        max: f64,
        /// Whether the maximum value is inclusive.
        inclusive: bool,
    },
    /// Range value constraint.
    Range {
        /// Minimum value of the range.
        min: f64,
        /// Maximum value of the range.
        max: f64,
        /// Whether the minimum value is inclusive.
        min_inclusive: bool,
        /// Whether the maximum value is inclusive.
        max_inclusive: bool,
    },
    /// No constraint.
    None,
}

/// Definition for an integer-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NumberConstraint {
    /// The actual constraint variant (none, min, max, or range).
    pub(crate) constraint_enum: NumberConstraintEnum,
}

impl NumberConstraint {
    /// Creates a new `NumberConstraint` with no constraint.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            constraint_enum: NumberConstraintEnum::None,
        }
    }

    /// Creates a new `NumberConstraint` with a minimum value constraint.
    #[must_use]
    pub const fn min(min: f64, inclusive: bool) -> Self {
        Self {
            constraint_enum: NumberConstraintEnum::Min { min, inclusive },
        }
    }

    /// Creates a new `NumberConstraint` with a maximum value constraint.
    #[must_use]
    pub const fn max(max: f64, inclusive: bool) -> Self {
        Self {
            constraint_enum: NumberConstraintEnum::Max { max, inclusive },
        }
    }

    /// Creates a new `NumberConstraint` with a range value constraint.
    ///
    /// If `value_1` is greater than `value_2`, the two values are swapped along with
    /// their corresponding inclusivity flags, so the resulting range is always valid.
    ///
    /// If `value_1` and `value_2` are equal (or within a hair's breadth of it due to
    /// floating-point imprecision), the range is widened symmetrically by `f64::EPSILON`
    /// so `min` and `max` never end up equal.
    #[must_use]
    pub fn range(
        value_1: f64,
        value_2: f64,
        value_1_inclusive: bool,
        value_2_inclusive: bool,
    ) -> Self {
        let (mut min, mut max, min_inclusive, max_inclusive) = if value_1 >= value_2 {
            (value_2, value_1, value_2_inclusive, value_1_inclusive)
        } else {
            (value_1, value_2, value_1_inclusive, value_2_inclusive)
        };

        // If the range is degenerate (or within a hair's breadth of it due to
        // floating-point imprecision), widen it symmetrically by `f64::EPSILON`
        // so `min` and `max` never end up equal.
        if (max - min).abs() < f64::EPSILON {
            min -= f64::EPSILON;
            max += f64::EPSILON;
        }

        Self {
            constraint_enum: NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
        }
    }
}

impl<'de> Deserialize<'de> for NumberConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Mirrors the shape produced by the derived `Serialize` impl above, so the
        // on-the-wire format is unchanged; only construction is routed through the
        // same normalization logic as `NumberConstraint::range` so a deserialized
        // `Range` can never end up with `min > max`.
        #[derive(Deserialize)]
        struct Raw {
            constraint_enum: NumberConstraintEnum,
        }

        let raw = Raw::deserialize(deserializer)?;
        let constraint_enum = match raw.constraint_enum {
            NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                return Ok(NumberConstraint::range(
                    min,
                    max,
                    min_inclusive,
                    max_inclusive,
                ));
            }
            other => other,
        };

        Ok(Self { constraint_enum })
    }
}
