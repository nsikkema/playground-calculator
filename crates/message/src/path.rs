use shareable_string::ShareableString;
use std::fmt::{Display, Formatter};

/// A path to a piece of data within the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    /// The object key part of the path.
    key: ShareableString,
    /// The segments of the path after the object key.
    segments: Vec<ShareableString>,
}

impl<S1, S2> From<(S1, S2)> for Path
where
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
{
    fn from((s1, s2): (S1, S2)) -> Self {
        Path::new(s1).with_segment(s2)
    }
}

impl<S1, S2, S3> From<(S1, S2, S3)> for Path
where
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
    S3: Into<ShareableString>,
{
    fn from((s1, s2, s3): (S1, S2, S3)) -> Self {
        Path::new(s1).with_segment(s2).with_segment(s3)
    }
}

impl<S1, S2, S3, S4> From<(S1, S2, S3, S4)> for Path
where
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
    S3: Into<ShareableString>,
    S4: Into<ShareableString>,
{
    fn from((s1, s2, s3, s4): (S1, S2, S3, S4)) -> Self {
        Path::new(s1)
            .with_segment(s2)
            .with_segment(s3)
            .with_segment(s4)
    }
}

impl<S1, S2, S3, S4, S5> From<(S1, S2, S3, S4, S5)> for Path
where
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
    S3: Into<ShareableString>,
    S4: Into<ShareableString>,
    S5: Into<ShareableString>,
{
    fn from((s1, s2, s3, s4, s5): (S1, S2, S3, S4, S5)) -> Self {
        Path::new(s1)
            .with_segment(s2)
            .with_segment(s3)
            .with_segment(s4)
            .with_segment(s5)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)?;
        for seg in &self.segments {
            write!(f, "/{seg}")?;
        }
        Ok(())
    }
}

impl Path {
    /// Creates a new `Path` pointing to an object.
    pub fn new(object_key: impl Into<ShareableString>) -> Self {
        Self {
            key: object_key.into(),
            segments: vec![],
        }
    }

    /// Returns the object key part of the path.
    #[must_use]
    pub const fn object_key(&self) -> &ShareableString {
        &self.key
    }

    /// Returns the segments of the path after the object key.
    #[must_use]
    pub const fn segments(&self) -> &Vec<ShareableString> {
        &self.segments
    }

    /// Pushes a segment key onto the path and returns the new path.
    #[must_use]
    pub fn with_segment(mut self, key: impl Into<ShareableString>) -> Self {
        self.segments.push(key.into());
        self
    }

    /// Returns a path that points only to the object.
    #[must_use]
    pub fn get_object(&self) -> Self {
        Self {
            key: self.key.clone(),
            segments: vec![],
        }
    }

    /// Returns the last key in the path (either the object key or the last segment's key).
    #[must_use]
    pub fn get_last_key(&self) -> ShareableString {
        self.segments
            .last()
            .cloned()
            .unwrap_or_else(|| self.key.clone())
    }

    /// Launders the path using the provided `SharedStringStore`, returning a new `Path` with laundered strings.
    #[must_use]
    pub fn launder(&self, store: &shareable_string::SharedStringStore) -> Self {
        Self {
            key: store.launder(self.key.clone()),
            segments: self
                .segments
                .iter()
                .map(|s| store.launder(s.clone()))
                .collect(),
        }
    }
}

impl PartialEq<&Path> for Path {
    fn eq(&self, other: &&Path) -> bool {
        self == *other
    }
}

impl PartialEq<Path> for &Path {
    fn eq(&self, other: &Path) -> bool {
        *self == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_paths() {
        // Object path
        let path = Path::new("obj");
        assert_eq!(path.to_string(), "obj");

        // parameter path
        let path = Path::new("obj").with_segment("prop");
        assert_eq!(path.to_string(), "obj/prop");

        // Map entry path
        let path = Path::new("obj").with_segment("prop").with_segment("key");
        assert_eq!(path.to_string(), "obj/prop/key");

        // Map entry item path from parameter
        let path = Path::new("obj").with_segment("prop").with_segment("item");
        assert_eq!(path.to_string(), "obj/prop/item");

        // Map entry item path from map entry
        let path = Path::new("obj")
            .with_segment("prop")
            .with_segment("key")
            .with_segment("item");
        assert_eq!(path.to_string(), "obj/prop/key/item");
    }

    #[test]
    fn test_ergonomic_paths() {
        // From tuple
        let p1: Path = ("obj", "prop").into();
        assert_eq!(p1.to_string(), "obj/prop");

        let p2: Path = ("obj", "prop", "key").into();
        assert_eq!(p2.to_string(), "obj/prop/key");

        let p3: Path = ("obj", "prop", "key", "item").into();
        assert_eq!(p3.to_string(), "obj/prop/key/item");

        let p4: Path = ("obj", "prop", "key", "item", "nested").into();
        assert_eq!(p4.to_string(), "obj/prop/key/item/nested");
    }

    #[test]
    fn test_get_object() {
        let path: Path = ("obj", "prop", "key").into();
        let obj_path = path.get_object();
        assert_eq!(obj_path.to_string(), "obj");
        assert!(obj_path.segments().is_empty());
    }

    #[test]
    fn test_get_last_key_object() {
        let path = Path::new("obj");
        let obj_path = path.get_last_key();
        assert_eq!(obj_path.to_string(), "obj");
    }

    #[test]
    fn test_get_last_key_full_path() {
        let path: Path = ("obj", "prop", "key").into();
        let obj_path = path.get_last_key();
        assert_eq!(obj_path.to_string(), "key");
    }

    #[test]
    fn test_path_equality() {
        let p1: Path = ("obj", "prop", "key").into();
        let p2: Path = ("obj", "prop", "key").into();
        let p3: Path = ("obj", "prop", "other").into();
        let p4: Path = ("other", "prop", "key").into();

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);

        let p5 = Path::new("obj");
        let p6 = Path::new("obj");
        assert_eq!(p5, p6);

        // Reference comparisons
        assert_eq!(p1, &p2);
        assert_eq!(&p1, p2);
        assert_eq!(&p1, &p2);
    }
}
