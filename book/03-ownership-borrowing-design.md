# Chapter 03: Ownership and Borrowing Through Molecule Design

## Big Idea

Rust asks: who owns this value, who can read it, and who can change it?

That maps naturally to molecule design:

- one owner keeps the molecule data
- many readers can inspect it
- one editor can change it
- validation keeps broken molecules from moving forward

## Read First

- Lesson: [Ownership and Borrowing Through Molecule Design](../lessons/03-ownership-borrowing-design.md)
- Visual: [Ownership and Borrowing Contract](../visuals/plantuml/ownership-borrowing-contract.puml)
- Visual: [Molecule Design Flow](../visuals/plantuml/molecule-design-flow.puml)

## Open These Files

```text
exercises/rust-molecule-model/src/molecule.rs
exercises/rust-molecule-model/src/main.rs
```

Find methods that borrow:

```rust
pub fn atoms(&self) -> &[Atom]
pub fn bonds(&self) -> &[Bond]
pub fn validate_bond_indices(&self) -> bool
pub fn formula(&self) -> String
```

The `&self` means the method borrows the molecule. It reads data without taking
ownership.

## Run Validation

```bash
cd exercises/rust-molecule-model
cargo run -- water validate
cargo run -- ethanol validate
```

Expected output:

```text
true
true
```

The toy molecules have bonds that point to real atom IDs.

## Why Borrowing Matters

This function reads the molecule:

```rust
fn print_summary(name: &str, molecule: &Molecule)
```

It borrows `molecule`. It does not copy the molecule and does not destroy it.

That matters because algorithms often need to inspect the same molecule many times:

- formula calculation
- validation
- neighbor queries
- scoring
- path finding

## Code Reading

Open:

```text
exercises/rust-molecule-model/src/main.rs
```

Find:

```rust
fn print_summary(name: &str, molecule: &Molecule)
```

Then find each method call inside it:

```rust
molecule.describe()
molecule.formula()
molecule.atom_count()
molecule.bond_count()
molecule.validate_bond_indices()
```

Each call asks a question without taking ownership.

## Try It

Add a new summary line in `print_summary`:

```rust
println!("components: {:?}", molecule.connected_components());
```

Run:

```bash
cargo run -- ethanol summary
```

Then run tests:

```bash
cargo test
```

## Checkpoint

1. What does `&self` mean?
2. Why does `print_summary` take `&Molecule` instead of `Molecule`?
3. What does validation check in this toy model?
4. Why is validation not the same as full chemical correctness?

