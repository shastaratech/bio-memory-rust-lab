# Rust Molecule Model

This exercise turns atoms and bonds into Rust data types.

Run:

```bash
cargo test
cargo run -- list
cargo run -- water summary
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
```

Explore:

- `Element` as a constrained vocabulary
- `Atom` and `Bond` as structs
- `Molecule` as an owner of vectors
- borrowed reads through `&self`
- graph neighbors as a simple algorithm
- molecular formulas as a small formatting problem
- adjacency lists as a graph representation
- shortest paths and connected components
- traits as reusable molecular capabilities
- functions as contracts over borrowed molecule data

Commands:

| Command | Meaning |
| --- | --- |
| `list` | show toy molecules |
| `<molecule> summary` | show formula, atom count, bond count, validation |
| `<molecule> formula` | print formula |
| `<molecule> atoms` | print atoms |
| `<molecule> bonds` | print bonds |
| `<molecule> neighbors <atom_id>` | print bonded neighbors |
| `<molecule> path <start> <goal>` | print shortest path if one exists |
| `<molecule> components` | print connected components |
| `<molecule> validate` | check bond indices |
