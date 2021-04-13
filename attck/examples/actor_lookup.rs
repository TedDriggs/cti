//! Demonstrate loading, searching, and displaying results from a data set.

use attck::{Collection, Node};
use stix::{IntrusionSet, Object};

fn display_actor<'a>(actor: &Node<'a, IntrusionSet>) {
    println!("{} ({})", actor.name(), actor.id());
    println!("============");

    if let Some(description) = actor.description() {
        println!("{}\n", description);
    }

    let mut malwares = actor.uses_malware().peekable();
    if malwares.peek().is_some() {
        println!("Malware:");
        for malware in malwares {
            match malware.resolve() {
                Some(malware) => {
                    println!("- {} ({})", malware.name(), malware.id());
                }
                None => {
                    println!("- Unknown Malware");
                }
            }
        }
    }

    let mut tools = actor.uses_tools().peekable();
    if tools.peek().is_some() {
        println!("Tools:");
        for tool in tools {
            match tool.resolve() {
                Some(tool) => {
                    println!("- {} ({})", tool.name(), tool.id());
                }
                None => {
                    println!("- Unknown Tool");
                }
            }
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
