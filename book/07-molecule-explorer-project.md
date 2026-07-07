# Chapter 07: Molecule Explorer CLI Project

## Big Idea

The Molecule Explorer CLI turns the course model into a tool students can run.

It is small enough to understand, but real enough to teach:

- command-line input
- library plus binary structure
- molecule lookup
- graph queries
- error messages
- project extension

## Read First

- Project: [Molecule Explorer CLI](../projects/molecule-explorer-cli.md)
- Visual: [CLI Flow](../visuals/mermaid/cli-flow.md)
- Code: [`src/main.rs`](../exercises/rust-molecule-model/src/main.rs)
- Code: [`src/lib.rs`](../exercises/rust-molecule-model/src/lib.rs)

## Library And CLI Split

The crate now has:

```text
src/lib.rs
src/main.rs
src/molecule.rs
```

`lib.rs` exports reusable code:

```rust
pub mod molecule;
```

`main.rs` handles command-line arguments and prints output.

`molecule.rs` contains the data model, traits, algorithms, and toy molecules.

## Run All Main Commands

From:

```bash
cd exercises/rust-molecule-model
```

Run:

```bash
cargo run -- list
cargo run -- water summary
cargo run -- methane formula
cargo run -- ethanol atoms
cargo run -- ethanol bonds
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
cargo run -- ethanol components
cargo run -- ethanol validate
```

## Expected Patterns

`list`:

```text
water, methane, ethanol
```

`water summary`:

```text
water: H2O molecule with 3 atoms and 2 bonds
formula: H2O
atoms: 3
bonds: 2
valid bond indices: true
```

`ethanol path 3 8`:

```text
[3, 0, 1, 2, 8]
```

## Code Reading

Open:

```text
exercises/rust-molecule-model/src/main.rs
```

Find:

```rust
let args: Vec<String> = std::env::args().skip(1).collect();
```

This collects command-line arguments after the program name.

Then find:

```rust
match command
```

This is where the CLI chooses what to do.

## Project Task 1: Add `degree`

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

## Project Task 2: Add A New Molecule

Add ammonia:

- formula: `H3N`
- atoms: one nitrogen and three hydrogens
- bonds: nitrogen connected to each hydrogen

Update:

```rust
molecule_by_name
molecule_names
```

Run:

```bash
cargo run -- ammonia summary
cargo test
```

## Project Task 3: Better Command Parsing

University extension:

Replace raw string matching with:

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

Then write:

```rust
fn parse_command(args: &[String]) -> Result<Command, String>
```

This makes command parsing testable.

## Checkpoint

1. What is the difference between `lib.rs` and `main.rs`?
2. Which file stores the molecule algorithms?
3. How does the CLI choose a molecule by name?
4. What command would you add next?

