use stix::ExternalReference;

/// Check if an external reference is to MITRE ATT&CK.
pub fn is_mitre_reference(xr: &ExternalReference) -> bool {
    xr.source_name == "mitre-attack"
}

/// Get the MITRE ATT&CK ID from an external reference, if the external reference
/// is of the correct type.
pub fn get_mitre_id(xr: &ExternalReference) -> Option<&str> {
    if is_mitre_reference(xr) {
        xr.external_id.as_ref().map(|s| s.as_str())
    } else {
        None
    }
}
