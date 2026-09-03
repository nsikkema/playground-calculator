/// The kind of component port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortKind {
    /// A port that accepts a signal input.
    SignalInput,
    /// A port that produces a signal output.
    SignalOutput,
}
