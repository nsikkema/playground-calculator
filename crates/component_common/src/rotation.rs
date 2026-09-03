/// A rotation constrained to quarter-turn increments.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// No rotation.
    #[default]
    Degrees0 = 0,
    /// A 90-degree rotation.
    Degrees90 = 90,
    /// A 180-degree rotation.
    Degrees180 = 180,
    /// A 270-degree rotation.
    Degrees270 = 270,
}

impl Rotation {
    /// Returns the rotation in degrees.
    #[must_use]
    pub const fn degrees(self) -> u16 {
        match self {
            Self::Degrees0 => 0,
            Self::Degrees90 => 90,
            Self::Degrees180 => 180,
            Self::Degrees270 => 270,
        }
    }
}
