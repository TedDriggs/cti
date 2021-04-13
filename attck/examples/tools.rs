use stix::{Id, Tool};

fn main() {
    let enterprise = attck::enterprise();
    let mimikatz_id = "tool--afc079f3-c0ea-4096-b75d-3f05338b7f60"
        .parse::<Id>()
        .unwrap();

    // Knowing the ID and the expected return type, request the resource from the
    // collection.
    let mimikatz = enterprise.get::<Tool>(&mimikatz_id).unwrap();

    println!(
        "{} {}",
        mimikatz.name(),
        mimikatz.description().unwrap_or("<NO DESCRIPTION>")
    );
}
