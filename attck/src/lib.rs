pub mod actor;
pub mod bundle;
pub mod malware;
pub mod relationship;
pub(crate) mod store;
pub mod technique;
pub mod tool;

pub use actor::Actor;
pub use bundle::Bundle;
pub use malware::Malware;
pub use relationship::{Relationship, RelationshipType};
pub use store::Store;
pub use technique::Technique;
pub use tool::Tool;

/// Load the complete Enterprise ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "enterprise")]
pub fn enterprise() -> Store {
    use std::convert::TryInto;

    const DATA: &str = include_str!("../json/enterprise.json");
    let bundle: Bundle =
        serde_json::from_str(DATA).expect("Cannot parse built-in enterprise ATT&CK bundle");
    bundle
        .try_into()
        .expect("Cannot index built-in enterprise ATT&CK data")
}

/// Load the complete ICS ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "ics")]
pub fn ics() -> Store {
    use std::convert::TryInto;

    const DATA: &str = include_str!("../json/ics.json");
    let bundle: Bundle =
        serde_json::from_str(DATA).expect("Cannot parse built-in ICS ATT&CK bundle");
    bundle
        .try_into()
        .expect("Cannot index built-in ICS ATT&CK data")
}

/// Load the complete Mobile ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "mobile")]
pub fn mobile() -> Store {
    use std::convert::TryInto;

    const DATA: &str = include_str!("../json/mobile.json");
    let bundle: Bundle =
        serde_json::from_str(DATA).expect("Cannot parse built-in mobile ATT&CK bundle");
    bundle
        .try_into()
        .expect("Cannot index built-in mobile ATT&CK data")
}
