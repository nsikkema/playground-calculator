use crate::{BuiltInComponent, BuiltInComponentDefinition};
use expression_engine::prelude::ExpressionEngine;
use message::message::{Message, MessageCategory};

/// Constructs a runtime instance of a built-in component.
pub type BuiltInComponentFactory = fn() -> Box<dyn BuiltInComponent>;

/// Connects a component's static definition to its runtime factory.
#[derive(Debug, Clone, Copy)]
pub struct BuiltInComponentRegistration {
    /// Static metadata for the registered component.
    definition: &'static BuiltInComponentDefinition,
    /// Factory for creating runtime component behavior.
    factory: BuiltInComponentFactory,
}

impl BuiltInComponentRegistration {
    /// Creates a component registration.
    #[must_use]
    pub const fn new(
        definition: &'static BuiltInComponentDefinition,
        factory: BuiltInComponentFactory,
    ) -> Self {
        Self {
            definition,
            factory,
        }
    }

    /// Returns the component's static definition.
    #[must_use]
    pub const fn definition(&self) -> &'static BuiltInComponentDefinition {
        self.definition
    }

    /// Instantiates the component with an isolated expression engine.
    ///
    /// # Errors
    ///
    /// Returns an error when the factory produces a different component version, a component
    /// function has an invalid name, or a component function conflicts with a default function.
    pub fn instantiate(&self) -> Result<BuiltInComponentInstance, Message> {
        let component = (self.factory)();
        let actual_definition = component.definition();
        if actual_definition.id() != self.definition.id()
            || actual_definition.version() != self.definition.version()
        {
            return Err(Message::error(
                MessageCategory::Component,
                "built_in_registry_factory_definition_mismatch",
            ));
        }

        let mut engine = ExpressionEngine::new();
        for function in component.functions() {
            let name = function.name().clone();
            if engine.functions().get(name.as_str()).is_some() {
                return Err(Message::error_with_param(
                    MessageCategory::Component,
                    "built_in_registry_function_name_conflict",
                    "name",
                    name,
                ));
            }
            engine.register_function(function)?;
        }

        Ok(BuiltInComponentInstance { component, engine })
    }
}

/// A runtime component and its isolated expression engine.
#[derive(Debug)]
pub struct BuiltInComponentInstance {
    /// Component-specific runtime behavior.
    component: Box<dyn BuiltInComponent>,
    /// Engine containing the default and component-specific functions.
    engine: ExpressionEngine,
}

impl BuiltInComponentInstance {
    /// Returns the component behavior.
    #[must_use]
    pub fn component(&self) -> &dyn BuiltInComponent {
        self.component.as_ref()
    }

    /// Returns the component's static definition.
    #[must_use]
    pub fn definition(&self) -> &'static BuiltInComponentDefinition {
        self.component.definition()
    }

    /// Returns the component's expression engine.
    #[must_use]
    pub const fn engine(&self) -> &ExpressionEngine {
        &self.engine
    }

    /// Returns the component's mutable expression engine.
    #[must_use]
    pub const fn engine_mut(&mut self) -> &mut ExpressionEngine {
        &mut self.engine
    }
}
