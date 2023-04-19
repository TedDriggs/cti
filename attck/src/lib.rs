mod attack_pattern;
mod course_of_action;
mod data_component;
mod data_source;
mod declaration;
mod malware;
mod matrix;
mod mitre_id;
mod tactic;
mod tool;

pub use attack_pattern::{AttackPattern, MitreAttackPattern};
pub use course_of_action::{CourseOfAction, MitreCourseOfAction};
pub use data_component::DataComponent;
pub use data_source::DataSource;
pub use declaration::*;
pub use malware::{Malware, MitreMalware};
pub use matrix::Matrix;
pub(crate) use mitre_id::get_mitre_id;
pub use tactic::Tactic;
pub use tool::{MitreTool, Tool};

/// Load the complete Enterprise ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "enterprise")]
pub fn enterprise() -> Collection {
    const DATA: &str = include_str!("../json/enterprise.json");
    let bundle: stix::Bundle<Declaration> =
        serde_json::from_str(DATA).expect("Cannot parse built-in enterprise ATT&CK bundle");
    bundle.into()
}

/// Load the complete ICS ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "ics")]
pub fn ics() -> Collection {
    const DATA: &str = include_str!("../json/ics.json");
    let bundle: stix::Bundle<Declaration> =
        serde_json::from_str(DATA).expect("Cannot parse built-in ICS ATT&CK bundle");
    bundle.into()
}

/// Load the complete Mobile ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "mobile")]
pub fn mobile() -> Collection {
    const DATA: &str = include_str!("../json/mobile.json");
    let bundle: stix::Bundle<Declaration> =
        serde_json::from_str(DATA).expect("Cannot parse built-in mobile ATT&CK bundle");
    bundle.into()
}
