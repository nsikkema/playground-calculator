/// Static definition of the gain component.
pub mod definition;

use crate::{BuiltInComponent, BuiltInComponentDefinition, BuiltInComponentRegistration};
use expression_engine::prelude::{ArgumentCount, ComputedItem, FunctionDefinition};
use keys::store_key;
use message::message::{Message, MessageCategory};
use std::ops::Mul;

/// Multiplies a signal value by its gain.
fn gain(arguments: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let [ComputedItem::Float(value), ComputedItem::Float(factor)] = arguments else {
        return Err(Message::error(
            MessageCategory::Component,
            "built_in_registry_gain_requires_two_numbers",
        ));
    };

    Ok(ComputedItem::Float(value.mul(factor)))
}

/// Runtime implementation of gain version 1.
#[derive(Debug)]
pub struct GainV1;

impl BuiltInComponent for GainV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &definition::GAIN_V1
    }

    fn functions(&self) -> Vec<FunctionDefinition> {
        vec![FunctionDefinition::new(
            store_key!("gain"),
            "Multiplies a signal value by a gain",
            ArgumentCount::Exact { count: 2 },
            gain,
        )]
    }
}

/// Registration for gain version 1.
pub static REGISTRATION: BuiltInComponentRegistration =
    BuiltInComponentRegistration::new(&definition::GAIN_V1, || Box::new(GainV1));
