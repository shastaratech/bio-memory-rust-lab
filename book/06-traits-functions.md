# Chapter 06: Traits and Functions for Molecular Behavior

## Big Idea

Types describe what data is. Functions describe what questions we can ask. Traits
describe shared behavior.

In this chapter, molecule-like data can:

- describe itself
- produce a formula
- behave like a graph

## Read First

- Lesson: [Traits and Functions for Molecular Behavior](../lessons/06-traits-functions-molecular-behavior.md)
- Visual: [Traits and Functions](../visuals/mermaid/traits-and-functions.md)
- Visual: [PlantUML Traits](../visuals/plantuml/traits-and-functions.puml)

## Open These Files

```text
exercises/rust-molecule-model/src/molecule.rs
exercises/rust-molecule-model/src/main.rs
```

Find these traits:

```rust
pub trait Describe
pub trait ChemicalFormula
pub trait MolecularGraph
```

## Trait 1: Describe

```rust
pub trait Describe {
    fn describe(&self) -> String;
}
```

`Atom`, `Bond`, and `Molecule` implement this trait.

Run:

```bash
cd exercises/rust-molecule-model
cargo run -- water summary
cargo run -- water atoms
cargo run -- water bonds
```

You are seeing `describe()` behavior in the CLI output.

## Trait 2: ChemicalFormula

```rust
pub trait ChemicalFormula {
    fn formula_counts(&self) -> Vec<(Element, usize)>;

    fn formula(&self) -> String {
        ...
    }
}
```

This trait requires `formula_counts()` and provides a default `formula()` method.

That is a powerful Rust idea:

> A trait can define required behavior and shared default behavior.

## Trait 3: MolecularGraph

```rust
pub trait MolecularGraph {
    fn atom_count(&self) -> usize;
    fn bond_count(&self) -> usize;
    fn neighbors(&self, atom_id: usize) -> Vec<usize>;
    fn shortest_path(&self, start: usize, goal: usize) -> Option<Vec<usize>>;
}
```

This lets algorithms accept any type that behaves like a molecular graph.

## Run Graph Commands

```bash
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
cargo run -- ethanol components
```

## Code Reading

Find the test:

```rust
fn can_use_molecular_graph_trait()
```

It contains:

```rust
fn degree(graph: &impl MolecularGraph, atom_id: usize) -> usize {
    graph.neighbors(atom_id).len()
}
```

This function does not require `&Molecule`. It accepts anything that implements
`MolecularGraph`.

## Try It

Add this helper inside the test:

```rust
fn is_isolated(graph: &impl MolecularGraph, atom_id: usize) -> bool {
    graph.neighbors(atom_id).is_empty()
}
```

Then test it with a molecule fragment that has an isolated atom.

## Checkpoint

1. What is a trait?
2. Which trait has a default method?
3. Why does `shortest_path` return `Option<Vec<usize>>`?
4. Why might a function accept `&impl MolecularGraph` instead of `&Molecule`?

