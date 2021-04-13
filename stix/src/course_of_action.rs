use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct CourseOfAction {
    #[serde(flatten)]
    base: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl CourseOfAction {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

impl AsRef<CommonProperties> for CourseOfAction {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
