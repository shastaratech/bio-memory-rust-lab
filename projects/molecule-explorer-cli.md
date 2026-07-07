# Project: Molecule Explorer CLI

## Goal

Build a command-line tool that lets students inspect toy molecules as Rust data
structures and graphs.

## Run It

```bash
cd exercises/rust-molecule-model
cargo run -- list
cargo run -- water summary
cargo run -- ethanol formula
cargo run -- ethanol atoms
cargo run -- ethanol bonds
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
cargo run -- ethanol components
```

## What It Teaches

| Command | Rust idea | Molecule idea |
| --- | --- | --- |
| `list` | fixed data source | known toy molecules |
| `summary` | functions and formatting | formula, atom count, bond count |
| `atoms` | `Vec<Atom>` iteration | atom records |
| `bonds` | `Vec<Bond>` iteration | bond records |
| `neighbors` | graph query | directly bonded atoms |
| `path` | `Option<Vec<usize>>` | maybe a route exists |
| `components` | graph traversal | separate fragments |
| `validate` | boolean validation | bond indices point to real atoms |

## Student Tasks

1. Add a new command called `degree`.
2. Add a new toy molecule.
3. Add tests for the new molecule's formula.
4. Add a command that prints formula counts.
5. Explain why `path` returns `no path` instead of crashing.

## University Extension

Refactor command handling into an enum:

```rust
enum Command {
    Summary,
    Formula,
    Atoms,
    Bonds,
    Neighbors(usize),
    Path { start: usize, goal: usize },
    Components,
    Validate,
}
```

Then write a parser function:

```rust
fn parse_command(args: &[String]) -> Result<Command, String>
```

This introduces `Result`, structured command parsing, and testable CLI logic.

