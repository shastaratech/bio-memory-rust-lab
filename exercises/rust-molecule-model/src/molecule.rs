#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    H,
    C,
    N,
    O,
    F,
    Cl,
    Br,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}

impl Atom {
    pub fn neutral(element: Element) -> Self {
        Self {
            element,
            formal_charge: 0,
            aromatic: false,
        }
    }

    pub fn charged(element: Element, formal_charge: i8) -> Self {
        Self {
            element,
            formal_charge,
            aromatic: false,
        }
    }

    pub fn aromatic(element: Element) -> Self {
        Self {
            element,
            formal_charge: 0,
            aromatic: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bond {
    pub from: usize,
    pub to: usize,
    pub order: BondOrder,
}

impl Bond {
    pub fn new(from: usize, to: usize, order: BondOrder) -> Self {
        Self { from, to, order }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

impl Molecule {
    pub fn new(atoms: Vec<Atom>, bonds: Vec<Bond>) -> Self {
        Self { atoms, bonds }
    }

    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }

    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }

    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
    }

    pub fn validate_bond_indices(&self) -> bool {
        self.bonds
            .iter()
            .all(|bond| bond.from < self.atoms.len() && bond.to < self.atoms.len())
    }

    pub fn neighbors(&self, atom_id: usize) -> Vec<usize> {
        let mut result = Vec::new();

        for bond in &self.bonds {
            if bond.from == atom_id {
                result.push(bond.to);
            } else if bond.to == atom_id {
                result.push(bond.from);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn water() -> Molecule {
        Molecule::new(
            vec![
                Atom::neutral(Element::O),
                Atom::neutral(Element::H),
                Atom::neutral(Element::H),
            ],
            vec![
                Bond::new(0, 1, BondOrder::Single),
                Bond::new(0, 2, BondOrder::Single),
            ],
        )
    }

    #[test]
    fn counts_atoms_and_bonds() {
        let molecule = water();

        assert_eq!(molecule.atom_count(), 3);
        assert_eq!(molecule.bond_count(), 2);
    }

    #[test]
    fn finds_neighbors() {
        let molecule = water();

        assert_eq!(molecule.neighbors(0), vec![1, 2]);
        assert_eq!(molecule.neighbors(1), vec![0]);
    }

    #[test]
    fn validates_bond_indices() {
        let valid = water();
        let invalid = Molecule::new(
            vec![Atom::neutral(Element::C)],
            vec![Bond::new(0, 4, BondOrder::Single)],
        );

        assert!(valid.validate_bond_indices());
        assert!(!invalid.validate_bond_indices());
    }
}
