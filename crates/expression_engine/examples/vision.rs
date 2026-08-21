//! Size a gantry crane's major components with a FIFO evaluation queue.

#![allow(clippy::print_stdout)]
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "This executable example keeps its private scaffolding self-explanatory."
)]

use datastore::prelude::*;
use expression_engine::prelude::*;
use std::collections::VecDeque;
use std::process::ExitCode;

const MODEL_NAME: &str = "Gantry Crane";

#[derive(Debug)]
struct Component {
    name: ShareableString,
    parameters: ParameterObjectInputData,
    variables: VariableObjectInputData,
    children: Vec<Self>,
}

impl Component {
    #[hotpath::measure]
    fn new(
        name: impl Into<ShareableString>,
        parameters: ParameterObjectInputData,
        variables: VariableObjectInputData,
    ) -> Self {
        Self {
            name: name.into(),
            parameters,
            variables,
            children: Vec::new(),
        }
    }

    #[hotpath::measure]
    fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    #[hotpath::measure]
    fn evaluate(
        &self,
        engine: &ExpressionEngine,
        parent_parameters: &ParameterObjectComputedData,
        parent_variables: &VariableObjectComputedData,
    ) -> Result<(ParameterObjectComputedData, VariableObjectComputedData), Vec<Message>> {
        let parameters = engine.evaluate_child_parameters(
            parent_parameters,
            parent_variables,
            &self.parameters,
        )?;
        let variables = engine.evaluate_variables(&parameters, &self.variables)?;

        Ok((parameters, variables))
    }
}

#[derive(Debug)]
struct QueuedComponent<'a> {
    component: &'a Component,
    path: Vec<ShareableString>,
    parent_parameters: ParameterObjectComputedData,
    parent_variables: VariableObjectComputedData,
}

#[derive(Debug, Default)]
struct EvaluationQueue<'a> {
    pending: VecDeque<QueuedComponent<'a>>,
}

impl<'a> EvaluationQueue<'a> {
    #[hotpath::measure]
    fn enqueue_children(
        &mut self,
        children: &'a [Component],
        parent_path: &[ShareableString],
        parent_parameters: &ParameterObjectComputedData,
        parent_variables: &VariableObjectComputedData,
    ) {
        for child in children {
            let mut path = parent_path.to_vec();
            path.push(child.name.clone());
            self.pending.push_back(QueuedComponent {
                component: child,
                path,
                parent_parameters: parent_parameters.clone(),
                parent_variables: parent_variables.clone(),
            });
        }
    }

    #[hotpath::measure]
    fn dequeue(&mut self) -> Option<QueuedComponent<'a>> {
        self.pending.pop_front()
    }
}

#[derive(Debug)]
struct EvaluatedComponent {
    name: ShareableString,
    variables: VariableObjectComputedData,
}

#[derive(Debug)]
struct EvaluationFailure {
    path: Vec<ShareableString>,
    errors: Vec<Message>,
}

#[derive(Debug)]
struct Model {
    parameters: ParameterObjectInputData,
    variables: VariableObjectInputData,
    settings: GlobalObjectInputData,
    children: Vec<Component>,
}

impl Model {
    const fn new(
        parameters: ParameterObjectInputData,
        variables: VariableObjectInputData,
        settings: GlobalObjectInputData,
    ) -> Self {
        Self {
            parameters,
            variables,
            settings,
            children: Vec::new(),
        }
    }

    #[hotpath::measure]
    fn add_child(&mut self, child: Component) {
        self.children.push(child);
    }

    #[hotpath::measure]
    fn evaluate(
        &self,
        engine: &mut ExpressionEngine,
    ) -> Result<Vec<EvaluatedComponent>, EvaluationFailure> {
        let model_path = vec![MODEL_NAME.into()];
        let parameters = engine
            .evaluate_parameters(&self.parameters)
            .map_err(|errors| EvaluationFailure {
                path: model_path.clone(),
                errors,
            })?;
        let variables = engine
            .evaluate_variables(&parameters, &self.variables)
            .map_err(|errors| EvaluationFailure {
                path: model_path.clone(),
                errors,
            })?;
        engine
            .extend_globals(&parameters, &variables, &self.settings)
            .map_err(|errors| EvaluationFailure {
                path: model_path.clone(),
                errors,
            })?;
        let mut evaluated = vec![EvaluatedComponent {
            name: MODEL_NAME.into(),
            variables: variables.clone(),
        }];

        let mut queue = EvaluationQueue::default();
        queue.enqueue_children(&self.children, &model_path, &parameters, &variables);

        while let Some(queued) = queue.dequeue() {
            let (parameters, variables) = queued
                .component
                .evaluate(engine, &queued.parent_parameters, &queued.parent_variables)
                .map_err(|errors| EvaluationFailure {
                    path: queued.path.clone(),
                    errors,
                })?;
            queue.enqueue_children(
                &queued.component.children,
                &queued.path,
                &parameters,
                &variables,
            );
            evaluated.push(EvaluatedComponent {
                name: queued.component.name.clone(),
                variables,
            });
        }

        Ok(evaluated)
    }
}

#[hotpath::measure]
fn input_parameters(key: ConstParameterKey, expression: &str) -> ParameterObjectInputData {
    let definition = ParameterObjectDefinition::builder("Component parameters")
        .with(
            key,
            NumberDefinition::new_with_default("Component input", expression),
        )
        .finish();

    ParameterObjectInputData::new(&ParameterObjectFrozen::new(definition))
}

#[hotpath::measure]
fn input_variables(key: ConstVariableKey, expression: &str) -> VariableObjectInputData {
    let definition = VariableObjectDefinition::builder("Component variables")
        .with(
            key,
            NumberDefinition::new_with_default("Calculated result", expression),
        )
        .finish();

    VariableObjectInputData::new(&VariableObjectFrozen::new(definition))
}

#[hotpath::measure]
fn create_extended_globals() -> GlobalObjectInputData {
    GlobalObjectInputData::new(&GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Gantry crane design assumptions")
            .with(
                global_key!("g_allowable_utilization"),
                NumberDefinition::new_with_default("Allowable yield utilization", "0.90"),
            )
            .with(
                global_key!("g_dynamic_factor"),
                NumberDefinition::new_with_default("Dynamic load factor", "1.15"),
            )
            .with(
                global_key!("g_girder_depth_m"),
                NumberDefinition::new_with_default("Girder section depth in meters", "1.80"),
            )
            .with(
                global_key!("g_girder_span_m"),
                NumberDefinition::new_with_default("Girder span in meters", "18.0"),
            )
            .with(
                global_key!("g_gravity_mps2"),
                NumberDefinition::new_with_default("Standard gravity", "9.80665"),
            )
            .with(
                global_key!("g_rope_allowable_stress_mpa"),
                NumberDefinition::new_with_default("Allowable rope stress in MPa", "1100.0"),
            )
            .with(
                global_key!("g_steel_yield_mpa"),
                NumberDefinition::new_with_default("Steel yield strength in MPa", "355.0"),
            )
            .with(
                global_key!("g_wheel_rating_n"),
                NumberDefinition::new_with_default("Wheel load rating in newtons", "80000.0"),
            )
            .finish(),
    ))
}

#[hotpath::measure]
fn create_settings() -> GlobalObjectInputData {
    GlobalObjectInputData::new(&GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Gantry crane calculated settings")
            .with(
                global_key!("g_design_load_n"),
                NumberDefinition::new_with_default(
                    "Factored payload load in newtons",
                    "p_payload_kg * g_gravity_mps2 * v_impact_factor * g_dynamic_factor",
                ),
            )
            .finish(),
    ))
}

#[hotpath::measure]
fn create_girder_component() -> Component {
    let mut girder = Component::new(
        "Main Girder",
        input_parameters(parameter_key!("p_girder_load_n"), "g_design_load_n"),
        input_variables(
            variable_key!("v_girder_moment_knm"),
            "p_girder_load_n * g_girder_span_m / 4000.0",
        ),
    );
    girder.add_child(Component::new(
        "Girder Flange",
        input_parameters(
            parameter_key!("p_flange_force_n"),
            "v_girder_moment_knm * 1000.0 / g_girder_depth_m",
        ),
        input_variables(
            variable_key!("v_required_flange_area_mm2"),
            "ceil(p_flange_force_n / (g_steel_yield_mpa * g_allowable_utilization))",
        ),
    ));
    girder
}

#[hotpath::measure]
fn create_model() -> Model {
    let settings = create_settings();
    let mut model = Model::new(
        input_parameters(parameter_key!("p_payload_kg"), "12000.0"),
        input_variables(
            variable_key!("v_impact_factor"),
            "max(1.10, 1.0 + p_payload_kg / 100000.0)",
        ),
        settings,
    );

    let girder = create_girder_component();

    model.add_child(girder);
    model.add_child(Component::new(
        "Hoist Rope",
        input_parameters(parameter_key!("p_rope_load_n"), "g_design_load_n"),
        input_variables(
            variable_key!("v_required_rope_diameter_mm"),
            "sqrt(4.0 * p_rope_load_n / (g_pi * g_rope_allowable_stress_mpa))",
        ),
    ));
    model.add_child(Component::new(
        "Wheel Selection",
        input_parameters(parameter_key!("p_total_wheel_load_n"), "g_design_load_n"),
        input_variables(
            variable_key!("v_wheels_required"),
            "ceil(p_total_wheel_load_n / g_wheel_rating_n)",
        ),
    ));
    model
}

#[hotpath::main]
fn main() -> ExitCode {
    let extended_globals = create_extended_globals();

    let model = create_model();

    let mut engine = ExpressionEngine::new();
    if let Err(errors) = engine.evaluate_globals(&extended_globals) {
        for error in errors {
            eprintln!("{MODEL_NAME}: {error:?}");
        }
        return ExitCode::FAILURE;
    }

    match model.evaluate(&mut engine) {
        Ok(evaluated) => {
            for component in evaluated {
                let values = component
                    .variables
                    .iter()
                    .map(|(key, value)| format!("{key} = {value}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{}: {values}", component.name);
            }
            ExitCode::SUCCESS
        }
        Err(failure) => {
            let path = failure
                .path
                .iter()
                .map(ShareableString::as_str)
                .collect::<Vec<_>>()
                .join(" > ");
            for error in failure.errors {
                eprintln!("{path}: {error:?}");
            }
            ExitCode::FAILURE
        }
    }
}
