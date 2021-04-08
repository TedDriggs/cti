use serde::Deserialize;

use crate::{CommonProperties, TypedObject};

#[derive(Deserialize)]
pub struct Tool {
    #[serde(flatten)]
    base: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl Tool {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

impl TypedObject for Tool {
    const TYPE: &'static str = "tool";
}

impl AsRef<CommonProperties> for Tool {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
