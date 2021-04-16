use serde_json;

use stix::{standard::Declaration, vocab::PatternType};

const DECLARATION: &'static str = r#"
{
    "type": "indicator",
    "spec_version": "2.1",
    "id": "indicator--8e2e2d2b-17d4-4cbf-938f-98ee46b3cd3f",
    "created_by_ref": "identity--f431f809-377b-45e0-aa1c-6a4751cae5ff",
    "created": "2016-04-06T20:03:48.000Z",
    "modified": "2016-04-06T20:03:48.000Z",
    "indicator_types": ["malicious-activity"],
    "name": "Poison Ivy Malware",
    "description": "This file is part of Poison Ivy",
    "pattern": "[ file:hashes.'SHA-256' = '4bac27393bdd9777ce02453256c5577cd02275510b2227f473d03f533924f877' ]",
    "pattern_type": "stix",
    "valid_from": "2016-01-01T00:00:00Z"
}
"#;

#[test]
fn reads_pattern() {
    let declaration: Declaration = serde_json::from_str(DECLARATION).unwrap();
    if let Declaration::Indicator(indicator) = declaration {
        assert_eq!(indicator.pattern.pattern_type, PatternType::STIX);
    } else {
        panic!("Declaration did not deserialize as pattern");
    }
}
