# Bio Memory Rust Lab

Bio Memory Rust Lab teaches Rust and computer science through molecular
structure. Students model atoms, bonds, molecules, memory, graph algorithms, and
quantum state with careful analogies and runnable Rust exercises.

## Audience

This lab is for students who are beginning Rust, molecular informatics, or
computer science. It does not assume production chemistry software experience.

## Start Here

1. Pick a path in the [course map](course/README.md).
2. For younger students, start with the [school track](course/school-track.md).
3. For university students, start with the [university track](course/university-track.md).
4. Read [Lesson 01: Atoms, Memory, and Types](lessons/01-atoms-memory-types.md).
5. Use the [student handout](handouts/atoms-memory-rust.md) during class.
6. Try the [exercise guide](exercises/README.md).
7. Compile the runnable Rust exercise:

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

## Lesson Path

| Lesson | School version | University version |
| --- | --- | --- |
| [01: Atoms, Memory, and Types](lessons/01-atoms-memory-types.md) | Build a molecule out of cards, then turn it into Rust. | Type design, ownership, and representation invariants. |
| [02: Molecules as Graphs](lessons/02-molecules-as-graphs.md) | Students become atoms in a human graph. | Adjacency lists, traversal, connected components, and paths. |
| [03: Ownership and Borrowing Through Molecule Design](lessons/03-ownership-borrowing-design.md) | "Who owns the molecule?" lab roles and safe sharing. | Ownership, borrowing, mutation, clone cost, and API design. |
| [04: Quantum State and Molecular Simulation](lessons/04-quantum-state-and-simulation.md) | Wave cards and measurement games. | Amplitudes, Hamiltonians, VQE intuition, and simulation limits. |

## Classroom Energy

The [activity pack](activities/classroom-games.md) turns abstract ideas into
movement: atom cards, bond strings, a human stack and heap, graph traversal races,
and a quantum measurement game. The point is not to make chemistry cute; it is to
give students a body-level memory of why structure, constraints, and state matter.

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
