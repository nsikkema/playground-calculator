use crate::definitions::{IconDefinition, PortDefinition};
use datastore::compile_time::{ParameterObjectCompileTime, VariableObjectCompileTime};
use expression_engine::prelude::FunctionDefinition;
use keys::ConstComponentKey;

/// Re-exports the [`component_key!`] macro for use in [`built_in_component_definition!`].
#[allow(unused_imports)]
pub(crate) use keys::component_key as __component_key;

/// Static metadata shared by a built-in component version.
#[derive(Debug)]
pub struct BuiltInComponentDefinition {
    /// Stable identifier shared by all versions of the component.
    id: ConstComponentKey,
    /// Version of this component definition.
    version: u16,
    /// Human-readable description of this component version.
    display_name: &'static str,
    /// Compile-time parameter schema.
    parameters: ParameterObjectCompileTime,
    /// Compile-time variable schema.
    variables: VariableObjectCompileTime,
    /// Icon for this component.
    icon: IconDefinition,
    /// Ports of the component.
    ports: &'static [PortDefinition],
}

impl BuiltInComponentDefinition {
    /// Backing constructor for [`built_in_component_definition!`].
    #[must_use]
    pub(crate) const fn __new(
        id: ConstComponentKey,
        version: u16,
        display_name: &'static str,
        parameters: ParameterObjectCompileTime,
        variables: VariableObjectCompileTime,
        icon: IconDefinition,
        ports: &'static [PortDefinition],
    ) -> Self {
        Self {
            id,
            version,
            display_name,
            parameters,
            variables,
            icon,
            ports,
        }
    }

    /// Returns the component identifier.
    #[must_use]
    pub const fn id(&self) -> ConstComponentKey {
        self.id
    }

    /// Returns the component version.
    #[must_use]
    pub const fn version(&self) -> u16 {
        self.version
    }

    /// Returns the component display name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        self.display_name
    }

    /// Returns the component parameters.
    #[must_use]
    pub const fn parameters(&self) -> ParameterObjectCompileTime {
        self.parameters
    }

    /// Returns the component variables.
    #[must_use]
    pub const fn variables(&self) -> VariableObjectCompileTime {
        self.variables
    }

    /// Returns the component icon.
    #[must_use]
    pub const fn icon(&self) -> IconDefinition {
        self.icon
    }

    /// Returns the component ports.
    #[must_use]
    pub const fn ports(&self) -> &'static [PortDefinition] {
        self.ports
    }
}

/// Creates a [`BuiltInComponentDefinition`] from a component's static metadata.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression. The definition is validated at compile time even when the
/// result is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// built_in_component_definition!(
///     id,
///     version,
///     display_name,
///     parameters,
///     variables,
///     icon,
///     ports,
/// )
/// ```
///
/// # Arguments
/// - `id`: `&'static str` stable identifier shared by every version of the component.
/// - `version`: `u16` version of this definition.
/// - `display_name`: `&'static str` human-readable display name of this version.
/// - `parameters`: [`ParameterObjectCompileTime`] schema for user-configurable values.
/// - `variables`: [`VariableObjectCompileTime`] schema for values managed by the
///   component.
/// - `icon`: [`IconDefinition`] describing the component's image and dimensions.
/// - `ports`: an array expression or `&'static [PortDefinition]` containing the
///   component's ports. Port identifiers must be unique.
///
/// # Examples
/// ```text
/// const DEFINITION: BuiltInComponentDefinition = built_in_component_definition!(
///     "adder",
///     1,
///     "Adds two input values",
///     parameter_object_compile_time!("Parameters", &[]),
///     variable_object_compile_time!("Variables", []),
///     icon_definition!("<svg></svg>", (32, 32)),
///     [port_definition!(
///         "input",
///         "Input",
///         PortKind::SignalInput,
///         (0, 16),
///         (32, 32),
///         Rotation::Degrees0,
///     )],
/// );
///
/// assert_eq!(DEFINITION.id(), "adder");
/// assert_eq!(DEFINITION.display_name(), "Adds two input values");
/// assert_eq!(DEFINITION.ports().len(), 1);
/// ```
macro_rules! built_in_component_definition {
    (
        $id:expr,
        $version:expr,
        $display_name:expr,
        $parameters:expr,
        $variables:expr,
        $icon:expr,
        [$($port:expr),* $(,)?] $(,)?
    ) => {
        built_in_component_definition!(
            $id,
            $version,
            $display_name,
            $parameters,
            $variables,
            $icon,
            &[$($port),*],
        )
    };
    (
        $id:expr,
        $version:expr,
        $display_name:expr,
        $parameters:expr,
        $variables:expr,
        $icon:expr,
        $ports:expr $(,)?
    ) => {
        const {
            let ports: &'static [$crate::PortDefinition] = $ports;
            let mut unchecked = ports;
            while let [port, remaining @ ..] = unchecked {
                let mut candidates = remaining;
                while let [candidate, rest @ ..] = candidates {
                    let mut left = port.id().as_str().as_bytes();
                    let mut right = candidate.id().as_str().as_bytes();
                    let equal = loop {
                        match (left, right) {
                            ([], []) => break true,
                            ([left_byte, left_rest @ ..], [right_byte, right_rest @ ..]) => {
                                if *left_byte != *right_byte {
                                    break false;
                                }
                                left = left_rest;
                                right = right_rest;
                            }
                            _ => break false,
                        }
                    };
                    assert!(!equal, "BuiltInComponentDefinition port ids must be unique");
                    candidates = rest;
                }
                unchecked = remaining;
            }

            #[allow(clippy::disallowed_methods)]
            $crate::BuiltInComponentDefinition::__new(
                $crate::definitions::built_in_component_definition::__component_key!($id),
                $version,
                $display_name,
                $parameters,
                $variables,
                $icon,
                ports,
            )
        }
    };
}
pub(crate) use built_in_component_definition;

/// Runtime behavior supplied by a built-in component instance.
pub trait BuiltInComponent: std::fmt::Debug + Send + Sync {
    /// Returns the definition of the built-in component.
    fn definition(&self) -> &'static BuiltInComponentDefinition;

    /// Returns the functions available to this component's expressions.
    #[must_use]
    fn functions(&self) -> Vec<FunctionDefinition> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::icon_definition::icon_definition;
    use crate::definitions::port_definition::port_definition;
    use component_common::{PortKind, Rotation};

    #[derive(Debug)]
    struct ComponentV1;

    static PORTS: &[PortDefinition] = &[
        port_definition!(
            "input",
            "Input",
            PortKind::SignalInput,
            (0, 16),
            (32, 32),
            Rotation::Degrees0,
            true
        ),
        port_definition!(
            "output",
            "Output",
            PortKind::SignalOutput,
            (32, 16),
            (32, 32),
            Rotation::Degrees0,
            false
        ),
    ];

    static V1_DEFINITION: BuiltInComponentDefinition = built_in_component_definition!(
        "example",
        1,
        "Version one",
        datastore::parameter_object_compile_time!("Parameters", &[]),
        datastore::variable_object_compile_time!("Variables", []),
        icon_definition!("<svg></svg>", (32, 32)),
        PORTS,
    );

    impl BuiltInComponent for ComponentV1 {
        fn definition(&self) -> &'static BuiltInComponentDefinition {
            &V1_DEFINITION
        }
    }

    #[test]
    fn component_exposes_its_definition() {
        let component: &dyn BuiltInComponent = &ComponentV1;
        let definition = component.definition();

        assert_eq!(definition.id(), "example");
        assert_eq!(definition.version(), 1);
        assert_eq!(definition.display_name(), "Version one");
        assert_eq!(definition.parameters().count(), 0);
        assert_eq!(definition.variables().count(), 0);
        assert_eq!(definition.icon().svg_image(), "<svg></svg>");
        assert_eq!(definition.ports().len(), 2);
    }
}
