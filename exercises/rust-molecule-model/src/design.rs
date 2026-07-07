use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

use crate::molecule::{Atom, Bond, BondOrder, Element, Molecule};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Substituent {
    H,
    F,
    Cl,
    Br,
}

impl Substituent {
    pub fn element(self) -> Element {
        match self {
            Self::H => Element::H,
            Self::F => Element::F,
            Self::Cl => Element::Cl,
            Self::Br => Element::Br,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::H => "H",
            Self::F => "F",
            Self::Cl => "Cl",
            Self::Br => "Br",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub name: String,
    pub molecule: Molecule,
    pub substituent: Substituent,
    pub valid: bool,
    pub score: Option<f64>,
}

impl Candidate {
    pub fn new(name: String, molecule: Molecule, substituent: Substituent) -> Self {
        let valid = validate_candidate(&molecule);

        Self {
            name,
            molecule,
            substituent,
            valid,
            score: None,
        }
    }
}

pub fn substituent_options() -> Vec<Substituent> {
    vec![
        Substituent::H,
        Substituent::F,
        Substituent::Cl,
        Substituent::Br,
    ]
}

pub fn attach_substituent(
    scaffold_name: &str,
    scaffold: &Molecule,
    attachment_atom: usize,
    substituent: Substituent,
) -> Candidate {
    let mut atoms = scaffold.atoms().to_vec();
    let mut bonds = scaffold.bonds().to_vec();
    let new_atom_id = atoms.len();

    atoms.push(Atom::neutral(substituent.element()));
    bonds.push(Bond::new(attachment_atom, new_atom_id, BondOrder::Single));

    let name = format!("{scaffold_name}-{}", substituent.label());
    Candidate::new(name, Molecule::new(atoms, bonds), substituent)
}

pub fn validate_candidate(molecule: &Molecule) -> bool {
    molecule.validate_bond_indices() && molecule.connected_components().len() == 1
}

pub fn toy_score(molecule: &Molecule) -> f64 {
    let hetero_atoms = molecule
        .atoms()
        .iter()
        .filter(|atom| matches!(atom.element, Element::N | Element::O | Element::F))
        .count();

    hetero_atoms as f64 * 2.0 + molecule.bond_count() as f64 - molecule.atom_count() as f64 * 0.1
}

pub fn score_candidate(candidate: &mut Candidate) {
    candidate.valid = validate_candidate(&candidate.molecule);
    candidate.score = candidate.valid.then(|| toy_score(&candidate.molecule));
}

pub fn generate_design_round(
    scaffold_name: &str,
    scaffold: &Molecule,
    attachment_atom: usize,
    options: &[Substituent],
) -> Vec<Candidate> {
    options
        .iter()
        .map(|&substituent| {
            attach_substituent(scaffold_name, scaffold, attachment_atom, substituent)
        })
        .collect()
}

pub fn score_candidates(candidates: &mut [Candidate]) {
    for candidate in candidates {
        score_candidate(candidate);
    }
}

pub fn score_lookup(candidates: &[Candidate]) -> HashMap<String, f64> {
    candidates
        .iter()
        .filter_map(|candidate| candidate.score.map(|score| (candidate.name.clone(), score)))
        .collect()
}

pub fn rank_candidates(mut candidates: Vec<Candidate>) -> Vec<Candidate> {
    candidates.sort_by(|left, right| match (left.score, right.score) {
        (Some(left_score), Some(right_score)) => right_score
            .partial_cmp(&left_score)
            .unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left.name.cmp(&right.name),
    });

    candidates
}

pub fn next_frontier(candidates: &[Candidate], limit: usize) -> VecDeque<Candidate> {
    candidates.iter().take(limit).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::molecule::{ethanol, water};

    #[test]
    fn generates_candidates_from_substituent_options() {
        let candidates = generate_design_round("ethanol", &ethanol(), 1, &substituent_options());

        assert_eq!(candidates.len(), 4);
        assert_eq!(candidates[0].name, "ethanol-H");
        assert_eq!(candidates[0].molecule.atom_count(), 10);
        assert!(candidates.iter().all(|candidate| candidate.valid));
    }

    #[test]
    fn rejects_invalid_attachment_atom() {
        let candidate = attach_substituent("water", &water(), 99, Substituent::F);

        assert!(!candidate.valid);
        assert_eq!(candidate.score, None);
    }

    #[test]
    fn scores_and_ranks_candidates() {
        let mut candidates = generate_design_round("water", &water(), 0, &substituent_options());
        score_candidates(&mut candidates);

        let ranked = rank_candidates(candidates);

        assert!(ranked[0].score >= ranked[1].score);
        assert!(ranked.iter().all(|candidate| candidate.score.is_some()));
    }

    #[test]
    fn builds_score_lookup_by_candidate_name() {
        let mut candidates = generate_design_round("water", &water(), 0, &substituent_options());
        score_candidates(&mut candidates);

        let lookup = score_lookup(&candidates);

        assert_eq!(lookup.len(), 4);
        assert!(lookup.contains_key("water-F"));
    }

    #[test]
    fn keeps_top_candidates_as_search_frontier() {
        let mut candidates = generate_design_round("water", &water(), 0, &substituent_options());
        score_candidates(&mut candidates);
        let ranked = rank_candidates(candidates);

        let frontier = next_frontier(&ranked, 2);

        assert_eq!(frontier.len(), 2);
        assert!(frontier[0].score >= frontier[1].score);
    }
}
