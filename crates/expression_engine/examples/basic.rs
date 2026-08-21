//! Evaluate a simple expression stored in parameter data.

#![allow(clippy::print_stdout)]

use datastore::prelude::*;
use expression_engine::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    let definition = ParameterObjectDefinition::builder("Example Parameters")
        .with(
            parameter_key!("p_answer"),
            IntegerDefinition::new_with_default("The expression to evaluate", "6 * 7"),
        )
        .finish();
    let frozen = ParameterObjectFrozen::new(definition);
    let input = ParameterObjectInputData::new(&frozen);

    match ExpressionEngine::new().evaluate_parameters(&input) {
        Ok(output) => {
            if let Some(value) = output.get("p_answer") {
                println!("6 * 7 = {value}");
                ExitCode::SUCCESS
            } else {
                eprintln!("The evaluated output did not contain `p_answer`.");
                ExitCode::FAILURE
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{error:?}");
            }
            ExitCode::FAILURE
        }
    }
}
