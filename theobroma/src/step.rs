mod common;
mod if_condition;
mod parallel;
mod playbook;
mod single;
mod switch_condition;
mod target;
mod while_condition;

pub use common::CommonProperties;
pub use if_condition::StepIfCondition;
pub use parallel::StepParallel;
pub use playbook::StepPlaybook;
pub use single::StepSingle;
pub use switch_condition::StepSwitchCondition;
pub use target::Target;
pub use while_condition::StepWhileCondition;

/// TODO: Implement condition parsing
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Condition(String);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, strum::EnumDiscriminants)]
#[strum_discriminants(name(StepType))]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Step<T = crate::Target, C = crate::Command> {
    Start(CommonProperties),
    End(CommonProperties),
    Single(StepSingle<T, C>),
    Playbook(StepPlaybook<T>),
    Parallel(StepParallel),
    IfCondition(StepIfCondition),
    WhileCondition(StepWhileCondition),
    SwitchCondition(StepSwitchCondition),
}
