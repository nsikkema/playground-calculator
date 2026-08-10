use crate::definition::{MapDefinition, MapItemDefinition};
use crate::editable::{MapEditable, MapEntryEditable, MapItemEditable};
use crate::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, IntegerFrozen, NumberFrozen, NumberWithUnitsFrozen,
    StringFrozen, TableFrozen, TableWithUnitsFrozen, UnitFrozen,
};
use crate::traits::TreePrint;
use errors::StoreError;
use keys::store_key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents an item within a frozen map entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapItemFrozen {
    /// A boolean value.
    Boolean(BooleanFrozen),
    /// A choice value.
    Choice(ChoiceFrozen),
    /// A file value.
    File(FileFrozen),
    /// An integer value.
    Integer(IntegerFrozen),
    /// A number value.
    Number(NumberFrozen),
    /// A number value with associated units.
    NumberWithUnits(NumberWithUnitsFrozen),
    /// A string value.
    String(StringFrozen),
    /// A table value.
    Table(TableFrozen),
    /// A table value with associated units.
    TableWithUnits(TableWithUnitsFrozen),
    /// A unit value.
    Unit(UnitFrozen),
}

impl MapItemFrozen {
    /// Returns the string value if this item is a string value.
    #[must_use]
    pub const fn get_string(&self) -> Option<&StringFrozen> {
        match self {
            MapItemFrozen::String(string) => Some(string),
            _ => None,
        }
    }

    /// Returns the table value if this item is a table value.
    #[must_use]
    pub const fn get_table(&self) -> Option<&TableFrozen> {
        match self {
            MapItemFrozen::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Returns the unit value if this item is a unit value.
    #[must_use]
    pub const fn get_unit(&self) -> Option<&UnitFrozen> {
        match self {
            MapItemFrozen::Unit(unit) => Some(unit),
            _ => None,
        }
    }

    /// Returns the map item definition.
    #[must_use]
    pub fn definition(&self) -> MapItemDefinition {
        match self {
            MapItemFrozen::Boolean(boolean) => {
                MapItemDefinition::Boolean(boolean.definition().clone())
            }
            MapItemFrozen::Choice(choice) => MapItemDefinition::Choice(choice.definition().clone()),
            MapItemFrozen::File(file) => MapItemDefinition::File(file.definition().clone()),
            MapItemFrozen::Integer(integer) => {
                MapItemDefinition::Integer(integer.definition().clone())
            }
            MapItemFrozen::Number(number) => MapItemDefinition::Number(number.definition().clone()),
            MapItemFrozen::NumberWithUnits(number_with_units) => {
                MapItemDefinition::NumberWithUnits(number_with_units.definition().clone())
            }
            MapItemFrozen::String(basic) => MapItemDefinition::String(basic.definition().clone()),
            MapItemFrozen::Table(table) => MapItemDefinition::Table(table.definition().clone()),
            MapItemFrozen::TableWithUnits(table_with_units) => {
                MapItemDefinition::TableWithUnits(table_with_units.definition().clone())
            }
            MapItemFrozen::Unit(unit) => MapItemDefinition::Unit(unit.definition().clone()),
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the item.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        match self {
            MapItemFrozen::Boolean(boolean) => boolean.hash(),
            MapItemFrozen::Choice(choice) => choice.hash(),
            MapItemFrozen::File(file) => file.hash(),
            MapItemFrozen::Integer(integer) => integer.hash(),
            MapItemFrozen::Number(number) => number.hash(),
            MapItemFrozen::NumberWithUnits(number_with_units) => number_with_units.hash(),
            MapItemFrozen::String(basic) => basic.hash(),
            MapItemFrozen::Table(table) => table.hash(),
            MapItemFrozen::TableWithUnits(table_with_units) => table_with_units.hash(),
            MapItemFrozen::Unit(unit) => unit.hash(),
        }
    }

    /// Creates a new `MapItemFrozen` instance from a given `MapItemEditable` value.
    #[must_use]
    pub fn new_from_editable(item: &MapItemEditable) -> Self {
        match item {
            MapItemEditable::Boolean(boolean) => {
                MapItemFrozen::Boolean(BooleanFrozen::new_from_editable(boolean))
            }
            MapItemEditable::Choice(choice) => {
                MapItemFrozen::Choice(ChoiceFrozen::new_from_editable(choice))
            }
            MapItemEditable::File(file) => MapItemFrozen::File(FileFrozen::new_from_editable(file)),
            MapItemEditable::Integer(integer) => {
                MapItemFrozen::Integer(IntegerFrozen::new_from_editable(integer))
            }
            MapItemEditable::Number(number) => {
                MapItemFrozen::Number(NumberFrozen::new_from_editable(number))
            }
            MapItemEditable::NumberWithUnits(number_with_units) => MapItemFrozen::NumberWithUnits(
                NumberWithUnitsFrozen::new_from_editable(number_with_units),
            ),
            MapItemEditable::String(basic) => {
                MapItemFrozen::String(StringFrozen::new_from_editable(basic))
            }
            MapItemEditable::Table(table) => {
                MapItemFrozen::Table(TableFrozen::new_from_editable(table))
            }
            MapItemEditable::TableWithUnits(table_with_units) => MapItemFrozen::TableWithUnits(
                TableWithUnitsFrozen::new_from_editable(table_with_units),
            ),
            MapItemEditable::Unit(unit) => MapItemFrozen::Unit(UnitFrozen::new_from_editable(unit)),
        }
    }

    /// Converts the current `MapItemFrozen` instance into a `MapItemEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> MapItemEditable {
        MapItemEditable::new(self)
    }
}

impl PartialEq<&MapItemFrozen> for MapItemFrozen {
    fn eq(&self, other: &&MapItemFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapItemFrozen> for &MapItemFrozen {
    fn eq(&self, other: &MapItemFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapItemFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            MapItemFrozen::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            MapItemFrozen::Choice(choice) => choice.tree_print(f, label, prefix, last),
            MapItemFrozen::File(file) => file.tree_print(f, label, prefix, last),
            MapItemFrozen::Integer(integer) => integer.tree_print(f, label, prefix, last),
            MapItemFrozen::Number(number) => number.tree_print(f, label, prefix, last),
            MapItemFrozen::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            MapItemFrozen::String(basic) => basic.tree_print(f, label, prefix, last),
            MapItemFrozen::Table(table) => table.tree_print(f, label, prefix, last),
            MapItemFrozen::TableWithUnits(table_with_units) => {
                table_with_units.tree_print(f, label, prefix, last)
            }
            MapItemFrozen::Unit(unit) => unit.tree_print(f, label, prefix, last),
        }
    }
}

/// Represents a single entry's value within a frozen map, following the map's entry schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapEntryFrozen {
    /// The items in the map entry.
    items: BTreeMap<StoreKey, MapItemFrozen>,
    /// The pre-calculated BLAKE3 hash of the entry's content.
    hash: [u8; 32],
}

impl MapEntryFrozen {
    /// Creates a new `MapEntryFrozen` from the map's entry schema.
    #[must_use]
    pub fn new(item_type: &BTreeMap<StoreKey, MapItemDefinition>) -> Self {
        let mut items = BTreeMap::new();
        for (key, item_definition) in item_type {
            match item_definition {
                MapItemDefinition::Boolean(boolean_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::Boolean(BooleanFrozen::new(boolean_definition.clone())),
                    );
                }
                MapItemDefinition::Choice(choice_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::Choice(ChoiceFrozen::new(choice_definition.clone())),
                    );
                }
                MapItemDefinition::File(file_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::File(FileFrozen::new(file_definition.clone())),
                    );
                }
                MapItemDefinition::Integer(integer_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::Integer(IntegerFrozen::new(integer_definition.clone())),
                    );
                }
                MapItemDefinition::Number(number_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::Number(NumberFrozen::new(number_definition.clone())),
                    );
                }
                MapItemDefinition::NumberWithUnits(number_with_units_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::NumberWithUnits(NumberWithUnitsFrozen::new(
                            number_with_units_definition.clone(),
                        )),
                    );
                }
                MapItemDefinition::String(basic_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::String(StringFrozen::new(basic_definition.clone())),
                    );
                }
                MapItemDefinition::Table(table_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::Table(TableFrozen::new(table_definition.clone())),
                    );
                }
                MapItemDefinition::TableWithUnits(table_with_units_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::TableWithUnits(TableWithUnitsFrozen::new(
                            table_with_units_definition.clone(),
                        )),
                    );
                }
                MapItemDefinition::Unit(unit_definition) => {
                    items.insert(
                        key.clone(),
                        MapItemFrozen::Unit(UnitFrozen::new(unit_definition.clone())),
                    );
                }
            }
        }

        let mut s = Self {
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapEntryFrozen` from a set of items.
    #[must_use]
    pub fn new_from_items(items: BTreeMap<StoreKey, MapItemFrozen>) -> Self {
        let mut s = Self {
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapEntryFrozen` from a given `MapEntryEditable` value.
    #[must_use]
    pub fn new_from_editable(editable_entry: &MapEntryEditable) -> Self {
        let items = editable_entry
            .iter()
            .map(|(key, value)| (key.clone(), MapItemFrozen::new_from_editable(value)))
            .collect();
        Self::new_from_items(items)
    }

    /// Converts the current `MapEntryFrozen` instance into a `MapEntryEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> MapEntryEditable {
        MapEntryEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of all items in this map entry.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"MapEntry");

        h.update(
            &u64::try_from(self.items.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );

        for (key, item) in &self.items {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the entry.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the item with the specified key if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapItemFrozen> {
        self.items.get(&key.into())
    }

    /// Return the string value if this item is a string value.
    pub fn get_string<S: Into<ShareableString>>(&self, key: S) -> Option<&StringFrozen> {
        if let Some(item) = self.get(key) {
            item.get_string()
        } else {
            None
        }
    }

    /// Return the table value if this item is a table value.
    pub fn get_table<S: Into<ShareableString>>(&self, key: S) -> Option<&TableFrozen> {
        if let Some(item) = self.get(key) {
            item.get_table()
        } else {
            None
        }
    }

    /// Return the unit value if this item is a unit value.
    pub fn get_unit<S: Into<ShareableString>>(&self, key: S) -> Option<&UnitFrozen> {
        self.get(key).and_then(MapItemFrozen::get_unit)
    }

    /// Returns an iterator over the key-item pairs in the entry.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapItemFrozen)> {
        self.items.iter()
    }

    /// Returns the schema of this entry, derived from its current items.
    #[must_use]
    pub fn definition(&self) -> BTreeMap<StoreKey, MapItemDefinition> {
        self.items
            .iter()
            .map(|(k, v)| (k.clone(), v.definition()))
            .collect()
    }
}

impl PartialEq<&MapEntryFrozen> for MapEntryFrozen {
    fn eq(&self, other: &&MapEntryFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapEntryFrozen> for &MapEntryFrozen {
    fn eq(&self, other: &MapEntryFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapEntryFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "{}{}{}", prefix, Self::branch_char(last), label)?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.items.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

/// Represents a map of parameter in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapFrozen {
    /// The definition of the map.
    definition: MapDefinition,
    /// The items in the map.
    items: BTreeMap<StoreKey, MapEntryFrozen>,
    /// The pre-calculated BLAKE3 hash of the map's content.
    hash: [u8; 32],
}

impl MapFrozen {
    /// Creates a new `MapFrozen` with a definition.
    #[must_use]
    pub fn new(definition: MapDefinition) -> Self {
        let mut s = Self {
            definition,
            items: BTreeMap::new(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapFrozen` with a description and items.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::SchemaMismatch` if the items do not all share the same
    /// entry schema, or `StoreError::MissingSchema` if `items` is empty.
    pub fn new_from_items<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<StoreKey, MapEntryFrozen>,
    ) -> Result<Self, StoreError> {
        let item_schema = if let Some(first_item) = items.values().next() {
            let first_schema = first_item.definition();
            for item in items.values().skip(1) {
                let schema = item.definition();
                if first_schema != schema {
                    return Err(StoreError::SchemaMismatch(format!(
                        "FrozenMap items must have the same entry schema. Expected: {first_schema:?}, Found: {schema:?}"
                    )));
                }
            }
            first_schema
        } else {
            return Err(StoreError::MissingSchema(
                "FrozenMap cannot be empty as item type cannot be inferred".into(),
            ));
        };

        let definition = MapDefinition::new(description, item_schema.into_iter().collect());
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        Ok(s)
    }

    /// Creates a new `MapFrozen` from a given `MapEditable` value.
    #[must_use]
    pub fn new_from_editable(editable_map: &MapEditable) -> Self {
        let definition = editable_map.definition().clone();
        let items = editable_map
            .iter()
            .map(|(key, value)| (key.clone(), MapEntryFrozen::new_from_editable(value)))
            .collect();
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `MapFrozen` instance into a `MapEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> MapEditable {
        MapEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of all entries in this map.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"Map");

        h.update(
            &u64::try_from(self.items.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );

        for (key, item) in &self.items {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the map.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the item with the specified key if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapEntryFrozen> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-item pairs in the map.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapEntryFrozen)> {
        self.items.iter()
    }

    /// Returns a reference to the map definition.
    #[must_use]
    pub const fn definition(&self) -> &MapDefinition {
        &self.definition
    }

    /// Returns the number of items in the map.
    #[must_use]
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl PartialEq<&MapFrozen> for MapFrozen {
    fn eq(&self, other: &&MapFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapFrozen> for &MapFrozen {
    fn eq(&self, other: &MapFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapFrozen {
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
            self.definition.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.items.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}
