use serde::Deserialize;
use stix::CommonProperties;

#[derive(Deserialize, stix::TypedObject)]
#[typed_object(name = "x-mitre-tactic")]
pub struct Tactic {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// The kebab-case name used in [`KillChainPhase`](::stix::KillChainPhase) references to this tactic.
    #[serde(rename = "x_mitre_shortname")]
    pub shortname: String,
}

impl AsRef<CommonProperties> for Tactic {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
