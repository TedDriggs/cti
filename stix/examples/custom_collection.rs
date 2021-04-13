#![allow(dead_code)]
use stix::{AttackPattern, Identity, Malware, Relationship, Tool};

#[derive(stix::Collection)]
enum Declaration {
    #[stix(rel(Uses, Tool), rel(Uses, Malware))]
    AttackPattern(AttackPattern),
    Identity(Identity),
    Malware(Malware),
    Relationship(Relationship),
    Tool(Tool),
}

fn act_on_collection(coll: &Collection) {
    for ap in coll.attack_patterns() {
        for _malware in ap.uses_malwares() {}
    }
}

fn main() {
    let _builder = CollectionBuilder::default();
}
