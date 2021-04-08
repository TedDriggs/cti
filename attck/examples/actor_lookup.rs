//! Demonstrate loading, searching, and displaying results from a data set.

use stix::{Collection, IntrusionSet, Node, Object};

fn display_actor<'a>(actor: &Node<'a, IntrusionSet>) {
    println!("{} ({})", actor.name(), actor.id());
    println!("============");

    if let Some(description) = actor.description() {
        println!("{}\n", description);
    }

    let mut malwares = actor.malwares().peekable();
    if malwares.peek().is_some() {
        println!("Malware:");
        for malware in malwares {
            println!("- {} ({})", malware.name(), malware.id());
        }
    }

    let mut tools = actor.tools().peekable();
    if tools.peek().is_some() {
        println!("Tools:");
        for tool in tools {
            println!("- {} ({})", tool.name(), tool.id());
        }
    }
}

fn search<'store>(store: &'store Collection, term: &str) -> Option<Node<'store, IntrusionSet>> {
    store
        .intrusion_sets()
        .find(|a| a.name() == term || a.aliases().contains(term))
}

fn main() {
    let enterprise = attck::enterprise();
    if let Some(actor) = search(&enterprise, "APT1") {
        display_actor(&actor);
    } else {
        println!("No results");
    }
}
