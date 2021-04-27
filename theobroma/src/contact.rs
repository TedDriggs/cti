use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::MaybeVariable;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<MaybeVariable<IndexMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<MaybeVariable<IndexMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<MaybeVariable<String>>,
}
