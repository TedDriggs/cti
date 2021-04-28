use std::{convert::TryFrom, fmt};

use petgraph::dot;
use theobroma::{step::CommonProperties, step_graph::StepGraph, Playbook, Step, WithId};

/// Helper struct for rendering a workflow step on multiple lines.
/// This is used to integrate with `petgraph::dot`.
struct Multiline<'a>(WithId<'a, &'a Step>);

impl fmt::Display for Multiline<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = *self.0.data();
        let cp: &CommonProperties = data.as_ref();
        write!(f, "{}", self.0.id())?;
        if let Some(name) = &cp.name {
            write!(f, "\n{}", name)?;
        }

        if let Some(desc) = &cp.description {
            write!(f, "\n{}", desc)?;
        }

        Ok(())
    }
}

/// To see visualized output, run this from the `target` directory and pipe the output to `dot`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playbook: Playbook = serde_json::from_str(include_str!("../samples/appendix_a_1.json"))?;
    let step_graph = StepGraph::try_from(&playbook).map_err(|e| e.to_owned())?;
    let display_graph = step_graph
        .graph()
        .map(|_, node_weight| Multiline(*node_weight), |_, e| e);
    let something = dot::Dot::new(&display_graph);

    println!("{}", something);

    Ok(())
}
