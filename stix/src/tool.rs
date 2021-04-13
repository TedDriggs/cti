use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
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

impl AsRef<CommonProperties> for Tool {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
