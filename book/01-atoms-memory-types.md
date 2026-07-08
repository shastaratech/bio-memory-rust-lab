# Chapter 01: Atoms, Memory, and Types

## Big Idea

A type is a rule for what kind of value something is. A molecule is full of typed
facts: element, charge, aromaticity, atom count, bond order, and formula.

Rust makes those rules explicit.

Two beginner chemistry terms matter immediately:

- A bond is a chemical connection between atoms. In a program, we represent it as a
  graph edge between two atom IDs.
- Valence is the number of bonds an atom commonly forms. In a graph analogy, it is
  like a limit on how many weighted edges an atom node may have.
- Aromaticity is a special delocalized bonding pattern in some rings. In this toy
  model, we represent it with a boolean atom flag or an aromatic bond order.

If `H2O` tells us "two hydrogen atoms and one oxygen atom," Rust types help us say
what a hydrogen atom, oxygen atom, bond, and molecule are inside a program.

## Read First

- Primer: [Atoms, Elements, Bonds, Molecules, And Formulas](00-chemistry-primer.md)
- Bridge: [How Formulas Become Graphs](00b-how-formulas-become-graphs.md)
- Lesson: [Atoms, Memory, and Types](../lessons/01-atoms-memory-types.md)
- Handout: [Student Handout](../handouts/atoms-memory-rust.md)
- Visual: [Course Map](../visuals/mermaid/course-map.md)

## Open These Files

```text
exercises/rust-molecule-model/src/molecule.rs
exercises/rust-molecule-model/src/main.rs
```

Find these definitions:

```rust
pub enum Element
pub enum BondOrder
pub struct Atom
pub struct Bond
pub struct Molecule
```

## Concept Walkthrough

An atom has properties:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

This says:

- `element` must be one of the supported element variants.
- `formal_charge` is a signed integer because charge can be negative or positive.
- `aromatic` is a boolean because it is true or false in this toy model.

The element is an enum:

```rust
pub enum Element {
    H,
    C,
    N,
    O,
    F,
    Cl,
    Br,
}
```

An enum prevents misspellings such as `"cl"`, `"CL"`, or `"chlorene"` inside the
clean Rust model.

A bond is also typed:

```rust
pub enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}
```

This does not mean the program fully understands quantum chemistry. It means the
toy model has enough vocabulary to distinguish a single bond from a double bond or
an aromatic connection.

## Compile And Run

From the repo root:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- water atoms
```

Expected output includes:

```text
0: O atom (neutral)
1: H atom (neutral)
2: H atom (neutral)
```

Now inspect bonds:

```bash
cargo run -- water bonds
```

Expected output:

```text
0: single bond: atom 0 <-> atom 1
1: single bond: atom 0 <-> atom 2
```

## Try It

Open:

```text
exercises/rust-molecule-model/src/molecule.rs
```

Find:

```rust
pub fn methane() -> Molecule
```

Read the atom list and bond list. Then run:

```bash
cargo run -- methane summary
cargo run -- methane atoms
cargo run -- methane bonds
```

## Checkpoint

1. Why is `Element` an enum instead of a string?
2. Why is `formal_charge` signed?
3. What does `Atom::neutral(Element::O)` create?
4. Which atom ID is oxygen in water?
