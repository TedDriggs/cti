use theobroma::Playbook;

/// The contents of appendix A.1, with real UUIDs inserted in lieu of placeholders.
const CONTENT: &'static str = include_str!("../samples/appendix_a_1.json");

#[test]
fn deserialize() {
    let playbook = serde_json::from_str::<Playbook>(CONTENT).unwrap();
    assert_eq!(playbook.playbook_variables.len(), 0);
}
