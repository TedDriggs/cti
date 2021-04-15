# attck

Crate for working with MITRE ATT&CK matrices in Rust.

This crate includes the schema and data for each MITRE ATT&CK matrix.

## Usage

```rust
// initialize the matrix once; this parses JSON so it can be expensive.
let enterprise = attck::enterprise();

let threat = enterprise
    .intrusion_sets()
    .find(|int_set| int_set.name() == "BRONZE BUTLER")
    .unwrap();

// For the example only look at the attack patterns whose IDs resolve in the collection.
// STIX data will not always be so clean, so the extra call to `resolve` enables graph
// traversal without panicking in those cases.
for pat in threat.uses_attack_patterns().filter_map(|r| r.resolve()) {
    println!("{}", pat.name());

    // Every SRO relationship is expressed as a pair of methods for forward and backward
    // traversal, making typesafe navigation easy.
    for mitigation in pat
        .mitigated_by_courses_of_action()
        .filter_map(|r| r.resolve())
    {
        println!(" - {}", mitigation.name());
    }
}
```
