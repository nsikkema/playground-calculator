use crate::definition::{MapDefinition, MapItemDefinition};
use crate::editable::{
    BooleanEditable, ChoiceEditable, FileEditable, IntegerEditable, NumberEditable,
    NumberWithUnitsEditable, StringEditable, TableEditable, TableWithUnitsEditable, UnitEditable,
};
use crate::frozen::{MapEntryFrozen, MapFrozen, MapItemFrozen};
use crate::traits::TreePrint;
use keys::store_key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents an item within an editable map entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapItemEditable {
    /// A boolean value.
    Boolean(BooleanEditable),
    /// A choice value.
    Choice(ChoiceEditable),
    /// A file value.
    File(FileEditable),
    /// An integer value.
    Integer(IntegerEditable),
    /// A number value.
    Number(NumberEditable),
    /// A number with units value.
    NumberWithUnits(NumberWithUnitsEditable),
    /// A string value.
    String(StringEditable),
    /// A table value.
    Table(TableEditable),
    /// A table with units value.
    TableWithUnits(TableWithUnitsEditable),
    /// A unit value.
    Unit(UnitEditable),
}

impl MapItemEditable {
    /// Creates a new `MapItemEditable` instance from a given `MapItemFrozen` value.
    #[must_use]
    pub fn new(frozen_item: &MapItemFrozen) -> Self {
        match frozen_item {
            MapItemFrozen::Boolean(boolean) => {
                MapItemEditable::Boolean(BooleanEditable::new(boolean))
            }
            MapItemFrozen::Choice(choice) => MapItemEditable::Choice(ChoiceEditable::new(choice)),
            MapItemFrozen::File(file) => MapItemEditable::File(FileEditable::new(file)),
            MapItemFrozen::Integer(integer) => {
                MapItemEditable::Integer(IntegerEditable::new(integer))
            }
            MapItemFrozen::Number(number) => MapItemEditable::Number(NumberEditable::new(number)),
            MapItemFrozen::NumberWithUnits(number_with_units) => {
                MapItemEditable::NumberWithUnits(NumberWithUnitsEditable::new(number_with_units))
            }
            MapItemFrozen::String(basic) => MapItemEditable::String(StringEditable::new(basic)),
            MapItemFrozen::Table(table) => MapItemEditable::Table(TableEditable::new(table)),
            MapItemFrozen::TableWithUnits(table_with_units) => {
                MapItemEditable::TableWithUnits(TableWithUnitsEditable::new(table_with_units))
            }
            MapItemFrozen::Unit(unit) => MapItemEditable::Unit(UnitEditable::new(unit)),
        }
    }

    /// Converts the current `MapItemEditable` instance into a `MapItemFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> MapItemFrozen {
        match self {
            MapItemEditable::Boolean(boolean) => MapItemFrozen::Boolean(boolean.freeze()),
            MapItemEditable::Choice(choice) => MapItemFrozen::Choice(choice.freeze()),
            MapItemEditable::File(file) => MapItemFrozen::File(file.freeze()),
            MapItemEditable::Integer(integer) => MapItemFrozen::Integer(integer.freeze()),
            MapItemEditable::Number(number) => MapItemFrozen::Number(number.freeze()),
            MapItemEditable::NumberWithUnits(number_with_units) => {
                MapItemFrozen::NumberWithUnits(number_with_units.freeze())
            }
            MapItemEditable::String(basic) => MapItemFrozen::String(basic.freeze()),
            MapItemEditable::Table(table) => MapItemFrozen::Table(table.freeze()),
            MapItemEditable::TableWithUnits(table_with_units) => {
                MapItemFrozen::TableWithUnits(table_with_units.freeze())
            }
            MapItemEditable::Unit(unit) => MapItemFrozen::Unit(unit.freeze()),
        }
    }

    /// Returns the boolean value if this item is a boolean value.
    #[must_use]
    pub const fn get_boolean(&self) -> Option<&BooleanEditable> {
        match self {
            MapItemEditable::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    /// Returns a mutable reference to the boolean value if this item is a boolean value.
    #[must_use]
    pub const fn get_mut_boolean(&mut self) -> Option<&mut BooleanEditable> {
        match self {
            MapItemEditable::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    /// Returns the choice value if this item is a choice value.
    #[must_use]
    pub const fn get_choice(&self) -> Option<&ChoiceEditable> {
        match self {
            MapItemEditable::Choice(choice) => Some(choice),
            _ => None,
        }
    }

    /// Returns a mutable reference to the choice value if this item is a choice value.
    #[must_use]
    pub const fn get_mut_choice(&mut self) -> Option<&mut ChoiceEditable> {
        match self {
            MapItemEditable::Choice(choice) => Some(choice),
            _ => None,
        }
    }

    /// Returns the file value if this item is a file value.
    #[must_use]
    pub const fn get_file(&self) -> Option<&FileEditable> {
        match self {
            MapItemEditable::File(file) => Some(file),
            _ => None,
        }
    }

    /// Returns a mutable reference to the file value if this item is a file value.
    #[must_use]
    pub const fn get_mut_file(&mut self) -> Option<&mut FileEditable> {
        match self {
            MapItemEditable::File(file) => Some(file),
            _ => None,
        }
    }

    /// Returns the integer value if this item is an integer value.
    #[must_use]
    pub const fn get_integer(&self) -> Option<&IntegerEditable> {
        match self {
            MapItemEditable::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    /// Returns a mutable reference to the integer value if this item is an integer value.
    #[must_use]
    pub const fn get_mut_integer(&mut self) -> Option<&mut IntegerEditable> {
        match self {
            MapItemEditable::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    /// Returns the number value if this item is a number value.
    #[must_use]
    pub const fn get_number(&self) -> Option<&NumberEditable> {
        match self {
            MapItemEditable::Number(number) => Some(number),
            _ => None,
        }
    }

    /// Returns a mutable reference to the number value if this item is a number value.
    #[must_use]
    pub const fn get_mut_number(&mut self) -> Option<&mut NumberEditable> {
        match self {
            MapItemEditable::Number(number) => Some(number),
            _ => None,
        }
    }

    /// Returns the string value if this item is a string value.
    #[must_use]
    pub const fn get_string(&self) -> Option<&StringEditable> {
        match self {
            MapItemEditable::String(string) => Some(string),
            _ => None,
        }
    }

    /// Returns a mutable reference to the string value if this item is a string value.
    #[must_use]
    pub const fn get_mut_string(&mut self) -> Option<&mut StringEditable> {
        match self {
            MapItemEditable::String(string) => Some(string),
            _ => None,
        }
    }

    /// Returns the table value if this item is a table value.
    #[must_use]
    pub const fn get_table(&self) -> Option<&TableEditable> {
        match self {
            MapItemEditable::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Returns a mutable reference to the table value if this item is a table value.
    #[must_use]
    pub const fn get_mut_table(&mut self) -> Option<&mut TableEditable> {
        match self {
            MapItemEditable::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Returns the unit value if this item is a unit value.
    #[must_use]
    pub const fn get_unit(&self) -> Option<&UnitEditable> {
        match self {
            MapItemEditable::Unit(unit) => Some(unit),
            _ => None,
        }
    }

    /// Returns a mutable reference to the unit value if this item is a unit value.
    #[must_use]
    pub const fn get_mut_unit(&mut self) -> Option<&mut UnitEditable> {
        match self {
            MapItemEditable::Unit(unit) => Some(unit),
            _ => None,
        }
    }

    /// Returns the map item definition.
    #[must_use]
    pub fn definition(&self) -> MapItemDefinition {
        match self {
            MapItemEditable::Boolean(boolean) => {
                MapItemDefinition::Boolean(boolean.definition().clone())
            }
            MapItemEditable::Choice(choice) => {
                MapItemDefinition::Choice(choice.definition().clone())
            }
            MapItemEditable::File(file) => MapItemDefinition::File(file.definition().clone()),
            MapItemEditable::Integer(integer) => {
                MapItemDefinition::Integer(integer.definition().clone())
            }
            MapItemEditable::Number(number) => {
                MapItemDefinition::Number(number.definition().clone())
            }
            MapItemEditable::NumberWithUnits(number_with_units) => {
                MapItemDefinition::NumberWithUnits(number_with_units.definition().clone())
            }
            MapItemEditable::String(basic) => MapItemDefinition::String(basic.definition().clone()),
            MapItemEditable::Table(table) => MapItemDefinition::Table(table.definition().clone()),
            MapItemEditable::TableWithUnits(table_with_units) => {
                MapItemDefinition::TableWithUnits(table_with_units.definition().clone())
            }
            MapItemEditable::Unit(unit) => MapItemDefinition::Unit(unit.definition().clone()),
        }
    }
}

impl PartialEq<&MapItemEditable> for MapItemEditable {
    fn eq(&self, other: &&MapItemEditable) -> bool {
        self == *other
    }
}

impl PartialEq<MapItemEditable> for &MapItemEditable {
    fn eq(&self, other: &MapItemEditable) -> bool {
        *self == other
    }
}

impl TreePrint for MapItemEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            MapItemEditable::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            MapItemEditable::Choice(choice) => choice.tree_print(f, label, prefix, last),
            MapItemEditable::File(file) => file.tree_print(f, label, prefix, last),
            MapItemEditable::Integer(integer) => integer.tree_print(f, label, prefix, last),
            MapItemEditable::Number(number) => number.tree_print(f, label, prefix, last),
            MapItemEditable::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            MapItemEditable::String(basic) => basic.tree_print(f, label, prefix, last),
            MapItemEditable::Table(table) => table.tree_print(f, label, prefix, last),
            MapItemEditable::TableWithUnits(table_with_units) => {
                table_with_units.tree_print(f, label, prefix, last)
            }
            MapItemEditable::Unit(unit) => unit.tree_print(f, label, prefix, last),
        }
    }
}

/// Represents a single entry's value within an editable map, following the map's entry schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapEntryEditable {
    /// The items in the map entry.
    items: BTreeMap<StoreKey, MapItemEditable>,
}

impl MapEntryEditable {
    /// Creates a new `MapEntryEditable` from a `MapEntryFrozen`.
    #[must_use]
    pub fn new(frozen_entry: &MapEntryFrozen) -> Self {
        Self {
            items: frozen_entry
                .iter()
                .map(|(key, value)| (key.clone(), MapItemEditable::new(value)))
                .collect(),
        }
    }

    /// Converts this `MapEntryEditable` into a `MapEntryFrozen`.
    #[must_use]
    pub fn freeze(&self) -> MapEntryFrozen {
        MapEntryFrozen::new_from_editable(self)
    }

    /// Returns a reference to the item with the specified key if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapItemEditable> {
        self.items.get(&key.into())
    }

    /// Returns a mutable reference to the item with the specified key if it exists.
    pub fn get_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut MapItemEditable> {
        self.items.get_mut(key.as_ref())
    }

    /// Return the string value if this item is a string value.
    pub fn get_string<S: Into<ShareableString>>(&self, key: S) -> Option<&StringEditable> {
        if let Some(item) = self.get(key) {
            item.get_string()
        } else {
            None
        }
    }

    /// Return the table value if this item is a table value.
    pub fn get_table<S: Into<ShareableString>>(&self, key: S) -> Option<&TableEditable> {
        if let Some(item) = self.get(key) {
            item.get_table()
        } else {
            None
        }
    }

    /// Returns an iterator over the key-item pairs in the entry.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapItemEditable)> {
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

impl PartialEq<&MapEntryEditable> for MapEntryEditable {
    fn eq(&self, other: &&MapEntryEditable) -> bool {
        self == *other
    }
}

impl PartialEq<MapEntryEditable> for &MapEntryEditable {
    fn eq(&self, other: &MapEntryEditable) -> bool {
        *self == other
    }
}

impl TreePrint for MapEntryEditable {
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

/// Represents a map of parameter in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapEditable {
    /// The definition of the map.
    definition: MapDefinition,
    /// The items in the map.
    items: BTreeMap<StoreKey, MapEntryEditable>,
}

impl MapEditable {
    /// Creates a new `MapEditable` from a `MapFrozen`.
    #[must_use]
    pub fn new(frozen_map: &MapFrozen) -> Self {
        Self {
            definition: frozen_map.definition().clone(),
            items: frozen_map
                .iter()
                .map(|(key, value)| (key.clone(), MapEntryEditable::new(value)))
                .collect(),
        }
    }

    /// Converts this `MapEditable` into a `MapFrozen`.
    #[must_use]
    pub fn freeze(&self) -> MapFrozen {
        MapFrozen::new_from_editable(self)
    }

    /// Returns a reference to the item with the specified key if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapEntryEditable> {
        self.items.get(&key.into())
    }

    /// Returns a mutable reference to the item with the specified key if it exists.
    pub fn get_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut MapEntryEditable> {
        self.items.get_mut(key.as_ref())
    }

    /// Creates a new entry in the map with the specified key.
    pub fn create<S: Into<StoreKey>>(&mut self, key: S) {
        let key = key.into();
        let frozen = MapEntryFrozen::new(self.definition.item_type());
        let entry = frozen.thaw();
        self.items.insert(key, entry);
    }

    /// Returns an iterator over the key-item pairs in the map.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapEntryEditable)> {
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

impl PartialEq<&MapEditable> for MapEditable {
    fn eq(&self, other: &&MapEditable) -> bool {
        self == *other
    }
}

impl PartialEq<MapEditable> for &MapEditable {
    fn eq(&self, other: &MapEditable) -> bool {
        *self == other
    }
}

impl TreePrint for MapEditable {
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
