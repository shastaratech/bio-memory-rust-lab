# Student Handout: Molecule Explorer CLI

## Big Idea

The Molecule Explorer CLI lets you ask questions about toy molecules from the
terminal.

Each command connects three things:

- a Rust idea
- a molecule idea
- an output you can inspect

## Setup

From the repository root:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- list
```

You should see:

```text
water, methane, ethanol
```

## Run These Commands

```bash
cargo run -- water summary
cargo run -- methane formula
cargo run -- ethanol atoms
cargo run -- ethanol bonds
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
cargo run -- ethanol components
cargo run -- ethanol validate
```

## What Each Command Asks

| Command | Question |
| --- | --- |
| `list` | Which toy molecules are available? |
| `summary` | What is the molecule's formula, atom count, and bond count? |
| `formula` | What element counts make up the molecule? |
| `atoms` | What atom records are stored? |
| `bonds` | What atom indices are connected? |
| `neighbors` | Which atoms are directly bonded to this atom? |
| `path` | What route connects two atoms? |
| `components` | How many connected pieces are in the molecule graph? |
| `validate` | Do all bonds point to real atom indices? |

## Prediction Table

Fill this in before running the commands.

| Command | Prediction | Actual output |
| --- | --- | --- |
| `cargo run -- water formula` | | |
| `cargo run -- methane neighbors 0` | | |
| `cargo run -- ethanol neighbors 1` | | |
| `cargo run -- ethanol path 3 8` | | |
| `cargo run -- ethanol validate` | | |

## Code Hunt

Open:

```text
src/main.rs
src/molecule.rs
```

Find the code that:

1. Collects command-line arguments.
2. Prints the list of molecule names.
3. Chooses a molecule by name.
4. Matches on the command.
5. Finds neighbors.
6. Finds the shortest path.

## Extension 1: Add `degree`

Add a command:

```bash
cargo run -- ethanol degree 1
```

Expected output:

```text
4
```

Hint:

```rust
molecule.neighbors(atom_id).len()
```

Checkpoint:

Why does degree mean "number of neighbors" in a molecule graph?

## Extension 2: Add Ammonia

Add ammonia:

- formula: `H3N`
- atoms: one nitrogen and three hydrogens
- bonds: nitrogen connected to each hydrogen

Update the molecule lookup so this works:

```bash
cargo run -- ammonia summary
```

Then add a test for its formula.

## Reflection

Answer in two or three sentences:

1. Which command felt most like a Rust question?
2. Which command felt most like a chemistry question?
3. What real molecule information does this toy model leave out?
