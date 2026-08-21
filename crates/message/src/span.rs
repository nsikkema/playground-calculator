use core::fmt;

/// A half-open byte range `[start, start + size)` within an expression string,
/// used to point at tokens and sub-expressions for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The byte offset of the first character covered by this span.
    start: usize,
    /// The number of bytes covered by this span (minimum 1).
    size: usize,
}

impl Span {
    /// Creates a new `Span` starting at `start` with the given `size`.
    ///
    /// The size is clamped to a minimum of 1 so that every span covers at least one character.
    #[hotpath::measure]
    #[must_use]
    pub fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            size: size.max(1),
        }
    }

    /// Returns the byte offset of the first character covered by this span.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the byte offset one past the last character covered by this span.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.start.saturating_add(self.size)
    }

    /// Returns the smallest span that covers both `self` and `other`.
    #[hotpath::measure]
    #[must_use]
    pub fn join(&self, other: &Span) -> Span {
        let new_start = self.start.min(other.start);
        let new_end = self.end().max(other.end());
        let new_size = new_end.saturating_sub(new_start).max(1);
        Span::new(new_start, new_size)
    }

    /// Returns `true` if this span overlaps with `other` (i.e., they share at least one byte position).
    #[must_use]
    pub const fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

impl fmt::Display for Span {
    #[hotpath::measure]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end())
    }
}

/// An ordered, non-overlapping collection of [`Span`]s, used to highlight multiple
/// disjoint regions of source text in a single error message.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpanSet {
    /// The sorted, merged list of spans that make up this set.
    indices: Vec<Span>,
}

impl SpanSet {
    /// Creates an empty `SpanSet`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            indices: Vec::new(),
        }
    }

    /// Creates a `SpanSet` containing a single span.
    #[hotpath::measure]
    #[must_use]
    pub fn from_span(index: Span) -> Self {
        Self {
            indices: vec![index],
        }
    }

    #[allow(dead_code)]
    /// Creates a `SpanSet` from an unsorted slice of spans, merging any overlapping spans.
    #[hotpath::measure]
    #[must_use]
    pub fn new_with_indices(indices: Vec<Span>) -> Self {
        let mut index_set = Self { indices };
        index_set.sort_and_merge();
        index_set
    }

    /// Sorts the spans by start position and merges any that overlap.
    #[hotpath::measure]
    fn sort_and_merge(&mut self) {
        self.indices.sort_by_key(Span::start);
        let mut merged_indices: Vec<Span> = Vec::new();
        for index in &self.indices {
            if let Some(last) = merged_indices.last_mut() {
                if index.overlaps(last) {
                    *last = last.join(index);
                } else {
                    merged_indices.push(*index);
                }
            } else {
                merged_indices.push(*index);
            }
        }
        self.indices = merged_indices;
    }

    /// Returns an iterator over the spans in this set, in sorted order.
    #[hotpath::measure]
    pub fn iter(&self) -> impl Iterator<Item = &Span> {
        self.indices.iter()
    }

    /// Returns `true` if this set contains no spans.
    #[hotpath::measure]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    #[allow(dead_code)]
    /// Adds a span to the set, re-sorting and merging as needed.
    #[hotpath::measure]
    pub fn add(&mut self, index: Span) {
        self.indices.push(index);
        self.sort_and_merge();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join() {
        let index1 = Span::new(0, 5);
        let index2 = Span::new(3, 4);
        let joined_index = index1.join(&index2);
        assert_eq!(joined_index.start(), 0);
        assert_eq!(joined_index.end(), 7);
    }

    #[test]
    fn test_join_overlapping() {
        let index1 = Span::new(0, 5);
        let index2 = Span::new(3, 4);
        let joined_index = index1.join(&index2);
        assert_eq!(joined_index.start(), 0);
        assert_eq!(joined_index.end(), 7);
    }

    #[test]
    fn test_join_non_overlapping() {
        let index1 = Span::new(0, 5);
        let index2 = Span::new(6, 4);
        let joined_index = index1.join(&index2);
        assert_eq!(joined_index.start(), 0);
        assert_eq!(joined_index.end(), 10);
    }

    #[test]
    fn test_sort_and_merge() {
        let mut index_set = SpanSet::new();
        index_set.add(Span::new(0, 5));
        index_set.add(Span::new(3, 4));
        index_set.add(Span::new(10, 2));
        index_set.add(Span::new(9, 3));
        index_set.sort_and_merge();
        assert_eq!(index_set.indices.len(), 2);
        assert_eq!(index_set.indices[0].start(), 0);
        assert_eq!(index_set.indices[0].end(), 7);
        assert_eq!(index_set.indices[1].start(), 9);
        assert_eq!(index_set.indices[1].end(), 12);
    }

    #[test]
    fn test_from_index() {
        let index = Span::new(0, 5);
        let index_set = SpanSet::from_span(index);
        assert_eq!(index_set.indices.len(), 1);
        assert_eq!(index_set.indices[0].start(), 0);
        assert_eq!(index_set.indices[0].end(), 5);
    }

    #[test]
    fn test_new_with_indices() {
        let indices = vec![Span::new(0, 5), Span::new(3, 4), Span::new(10, 2)];
        let index_set = SpanSet::new_with_indices(indices);
        assert_eq!(index_set.indices.len(), 2);
        assert_eq!(index_set.indices[0].start(), 0);
        assert_eq!(index_set.indices[0].end(), 7);
        assert_eq!(index_set.indices[1].start(), 10);
        assert_eq!(index_set.indices[1].end(), 12);
    }
}
