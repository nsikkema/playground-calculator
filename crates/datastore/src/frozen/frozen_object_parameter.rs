use crate::definition::{ItemDefinitionType, ParameterObjectDefinition};
use crate::editable::ParameterObjectEditable;
use crate::frozen::{FileFrozen, ItemFrozen, MapFrozen, StringFrozen, TableFrozen};
use crate::traits::TreePrint;
use keys::parameter_key::ParameterKey;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterObjectFrozen {
    /// The definition of the object.
    definition: ParameterObjectDefinition,
    /// The items of the object.
    items: BTreeMap<ParameterKey, ItemFrozen>,
    /// The pre-calculated BLAKE3 hash of the object's content.
    hash: [u8; 32],
}

impl ParameterObjectFrozen {
    /// Creates a new `ParameterObjectFrozen` with a definition.
    #[must_use]
    pub fn new(definition: ParameterObjectDefinition) -> Self {
        let mut items = BTreeMap::new();
        for (item_key, item_definition_type) in definition.iter() {
            let key = item_key.clone();
            match item_definition_type {
                ItemDefinitionType::Boolean(boolean_def) => {
                    items.insert(
                        key,
                        ItemFrozen::Boolean(crate::frozen::BooleanFrozen::new(boolean_def.clone())),
                    );
                }
                ItemDefinitionType::Choice(choice_def) => {
                    items.insert(
                        key,
                        ItemFrozen::Choice(crate::frozen::ChoiceFrozen::new(choice_def.clone())),
                    );
                }
                ItemDefinitionType::File(file_def) => {
                    items.insert(key, ItemFrozen::File(FileFrozen::new(file_def.clone())));
                }
                ItemDefinitionType::Integer(integer_def) => {
                    items.insert(
                        key,
                        ItemFrozen::Integer(crate::frozen::IntegerFrozen::new(integer_def.clone())),
                    );
                }
                ItemDefinitionType::Map(map_def) => {
                    items.insert(key, ItemFrozen::Map(MapFrozen::new(map_def.clone())));
                }
                ItemDefinitionType::Number(number_def) => {
                    items.insert(
                        key,
                        ItemFrozen::Number(crate::frozen::NumberFrozen::new(number_def.clone())),
                    );
                }
                ItemDefinitionType::NumberWithUnits(number_with_units_def) => {
                    items.insert(
                        key,
                        ItemFrozen::NumberWithUnits(crate::frozen::NumberWithUnitsFrozen::new(
                            number_with_units_def.clone(),
                        )),
                    );
                }
                ItemDefinitionType::String(basic_def) => {
                    items.insert(
                        key,
                        ItemFrozen::String(StringFrozen::new(basic_def.clone())),
                    );
                }
                ItemDefinitionType::Table(table_def) => {
                    items.insert(key, ItemFrozen::Table(TableFrozen::new(table_def.clone())));
                }
            }
        }

        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `ParameterObjectFrozen` with a description and items.
    pub fn new_from_items<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<ParameterKey, ItemFrozen>,
    ) -> Self {
        let mut builder = ParameterObjectDefinition::builder(description);
        for (k, v) in &items {
            builder.insert(k.clone(), v.definition());
        }
        let definition = builder.finish();
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `ParameterObjectFrozen` from a given `ParameterObjectEditable` value.
    #[must_use]
    pub fn new_from_editable(editable_object: &ParameterObjectEditable) -> Self {
        let definition = editable_object.definition().clone();
        let items = editable_object
            .iter()
            .map(|(key, value)| (key.clone(), value.freeze()))
            .collect();
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `ParameterObjectFrozen` instance into a `ParameterObjectEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> ParameterObjectEditable {
        ParameterObjectEditable::new_from_frozen(self)
    }

    /// Recomputes and stores the BLAKE3 hash of all items in this parameter object.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"ParameterObject");

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

    /// Returns the pre-calculated BLAKE3 hash of the object.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the parameter with the specified key if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemFrozen> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-parameter pairs in the object.
    pub fn iter(&self) -> impl Iterator<Item = (&ParameterKey, &ItemFrozen)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    #[must_use]
    pub const fn definition(&self) -> &ParameterObjectDefinition {
        &self.definition
    }
}

impl PartialEq<&ParameterObjectFrozen> for ParameterObjectFrozen {
    fn eq(&self, other: &&ParameterObjectFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<ParameterObjectFrozen> for &ParameterObjectFrozen {
    fn eq(&self, other: &ParameterObjectFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for ParameterObjectFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "Parameter Object Frozen ({})",
            self.definition.description()
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

impl std::fmt::Display for ParameterObjectFrozen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
