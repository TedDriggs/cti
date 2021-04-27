use serde::{Deserialize, Serialize};

use crate::MaybeVariableRef;

stix::vocabulary!(
    TargetType = [
        individual,
        group,
        organization,
        location,
        sector,
        http_api,
        ssh,
        security_infrastructure_category,
        net_address,
        kali,
        attacker,
        attack_agent,
        attack_group,
    ]
);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StandardTarget {}

/// Common properties on the Target object.
pub trait Target {
    fn target_type(&self) -> &TargetType;
    fn name(&self) -> MaybeVariableRef<&str>;
    fn description(&self) -> Option<&str>;
}
