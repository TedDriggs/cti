use proc_macro2::Span;
use syn::{parse_quote, Ident, Type};

use crate::{Relationship, StixCratePath, Variant};

fn rel(rel_type: &'static str, dest: &'static str) -> Relationship {
    Relationship {
        rel: Ident::new(rel_type, Span::call_site()).into(),
        to: Ident::new(dest, Span::call_site()),
    }
}

fn variant(crate_path: StixCratePath, name: &'static str, rels: Vec<Relationship>) -> Variant {
    let ident = Ident::new(name, Span::call_site());
    let ty = Type::Path(parse_quote!(#crate_path::#ident));
    Variant::new(ident, ty, rels)
}

pub(crate) fn standard_objects(crate_path: StixCratePath) -> Vec<Variant> {
    standard_tuples()
        .into_iter()
        .map(|(name, rels)| variant(crate_path, name, rels))
        .collect()
}

fn standard_tuples() -> Vec<(&'static str, Vec<Relationship>)> {
    vec![
        (
            "AttackPattern",
            vec![
                rel("Compromises", "Infrastructure"),
                // XXX this is not in the OASIS standard, but custom relationships are not
                // yet supported and `attck` is the crate's primary consumer.
                rel("SubtechniqueOf", "AttackPattern"),
                rel("Delivers", "Malware"),
                rel("Targets", "Identity"),
                rel("Targets", "Location"),
                rel("Targets", "Vulnerability"),
                rel("Uses", "Tool"),
                rel("Uses", "Malware"),
                rel("Uses", "Vulnerability"),
            ],
        ),
        (
            "Campaign",
            vec![
                rel("AttributedTo", "IntrusionSet"),
                rel("AttributedTo", "ThreatActor"),
                rel("Compromises", "Infrastructure"),
                rel("OriginatesFrom", "Location"),
                rel("Targets", "Identity"),
                rel("Targets", "Location"),
                rel("Targets", "Vulnerability"),
                rel("Uses", "AttackPattern"),
                rel("Uses", "Infrastructure"),
                rel("Uses", "Malware"),
                rel("Uses", "Tool"),
            ],
        ),
        (
            "CourseOfAction",
            vec![
                rel("Investigates", "Indicator"),
                rel("Mitigates", "AttackPattern"),
                rel("Mitigates", "Indicator"),
                rel("Mitigates", "Malware"),
                rel("Mitigates", "Tool"),
                rel("Mitigates", "Vulnerability"),
                rel("Remediates", "Malware"),
                rel("Remediates", "Vulnerability"),
            ],
        ),
        ("Grouping", vec![]),
        ("Identity", vec![rel("LocatedAt", "Location")]),
        (
            "Indicator",
            vec![
                rel("BasedOn", "ObservedData"),
                rel("Indicates", "AttackPattern"),
                rel("Indicates", "Campaign"),
                rel("Indicates", "Infrastructure"),
                rel("Indicates", "IntrusionSet"),
                rel("Indicates", "Malware"),
                rel("Indicates", "ThreatActor"),
                rel("Indicates", "Tool"),
            ],
        ),
        (
            "IntrusionSet",
            vec![
                rel("AttributedTo", "ThreatActor"),
                rel("Hosts", "Infrastructure"),
                rel("Owns", "Infrastructure"),
                rel("Targets", "Identity"),
                rel("Targets", "Location"),
                rel("Targets", "Vulnerability"),
                rel("Uses", "AttackPattern"),
                rel("Uses", "Infrastructure"),
                rel("Uses", "Malware"),
                rel("Uses", "Tool"),
            ],
        ),
        (
            "Infrastructure",
            vec![
                rel("ConsistsOf", "ObservedData"),
                rel("Controls", "Infrastructure"),
                rel("Controls", "Malware"),
                rel("Delivers", "Malware"),
                rel("Has", "Vulnerability"),
                rel("Hosts", "Tool"),
                rel("Hosts", "Malware"),
                rel("LocatedAt", "Location"),
                rel("Uses", "Infrastructure"),
            ],
        ),
        ("Location", vec![]),
        (
            "Malware",
            vec![
                rel("AuthoredBy", "ThreatActor"),
                rel("BeaconsTo", "Infrastructure"),
                rel("Controls", "Malware"),
                // rel("Downloads", "File"),
                rel("Downloads", "Malware"),
                rel("Downloads", "Tool"),
                // rel("Drops", "File"),
                rel("Drops", "Malware"),
                rel("Drops", "Tool"),
                rel("ExfiltratesTo", "Infrastructure"),
                rel("OriginatesFrom", "Location"),
                rel("Targets", "Identity"),
                rel("Targets", "Infrastructure"),
                rel("Targets", "Location"),
                rel("Targets", "Vulnerability"),
                rel("Uses", "AttackPattern"),
                rel("Uses", "Infrastructure"),
                rel("Uses", "Malware"),
                rel("Uses", "Tool"),
            ],
        ),
        (
            "MalwareAnalysis",
            vec![
                rel("AnalysisOf", "Malware"),
                rel("Characterizes", "Malware"),
                rel("StaticAnalysisOf", "Malware"),
                rel("DynamicAnalysisOf", "Malware"),
            ],
        ),
        ("MarkingDefinition", vec![]),
        ("Note", vec![]),
        ("ObservedData", vec![]),
        ("Opinion", vec![]),
        ("Relationship", vec![]),
        ("Report", vec![]),
        (
            "ThreatActor",
            vec![
                rel("AttributedTo", "Identity"),
                rel("Compromises", "Infrastructure"),
                rel("Hosts", "Infrastructure"),
                rel("Owns", "Infrastructure"),
                rel("Impersonates", "Identity"),
                rel("LocatedAt", "Location"),
                rel("Targets", "Identity"),
                rel("Targets", "Location"),
                rel("Targets", "Vulnerability"),
                rel("Uses", "AttackPattern"),
                rel("Uses", "Infrastructure"),
                rel("Uses", "Malware"),
                rel("Uses", "Tool"),
            ],
        ),
        (
            "Tool",
            vec![
                rel("Delivers", "Malware"),
                rel("Drops", "Malware"),
                rel("Has", "Vulnerability"),
                rel("Targets", "Identity"),
                rel("Targets", "Infrastructure"),
                rel("Targets", "Location"),
                rel("Targets", "Vulnerability"),
                rel("Uses", "Infrastructure"),
            ],
        ),
        ("Vulnerability", vec![]),
    ]
}
