# Lesson 06: Traits and Functions for Molecular Behavior

## Goal

Students learn how Rust traits and functions describe what molecule-like data can
do: produce formulas, expose graph behavior, validate itself, and return readable
descriptions.

## Big Question

What should a molecule know how to do?

## Visuals

- [Traits and molecular behavior](../visuals/mermaid/traits-and-functions.md)
- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)

## Functions: Named Questions

A function is a named question or command.

```rust
pub fn atom_count(&self) -> usize {
    self.atoms.len()
}
```

Read this as:

> If I borrow this molecule, how many atoms does it contain?

More examples:

```rust
pub fn validate_bond_indices(&self) -> bool
pub fn neighbors(&self, atom_id: usize) -> Vec<usize>
pub fn shortest_path(&self, start: usize, goal: usize) -> Option<Vec<usize>>
```

Each function has a contract:

- what it needs
- what it returns
- whether it changes the molecule
- what happens when the answer may not exist

## `Option`: The Honest "Maybe"

Shortest path returns:

```rust
Option<Vec<usize>>
```

Why?

There may be no valid path, or the atom ID may not exist.

```rust
Some(vec![1, 0, 2])
None
```

Teaching line:

`Option` is Rust's way of making absence visible.

## Traits: Shared Capabilities

A trait says: "Any type that implements me can do these things."

```rust
pub trait Describe {
    fn describe(&self) -> String;
}
```

Now `Atom`, `Bond`, and `Molecule` can all describe themselves:

```rust
oxygen.describe()
bond.describe()
molecule.describe()
```

This is useful because different data types can share the same capability.

## Formula Trait

```rust
pub trait ChemicalFormula {
    fn formula_counts(&self) -> Vec<(Element, usize)>;

    fn formula(&self) -> String {
        // default behavior built from formula_counts
    }
}
```

This trait teaches an important Rust idea:

A trait can require one function and provide another.

For a molecule:

- `formula_counts` returns structured data
- `formula` turns that structure into text

## Graph Trait

```rust
pub trait MolecularGraph {
    fn atom_count(&self) -> usize;
    fn bond_count(&self) -> usize;
    fn neighbors(&self, atom_id: usize) -> Vec<usize>;
    fn shortest_path(&self, start: usize, goal: usize) -> Option<Vec<usize>>;
}
```

This lets a function accept anything that behaves like a molecular graph:

```rust
fn degree(graph: &impl MolecularGraph, atom_id: usize) -> usize {
    graph.neighbors(atom_id).len()
}
```

Teaching line:

Traits let us program against behavior, not only against one concrete struct.

## School Version

Use role cards:

- `Describe`: "Tell the class what you are."
- `ChemicalFormula`: "Count the element cards."
- `MolecularGraph`: "Name your neighbors and find a path."

Students can act out traits before seeing code.

## University Version

Ask:

- Which functions should be inherent methods on `Molecule`?
- Which functions belong in traits?
- Which trait methods should have default implementations?
- Should `MolecularGraph` return owned `Vec<usize>` or borrowed slices/iterators?
- When does beginner clarity beat performance?

## Lab

Run:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run
```

Find tests for:

- `formula_counts`
- `describe`
- `MolecularGraph`

Then add a new function:

```rust
fn is_isolated(graph: &impl MolecularGraph, atom_id: usize) -> bool {
    graph.neighbors(atom_id).is_empty()
}
```

## Reflection

Data types describe what a molecule is.

Functions describe what questions we can ask.

Traits describe shared behavior across types.

