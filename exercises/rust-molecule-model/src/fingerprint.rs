use crate::index::MoleculeRecord;
use crate::molecule::{Element, Molecule};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    HasCarbon,
    HasOxygen,
    HasHalogen,
    HasAtLeastFiveAtoms,
    HasAtLeastEightAtoms,
    HasBranchingAtom,
    HasOxygenHydrogenBond,
    HasCarbonCarbonBond,
}

impl Feature {
    pub fn label(self) -> &'static str {
        match self {
            Self::HasCarbon => "has carbon",
            Self::HasOxygen => "has oxygen",
            Self::HasHalogen => "has halogen",
            Self::HasAtLeastFiveAtoms => "has at least five atoms",
            Self::HasAtLeastEightAtoms => "has at least eight atoms",
            Self::HasBranchingAtom => "has branching atom",
            Self::HasOxygenHydrogenBond => "has oxygen-hydrogen bond",
            Self::HasCarbonCarbonBond => "has carbon-carbon bond",
        }
    }

    fn bit(self) -> u64 {
        1 << (self as u8)
    }
}

const FEATURES: [Feature; 8] = [
    Feature::HasCarbon,
    Feature::HasOxygen,
    Feature::HasHalogen,
    Feature::HasAtLeastFiveAtoms,
    Feature::HasAtLeastEightAtoms,
    Feature::HasBranchingAtom,
    Feature::HasOxygenHydrogenBond,
    Feature::HasCarbonCarbonBond,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToyFingerprint {
    bits: u64,
}

impl ToyFingerprint {
    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn from_molecule(molecule: &Molecule) -> Self {
        let mut fingerprint = Self::empty();

        if molecule
            .atoms()
            .iter()
            .any(|atom| atom.element == Element::C)
        {
            fingerprint.insert(Feature::HasCarbon);
        }

        if molecule
            .atoms()
            .iter()
            .any(|atom| atom.element == Element::O)
        {
            fingerprint.insert(Feature::HasOxygen);
        }

        if molecule
            .atoms()
            .iter()
            .any(|atom| matches!(atom.element, Element::F | Element::Cl | Element::Br))
        {
            fingerprint.insert(Feature::HasHalogen);
        }

        if molecule.atom_count() >= 5 {
            fingerprint.insert(Feature::HasAtLeastFiveAtoms);
        }

        if molecule.atom_count() >= 8 {
            fingerprint.insert(Feature::HasAtLeastEightAtoms);
        }

        if (0..molecule.atom_count()).any(|atom_id| molecule.neighbors(atom_id).len() >= 3) {
            fingerprint.insert(Feature::HasBranchingAtom);
        }

        if has_bond_between(molecule, Element::O, Element::H) {
            fingerprint.insert(Feature::HasOxygenHydrogenBond);
        }

        if has_bond_between(molecule, Element::C, Element::C) {
            fingerprint.insert(Feature::HasCarbonCarbonBond);
        }

        fingerprint
    }

    pub fn insert(&mut self, feature: Feature) {
        self.bits |= feature.bit();
    }

    pub fn contains(&self, feature: Feature) -> bool {
        self.bits & feature.bit() != 0
    }

    pub fn feature_labels(&self) -> Vec<&'static str> {
        FEATURES
            .iter()
            .filter(|&&feature| self.contains(feature))
            .map(|&feature| feature.label())
            .collect()
    }

    pub fn shared_count(&self, other: &Self) -> u32 {
        (self.bits & other.bits).count_ones()
    }

    pub fn union_count(&self, other: &Self) -> u32 {
        (self.bits | other.bits).count_ones()
    }

    pub fn similarity(&self, other: &Self) -> f64 {
        let union_count = self.union_count(other);

        if union_count == 0 {
            1.0
        } else {
            self.shared_count(other) as f64 / union_count as f64
        }
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FingerprintMatch {
    pub record_id: String,
    pub common_name: String,
    pub similarity: f64,
}

pub fn rank_by_similarity(query: &Molecule, records: &[MoleculeRecord]) -> Vec<FingerprintMatch> {
    let query_fingerprint = ToyFingerprint::from_molecule(query);
    let mut matches: Vec<FingerprintMatch> = records
        .iter()
        .map(|record| {
            let record_fingerprint = ToyFingerprint::from_molecule(&record.molecule);

            FingerprintMatch {
                record_id: record.id.clone(),
                common_name: record.common_name.clone(),
                similarity: query_fingerprint.similarity(&record_fingerprint),
            }
        })
        .collect();

    matches.sort_by(|left, right| {
        right
            .similarity
            .partial_cmp(&left.similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.common_name.cmp(&right.common_name))
    });

    matches
}

fn has_bond_between(molecule: &Molecule, first: Element, second: Element) -> bool {
    molecule.bonds().iter().any(|bond| {
        let from = molecule.atoms()[bond.from].element;
        let to = molecule.atoms()[bond.to].element;

        (from == first && to == second) || (from == second && to == first)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::MoleculeLibrary;
    use crate::molecule::{ethanol, methane, water};

    #[test]
    fn builds_fingerprint_from_molecule_features() {
        let fingerprint = ToyFingerprint::from_molecule(&ethanol());

        assert!(fingerprint.contains(Feature::HasCarbon));
        assert!(fingerprint.contains(Feature::HasOxygen));
        assert!(fingerprint.contains(Feature::HasAtLeastEightAtoms));
        assert!(fingerprint.contains(Feature::HasOxygenHydrogenBond));
        assert!(fingerprint.contains(Feature::HasCarbonCarbonBond));
        assert!(!fingerprint.contains(Feature::HasHalogen));
    }

    #[test]
    fn exposes_feature_labels_for_students() {
        let labels = ToyFingerprint::from_molecule(&water()).feature_labels();

        assert_eq!(labels, vec!["has oxygen", "has oxygen-hydrogen bond"]);
    }

    #[test]
    fn compares_fingerprints_with_shared_over_union_similarity() {
        let water_fingerprint = ToyFingerprint::from_molecule(&water());
        let ethanol_fingerprint = ToyFingerprint::from_molecule(&ethanol());

        assert_eq!(water_fingerprint.shared_count(&ethanol_fingerprint), 2);
        assert_eq!(water_fingerprint.union_count(&ethanol_fingerprint), 7);
        assert!((water_fingerprint.similarity(&ethanol_fingerprint) - 0.286).abs() < 0.01);
    }

    #[test]
    fn ranks_library_records_by_similarity_to_query() {
        let library = MoleculeLibrary::toy_library();
        let matches = rank_by_similarity(&ethanol(), library.records());

        assert_eq!(matches[0].common_name, "ethanol");
        assert_eq!(matches[0].similarity, 1.0);
        assert_eq!(matches[1].common_name, "methane");
        assert_eq!(matches[2].common_name, "water");
    }

    #[test]
    fn methane_and_water_have_no_shared_toy_features() {
        let methane_fingerprint = ToyFingerprint::from_molecule(&methane());
        let water_fingerprint = ToyFingerprint::from_molecule(&water());

        assert_eq!(methane_fingerprint.shared_count(&water_fingerprint), 0);
        assert_eq!(methane_fingerprint.similarity(&water_fingerprint), 0.0);
    }
}
