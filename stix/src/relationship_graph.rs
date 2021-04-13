use std::{collections::HashMap, iter::FromIterator};

use petgraph::{
    graph::{EdgeReference, NodeIndex},
    EdgeDirection, Graph,
};

use crate::{relationship, Id, Relationship};

pub struct RelationshipGraph<'a> {
    object_indices: HashMap<&'a Id, NodeIndex>,
    graph: Graph<&'a Id, &'a Relationship>,
}

impl<'a> RelationshipGraph<'a> {
    fn node_index(&self, id: &Id) -> Option<NodeIndex> {
        self.object_indices.get(id).copied()
    }

    fn edges_directed(
        &'a self,
        id: &Id,
        dir: EdgeDirection,
    ) -> impl Iterator<Item = &'a Relationship> {
        match self.node_index(id) {
            None => EdgeIter::Empty,
            Some(idx) => EdgeIter::Edges(self.graph.edges_directed(idx, dir)),
        }
    }

    pub fn peers_matching(
        &'a self,
        id: &Id,
        filter: relationship::Filter,
    ) -> impl Iterator<Item = &'a Id> {
        self.edges_directed(id, filter.direction)
            .filter_map(move |d| {
                if filter.matches(d) {
                    match filter.direction {
                        EdgeDirection::Incoming => Some(&d.source_ref),
                        EdgeDirection::Outgoing => Some(&d.target_ref),
                    }
                } else {
                    None
                }
            })
    }
}

impl<'a> FromIterator<&'a Relationship> for RelationshipGraph<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Relationship>>(iter: T) -> Self {
        let mut object_indices = HashMap::new();
        let mut graph = Graph::new();
        for relationship in iter {
            let source_idx = *object_indices
                .entry(&relationship.source_ref)
                .or_insert_with(|| graph.add_node(&relationship.source_ref));

            let target_idx = *object_indices
                .entry(&relationship.target_ref)
                .or_insert_with(|| graph.add_node(&relationship.target_ref));

            graph.add_edge(source_idx, target_idx, relationship);
        }

        Self {
            object_indices,
            graph,
        }
    }
}

enum EdgeIter<E> {
    Empty,
    Edges(E),
}

impl<'a, E: Iterator<Item = EdgeReference<'a, &'a Relationship>>> Iterator for EdgeIter<E> {
    type Item = &'a Relationship;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EdgeIter::Empty => None,
            EdgeIter::Edges(ref mut edges) => edges.next().map(|e| *e.weight()),
        }
    }
}
