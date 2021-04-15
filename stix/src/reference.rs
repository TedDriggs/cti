use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalReference {
    pub source_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KillChainPhase {
    pub kill_chain_name: String,
    pub phase_name: String,
}

impl KillChainPhase {
    /// Create a new phase reference to the `mitre-attack` kill chain.
    pub fn mitre(phase_name: impl Into<String>) -> Self {
        Self {
            kill_chain_name: "mitre-attack".into(),
            phase_name: phase_name.into(),
        }
    }
}
