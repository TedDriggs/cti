//! Graph representation of steps in a workflow.

use std::{borrow::Cow, collections::HashMap, convert::TryFrom, fmt};

use petgraph::{
    graph::{EdgeReference, NodeIndex},
    EdgeDirection, Graph,
};
use stix::Id;

use crate::{OnError, Playbook, Step, WithId};

/// Directed graph representation of workflow steps in a playbook.
///
/// Edges run from the referencing step to the referenced step, as all relationships in the
/// CACAO spec point from earlier steps to later steps.
pub struct StepGraph<'a, T = crate::StandardTarget, C = crate::Command> {
    step_node_indices: HashMap<&'a Id, NodeIndex>,
    graph: Graph<WithId<'a, &'a Step<T, C>>, FieldPath<'a>>,
}

impl<'a, T, C> StepGraph<'a, T, C> {
    /// Create a new graph from a playbook with the specified error-handling strategy.
    pub fn with_failure_behavior(
        value: &'a Playbook<T, C>,
        on_error: OnError,
    ) -> Result<Self, Vec<DanglingRefError<'a>>> {
        let mut errors = vec![];
        let mut graph = Graph::new();
        let step_node_indices = value
            .workflow
            .iter()
            .map(WithId::new)
            .map(|entry| (entry.id(), graph.add_node(entry)))
            .collect::<HashMap<_, _>>();
        for entry in value.workflow.iter().map(WithId::new) {
            let mut rels = RelStream::default();
            entry.data().to_step_rels(&mut rels);
            let source_index = *step_node_indices.get(entry.id()).unwrap();

            // This `drain` is needed because `RelStream` has a drop-bomb to ensure
            // `exit_field` is always called with `enter_field`.
            for rel in rels.rels.drain(0..) {
                let target_index = match step_node_indices
                    .get(rel.target)
                    .ok_or_else(|| DanglingRefError::new(rel.target, entry.id()))
                {
                    Ok(idx) => *idx,
                    Err(e) if on_error == OnError::FailFast => {
                        return Err(vec![e]);
                    }
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                graph.add_edge(source_index, target_index, FieldPath(rel.path));
            }
        }

        Ok(Self {
            step_node_indices,
            graph,
        })
    }

    pub fn graph(&self) -> &Graph<WithId<'a, &'a Step<T, C>>, FieldPath<'a>> {
        &self.graph
    }

    pub fn edges_directed<'s>(
        &'s self,
        id: &Id,
        direction: EdgeDirection,
    ) -> impl Iterator<Item = EdgeReference<'s, FieldPath<'a>>> {
        self.step_node_indices
            .get(id)
            .copied()
            .into_iter()
            .flat_map(move |idx| self.graph.edges_directed(idx, direction))
    }
}

impl<'a, T, C> TryFrom<&'a Playbook<T, C>> for StepGraph<'a, T, C> {
    type Error = DanglingRefError<'a>;

    fn try_from(value: &'a Playbook<T, C>) -> Result<Self, Self::Error> {
        Self::with_failure_behavior(value, OnError::FailFast).map_err(|mut e| e.remove(0))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("ID `{}` referenced in `{}` but not declared in playbook", .dangling, .referenced_in)]
pub struct DanglingRefError<'a> {
    dangling: Cow<'a, Id>,
    referenced_in: Cow<'a, Id>,
}

impl<'a> DanglingRefError<'a> {
    fn new(dangling: &'a Id, referenced_in: &'a Id) -> Self {
        DanglingRefError {
            dangling: Cow::Borrowed(dangling),
            referenced_in: Cow::Borrowed(referenced_in),
        }
    }

    pub fn to_owned(&self) -> DanglingRefError<'static> {
        DanglingRefError {
            dangling: Cow::Owned(self.dangling.clone().into_owned()),
            referenced_in: Cow::Owned(self.referenced_in.clone().into_owned()),
        }
    }
}

/// A property path. Array indices are not preserved.
#[derive(Debug)]
pub struct FieldPath<'a>(Vec<&'a str>);

impl fmt::Display for FieldPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.join("/").fmt(f)
    }
}

struct Rel<'a> {
    path: Vec<&'a str>,
    target: &'a Id,
}

#[derive(Default)]
pub(crate) struct RelStream<'data> {
    rels: Vec<Rel<'data>>,
    field_path: Vec<&'data str>,
    manual_entry_count: usize,
}

impl<'data> RelStream<'data> {
    /// Push a field key onto the stack, nesting all calls to the append methods under this field name
    /// until [`exit_field`](Self::exit_field) is called.
    pub fn enter_field(&mut self, field: &'data str) {
        self.field_path.push(field);
        self.manual_entry_count += 1;
    }

    pub fn exit_field(&mut self, field: &'data str) {
        if self.manual_entry_count == 0 {
            panic!(
                "Attempted to exit field `{}` but no enter_field calls are still outstanding",
                field
            );
        }

        let top_of_stack = self.field_path.pop();
        match top_of_stack {
            Some(v) if v == field => {
                self.manual_entry_count -= 1;
            }
            Some(v) => panic!("Attempted to exit field `{}` but found `{}`", field, v),
            None => panic!("Attempted to exit field `{}` but stack was empty", field),
        }
    }

    pub fn append_all_field(&mut self, field: &'data str, ids: impl ToStepRels<'data>) {
        self.field_path.push(field);
        ids.to_step_rels(self);
        self.field_path.pop();
    }

    pub fn append_all_flat(&mut self, ids: impl IntoIterator<Item = &'data Id>) {
        for id in ids {
            self.append_flat(id);
        }
    }

    pub fn append_flat(&mut self, id: &'data Id) {
        self.rels.push(Rel {
            path: self.field_path.clone(),
            target: id,
        });
    }
}

impl Drop for RelStream<'_> {
    fn drop(&mut self) {
        if self.manual_entry_count > 0 {
            panic!("`exit_field` was not called");
        }
    }
}

/// Trait for steps to expose their outbound relationships to other steps.
pub(crate) trait ToStepRels<'a> {
    fn to_step_rels(self, rels: &mut RelStream<'a>);
}

impl<'a, T: IntoIterator<Item = &'a Id>> ToStepRels<'a> for T {
    fn to_step_rels(self, rels: &mut RelStream<'a>) {
        rels.append_all_flat(self)
    }
}
