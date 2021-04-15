use std::{
    collections::{BTreeSet, HashMap},
    fmt,
};

use attck::{AttackPattern, Matrix, Node, Tactic};
use stix::Id;

struct DisplayMatrix<'a> {
    /// The tactic columns of the MITRE ATT&CK matrix and their associated techniques.
    /// Subtechniques are not included, but parent technique `Node` instances are preserved
    /// so that the `subtechnique_attack_patterns` method can be used to retrieve those.
    ///
    /// Tactics are in declaration order; techniques are in order of their declaration in
    /// the STIX bundle.
    columns: Vec<(&'a Tactic, Vec<Node<'a, AttackPattern>>)>,
    dangling_ids: BTreeSet<&'a Id>,
}

impl<'a> DisplayMatrix<'a> {
    pub fn new(matrix: Node<'a, Matrix>) -> Self {
        let collection = matrix.collection();
        // Annoyingly, MITRE does not publish SROs from tactics to techniques and instead relies on
        // the kill_chain_phases property. The `stix` crate doesn't know about this relationship, so
        // we do one pass to gather all the tactics and visit every attack_pattern to build the grid.
        let mut tactics_by_shortname = HashMap::new();

        // Cells are ordered to preserve the ordering from the Matrix declaration.
        let mut columns = vec![];

        let mut dangling_ids = BTreeSet::new();

        for tactic_ref in &matrix.data().tactic_refs {
            if let Some(tactic) = collection.get::<Tactic>(tactic_ref) {
                // Use data() because &tactic.shortname doesn't live long enough
                tactics_by_shortname.insert(&tactic.data().shortname, columns.len());
                // Insert _after_ capturing the index of the value we just added or else our
                // indices will be off by one.
                columns.push((tactic.data(), vec![]));
            } else {
                dangling_ids.insert(tactic_ref);
            }
        }

        for attack_pattern in matrix
            .collection()
            .attack_patterns()
            .filter(|ap| ap.mitre.is_subtechnique == Some(false))
        {
            for phase in attack_pattern
                .base
                .kill_chain_phases
                .iter()
                .filter(|phase| phase.kill_chain_name == "mitre-attack")
            {
                if let Some(cell_index) = tactics_by_shortname.get(&phase.phase_name).copied() {
                    columns[cell_index].1.push(attack_pattern.clone());
                }
            }
        }

        Self {
            columns,
            dangling_ids,
        }
    }
}

impl fmt::Display for DisplayMatrix<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (tactic, techniques) in &self.columns {
            write!(f, "{} | ", tactic.name)?;
            if techniques.is_empty() {
                writeln!(f, "No techniques")?;
                continue;
            }

            write!(f, "{}", techniques[0].name())?;

            for technique in &techniques[1..3] {
                write!(f, ", {}", technique.name())?;
            }

            if techniques.len() > 4 {
                writeln!(f, " +{} more", techniques.len() - 4)?;
            } else {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let enterprise = attck::enterprise();

    for matrix in enterprise.matrices() {
        println!("===== {} =====", matrix.name);
        let disp = DisplayMatrix::new(matrix);
        println!("{}", disp);
        for id in &disp.dangling_ids {
            eprintln!("Found reference to {} but it was not in the collection", id);
        }
    }
}
