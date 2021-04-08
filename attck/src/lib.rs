use stix::{Collection, Declaration};

/// Load the complete Enterprise ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "enterprise")]
pub fn enterprise() -> Collection {
    const DATA: &str = include_str!("../json/enterprise.json");
    let bundle: stix::Bundle<Declaration> =
        serde_json::from_str(DATA).expect("Cannot parse built-in enterprise ATT&CK bundle");
    bundle.into()
}

/// Load the complete ICS ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "ics")]
pub fn ics() -> Collection {
    const DATA: &str = include_str!("../json/ics.json");
    let bundle: stix::Bundle<Declaration> =
        serde_json::from_str(DATA).expect("Cannot parse built-in ICS ATT&CK bundle");
    bundle.into()
}

/// Load the complete Mobile ATT&CK framework packaged with the crate.
///
/// # Panics
/// This function panics if the embedded JSON or its contents are invalid.
#[cfg(feature = "mobile")]
pub fn mobile() -> Collection {
    const DATA: &str = include_str!("../json/mobile.json");
    let bundle: stix::Bundle<Declaration> =
        serde_json::from_str(DATA).expect("Cannot parse built-in mobile ATT&CK bundle");
    bundle.into()
}
