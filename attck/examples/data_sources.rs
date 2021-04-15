//! This example illustrates using custom properties and how to keep data
//! alive when iterating through STIX objects.

use std::collections::BTreeSet;

fn main() {
    let enterprise = attck::enterprise();

    let data_sources = enterprise
        .attack_patterns()
        .flat_map(|ap| &ap.data().mitre.data_sources)
        .collect::<BTreeSet<_>>();

    for data_source in data_sources {
        println!("{}", data_source);
    }
}
