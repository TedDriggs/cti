use serde::Deserialize;

use crate::{CommonProperties, KillChainPhase};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct AttackPattern {
    #[serde(flatten)]
    base: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub kill_chain_phases: Vec<KillChainPhase>,
}

impl AttackPattern {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

impl AsRef<CommonProperties> for AttackPattern {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
