use std::fmt;

use crate::step_graph::ToStepRels;

mod common;
mod if_condition;
mod parallel;
mod playbook;
mod single;
mod switch_condition;
mod target;
mod while_condition;

pub use common::{CommonProperties, OnCompletion};
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
pub enum Step<T = crate::StandardTarget, C = crate::Command> {
    Start(CommonProperties),
    End(CommonProperties),
    Single(StepSingle<T, C>),
    Playbook(StepPlaybook<T>),
    Parallel(StepParallel),
    IfCondition(StepIfCondition),
    WhileCondition(StepWhileCondition),
    SwitchCondition(StepSwitchCondition),
}

impl<T, C> AsRef<CommonProperties> for Step<T, C> {
    fn as_ref(&self) -> &CommonProperties {
        match self {
            Step::Start(v) => v,
            Step::End(v) => v,
            Step::Single(v) => v.as_ref(),
            Step::Playbook(v) => v.as_ref(),
            Step::Parallel(v) => v.as_ref(),
            Step::IfCondition(v) => v.as_ref(),
            Step::WhileCondition(v) => v.as_ref(),
            Step::SwitchCondition(v) => v.as_ref(),
        }
    }
}

impl<T, C> fmt::Display for Step<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        AsRef::<CommonProperties>::as_ref(self).fmt(f)
    }
}

impl<'a, T, C> ToStepRels<'a> for &'a Step<T, C> {
    fn to_step_rels(self, rels: &mut crate::step_graph::RelStream<'a>) {
        match self {
            Step::Start(v) => v.to_step_rels(rels),
            Step::End(v) => v.to_step_rels(rels),
            Step::Single(v) => v.to_step_rels(rels),
            Step::Playbook(v) => v.to_step_rels(rels),
            Step::Parallel(v) => v.to_step_rels(rels),
            Step::IfCondition(v) => v.to_step_rels(rels),
            Step::WhileCondition(v) => v.to_step_rels(rels),
            Step::SwitchCondition(v) => v.to_step_rels(rels),
        }
    }
}
