use crate::definition::ItemDefinitionType;
use crate::editable::{
    BooleanEditable, ChoiceEditable, FileEditable, IntegerEditable, MapEditable, NumberEditable,
    NumberWithUnitsEditable, StringEditable, TableEditable,
};
use crate::frozen::ItemFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a parameter value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemEditable {
    /// A boolean parameter.
    Boolean(BooleanEditable),
    /// A choice parameter.
    Choice(ChoiceEditable),
    /// A file parameter.
    File(FileEditable),
    /// An integer parameter.
    Integer(IntegerEditable),
    /// A map parameter.
    Map(MapEditable),
    /// A number parameter.
    Number(NumberEditable),
    /// A number parameter with units.
    NumberWithUnits(NumberWithUnitsEditable),
    /// A string parameter.
    String(StringEditable),
    /// A table parameter.
    Table(TableEditable),
}

impl ItemEditable {
    /// Creates a new `ItemEditable` instance from a given `ItemFrozen` value.
    #[must_use]
    pub fn new_from_frozen(static_item: &ItemFrozen) -> Self {
        match static_item {
            ItemFrozen::Boolean(boolean) => ItemEditable::Boolean(BooleanEditable::new(boolean)),
            ItemFrozen::Choice(choice) => ItemEditable::Choice(ChoiceEditable::new(choice)),
            ItemFrozen::File(file) => ItemEditable::File(FileEditable::new(file)),
            ItemFrozen::Integer(integer) => ItemEditable::Integer(IntegerEditable::new(integer)),
            ItemFrozen::Map(map) => ItemEditable::Map(MapEditable::new(map)),
            ItemFrozen::Number(number) => ItemEditable::Number(NumberEditable::new(number)),
            ItemFrozen::NumberWithUnits(number_with_units) => {
                ItemEditable::NumberWithUnits(NumberWithUnitsEditable::new(number_with_units))
            }
            ItemFrozen::String(string) => ItemEditable::String(StringEditable::new(string)),
            ItemFrozen::Table(table) => ItemEditable::Table(TableEditable::new(table)),
        }
    }

    /// Converts the current `ItemEditable` instance into an `ItemFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> ItemFrozen {
        match self {
            ItemEditable::Boolean(boolean) => ItemFrozen::Boolean(boolean.freeze()),
            ItemEditable::Choice(choice) => ItemFrozen::Choice(choice.freeze()),
            ItemEditable::File(file) => ItemFrozen::File(file.freeze()),
            ItemEditable::Integer(integer) => ItemFrozen::Integer(integer.freeze()),
            ItemEditable::Map(map) => ItemFrozen::Map(map.freeze()),
            ItemEditable::Number(number) => ItemFrozen::Number(number.freeze()),
            ItemEditable::NumberWithUnits(number_with_units) => {
                ItemFrozen::NumberWithUnits(number_with_units.freeze())
            }
            ItemEditable::String(string) => ItemFrozen::String(string.freeze()),
            ItemEditable::Table(table) => ItemFrozen::Table(table.freeze()),
        }
    }

    /// Returns the parameter definition.
    #[must_use]
    pub fn definition(&self) -> ItemDefinitionType {
        match self {
            ItemEditable::Boolean(b) => ItemDefinitionType::Boolean(b.definition().clone()),
            ItemEditable::Choice(c) => ItemDefinitionType::Choice(c.definition().clone()),
            ItemEditable::File(f) => ItemDefinitionType::File(f.definition().clone()),
            ItemEditable::Integer(i) => ItemDefinitionType::Integer(i.definition().clone()),
            ItemEditable::Map(m) => ItemDefinitionType::Map(m.definition().clone()),
            ItemEditable::Number(n) => ItemDefinitionType::Number(n.definition().clone()),
            ItemEditable::NumberWithUnits(nwu) => {
                ItemDefinitionType::NumberWithUnits(nwu.definition().clone())
            }
            ItemEditable::String(b) => ItemDefinitionType::String(b.definition().clone()),
            ItemEditable::Table(t) => ItemDefinitionType::Table(t.definition().clone()),
        }
    }

    /// Returns the boolean value if this parameter is a boolean parameter.
    #[must_use]
    pub const fn get_boolean(&self) -> Option<&BooleanEditable> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns a mutable reference to the boolean value if this parameter is a boolean parameter.
    #[must_use]
    pub const fn get_mut_boolean(&mut self) -> Option<&mut BooleanEditable> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the choice value if this parameter is a choice parameter.
    #[must_use]
    pub fn get_choice(&self) -> Option<ChoiceEditable> {
        match self {
            Self::Choice(c) => Some(c.clone()),
            _ => None,
        }
    }

    /// Returns a mutable reference to the choice value if this parameter is a choice parameter.
    #[must_use]
    pub const fn get_mut_choice(&mut self) -> Option<&mut ChoiceEditable> {
        match self {
            Self::Choice(c) => Some(c),
            _ => None,
        }
    }

    /// Returns the file value if this parameter is a file parameter.
    #[must_use]
    pub const fn get_file(&self) -> Option<&FileEditable> {
        match self {
            Self::File(f) => Some(f),
            _ => None,
        }
    }

    /// Returns a mutable reference to the file value if this parameter is a file parameter.
    #[must_use]
    pub const fn get_mut_file(&mut self) -> Option<&mut FileEditable> {
        match self {
            Self::File(f) => Some(f),
            _ => None,
        }
    }

    /// Returns the integer value if this parameter is an integer parameter.
    #[must_use]
    pub const fn get_integer(&self) -> Option<&IntegerEditable> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Returns a mutable reference to the integer value if this parameter is an integer parameter.
    #[must_use]
    pub const fn get_mut_integer(&mut self) -> Option<&mut IntegerEditable> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Returns the map value if this parameter is a map parameter.
    #[must_use]
    pub const fn get_map(&self) -> Option<&MapEditable> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Returns a mutable reference to the map value if this parameter is a map parameter.
    #[must_use]
    pub const fn get_mut_map(&mut self) -> Option<&mut MapEditable> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the number value if this parameter is a number parameter.
    #[must_use]
    pub const fn get_number(&self) -> Option<&NumberEditable> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns a mutable reference to the number value if this parameter is a number parameter.
    #[must_use]
    pub const fn get_mut_number(&mut self) -> Option<&mut NumberEditable> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the string value if this parameter is a string parameter.
    #[must_use]
    pub const fn get_string(&self) -> Option<&StringEditable> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a mutable reference to the string value if this parameter is a string parameter.
    #[must_use]
    pub const fn get_mut_string(&mut self) -> Option<&mut StringEditable> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the table value if this parameter is a table parameter.
    #[must_use]
    pub const fn get_table(&self) -> Option<&TableEditable> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns a mutable reference to the table value if this parameter is a table parameter.
    #[must_use]
    pub const fn get_mut_table(&mut self) -> Option<&mut TableEditable> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }
}

impl TreePrint for ItemEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            Self::Choice(choice) => choice.tree_print(f, label, prefix, last),
            Self::File(file) => file.tree_print(f, label, prefix, last),
            Self::Integer(integer) => integer.tree_print(f, label, prefix, last),
            Self::Map(map) => map.tree_print(f, label, prefix, last),
            Self::Number(number) => number.tree_print(f, label, prefix, last),
            Self::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            Self::String(string) => string.tree_print(f, label, prefix, last),
            Self::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}
