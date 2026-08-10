use crate::editable::{ItemEditable, MapEntryEditable, MapItemEditable};
use crate::traits::ObjectEditable;
use errors::StoreError;
use shareable_string::ShareableString;

/// Helper function to set the value of an editable object by key.
///
/// # Errors
///
/// Returns a `StoreError` if the key does not exist or if the value type does not match the expected type.
pub fn editable_set_value<
    T: ObjectEditable,
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
>(
    obj: &mut T,
    key: S1,
    value: S2,
) -> Result<(), StoreError> {
    let key = key.into();
    let value = value.into();

    let item = obj.get_mut(&key).ok_or(StoreError::KeyNotFound)?;

    match item {
        ItemEditable::Boolean(boolean) => {
            boolean.set(value);
        }
        ItemEditable::Choice(choice) => {
            choice.set(value);
        }
        ItemEditable::File(file) => {
            file.set(value);
        }
        ItemEditable::Integer(integer) => {
            integer.set(value);
        }
        ItemEditable::Map(_) => {
            return Err(StoreError::InvalidType("Cannot set a value for a Map item directly. Use the appropriate methods to modify the map.".to_string()));
        }
        ItemEditable::Number(number) => {
            number.set(value);
        }
        ItemEditable::NumberWithUnits(number_with_units) => {
            number_with_units.set(value);
        }
        ItemEditable::String(string) => {
            string.set(value);
        }
        ItemEditable::Table(table) => {
            table.set_parameter(value);
        }
        ItemEditable::TableWithUnits(table_with_units) => {
            table_with_units.set_parameter(value);
        }
        ItemEditable::Unit(unit) => {
            unit.set(value);
        }
    }

    Ok(())
}

/// Helper function to set the value of a map item in an editable map by key and item key.
///
/// # Errors
///
/// Returns a `StoreError` if the key or item key does not exist.
pub fn editable_set_map_value<S1: Into<ShareableString>, S2: Into<ShareableString>>(
    entry: &mut MapEntryEditable,
    key: S1,
    value: S2,
) -> Result<(), StoreError> {
    let key = key.into();

    let item = entry.get_mut(&key).ok_or(StoreError::KeyNotFound)?;

    match item {
        MapItemEditable::Boolean(boolean) => {
            boolean.set(value);
        }
        MapItemEditable::Choice(choice) => {
            choice.set(value);
        }
        MapItemEditable::File(file) => {
            file.set(value);
        }
        MapItemEditable::Integer(integer) => {
            integer.set(value);
        }
        MapItemEditable::Number(number) => {
            number.set(value);
        }
        MapItemEditable::NumberWithUnits(number_with_units) => {
            number_with_units.set(value);
        }
        MapItemEditable::String(string) => {
            string.set(value);
        }
        MapItemEditable::Table(table) => {
            table.set_parameter(value);
        }
        MapItemEditable::TableWithUnits(table_with_units) => {
            table_with_units.set_parameter(value);
        }
        MapItemEditable::Unit(unit) => {
            unit.set(value);
        }
    }

    Ok(())
}
