use crate::definition::ItemDefinitionType;
use crate::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, IntegerFrozen, MapFrozen, NumberFrozen,
    NumberWithUnitsFrozen, StringFrozen, TableFrozen,
};
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a parameter value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemFrozen {
    /// A boolean parameter.
    Boolean(BooleanFrozen),
    /// A choice parameter.
    Choice(ChoiceFrozen),
    /// A file parameter.
    File(FileFrozen),
    /// An integer parameter.
    Integer(IntegerFrozen),
    /// A map parameter.
    Map(MapFrozen),
    /// A number parameter.
    Number(NumberFrozen),
    /// A number parameter with units.
    NumberWithUnits(NumberWithUnitsFrozen),
    /// A string parameter.
    String(StringFrozen),
    /// A table parameter.
    Table(TableFrozen),
}

impl ItemFrozen {
    /// Returns the parameter definition.
    #[must_use]
    pub fn definition(&self) -> ItemDefinitionType {
        match self {
            ItemFrozen::Boolean(b) => ItemDefinitionType::Boolean(b.definition().clone()),
            ItemFrozen::Choice(c) => ItemDefinitionType::Choice(c.definition().clone()),
            ItemFrozen::File(f) => ItemDefinitionType::File(f.definition().clone()),
            ItemFrozen::Integer(i) => ItemDefinitionType::Integer(i.definition().clone()),
            ItemFrozen::Map(m) => ItemDefinitionType::Map(m.definition().clone()),
            ItemFrozen::Number(n) => ItemDefinitionType::Number(n.definition().clone()),
            ItemFrozen::NumberWithUnits(nwu) => {
                ItemDefinitionType::NumberWithUnits(nwu.definition().clone())
            }
            ItemFrozen::String(s) => ItemDefinitionType::String(s.definition().clone()),
            ItemFrozen::Table(t) => ItemDefinitionType::Table(t.definition().clone()),
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the parameter.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        match self {
            Self::Boolean(b) => b.hash(),
            Self::Choice(c) => c.hash(),
            Self::File(f) => f.hash(),
            Self::Integer(i) => i.hash(),
            Self::Map(m) => m.hash(),
            Self::Number(n) => n.hash(),
            Self::NumberWithUnits(nwu) => nwu.hash(),
            Self::String(s) => s.hash(),
            Self::Table(t) => t.hash(),
        }
    }

    /// Returns the choice value if this parameter is a choice parameter.
    #[must_use]
    pub fn get_choice(&self) -> Option<ChoiceFrozen> {
        match self {
            Self::Choice(c) => Some(c.clone()),
            _ => None,
        }
    }

    /// Returns the file value if this parameter is a file parameter.
    #[must_use]
    pub const fn get_file(&self) -> Option<&FileFrozen> {
        match self {
            Self::File(f) => Some(f),
            _ => None,
        }
    }

    /// Returns the map value if this parameter is a map parameter.
    #[must_use]
    pub const fn get_map(&self) -> Option<&MapFrozen> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the number value if this parameter is a number parameter.
    #[must_use]
    pub const fn get_number(&self) -> Option<&NumberFrozen> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the string value if this parameter is a string parameter.
    #[must_use]
    pub const fn get_string(&self) -> Option<&StringFrozen> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the table value if this parameter is a table parameter.
    #[must_use]
    pub const fn get_table(&self) -> Option<&TableFrozen> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }
}

impl TreePrint for ItemFrozen {
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
            Self::String(basic) => basic.tree_print(f, label, prefix, last),
            Self::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}
