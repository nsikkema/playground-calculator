/// Empty canvas units reserved around each edge of every icon.
pub const ICON_BUFFER: u8 = 1;

/// Static icon metadata for a component.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconDefinition {
    /// SVG data for the icon image.
    svg_image: &'static str,
    /// Width of the icon in canvas units.
    width: u8,
    /// Height of the icon in canvas units.
    height: u8,
}

impl IconDefinition {
    /// Backing constructor for [`icon_definition!`].
    #[must_use]
    pub(crate) const fn __new(svg_image: &'static str, width: u8, height: u8) -> Self {
        Self {
            svg_image,
            width,
            height,
        }
    }

    /// Returns the SVG image data for this icon.
    #[must_use]
    pub const fn svg_image(&self) -> &'static str {
        self.svg_image
    }

    /// Returns the width of the icon in canvas units.
    #[must_use]
    pub const fn width(&self) -> u8 {
        self.width
    }

    /// Returns the height of the icon in canvas units.
    #[must_use]
    pub const fn height(&self) -> u8 {
        self.height
    }
}

/// Creates an [`IconDefinition`] containing the static image and dimensions for a
/// component icon.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression. The dimensions are validated at compile time even when the
/// result is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// icon_definition!(svg_image, width, height)
/// ```
///
/// # Arguments
/// - `svg_image`: `&'static str` containing the SVG image data.
/// - `(width, height)`: `(u8, u8)` tuple containing the width and height of the icon in canvas units.
///
/// Both dimensions must be at least 10 canvas units.
///
/// # Examples
/// ```text
/// const ICON: IconDefinition = icon_definition!("<svg></svg>", (32, 24));
///
/// assert_eq!(ICON.svg_image(), "<svg></svg>");
/// assert_eq!(ICON.width(), 32);
/// assert_eq!(ICON.height(), 24);
/// ```
macro_rules! icon_definition {
    ($svg_image:expr, ($width:expr, $height:expr) $(,)?) => {
        const {
            let width: u8 = $width;
            let height: u8 = $height;
            assert!(
                width >= 10 && height >= 10,
                "icon width and height must be at least 10 canvas units"
            );
            #[allow(clippy::disallowed_methods)]
            $crate::IconDefinition::__new($svg_image, width, height)
        }
    };
}
pub(crate) use icon_definition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_creates_icon_definition() {
        const ICON: IconDefinition = icon_definition!("<svg></svg>", (32, 24));
        const MINIMUM_SIZE_ICON: IconDefinition = icon_definition!("<svg></svg>", (10, 10));

        assert_eq!(ICON.svg_image(), "<svg></svg>");
        assert_eq!(ICON.width(), 32);
        assert_eq!(ICON.height(), 24);
        assert_eq!(MINIMUM_SIZE_ICON.width(), 10);
        assert_eq!(MINIMUM_SIZE_ICON.height(), 10);
    }
}
