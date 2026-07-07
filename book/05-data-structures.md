# Chapter 05: Rust Data Structures for Molecules

## Big Idea

Data structures are choices. Each choice makes some questions easy and other
questions harder.

For molecules, we use:

- enums for controlled vocabularies
- structs for records
- vectors for ordered collections
- graph views for connectivity
- counts for formulas

## Read First

- Lesson: [Rust Data Structures for Molecules](../lessons/05-rust-data-structures-for-molecules.md)
- Visual: [Molecule Data Model](../visuals/plantuml/molecule-data-model.puml)
- Visual: [Models, Contracts, and Flows](../visuals/mermaid/model-contracts-flows.md)

## Open These Files

```text
exercises/rust-molecule-model/src/molecule.rs
visuals/mermaid/model-contracts-flows.md
```

## Representation Choices

### Element

```rust
pub enum Element
```

Use an enum because the set of supported toy elements is fixed.

### Atom

```rust
pub struct Atom
```

Use a struct because atom facts belong together.

### Molecule

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

Use vectors because molecules can have different numbers of atoms and bonds.

### Formula Counts

```rust
Vec<(Element, usize)>
```

Use pairs because each element has a count.

## Run Formula Commands

```bash
cd exercises/rust-molecule-model
cargo run -- water formula
cargo run -- methane formula
cargo run -- ethanol formula
```

Expected output:

```text
H2O
CH4
C2H6O
```

## Code Reading

Find:

```rust
pub fn formula_counts(&self) -> Vec<(Element, usize)>
```

This function turns a list of atoms into element counts.

Then find:

```rust
pub fn formula(&self) -> String
```

This function turns structured counts into text.

## Try It

Add a new toy molecule:

```rust
pub fn ammonia() -> Molecule
```

Use:

- one nitrogen
- three hydrogens
- three single bonds from nitrogen to hydrogen

Then update:

```rust
pub fn molecule_by_name(name: &str) -> Option<Molecule>
pub fn molecule_names() -> &'static [&'static str]
```

Run:

```bash
cargo run -- ammonia summary
cargo test
```

## Checkpoint

1. Why is `Vec<Atom>` better than a fixed array for this course?
2. What does `Vec<(Element, usize)>` represent?
3. What is the difference between storage representation and graph view?
4. Which structure would you use for fast repeated neighbor lookup?

