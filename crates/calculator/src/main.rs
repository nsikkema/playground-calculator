//! A simple calculator app built with eframe/egui.

// Hide console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

use datastore::prelude::*;
use expression_engine::ParameterObjectInputData;
use expression_engine::evaluation::engine::ExpressionEngine;

/// Evaluates the given `expression` string using `engine` and returns the result as a displayable string.
fn evaluate_expression(engine: &ExpressionEngine, expression: &str) -> String {
    let definition = ParameterObjectDefinition::builder("Calculator Input")
        .with(
            parameter_key!("p_expression"),
            StringDefinition::new_with_default("The expression to evaluate", expression),
        )
        .finish();
    let frozen = ParameterObjectFrozen::new(definition);
    let input_data = ParameterObjectInputData::new(&frozen);

    engine.evaluate_parameters(&input_data).map_or_else(
        |err| {
            let err_str = err.first().map_or_else(
                || "Unknown error".to_string(),
                |message| {
                    message
                        .translate_data()
                        .message_params()
                        .get("message")
                        .map_or_else(
                            || message.translate_data().message_key().to_string(),
                            ToString::to_string,
                        )
                },
            );

            format!("Error: {err_str}")
        },
        |output| {
            if let Some(result) = output.get("p_expression") {
                result.to_string()
            } else {
                "No result".to_string()
            }
        },
    )
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let engine = ExpressionEngine::new();

    // Our application state:
    let mut expression = String::new();
    let mut result = String::new();

    eframe::run_ui_native("Calculator", options, move |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Calculator");
            ui.horizontal(|ui| {
                let expression_label = ui.label("Expression: ");
                ui.text_edit_singleline(&mut expression)
                    .labelled_by(expression_label.id);
            });

            if ui.button("Evaluate").clicked() || ui.input(|i| i.key_released(egui::Key::Enter)) {
                result = evaluate_expression(&engine, &expression);
            }

            ui.label(egui::RichText::new(&result).monospace());
        });
    })
}
