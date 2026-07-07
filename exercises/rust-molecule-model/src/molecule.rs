#![allow(dead_code)]

use std::collections::VecDeque;
use std::fmt;

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

impl Element {
    pub fn symbol(self) -> &'static str {
        match self {
            Element::H => "H",
            Element::C => "C",
            Element::N => "N",
            Element::O => "O",
            Element::F => "F",
            Element::Cl => "Cl",
            Element::Br => "Br",
        }
    }

    pub fn sort_key(self) -> usize {
        match self {
            Element::C => 0,
            Element::H => 1,
            Element::N => 2,
            Element::O => 3,
            Element::F => 4,
            Element::Cl => 5,
            Element::Br => 6,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
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

    pub fn adjacency_list(&self) -> Vec<Vec<usize>> {
        let mut adjacency = vec![Vec::new(); self.atoms.len()];

        for bond in &self.bonds {
            if bond.from < self.atoms.len() && bond.to < self.atoms.len() {
                adjacency[bond.from].push(bond.to);
                adjacency[bond.to].push(bond.from);
            }
        }

        adjacency
    }

    pub fn formula(&self) -> String {
        let mut counts: Vec<(Element, usize)> = Vec::new();

        for atom in &self.atoms {
            match counts.iter_mut().find(|(element, _)| *element == atom.element) {
                Some((_, count)) => *count += 1,
                None => counts.push((atom.element, 1)),
            }
        }

        counts.sort_by_key(|(element, _)| element.sort_key());

        counts
            .into_iter()
            .map(|(element, count)| {
                if count == 1 {
                    element.to_string()
                } else {
                    format!("{element}{count}")
                }
            })
            .collect()
    }

    pub fn shortest_path(&self, start: usize, goal: usize) -> Option<Vec<usize>> {
        if start >= self.atoms.len() || goal >= self.atoms.len() {
            return None;
        }

        let adjacency = self.adjacency_list();
        let mut visited = vec![false; self.atoms.len()];
        let mut previous = vec![None; self.atoms.len()];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if current == goal {
                break;
            }

            for &neighbor in &adjacency[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    previous[neighbor] = Some(current);
                    queue.push_back(neighbor);
                }
            }
        }

        if !visited[goal] {
            return None;
        }

        let mut path = Vec::new();
        let mut current = goal;
        path.push(current);

        while current != start {
            current = previous[current]?;
            path.push(current);
        }

        path.reverse();
        Some(path)
    }

    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let adjacency = self.adjacency_list();
        let mut visited = vec![false; self.atoms.len()];
        let mut components = Vec::new();

        for start in 0..self.atoms.len() {
            if visited[start] {
                continue;
            }

            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            visited[start] = true;
            queue.push_back(start);

            while let Some(current) = queue.pop_front() {
                component.push(current);

                for &neighbor in &adjacency[current] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }

            components.push(component);
        }

        components
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

    fn ethanol() -> Molecule {
        Molecule::new(
            vec![
                Atom::neutral(Element::C),
                Atom::neutral(Element::C),
                Atom::neutral(Element::O),
                Atom::neutral(Element::H),
                Atom::neutral(Element::H),
                Atom::neutral(Element::H),
                Atom::neutral(Element::H),
                Atom::neutral(Element::H),
                Atom::neutral(Element::H),
            ],
            vec![
                Bond::new(0, 1, BondOrder::Single),
                Bond::new(1, 2, BondOrder::Single),
                Bond::new(0, 3, BondOrder::Single),
                Bond::new(0, 4, BondOrder::Single),
                Bond::new(0, 5, BondOrder::Single),
                Bond::new(1, 6, BondOrder::Single),
                Bond::new(1, 7, BondOrder::Single),
                Bond::new(2, 8, BondOrder::Single),
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

    #[test]
    fn builds_adjacency_list() {
        let molecule = water();

        assert_eq!(molecule.adjacency_list(), vec![vec![1, 2], vec![0], vec![0]]);
    }

    #[test]
    fn writes_formula() {
        assert_eq!(water().formula(), "H2O");
        assert_eq!(ethanol().formula(), "C2H6O");
    }

    #[test]
    fn finds_shortest_path() {
        let molecule = ethanol();

        assert_eq!(molecule.shortest_path(3, 8), Some(vec![3, 0, 1, 2, 8]));
        assert_eq!(molecule.shortest_path(3, 99), None);
    }

    #[test]
    fn finds_connected_components() {
        let fragments = Molecule::new(
            vec![
                Atom::neutral(Element::O),
                Atom::neutral(Element::H),
                Atom::neutral(Element::C),
            ],
            vec![Bond::new(0, 1, BondOrder::Single)],
        );

        assert_eq!(fragments.connected_components(), vec![vec![0, 1], vec![2]]);
    }
}
