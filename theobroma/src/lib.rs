//! `theobroma` is a crate for parsing OASIS collaborative automated course of action operations (CACAO)
//! security playbooks. Its name comes from "Theobroma Cacao", the tree responsible for the cocoa bean.

mod bounded_u8;
mod civic_location;
mod command;
mod contact;
mod playbook;
pub mod step;
pub mod target;
mod variable;

pub use bounded_u8::OutOfBoundsError;
pub use civic_location::CivicLocation;
pub use command::{Command, CommandType};
pub use contact::Contact;
pub use playbook::{
    Impact, Playbook, PlaybookFeature, PlaybookFeatures, PlaybookType, PlaybookVariableDef,
    Priority, Severity,
};
pub use step::{
    Step, StepIfCondition, StepParallel, StepPlaybook, StepSingle, StepSwitchCondition,
    StepWhileCondition,
};
pub use target::{StandardTarget, Target, TargetType};
pub use variable::{MaybeVariable, MaybeVariableRef, Variable, VariableType};
