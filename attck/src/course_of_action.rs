use serde::Deserialize;
use stix::{CommonProperties, Object};

use crate::get_mitre_id;

#[stix::custom_properties(namespace = "mitre")]
#[derive(Deserialize)]
pub struct MitreCourseOfAction {
    #[serde(default)]
    pub deprecated: bool,
}

#[derive(Deserialize, stix::TypedObject)]
pub struct CourseOfAction {
    #[serde(flatten)]
    pub base: stix::CourseOfAction,
    #[serde(flatten)]
    pub mitre: MitreCourseOfAction,
}

impl CourseOfAction {
    /// Gets the MITRE ID for this course of action, such as `M1049`.
    ///
    /// For deprecated courses of action, this may return an ID starting with a `T`.
    pub fn mitre_id(&self) -> Option<&str> {
        self.external_references().iter().find_map(get_mitre_id)
    }
}

impl AsRef<CommonProperties> for CourseOfAction {
    fn as_ref(&self) -> &CommonProperties {
        self.base.as_ref()
    }
}
