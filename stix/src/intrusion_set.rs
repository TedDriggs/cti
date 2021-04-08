use std::collections::BTreeSet;

use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize)]
pub struct Data {
    #[serde(flatten)]
    common: CommonProperties,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    aliases: BTreeSet<String>,
}

impl Data {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn aliases(&self) -> &BTreeSet<String> {
        &self.aliases
    }
}

impl AsRef<CommonProperties> for Data {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
