use std::convert::TryFrom;

use petgraph::dot;
use theobroma::{step_graph::StepGraph, Playbook};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playbook: Playbook = serde_json::from_str(include_str!("../samples/appendix_a_1.json"))?;
    let step_graph = StepGraph::try_from(&playbook).map_err(|e| e.to_owned())?;
    let something = dot::Dot::new(step_graph.graph());

    println!("{}", something);

    Ok(())
}
