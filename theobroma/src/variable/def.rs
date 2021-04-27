use serde::{Deserialize, Serialize};

use crate::MaybeVariable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariableType {
    String,
    Uuid,
    Integer,
    Long,
    MacAddr,
    Ipv4Addr,
    Ipv6Addr,
    Uri,
    Sha256Hash,
    #[serde(rename = "hexstring")]
    HexString,
    Dictionary,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonDef {
    #[serde(rename = "type")]
    ty: VariableType,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    value: Option<MaybeVariable<String>>,
    #[serde(default)]
    constant: bool,
}
