#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Index {
    start: usize,
    size: usize,
}

impl Index {
    pub(crate) fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            size: size.max(1),
        }
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }

    pub(crate) fn end(&self) -> usize {
        self.start + self.size
    }

    pub(crate) fn join(&self, other: &Index) -> Index {
        let new_start = self.start.min(other.start);
        let new_end = self.end().max(other.end());
        let new_size = (new_end - new_start).max(1);
        Index::new(new_start, new_size)
    }
}

#[test]
fn test_join() {
    let index1 = Index::new(0, 5);
    let index2 = Index::new(3, 4);
    let joined_index = index1.join(&index2);
    assert_eq!(joined_index.start(), 0);
    assert_eq!(joined_index.end(), 7);
}
