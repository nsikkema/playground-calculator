use crate::input_basic_with_units::BasicInputWithUnitsData;
use crate::input_data::input_basic::BasicInputData;
use crate::input_data::input_table::TableInputData;
use crate::{BasicDefinition, TableWithUnitsInputData};
use datastore::frozen::{
    GlobalObjectFrozen, ItemFrozen, MapItemFrozen, ParameterObjectFrozen, VariableObjectFrozen,
};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a single item of input data within an object,
/// which can be either basic or table input data.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectItemInputData {
    /// Basic input data item.
    Basic(BasicInputData),
    /// Basic input data item with units.
    BasicWithUnits(BasicInputWithUnitsData),
    /// Table input data item.
    Table(TableInputData),
    /// Table input data item with units.
    TableWithUnits(TableWithUnitsInputData),
}

/// Converts a single `MapItemFrozen` (an entry within a `Map` item) into its
/// corresponding `ObjectItemInputData`.
fn map_item_to_input_data(map_item: &MapItemFrozen) -> ObjectItemInputData {
    match map_item {
        MapItemFrozen::Boolean(boolean) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::Boolean(boolean.definition().clone()),
            boolean.value(),
        )),
        MapItemFrozen::Choice(choice) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::Choice(choice.definition().clone()),
            choice.value(),
        )),
        MapItemFrozen::File(file) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::File(file.definition().clone()),
            file.value(),
        )),
        MapItemFrozen::Integer(integer) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::Integer(integer.definition().clone()),
            integer.value(),
        )),
        MapItemFrozen::Number(number) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::Number(number.definition().clone()),
            number.value(),
        )),
        MapItemFrozen::NumberWithUnits(number_with_units) => {
            ObjectItemInputData::BasicWithUnits(BasicInputWithUnitsData::new(
                BasicDefinition::NumberWithUnits(number_with_units.definition().clone()),
                number_with_units.value(),
                number_with_units.units(),
            ))
        }
        MapItemFrozen::String(string) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::String(string.definition().clone()),
            string.value(),
        )),
        MapItemFrozen::Table(table) => ObjectItemInputData::Table(TableInputData::new(
            table.definition().clone(),
            table.parameter().clone(),
            table.rows().to_vec(),
        )),
        MapItemFrozen::TableWithUnits(table_with_units) => {
            ObjectItemInputData::TableWithUnits(TableWithUnitsInputData::new(
                table_with_units.definition().clone(),
                table_with_units.parameter().clone(),
                table_with_units.units().to_vec(),
                table_with_units.rows().to_vec(),
            ))
        }
        MapItemFrozen::Unit(unit) => ObjectItemInputData::Basic(BasicInputData::new(
            BasicDefinition::Unit(unit.definition().clone()),
            unit.value(),
        )),
    }
}

/// Converts a single `ItemFrozen` into one or more input data entries
/// and inserts them into `map`, keyed by `key`.
///
/// Most item kinds map to a single entry, but `Map` items are flattened:
/// each field of each entry becomes its own item, addressed by a
/// `key[entry].field` path.
fn item_to_input_data(
    map: &mut BTreeMap<ShareableString, ObjectItemInputData>,
    key: ShareableString,
    data: &ItemFrozen,
) {
    match data {
        ItemFrozen::Boolean(boolean) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::Boolean(boolean.definition().clone()),
                    boolean.value(),
                )),
            );
        }
        ItemFrozen::Choice(choice) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::Choice(choice.definition().clone()),
                    choice.value(),
                )),
            );
        }
        ItemFrozen::File(file) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::File(file.definition().clone()),
                    file.value(),
                )),
            );
        }
        ItemFrozen::Integer(integer) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::Integer(integer.definition().clone()),
                    integer.value(),
                )),
            );
        }
        ItemFrozen::Map(item_map) => {
            // Maps are flattened: each field of each entry becomes its own
            // item, addressed by a `key[entry][field]` path.
            for (entry_key, entry) in item_map.iter() {
                for (item_key, map_item) in entry.iter() {
                    let path: ShareableString = format!("{key}[{entry_key}][{item_key}]").into();
                    map.insert(path, map_item_to_input_data(map_item));
                }
            }
        }
        ItemFrozen::Number(number) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::Number(number.definition().clone()),
                    number.value(),
                )),
            );
        }
        ItemFrozen::NumberWithUnits(number_with_units) => {
            map.insert(
                key,
                ObjectItemInputData::BasicWithUnits(BasicInputWithUnitsData::new(
                    BasicDefinition::NumberWithUnits(number_with_units.definition().clone()),
                    number_with_units.value(),
                    number_with_units.units(),
                )),
            );
        }
        ItemFrozen::String(string) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::String(string.definition().clone()),
                    string.value(),
                )),
            );
        }
        ItemFrozen::Table(table) => {
            map.insert(
                key,
                ObjectItemInputData::Table(TableInputData::new(
                    table.definition().clone(),
                    table.parameter().clone(),
                    table.rows().to_vec(),
                )),
            );
        }
        ItemFrozen::TableWithUnits(table_with_units) => {
            map.insert(
                key,
                ObjectItemInputData::TableWithUnits(TableWithUnitsInputData::new(
                    table_with_units.definition().clone(),
                    table_with_units.parameter().clone(),
                    table_with_units.units().to_vec(),
                    table_with_units.rows().to_vec(),
                )),
            );
        }
        ItemFrozen::Unit(unit) => {
            map.insert(
                key,
                ObjectItemInputData::Basic(BasicInputData::new(
                    BasicDefinition::Unit(unit.definition().clone()),
                    unit.value(),
                )),
            );
        }
    }
}

/// Represents input data for an object, mapping field names
/// to their corresponding input data items.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalObjectInputData {
    /// The map from a field name to its input data item.
    data: BTreeMap<ShareableString, ObjectItemInputData>,
}

impl GlobalObjectInputData {
    /// Creates a new `GlobalObjectInputData` instance from the given `GlobalObjectFrozen`.
    #[must_use]
    pub fn new(frozen_data: &GlobalObjectFrozen) -> Self {
        let mut data = BTreeMap::new();
        for (key, item) in frozen_data.iter() {
            item_to_input_data(&mut data, key.into(), item);
        }
        Self { data }
    }

    /// Returns a reference to the underlying `ObjectInputData`.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ObjectItemInputData> {
        &self.data
    }
}

/// Represents input data for an object, mapping field names
/// to their corresponding input data items.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterObjectInputData {
    /// The map from a field name to its input data item.
    data: BTreeMap<ShareableString, ObjectItemInputData>,
}

impl ParameterObjectInputData {
    /// Creates a new `ParameterObjectInputData` instance from the given `ParameterObjectFrozen`.
    #[must_use]
    pub fn new(frozen_data: &ParameterObjectFrozen) -> Self {
        let mut data = BTreeMap::new();
        for (key, item) in frozen_data.iter() {
            item_to_input_data(&mut data, key.into(), item);
        }
        Self { data }
    }

    /// Returns a reference to the underlying `ObjectInputData`.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ObjectItemInputData> {
        &self.data
    }
}

/// Represents input data for an object, mapping field names
/// to their corresponding input data items.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableObjectInputData {
    /// The map from a field name to its input data item.
    data: BTreeMap<ShareableString, ObjectItemInputData>,
}

impl VariableObjectInputData {
    /// Creates a new `VariableObjectInputData` instance from the given `VariableObjectFrozen`.
    #[must_use]
    pub fn new(frozen_data: &VariableObjectFrozen) -> Self {
        let mut data = BTreeMap::new();
        for (key, item) in frozen_data.iter() {
            item_to_input_data(&mut data, key.into(), item);
        }
        Self { data }
    }

    /// Returns a reference to the underlying `ObjectInputData`.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ObjectItemInputData> {
        &self.data
    }
}
