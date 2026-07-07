# Worksheet: University CLI Extension

## Goal

Extend the Molecule Explorer CLI while keeping command parsing testable.

## Starting Point

Open:

```text
exercises/rust-molecule-model/src/main.rs
exercises/rust-molecule-model/src/molecule.rs
```

Run:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- list
cargo run -- ethanol summary
```

## Part 1: Current Command Surface

Fill in the behavior behind each command.

| Command | Function or method used | Return shape |
| --- | --- | --- |
| `summary` | | |
| `formula` | | |
| `atoms` | | |
| `bonds` | | |
| `neighbors` | | |
| `path` | | |
| `components` | | |
| `validate` | | |

## Part 2: Add `degree`

Target command:

```bash
cargo run -- ethanol degree 1
```

Expected output:

```text
4
```

Implementation hint:

```rust
molecule.neighbors(atom_id).len()
```

Test the command manually:

```bash
cargo run -- water degree 0
cargo run -- methane degree 0
cargo run -- ethanol degree 1
```

## Part 3: Add A Molecule

Add ammonia:

- formula: `H3N`
- atom 0: nitrogen
- atoms 1, 2, 3: hydrogen
- bonds: `0-1`, `0-2`, `0-3`

Update:

```rust
molecule_by_name
molecule_names
```

Add a formula test:

```rust
assert_eq!(ammonia().formula(), "H3N");
```

## Part 4: Refactor Command Parsing

Design this enum:

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
    Degree(usize),
}
```

Then design this parser:

```rust
fn parse_command(args: &[String]) -> Result<Command, String>
```

Write three parser tests:

| Input args | Expected result |
| --- | --- |
| `["summary"]` | |
| `["neighbors", "1"]` | |
| `["path", "3", "8"]` | |

## Part 5: Error Design

For each input, choose the best result.

| Input | Panic | `Err(String)` | Printed message | Why? |
| --- | --- | --- | --- | --- |
| unknown molecule | | | | |
| unknown command | | | | |
| missing atom ID | | | | |
| atom ID is not a number | | | | |
| no path exists | | | | |

## Final Check

Run:

```bash
cargo fmt --check
cargo test
cargo run -- ammonia summary
cargo run -- ethanol degree 1
```

## Reflection

Answer in three sentences:

1. What belongs in the library?
2. What belongs in the binary?
3. What did the CLI extension teach about scientific model boundaries?
