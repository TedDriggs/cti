use std::collections::BTreeSet;

use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct IntrusionSet {
    #[serde(flatten)]
    common: CommonProperties,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    aliases: BTreeSet<String>,
}

impl IntrusionSet {
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

impl AsRef<CommonProperties> for IntrusionSet {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
