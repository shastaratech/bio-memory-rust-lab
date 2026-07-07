mod molecule;

use molecule::{Atom, Bond, BondOrder, Element, Molecule};

fn main() {
    let water = Molecule::new(
        vec![
            Atom::neutral(Element::O),
            Atom::neutral(Element::H),
            Atom::neutral(Element::H),
        ],
        vec![
            Bond::new(0, 1, BondOrder::Single),
            Bond::new(0, 2, BondOrder::Single),
        ],
    );

    println!("water atoms: {}", water.atom_count());
    println!("water bonds: {}", water.bond_count());
    println!("water formula: {}", water.formula());
    println!("oxygen neighbors: {:?}", water.neighbors(0));
    println!("water adjacency list: {:?}", water.adjacency_list());
    println!("path H -> H: {:?}", water.shortest_path(1, 2));
    println!("bond indices valid: {}", water.validate_bond_indices());
}
