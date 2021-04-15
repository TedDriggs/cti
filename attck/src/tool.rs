use std::collections::BTreeSet;

use serde::Deserialize;
use stix::CommonProperties;

#[stix::custom_properties(namespace = "mitre")]
#[derive(Default, Deserialize)]
#[serde(default)]
pub struct MitreTool {
    pub aliases: BTreeSet<String>,
    pub platforms: BTreeSet<String>,
}

#[derive(Deserialize, stix::TypedObject)]
pub struct Tool {
    #[serde(flatten)]
    pub base: stix::Tool,
    #[serde(flatten)]
    pub mitre: MitreTool,
}

impl Tool {
    pub fn name(&self) -> &str {
        self.base.name()
    }

    pub fn description(&self) -> Option<&str> {
        self.base.description()
    }
}

impl AsRef<CommonProperties> for Tool {
    fn as_ref(&self) -> &CommonProperties {
        self.base.as_ref()
    }
}
