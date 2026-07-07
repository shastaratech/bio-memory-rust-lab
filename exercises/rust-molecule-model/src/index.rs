use std::collections::HashMap;

use crate::molecule::{ethanol, methane, water, Molecule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoleculeRecord {
    pub id: String,
    pub common_name: String,
    pub molecule: Molecule,
}

impl MoleculeRecord {
    pub fn new(id: &str, common_name: &str, molecule: Molecule) -> Self {
        Self {
            id: id.to_string(),
            common_name: common_name.to_string(),
            molecule,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoleculeLibrary {
    records: Vec<MoleculeRecord>,
}

impl MoleculeLibrary {
    pub fn new(records: Vec<MoleculeRecord>) -> Self {
        Self { records }
    }

    pub fn toy_library() -> Self {
        Self::new(vec![
            MoleculeRecord::new("mol-001", "water", water()),
            MoleculeRecord::new("mol-002", "methane", methane()),
            MoleculeRecord::new("mol-003", "ethanol", ethanol()),
        ])
    }

    pub fn records(&self) -> &[MoleculeRecord] {
        &self.records
    }

    pub fn names_as_array_like_view(&self) -> Vec<&str> {
        self.records
            .iter()
            .map(|record| record.common_name.as_str())
            .collect()
    }

    pub fn lookup_by_name(&self) -> HashMap<String, MoleculeRecord> {
        self.records
            .iter()
            .map(|record| (record.common_name.clone(), record.clone()))
            .collect()
    }

    pub fn formula_index(&self) -> HashMap<String, Vec<String>> {
        let mut index: HashMap<String, Vec<String>> = HashMap::new();

        for record in &self.records {
            index
                .entry(record.molecule.formula())
                .or_default()
                .push(record.common_name.clone());
        }

        index
    }

    pub fn sorted_by_atom_count(&self) -> Vec<MoleculeRecord> {
        let mut sorted = self.records.clone();

        sorted.sort_by_key(|record| {
            (
                record.molecule.atom_count(),
                record.molecule.formula(),
                record.common_name.clone(),
            )
        });

        sorted
    }

    pub fn merge_by_atom_count(
        left: &[MoleculeRecord],
        right: &[MoleculeRecord],
    ) -> Vec<MoleculeRecord> {
        let mut merged = Vec::with_capacity(left.len() + right.len());
        let mut left_index = 0;
        let mut right_index = 0;

        while left_index < left.len() && right_index < right.len() {
            if atom_sort_key(&left[left_index]) <= atom_sort_key(&right[right_index]) {
                merged.push(left[left_index].clone());
                left_index += 1;
            } else {
                merged.push(right[right_index].clone());
                right_index += 1;
            }
        }

        merged.extend_from_slice(&left[left_index..]);
        merged.extend_from_slice(&right[right_index..]);
        merged
    }

    pub fn first_with_at_least_atoms(sorted_records: &[MoleculeRecord], min_atoms: usize) -> usize {
        let mut low = 0;
        let mut high = sorted_records.len();

        while low < high {
            let mid = low + (high - low) / 2;

            if sorted_records[mid].molecule.atom_count() < min_atoms {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low
    }
}

fn atom_sort_key(record: &MoleculeRecord) -> (usize, String, String) {
    (
        record.molecule.atom_count(),
        record.molecule.formula(),
        record.common_name.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_array_like_name_view() {
        let library = MoleculeLibrary::toy_library();

        assert_eq!(
            library.names_as_array_like_view(),
            vec!["water", "methane", "ethanol"]
        );
    }

    #[test]
    fn builds_name_lookup_hashmap() {
        let library = MoleculeLibrary::toy_library();
        let lookup = library.lookup_by_name();

        assert_eq!(lookup["ethanol"].molecule.formula(), "C2H6O");
        assert!(lookup.get("unknown").is_none());
    }

    #[test]
    fn groups_records_by_formula() {
        let library = MoleculeLibrary::new(vec![
            MoleculeRecord::new("mol-001", "water-a", water()),
            MoleculeRecord::new("mol-002", "water-b", water()),
            MoleculeRecord::new("mol-003", "methane", methane()),
        ]);

        let index = library.formula_index();

        assert_eq!(
            index["H2O"],
            vec!["water-a".to_string(), "water-b".to_string()]
        );
        assert_eq!(index["CH4"], vec!["methane".to_string()]);
    }

    #[test]
    fn sorts_records_by_atom_count() {
        let library = MoleculeLibrary::toy_library();
        let sorted = library.sorted_by_atom_count();

        assert_eq!(
            sorted
                .iter()
                .map(|record| record.common_name.as_str())
                .collect::<Vec<_>>(),
            vec!["water", "methane", "ethanol"]
        );
    }

    #[test]
    fn merges_two_sorted_record_lists() {
        let left = vec![
            MoleculeRecord::new("mol-001", "water", water()),
            MoleculeRecord::new("mol-003", "ethanol", ethanol()),
        ];
        let right = vec![MoleculeRecord::new("mol-002", "methane", methane())];

        let merged = MoleculeLibrary::merge_by_atom_count(&left, &right);

        assert_eq!(
            merged
                .iter()
                .map(|record| record.common_name.as_str())
                .collect::<Vec<_>>(),
            vec!["water", "methane", "ethanol"]
        );
    }

    #[test]
    fn finds_first_record_with_enough_atoms() {
        let sorted = MoleculeLibrary::toy_library().sorted_by_atom_count();

        assert_eq!(MoleculeLibrary::first_with_at_least_atoms(&sorted, 5), 1);
        assert_eq!(MoleculeLibrary::first_with_at_least_atoms(&sorted, 10), 3);
    }
}
