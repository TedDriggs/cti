use serde::Deserialize;

use crate::{CommonProperties, TypedObject};

#[derive(Deserialize)]
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

impl TypedObject for CourseOfAction {
    const TYPE: &'static str = "course-of-action";
}

impl AsRef<CommonProperties> for CourseOfAction {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
