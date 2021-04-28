use theobroma::Playbook;

/// The contents of appendix A.1, with real UUIDs inserted in lieu of placeholders.
const CONTENT: &'static str = r#"
{
    "type": "playbook",
    "spec_version": "1.0",
    "id": "playbook--6b74199d-42a6-43a1-99cb-75d52207a778",
    "name": "Prevent FuzzyPanda Malware",
    "description": "This playbook will block traffic to the FuzzyPanda data exfil site",
    "playbook_types": ["prevention"],
    "created_by": "identity--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2",
    "created": "2021-04-19T23:32:24.399Z",
    "modified": "2021-04-19T23:32:24.399Z",
    "valid_from": "2021-04-19T23:32:24.39964Z",
    "valid_until": "2021-12-31T23:59:59.999999Z",
    "derived_from": ["playbook--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2"],
    "priority": 3,
    "severity": 70,
    "impact": 5,
    "labels": ["malware", "fuzzypanda", "apt"],
    "external_references": [
      {
        "name": "ACME Security FuzzyPanda Report",
        "description": "ACME security review of FuzzyPanda 2021",
        "source": "ACME Security Company, Solutions for FuzzyPanda 2021, January 2021. Available online: http://www.example.com/info/fuzzypanda2021.html",
        "url": "http://www.example.com/info/fuzzypanda2020.html",
        "external_id": "fuzzypanda 2021.01",
        "reference_id": "malware--2008c526-508f-4ad4-a565-b84a4949b2af"
      }
    ],
    "features": {
      "parallel_processing": true,
      "data_markings": true
    },
    "markings": [
      "marking-statement--16a48f6b-ab42-4f99-ba9b-8b21e1225836",
      "marking-tlp--a099a2eb-1113-4368-9b17-d7ef75841239"
    ],
    "workflow_start": "start--7269bda2-e651-44d3-9fe5-aa7e88484b93",
    "workflow": {
      "start--7269bda2-e651-44d3-9fe5-aa7e88484b93": {
        "type": "start",
        "on_completion": "single--a13c8450-2bd1-4a2b-9241-cf4f7e9f48cb"
      },
      "single--a13c8450-2bd1-4a2b-9241-cf4f7e9f48cb": {
        "type": "single",
        "name": "Receive IOC",
        "description": "Get FuzzyPanda Data Exfil Site IP Address of 1.2.3.4",
        "on_completion": "parallel--054c7e3a-20e7-4fdf-a95f-6c6e401c65c3",
        "commands": [
          {
            "type": "manual",
            "command": "Get IOC from threat feed"
          }
        ]
      },
      "parallel--054c7e3a-20e7-4fdf-a95f-6c6e401c65c3": {
        "type": "parallel",
        "name": "Update Protection Tools",
        "description": "This step will update the firewall and client EDR in parallel",
        "next_steps": [
          "single--8c46cab0-46a3-48f4-b4bb-9643dcfaf642",
          "single--3d930f08-e22c-4dd4-996f-61f2d022121c"
        ]
      },
      "single--8c46cab0-46a3-48f4-b4bb-9643dcfaf642": {
        "type": "single",
        "name": "Add IP to Firewall Blocklist",
        "description": "This step will add the IP address of the FuzzyPanda data exfil site to the firewall",
        "on_completion": "single--d5780323-5107-4cd0-bac4-6553c9d90c8e",
        "commands": [
          {
            "type": "manual",
            "command": "Open firewall console and add 1.2.3.4 to the firewall blocking policy"
          }
        ]
      },
      "single--3d930f08-e22c-4dd4-996f-61f2d022121c": {
        "type": "single",
        "name": "Add IP to Client EDR Blocklist",
        "description": "This step will add the IP address of the FuzzyPanda data exfil site to the client EDR solution",
        "on_completion": "single--d5780323-5107-4cd0-bac4-6553c9d90c8e",
        "commands": [
          {
            "type": "manual",
            "command": "Open EDR console and add 1.2.3.4 to the blocking policy"
          }
        ]
      },
      "single--d5780323-5107-4cd0-bac4-6553c9d90c8e": {
        "type": "single",
        "name": "Create Ticket",
        "description": "This step will create a ticket for this issue",
        "on_completion": "single--33dc512c-263d-4f8a-a07d-cfe9f6d6ed26",
        "commands": [
          {
            "type": "manual",
            "command": "Open case management tool and create a ticket with the details of what was done"
          }
        ]
      },
      "single--33dc512c-263d-4f8a-a07d-cfe9f6d6ed26": {
        "type": "single",
        "name": "Update SIEM",
        "description": "This step will update the SIEM to look for traffic attempts to the FuzzyPanda data exfil site",
        "on_completion": "end--6d43fbf3-54b3-432a-978b-e2b96647b786",
        "commands": [
          {
            "type": "manual",
            "command": "Open SIEM solution and add rule to look for 1.2.3.4"
          }
        ]
      },
      "end--6d43fbf3-54b3-432a-978b-e2b96647b786": {
        "type": "end"
      }
    },
    "data_marking_definitions": {
      "marking-statement--16a48f6b-ab42-4f99-ba9b-8b21e1225836": {
        "type": "marking-statement",
        "spec_version": "1.0",
        "id": "marking-statement--16a48f6b-ab42-4f99-ba9b-8b21e1225836",
        "created": "2021-04-19T23:32:24.399Z",
        "modified": "2021-04-19T23:32:24.399Z",
        "statement": "Copyright 2021 ACME Security Company"
      },
      "marking-tlp--a099a2eb-1113-4368-9b17-d7ef75841239": {
        "type": "marking-tlp",
        "spec_version": "1.0",
        "id": "marking-tlp--a099a2eb-1113-4368-9b17-d7ef75841239",
        "created": "2021-04-19T23:32:24.399Z",
        "modified": "2021-04-19T23:32:24.399Z",
        "tlp_level": "TLP-GREEN"
      }
    }
  }"#;

#[test]
fn deserialize() {
    let playbook = serde_json::from_str::<Playbook>(CONTENT).unwrap();
    assert_eq!(playbook.playbook_variables.len(), 0);
}