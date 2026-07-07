# Lesson 05: Rust Data Structures for Molecules

## Goal

Students learn how to choose Rust data structures for molecular information:
enums for controlled vocabulary, structs for records, vectors for collections, and
graphs for atom-bond connectivity.

## Big Question

If a molecule becomes data, what shape should the data have?

## Visuals

- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)
- [Models, contracts, and flows](../visuals/mermaid/model-contracts-flows.md)

## Layer 1: Element as an Enum

Use an enum when the allowed values are known.

```rust
pub enum Element {
    H,
    C,
    N,
    O,
    Cl,
}
```

Why not a string?

| String | Enum |
| --- | --- |
| `"cl"`, `"CL"`, and `"chlorine"` can all appear | `Element::Cl` has one spelling |
| mistakes appear at runtime | many mistakes fail at compile time |
| useful for messy input files | useful for internal clean models |

Teaching line:

An enum is a vocabulary fence. It says, "These are the element names this toy model
understands."

## Layer 2: Atom as a Struct

Use a struct when facts belong together.

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

This says:

- every atom has an element
- every atom has a formal charge
- every atom has an aromatic flag

School prompt:

Give students atom cards and ask them to fill these fields.

University prompt:

Which fields should be public? Which should require constructor functions?

## Layer 3: Bond as a Relation

```rust
pub struct Bond {
    pub from: usize,
    pub to: usize,
    pub order: BondOrder,
}
```

A bond connects two atom indices. It does not store Rust references to atoms.

Why indices?

- simple to print and serialize
- avoids self-referential Rust structures
- works naturally with graph algorithms
- can be validated with `validate_bond_indices`

Important limitation:

An index can point outside the atom list. That is why validation matters.

## Layer 4: Molecule Owns Collections

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

The molecule owns its atoms and bonds.

Use `Vec<T>` because molecules have variable size:

- water has 3 atoms
- methane has 5 atoms
- ethanol has 9 atoms in this toy explicit-hydrogen model

## Layer 5: Graph Views

The same molecule can be viewed as:

```rust
Vec<Atom> + Vec<Bond>
```

or as:

```rust
Vec<Vec<usize>>
```

The first is a storage model. The second is an adjacency list.

| Question | Helpful structure |
| --- | --- |
| What atoms exist? | `Vec<Atom>` |
| What bonds exist? | `Vec<Bond>` |
| Who are atom 3's neighbors? | adjacency list |
| Is there a path from atom 1 to atom 8? | adjacency list + BFS |
| What is the formula? | element counts |

## Code Walkthrough

Open:

```text
exercises/rust-molecule-model/src/molecule.rs
```

Find:

- `Element`
- `Atom`
- `Bond`
- `Molecule`
- `formula_counts`
- `adjacency_list`
- `shortest_path`

## Activity: Pick the Structure

For each task, choose a Rust structure.

| Task | Good choice | Why |
| --- | --- | --- |
| Store atom element | enum | controlled values |
| Store atom properties | struct | grouped fields |
| Store all atoms | `Vec<Atom>` | variable-size list |
| Store bond order | enum | limited set |
| Store connectivity | `Vec<Bond>` or adjacency list | graph data |
| Store formula counts | vector of pairs or map | element to count |

## Reflection

A data structure is not just storage. It is a claim about what questions should be
easy to answer.

