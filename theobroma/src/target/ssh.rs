use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::{MaybeVariable, MaybeVariableRef, Target, TargetType};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SshAddress {
    IpAddr(IpAddr),
    Domain(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetSsh {
    name: MaybeVariable<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    address: MaybeVariable<SshAddress>,
    port: MaybeVariable<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    username: Option<MaybeVariable<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    password: Option<MaybeVariable<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    private_key: Option<MaybeVariable<String>>,
}

impl Target for TargetSsh {
    fn target_type(&self) -> &TargetType {
        TargetType::SSH
    }

    fn name(&self) -> MaybeVariableRef<&str> {
        self.name.as_str()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}
