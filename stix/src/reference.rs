use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalReference {
    source_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KillChainPhase {
    kill_chain_name: String,
    phase_name: String,
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
