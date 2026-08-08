use crate::definition::{
    BooleanDefinition, ChoiceDefinition, IntegerDefinition, MapDefinition, NumberDefinition,
    NumberWithUnitsDefinition, StringDefinition, TableDefinition,
};
use crate::prelude::FileDefinition;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::SharedStringStore;

/// The type of item definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemDefinitionType {
    /// A boolean item.
    Boolean(BooleanDefinition),
    /// A choice item.
    Choice(ChoiceDefinition),
    /// A file item.
    File(FileDefinition),
    /// An integer item.
    Integer(IntegerDefinition),
    /// A map item.
    Map(MapDefinition),
    /// A number item.
    Number(NumberDefinition),
    /// A number item with units.
    NumberWithUnits(NumberWithUnitsDefinition),
    /// A string item.
    String(StringDefinition),
    /// A table item.
    Table(TableDefinition),
}

impl From<StringDefinition> for ItemDefinitionType {
    fn from(definition: StringDefinition) -> Self {
        ItemDefinitionType::String(definition)
    }
}

impl From<BooleanDefinition> for ItemDefinitionType {
    fn from(definition: BooleanDefinition) -> Self {
        ItemDefinitionType::Boolean(definition)
    }
}

impl From<ChoiceDefinition> for ItemDefinitionType {
    fn from(definition: ChoiceDefinition) -> Self {
        ItemDefinitionType::Choice(definition)
    }
}

impl From<FileDefinition> for ItemDefinitionType {
    fn from(definition: FileDefinition) -> Self {
        ItemDefinitionType::File(definition)
    }
}

impl From<IntegerDefinition> for ItemDefinitionType {
    fn from(definition: IntegerDefinition) -> Self {
        ItemDefinitionType::Integer(definition)
    }
}

impl From<MapDefinition> for ItemDefinitionType {
    fn from(definition: MapDefinition) -> Self {
        ItemDefinitionType::Map(definition)
    }
}

impl From<NumberDefinition> for ItemDefinitionType {
    fn from(definition: NumberDefinition) -> Self {
        ItemDefinitionType::Number(definition)
    }
}

impl From<NumberWithUnitsDefinition> for ItemDefinitionType {
    fn from(definition: NumberWithUnitsDefinition) -> Self {
        ItemDefinitionType::NumberWithUnits(definition)
    }
}

impl From<TableDefinition> for ItemDefinitionType {
    fn from(definition: TableDefinition) -> Self {
        ItemDefinitionType::Table(definition)
    }
}

impl ItemDefinitionType {
    /// Returns a new `ItemDefinitionType` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Boolean(def) => Self::Boolean(def.launder(store)),
            Self::Choice(def) => Self::Choice(def.launder(store)),
            Self::File(def) => Self::File(def.launder(store)),
            Self::Integer(def) => Self::Integer(def.launder(store)),
            Self::Map(def) => Self::Map(def.launder(store)),
            Self::Number(def) => Self::Number(def.launder(store)),
            Self::NumberWithUnits(def) => Self::NumberWithUnits(def.launder(store)),
            Self::String(def) => Self::String(def.launder(store)),
            Self::Table(def) => Self::Table(def.launder(store)),
        }
    }
}

impl PartialEq<&ItemDefinitionType> for ItemDefinitionType {
    fn eq(&self, other: &&ItemDefinitionType) -> bool {
        self == *other
    }
}

impl PartialEq<ItemDefinitionType> for &ItemDefinitionType {
    fn eq(&self, other: &ItemDefinitionType) -> bool {
        *self == other
    }
}

impl TreePrint for ItemDefinitionType {
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
