# Exercises

## Exercise 1: Element Enum

Create an enum for the elements needed to represent water, methane, ethanol, and
coumarin at a simplified graph level.

Questions:

1. Which elements are required?
2. Which elements can you leave out for now?
3. Why is an enum safer than a free-form string?

## Exercise 2: Atom Struct

Create this Rust struct:

```rust
struct Atom {
    element: Element,
    formal_charge: i8,
    aromatic: bool,
}
```

Then create atoms for:

- neutral carbon
- neutral oxygen
- positively charged nitrogen
- aromatic carbon

## Exercise 3: Bond Validation

Write a function that checks whether every bond points to valid atom indices.

Starter:

```rust
fn validate_bond_indices(molecule: &Molecule) -> bool {
    for bond in &molecule.bonds {
        if bond.from >= molecule.atoms.len() || bond.to >= molecule.atoms.len() {
            return false;
        }
    }

    true
}
```

Reflection:

How is this like checking whether every bond in a molecular drawing points to a real
atom?

## Exercise 4: Neighbors

Write a function that returns all atoms directly bonded to a given atom.

Starter:

```rust
fn neighbors(molecule: &Molecule, atom_id: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for bond in &molecule.bonds {
        if bond.from == atom_id {
            result.push(bond.to);
        } else if bond.to == atom_id {
            result.push(bond.from);
        }
    }

    result
}
```

Extension:

Convert the molecule to an adjacency list:

```rust
type AdjacencyList = Vec<Vec<usize>>;
```

Then compare which representation is easier for repeated neighbor lookup.

## Exercise 5: Molecule Design as Search

Imagine a simple molecule-design loop:

1. Start with a scaffold.
2. Add or replace one substituent.
3. Reject invalid structures.
4. Score the remaining structures.
5. Keep the best candidates.
6. Repeat.

Map each step to an algorithmic idea:

| Design step | Algorithm idea |
| --- | --- |
| Scaffold | initial state |
| Substituent replacement | state transition |
| Chemistry rules | constraints |
| Docking/selectivity score | objective function |
| Keep best candidates | ranking / priority queue |
| Repeat | search loop |

Question:

Where would Rust types prevent bad data before the scoring step?

## Exercise 6: Quantum State Caution

Answer in two sentences:

1. Why is a quantum state not the same as a classical memory value?
2. Why are quantum computers interesting for molecular simulation?

