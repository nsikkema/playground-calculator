use crate::definition::{
    BooleanDefinition, ChoiceDefinition, FileDefinition, IntegerDefinition, NumberDefinition,
    NumberWithUnitsDefinition, StringDefinition, TableDefinition,
};
use crate::traits::TreePrint;
use keys::store_key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// The definition of an item within a map entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapItemDefinition {
    /// A boolean parameter.
    Boolean(BooleanDefinition),
    /// A choice parameter.
    Choice(ChoiceDefinition),
    /// A file parameter.
    File(FileDefinition),
    /// An integer parameter.
    Integer(IntegerDefinition),
    /// A number parameter.
    Number(NumberDefinition),
    /// A number with units parameter.
    NumberWithUnits(NumberWithUnitsDefinition),
    /// A string parameter.
    String(StringDefinition),
    /// A table parameter.
    Table(TableDefinition),
}

impl From<BooleanDefinition> for MapItemDefinition {
    fn from(definition: BooleanDefinition) -> Self {
        Self::Boolean(definition)
    }
}

impl From<ChoiceDefinition> for MapItemDefinition {
    fn from(definition: ChoiceDefinition) -> Self {
        Self::Choice(definition)
    }
}

impl From<FileDefinition> for MapItemDefinition {
    fn from(definition: FileDefinition) -> Self {
        Self::File(definition)
    }
}

impl From<IntegerDefinition> for MapItemDefinition {
    fn from(definition: IntegerDefinition) -> Self {
        Self::Integer(definition)
    }
}

impl From<NumberDefinition> for MapItemDefinition {
    fn from(definition: NumberDefinition) -> Self {
        Self::Number(definition)
    }
}

impl From<StringDefinition> for MapItemDefinition {
    fn from(definition: StringDefinition) -> Self {
        Self::String(definition)
    }
}

impl From<TableDefinition> for MapItemDefinition {
    fn from(definition: TableDefinition) -> Self {
        Self::Table(definition)
    }
}

impl MapItemDefinition {
    /// Returns a new `MapItemDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Boolean(def) => Self::Boolean(def.launder(store)),
            Self::Choice(def) => Self::Choice(def.launder(store)),
            Self::File(def) => Self::File(def.launder(store)),
            Self::Integer(def) => Self::Integer(def.launder(store)),
            Self::Number(def) => Self::Number(def.launder(store)),
            Self::NumberWithUnits(def) => Self::NumberWithUnits(def.launder(store)),
            Self::String(def) => Self::String(def.launder(store)),
            Self::Table(def) => Self::Table(def.launder(store)),
        }
    }
}

impl PartialEq<&MapItemDefinition> for MapItemDefinition {
    fn eq(&self, other: &&MapItemDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<MapItemDefinition> for &MapItemDefinition {
    fn eq(&self, other: &MapItemDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for MapItemDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            MapItemDefinition::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            MapItemDefinition::Choice(choice) => choice.tree_print(f, label, prefix, last),
            MapItemDefinition::File(file) => file.tree_print(f, label, prefix, last),
            MapItemDefinition::Integer(integer) => integer.tree_print(f, label, prefix, last),
            MapItemDefinition::Number(number) => number.tree_print(f, label, prefix, last),
            MapItemDefinition::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            MapItemDefinition::String(string) => string.tree_print(f, label, prefix, last),
            MapItemDefinition::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}

/// Definition for a map parameter where keys are strings and values follow a fixed schema of
/// named `MapItemDefinition`s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapDefinition {
    /// Human-readable description of this map parameter.
    description: ShareableString,
    /// Keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<StoreKey>,
    /// Schema of the map entries, keyed by item name.
    item_type: Arc<BTreeMap<StoreKey, MapItemDefinition>>,
}

impl MapDefinition {
    /// Creates a new `MapDefinition` with a description and a list of entry items.
    pub fn new<S1: Into<ShareableString>, K: Into<StoreKey>, I: Into<MapItemDefinition>>(
        description: S1,
        item_type: Vec<(K, I)>,
    ) -> Self {
        let mut items = BTreeMap::new();
        let mut ordered_keys = Vec::new();
        for (k, v) in item_type {
            let key = k.into();
            items.insert(key.clone(), v.into());
            ordered_keys.push(key);
        }
        Self {
            description: description.into(),
            ordered_keys,
            item_type: Arc::new(items),
        }
    }

    /// Returns the description of the map.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the map item definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapItemDefinition> {
        self.item_type.get(&key.into())
    }

    /// Returns a reference to the map item definition for the specified key string.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<&MapItemDefinition> {
        self.item_type
            .iter()
            .find(|(k, _)| k.as_str() == key)
            .map(|(_, v)| v)
    }

    /// Returns true if the map's entry schema contains an item with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.item_type.contains_key(&key.into())
    }

    /// Returns an iterator over the keys of the map's entry schema.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.ordered_keys.iter()
    }

    /// Returns true if the map's entry schema contains an item with the specified key string.
    #[must_use]
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.item_type.iter().any(|(k, _)| k.as_str() == key)
    }

    /// Returns an iterator over the map's entry item definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapItemDefinition)> {
        self.ordered_keys
            .iter()
            .filter_map(move |key| self.item_type.get(key).map(|v| (key, v)))
    }

    /// Returns the number of items in the map's entry schema.
    #[must_use]
    pub fn count(&self) -> usize {
        self.item_type.len()
    }

    /// Returns a reference to the map's entry item type.
    #[must_use]
    pub fn item_type(&self) -> &BTreeMap<StoreKey, MapItemDefinition> {
        &self.item_type
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `MapDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            item_type: Arc::new(
                self.item_type
                    .iter()
                    .map(|(k, v)| (k.launder(store), v.launder(store)))
                    .collect(),
            ),
            ordered_keys: self.ordered_keys.iter().map(|k| k.launder(store)).collect(),
        }
    }
}

impl PartialEq<&MapDefinition> for MapDefinition {
    fn eq(&self, other: &&MapDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<MapDefinition> for &MapDefinition {
    fn eq(&self, other: &MapDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for MapDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Map",
            prefix,
            Self::branch_char(last),
            label,
            self.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.item_type.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}
