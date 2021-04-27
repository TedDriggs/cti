use theobroma::Playbook;

/// The contents of section 3.1, with real UUIDs inserted in lieu of placeholders.
const CONTENT: &'static str = r#"
{
    "type": "playbook",
    "spec_version": "1.0",
    "id": "playbook--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2",
    "name": "Find Malware FuzzyPanda",
    "description": "This playbook will look for FuzzyPanda on the network and in a SIEM",
    "playbook_types": ["investigation"],
    "created_by": "identity--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2",
    "created": "2020-03-04T15:56:00.123456Z",
    "modified": "2020-03-04T15:56:00.123456Z",
    "revoked": false,
    "valid_from": "2020-03-04T15:56:00.123456Z",
    "valid_until": "2020-07-31T23:59:59.999999Z",
    "derived_from": ["playbook--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2"],
    "priority": 3,
    "severity": 70,
    "impact": 5,
    "labels": [ "malware", "fuzzypanda", "apt"],
    "external_references": [
      {
        "name": "ACME Security FuzzyPanda Report",
        "description": "ACME security review of FuzzyPanda 2021",
        "source": "ACME Security Company, Solutions for FuzzyPanda 2021, January 2021.\nAvailable online: hxxp://www[.]example[.]com/info/fuzzypanda2021.html",
        "url": "hxxp://www[.]example[.]com/info/fuzzypanda2021.html",
        "hash": "f92d8b0291653d8790907fe55c024e155e460eabb165038ace33bb7f2c1b9019",
        "external_id": "fuzzypanda 2021.01"
      }
    ],
    "features": {
        "if_logic": true,
        "data_markings": true
    },
    "markings": [
      "marking-statement--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2"
    ],
    "playbook_variables": {
      "$$data_exfil_site$$": {
        "type": "ipv4-addr",
        "description": "The IP address for the data exfiltration site",
        "value": "1.2.3.4",
        "constant": false
      }
    },
    "workflow_start": "step--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2",
    "workflow_exception": "step--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2",
    "workflow": { },
    "targets": { },
    "extension_definitions": { },
    "data_marking_definitions": { },
    "signatures": [ ]
  }
  "#;

#[test]
fn deserialize() {
    let playbook = serde_json::from_str::<Playbook>(CONTENT).unwrap();
    assert_eq!(playbook.playbook_variables.len(), 1);
}
