use crate::BuiltInComponentDefinition;
use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use component_common::{PortKind, Rotation};
use datastore::{item_compile_time, number_compile_time};

/// Version 1 definition of the gain component.
pub static GAIN_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "gain",
    1,
    "Gain",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [(
            "p_gain",
            item_compile_time!(number = number_compile_time!("Gain", default = "1.0")),
        ),]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("gain.svg"), (32, 32)),
    [
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
    ],
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_definition() {
        let definition = &GAIN_V1;
        assert_eq!(definition.id(), "gain");
        assert_eq!(definition.version(), 1);
        assert_eq!(definition.display_name(), "Gain");
        let icon = definition.icon();
        assert_eq!(icon.height(), 32);
        assert_eq!(icon.width(), 32);
        let ports = definition.ports();
        assert_eq!(ports.len(), 2);
        let [input_port, output_port] = ports else {
            panic!("gain should have exactly two ports");
        };
        assert_eq!(input_port.id(), "input");
        assert_eq!(input_port.display_name(), "Input");
        assert_eq!(input_port.kind(), PortKind::SignalInput);
        assert_eq!(output_port.id(), "output");
        assert_eq!(output_port.display_name(), "Output");
        assert_eq!(output_port.kind(), PortKind::SignalOutput);
    }
}
