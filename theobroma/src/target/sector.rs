use serde::{Deserialize, Serialize};
use stix::vocab::IndustrySector;

use crate::{CivicLocation, MaybeVariable, MaybeVariableRef, Target, TargetType};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetSector {
    name: MaybeVariable<IndustrySector>,
    description: Option<String>,
    location: Vec<CivicLocation>,
}

impl Target for TargetSector {
    fn target_type(&self) -> &super::TargetType {
        TargetType::SECTOR
    }

    fn name(&self) -> crate::MaybeVariableRef<&str> {
        match self.name.as_ref() {
            MaybeVariableRef::Value(v) => MaybeVariableRef::Value(v.as_ref()),
            MaybeVariableRef::Variable(v) => MaybeVariableRef::Variable(v),
        }
    }

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}
