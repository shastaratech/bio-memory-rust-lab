# Bio Memory Rust Lab

Bio Memory Rust Lab teaches Rust and computer science through molecular
structure. Students model atoms, bonds, molecules, memory, graph algorithms, and
quantum state with careful analogies and runnable Rust exercises.

## Audience

This lab is for students who are beginning Rust, molecular informatics, or
computer science. It does not assume production chemistry software experience.

## Start Here

1. Read [Lesson 01: Atoms, Memory, and Types](lessons/01-atoms-memory-types.md).
2. Use the [student handout](handouts/atoms-memory-rust.md) during class.
3. Try the [exercise guide](exercises/README.md).
4. Compile the runnable Rust exercise:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run
```

## What Students Learn

- Rust scalar and compound data types
- enums, structs, vectors, references, and borrowing
- stack and heap memory at an introductory level
- molecules as graphs
- molecule design as constrained search
- why quantum state is not ordinary computer memory

## Repository Policy

This is a public teaching repository.

Allowed:

- lesson markdown
- small toy molecule examples encoded directly in source code
- diagrams, worksheets, and instructor notes
- runnable Rust teaching exercises
- public references

Not allowed:

- credentials, tokens, or secrets
- raw PDB, SDF, PDBQT, MOL2, or docking result files
- unpublished assay data
- proprietary compound structures
- trained model weights or checkpoints

## Project Boards, Issues, and Wiki

Use GitHub issues for lesson tasks, student exercise improvements, and teaching
roadmap items. The wiki can hold instructor notes, classroom variants, and
semester-specific pages that should not interrupt the core lessons.

## License

MIT. See [LICENSE](LICENSE).

