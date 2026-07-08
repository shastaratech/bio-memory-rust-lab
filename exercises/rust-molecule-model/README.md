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
- DNA bases, codons, slices, and mutation edits in `src/dna.rs`
- scaffold design, candidate scoring, hash maps, ranking, and frontiers in
  `src/design.rs`
- molecule library records, hash indexes, formula buckets, sorted lists, merge,
  and binary search in `src/index.rs`
- toy molecular fingerprints, bitsets, shared/union counts, and similarity search
  in `src/fingerprint.rs`
- screening predictions, thresholds, optional labels, confusion matrices, queues,
  and feedback loops in `src/screening.rs`
- reproducible lab records, serialization, parsing, round trips, and report rows
  in `src/lab_record.rs`
- typed concentrations, unit conversion, assay observations, validation, sorting,
  and mean response in `src/measurement.rs`
- replicate response series, variance, standard deviation, coefficient of
  variation, and consistency checks in `src/replicate.rs`

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

Library-only exercises:

| Module | Meaning |
| --- | --- |
| `dna` | typed DNA sequence representation and mutation operations |
| `design` | toy scaffold/substituent candidate generation, scoring, and ranking |
| `index` | toy molecule library indexing, sorting, merging, and range search |
| `fingerprint` | toy feature bitsets and similarity ranking |
| `screening` | score thresholds, labels, confusion matrices, and feedback queues |
| `lab_record` | toy record serialization, parsing, validation, and Markdown output |
| `measurement` | typed units, assay observations, validation, and normalized sorting |
| `replicate` | repeated response summaries and uncertainty-aware consistency checks |
