use crate::{ComputedItem, Message};
use keys::store_key::StoreKey;
use shareable_string::ShareableString;
use std::collections::BTreeMap;
use std::sync::Arc;

/// The callable body of a [`FunctionDefinition`], stored behind an `Arc` so that
/// definitions can be cheaply cloned (the owning `ExpressionEngine` is itself `Clone`).
type FunctionBody = Arc<dyn Fn(&[ComputedItem]) -> Result<ComputedItem, Message> + Send + Sync>;

/// Represents the number of arguments a function expects.
#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentCount {
    /// A function that accepts a minimum number of arguments, but no maximum.
    Min {
        /// Minimum number of arguments required.
        min: usize,
    },
    /// A function that accepts a maximum number of arguments, but no minimum.
    Max {
        /// Maximum number of arguments allowed.
        max: usize,
    },
    /// A function that accepts a range of arguments, inclusive.
    Range {
        /// Minimum number of arguments required.
        min: usize,
        /// Maximum number of arguments allowed.
        max: usize,
    },
    /// A function that accepts an exact number of arguments.
    Exact {
        /// Exact number of arguments required.
        count: usize,
    },
    /// A function that accepts any number of arguments.
    Unbounded,
}

/// A function that can be invoked from within an expression via a call such as
/// `add(2, 3)`.
pub struct FunctionDefinition {
    /// The name used to invoke this function in expression syntax.
    name: ShareableString,
    /// A human-readable description of what the function does.
    description: ShareableString,
    /// The number of arguments this function accepts.
    argument_count: ArgumentCount,
    /// The callable body, stored behind an `Arc` for cloning.
    function: FunctionBody,
}

impl FunctionDefinition {
    /// Creates a new `FunctionDefinition` wrapping the provided callable.
    #[hotpath::measure]
    pub fn new<F>(
        name: impl Into<StoreKey>,
        description: impl Into<ShareableString>,
        argument_count: ArgumentCount,
        function: F,
    ) -> Self
    where
        F: Fn(&[ComputedItem]) -> Result<ComputedItem, Message> + Send + Sync + 'static,
    {
        Self {
            name: name.into().into(),
            description: description.into(),
            argument_count,
            function: Arc::new(function),
        }
    }

    /// Returns the name of the function, as used in expressions.
    #[must_use]
    pub const fn name(&self) -> &ShareableString {
        &self.name
    }

    /// Returns a human-readable description of the function.
    #[must_use]
    pub const fn description(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the parameter constraints for the function.
    #[must_use]
    pub const fn parameter_constraints(&self) -> &ArgumentCount {
        &self.argument_count
    }

    /// Invokes the function with the provided pre-evaluated arguments.
    #[hotpath::measure]
    pub(crate) fn call(&self, arguments: &[ComputedItem]) -> Result<ComputedItem, Message> {
        (self.function)(arguments)
    }
}

impl std::fmt::Debug for FunctionDefinition {
    #[hotpath::measure]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDefinition")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("argument_count", &self.argument_count)
            .field("function", &"<function>")
            .finish()
    }
}

impl Clone for FunctionDefinition {
    #[hotpath::measure]
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            argument_count: self.argument_count.clone(),
            function: Arc::clone(&self.function),
        }
    }
}

impl PartialEq for FunctionDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &Self) -> bool {
        // The closure itself is opaque, so identity is established by the
        // function's name and description.
        self.name == other.name && self.description == other.description
    }
}

/// A registry of named functions available during expression evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinitions {
    /// The map from a function name to its definition.
    definitions: BTreeMap<ShareableString, FunctionDefinition>,
}

impl FunctionDefinitions {
    /// Creates a new, empty registry.
    pub(crate) const fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
        }
    }

    /// Registers a function definition, replacing any existing definition with
    /// the same name.
    #[hotpath::measure]
    pub(crate) fn insert(&mut self, definition: FunctionDefinition) {
        self.definitions.insert(definition.name.clone(), definition);
    }

    /// Looks up the function registered under `name`.
    ///
    /// `ShareableString` implements `Borrow<str>`, so a `&str` lookup avoids
    /// constructing a temporary `ShareableString` for the key.
    #[must_use]
    #[hotpath::measure]
    pub fn get(&self, name: &str) -> Option<&FunctionDefinition> {
        self.definitions.get(name)
    }

    /// Returns a new `FunctionDefinitions` with the provided function definition added.
    #[hotpath::measure]
    pub(crate) fn with(mut self, definition: FunctionDefinition) -> Self {
        self.insert(definition);
        self
    }

    /// Returns an iterator over the names of all registered functions.
    #[hotpath::measure]
    pub(crate) fn keys(&self) -> impl Iterator<Item = &ShareableString> {
        self.definitions.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datastore::prelude::*;
    use std::ops::AddAssign;

    fn add(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
        let mut sum = 0.0;
        for arg in args {
            match arg {
                ComputedItem::Float(v) => sum.add_assign(v),
                ComputedItem::Integer(v) => sum.add_assign(*v as f64),
                _ => {
                    return Err(crate::expression_message!(
                        crate::ExpressionCategory::Evaluation,
                        "expression_engine_evaluation_add_requires_numeric_arguments",
                        [],
                    ));
                }
            }
        }
        Ok(ComputedItem::Float(sum))
    }

    #[test]
    fn register_and_lookup_function() {
        let mut definitions = FunctionDefinitions::new();
        assert!(definitions.get("add").is_none());

        definitions.insert(FunctionDefinition::new(
            store_key!("add"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            add,
        ));

        let found = definitions.get("add").expect("add should be registered");
        assert_eq!(found.name(), "add");
        assert_eq!(found.description(), "sums its arguments");
    }

    #[test]
    fn lookup_missing_function_returns_none() {
        let definitions = FunctionDefinitions::new();
        assert!(definitions.get("missing").is_none());
    }

    #[test]
    fn calling_a_function_invokes_its_body() {
        let mut definitions = FunctionDefinitions::new();
        definitions.insert(FunctionDefinition::new(
            store_key!("add"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            add,
        ));

        let definition = definitions.get("add").unwrap();
        let result = definition
            .call(&[ComputedItem::Float(1.5), ComputedItem::Integer(2)])
            .unwrap();
        match result {
            ComputedItem::Float(v) => assert_eq!(v, 3.5),
            other => panic!("expected float, got {other:?}"),
        }
    }

    #[test]
    fn inserting_replaces_existing_definition() {
        let mut definitions = FunctionDefinitions::new();
        definitions.insert(FunctionDefinition::new(
            store_key!("add"),
            "original",
            ArgumentCount::Unbounded,
            |_| Ok(ComputedItem::Integer(1)),
        ));
        definitions.insert(FunctionDefinition::new(
            store_key!("add"),
            "replacement",
            ArgumentCount::Unbounded,
            |_| Ok(ComputedItem::Integer(2)),
        ));

        let definition = definitions.get("add").unwrap();
        assert_eq!(definition.description(), "replacement");
        match definition.call(&[]) {
            Ok(ComputedItem::Integer(v)) => assert_eq!(v, 2),
            other => panic!("unexpected result {other:?}"),
        }
    }

    #[test]
    fn clone_preserves_identity_and_callability() {
        let mut definitions = FunctionDefinitions::new();
        definitions.insert(FunctionDefinition::new(
            store_key!("add"),
            "sums its arguments",
            ArgumentCount::Unbounded,
            add,
        ));

        let cloned = definitions.clone();
        assert_eq!(definitions, cloned);

        let original_def = definitions.get("add").unwrap();
        let cloned_def = cloned.get("add").unwrap();
        assert_eq!(original_def, cloned_def);

        let original_result = original_def.call(&[ComputedItem::Float(2.0)]).unwrap();
        let cloned_result = cloned_def.call(&[ComputedItem::Float(2.0)]).unwrap();
        assert_eq!(original_result, cloned_result);
    }
}
