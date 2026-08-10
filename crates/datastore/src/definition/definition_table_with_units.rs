use crate::definition::NumberWithUnitsDefinition;
use crate::traits::TreePrint;
use keys::store_key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Definition for a table whose named columns are numbers with units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableWithUnitsDefinition {
    /// Human-readable description of this table parameter.
    description: ShareableString,
    /// Column keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<StoreKey>,
    /// Column definitions keyed by a column name.
    columns: Arc<BTreeMap<StoreKey, NumberWithUnitsDefinition>>,
}

impl TableWithUnitsDefinition {
    /// Creates a new `TableWithUnitsDefinition` with a description and a list of columns.
    pub fn new<S1: Into<ShareableString>, K: Into<StoreKey>>(
        description: S1,
        columns: Vec<(K, NumberWithUnitsDefinition)>,
    ) -> Self {
        let mut items = BTreeMap::new();
        let mut ordered_keys = Vec::new();
        for (k, v) in columns {
            let key = k.into();
            items.insert(key.clone(), v);
            ordered_keys.push(key);
        }
        Self {
            description: description.into(),
            ordered_keys,
            columns: Arc::new(items),
        }
    }

    /// Returns the description of the table.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns true if the table contains a column with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        let key = key.into();
        self.columns.keys().any(|column_key| column_key == &key)
    }

    /// Returns a reference to the column definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&NumberWithUnitsDefinition> {
        let key = key.into();
        self.columns
            .iter()
            .find_map(|(column_key, column_def)| (column_key == &key).then_some(column_def))
    }

    /// Returns a reference to the column definition for the specified index.
    #[must_use]
    pub fn get_by_index(&self, index: usize) -> Option<&NumberWithUnitsDefinition> {
        self.ordered_keys
            .get(index)
            .and_then(|key| self.columns.get(key))
    }

    /// Returns the index of the column with the specified key.
    pub fn get_column_index_by_name<S: Into<ShareableString>>(&self, key: S) -> Option<usize> {
        let key = key.into();
        self.ordered_keys
            .iter()
            .position(|column_key| column_key == &key)
    }

    /// Returns true if the table contains a column with the specified key string.
    #[must_use]
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.columns.contains_key(key)
    }

    /// Returns a reference to the column definition for the specified key string.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<&NumberWithUnitsDefinition> {
        self.columns.get(key)
    }

    /// Returns an iterator over the keys of the columns.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.ordered_keys.iter()
    }

    /// Returns an iterator over the column definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &NumberWithUnitsDefinition)> {
        self.ordered_keys
            .iter()
            .filter_map(move |key| self.columns.get(key).map(|v| (key, v)))
    }

    /// Returns the number of columns in the table.
    #[must_use]
    pub fn count(&self) -> usize {
        self.columns.len()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `TableWithUnitsDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            columns: Arc::new(
                self.columns
                    .iter()
                    .map(|(id, item)| (id.launder(store), item.launder(store)))
                    .collect(),
            ),
            ordered_keys: self.ordered_keys.iter().map(|k| k.launder(store)).collect(),
        }
    }
}

impl PartialEq<&TableWithUnitsDefinition> for TableWithUnitsDefinition {
    fn eq(&self, other: &&TableWithUnitsDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<TableWithUnitsDefinition> for &TableWithUnitsDefinition {
    fn eq(&self, other: &TableWithUnitsDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for TableWithUnitsDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Table with units",
            prefix,
            Self::branch_char(last),
            label,
            self.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);
        let mut column_iter = self.ordered_keys.iter().peekable();

        while let Some(key) = column_iter.next() {
            let is_last = column_iter.peek().is_none();
            if let Some(column) = self.columns.get(key) {
                column.tree_print(f, key.as_str(), &child_prefix, is_last)?;
            }
        }

        Ok(())
    }
}
