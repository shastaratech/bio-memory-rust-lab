# Lesson 07: Molecule Explorer CLI Capstone

## Goal

Students turn the molecule model into a small command-line tool, then explain how
each command maps to Rust data, graph algorithms, and molecular questions.

## Big Question

How does a model become a tool someone can run?

## Visuals

- [Molecule Explorer CLI flow](../visuals/mermaid/cli-flow.md)
- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)
- [Graph algorithms](../visuals/mermaid/graph-algorithms.md)
- [Traits and molecular behavior](../visuals/mermaid/traits-and-functions.md)

## Readiness Check

Students should already know:

- `Molecule` stores `Vec<Atom>` and `Vec<Bond>`
- bonds store atom indices
- `neighbors` asks a graph question
- `shortest_path` returns `Option<Vec<usize>>`
- traits describe reusable behavior

If students are still shaky on those ideas, use Lesson 06 before this capstone.

## Project Frame

The CLI has two jobs:

1. Read a command from the terminal.
2. Ask the molecule model a clear question.

Run:

```bash
cd exercises/rust-molecule-model
cargo run -- list
cargo run -- water summary
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
```

Teaching line:

The CLI is not a new molecule model. It is a user interface wrapped around the
model students already built.

## Command Map

| Command | Code idea | Molecule question |
| --- | --- | --- |
| `list` | known data source | Which toy molecules are available? |
| `summary` | formatting and methods | What is the quick description? |
| `formula` | trait default method | What elements are counted? |
| `atoms` | iterate `Vec<Atom>` | What atom records are stored? |
| `bonds` | iterate `Vec<Bond>` | Which atom indices are connected? |
| `neighbors` | graph query | Which atoms touch this atom? |
| `path` | `Option<Vec<usize>>` | Is there a route between two atoms? |
| `components` | traversal | Is the molecule one connected piece? |
| `validate` | boolean check | Do all bonds point to real atoms? |

## School Version

Run commands as predictions:

1. Students choose a command card.
2. They predict the output before running it.
3. One student acts as the terminal.
4. Other students act as atoms and bonds.
5. The class checks whether the output matches the model.

Good commands for this version:

```bash
cargo run -- water summary
cargo run -- methane atoms
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
```

Keep the extension small: add `degree`, which prints the number of neighbors for
one atom.

## University Version

Treat the CLI as an API boundary.

Ask:

- Which errors belong in command parsing?
- Which errors belong in molecule algorithms?
- Why does `path` print `no path` instead of panicking?
- What should be tested in the library instead of the binary?
- When should command strings become a `Command` enum?

Refactor target:

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

This makes command parsing testable without running the whole binary.

## Lab

Open:

```text
exercises/rust-molecule-model/src/main.rs
exercises/rust-molecule-model/src/molecule.rs
```

Tasks:

1. Run every existing command.
2. Add a `degree` command.
3. Add ammonia as a new toy molecule.
4. Add at least one test for the new molecule.
5. Explain one thing the toy model leaves out.

Suggested `degree` behavior:

```bash
cargo run -- ethanol degree 1
```

Expected output:

```text
4
```

Useful implementation hint:

```rust
molecule.neighbors(atom_id).len()
```

## Presentation

Each student or team should present:

1. One command they ran.
2. The Rust function or method behind it.
3. The molecule question it answers.
4. One invalid input the program handles.
5. One scientific detail the model does not represent.

## Assessment

Strong work shows:

- commands run successfully
- output matches the molecule data
- new code is small and readable
- at least one test covers the extension
- students can explain the boundary between chemistry analogy and Rust behavior

## Reflection

A useful scientific tool is not just code that runs.

It is a model, a set of questions, a user interface, and an honest statement about
what the model does not know.
