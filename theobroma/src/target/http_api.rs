use serde::{Deserialize, Serialize};

use crate::{MaybeVariable, MaybeVariableRef, Target, TargetType};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetHttpApi {
    name: MaybeVariable<String>,
    description: Option<String>,
    http_url: MaybeVariable<String>,
    http_auth_type: Option<MaybeVariable<String>>,
    user_id: Option<MaybeVariable<String>>,
    password: Option<MaybeVariable<String>>,
    token: Option<MaybeVariable<String>>,
    oauth_header: Option<MaybeVariable<String>>,
}

impl Target for TargetHttpApi {
    fn target_type(&self) -> &TargetType {
        TargetType::HTTP_API
    }

    fn name(&self) -> MaybeVariableRef<&str> {
        self.name.as_str()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}
