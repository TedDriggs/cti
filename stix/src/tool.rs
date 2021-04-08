use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize)]
pub struct Data {
    #[serde(flatten)]
    base: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl Data {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

impl AsRef<CommonProperties> for Data {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
