# Bio Memory Rust Lab

Bio Memory Rust Lab teaches Rust and computer science through molecular
structure. Students model atoms, bonds, molecules, memory, graph algorithms, and
quantum state with careful analogies and runnable Rust exercises.

## Audience

This lab is for students who are beginning Rust, molecular informatics, or
computer science. It does not assume production chemistry software experience.

## Start Here

1. Start with the [self-study book](book/README.md).
2. Use [Setup](book/00-setup.md) to compile and run the project.
3. Pick a path in the [course map](course/README.md).
4. For younger students, start with the [school track](course/school-track.md).
5. For university students, start with the [university track](course/university-track.md).
6. Read [Lesson 01: Atoms, Memory, and Types](lessons/01-atoms-memory-types.md).
7. Use the [student handout](handouts/atoms-memory-rust.md) during class.
8. Try the [exercise guide](exercises/README.md).
9. Compile the runnable Rust exercise:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- water summary
```

10. Open the [visual language guide](visuals/README.md) and compare a diagram with
   the Rust code it describes.
11. Try the [Molecule Explorer CLI project](projects/molecule-explorer-cli.md).

## Self-Study Book

The [book](book/README.md) is the best entry point for students working alone. It
contains chapter-by-chapter reading, repo links, compile commands, run commands,
expected output, exercises, and checkpoint questions.

Book chapters:

- [Setup](book/00-setup.md)
- [01: Atoms, Memory, and Types](book/01-atoms-memory-types.md)
- [02: Molecules as Graphs](book/02-molecules-as-graphs.md)
- [03: Ownership and Borrowing](book/03-ownership-borrowing-design.md)
- [04: Quantum State and Simulation](book/04-quantum-state-simulation.md)
- [05: Data Structures](book/05-data-structures.md)
- [06: Traits and Functions](book/06-traits-functions.md)
- [07: Molecule Explorer CLI Project](book/07-molecule-explorer-project.md)
- [Command Reference](book/appendix-command-reference.md)

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
| [05: Rust Data Structures for Molecules](lessons/05-rust-data-structures-for-molecules.md) | Pick the right container for atom cards and bond strings. | Representation choices: enums, structs, vectors, graphs, and counts. |
| [06: Traits and Functions for Molecular Behavior](lessons/06-traits-functions-molecular-behavior.md) | Role cards for shared capabilities. | Traits, default methods, function contracts, and generic graph APIs. |

## Classroom Energy

The [activity pack](activities/classroom-games.md) turns abstract ideas into
movement: atom cards, bond strings, a human stack and heap, graph traversal races,
and a quantum measurement game. The point is not to make chemistry cute; it is to
give students a body-level memory of why structure, constraints, and state matter.

## Projects

Start with the [Molecule Explorer CLI](projects/molecule-explorer-cli.md). It turns
the molecule model into commands students can run:

```bash
cd exercises/rust-molecule-model
cargo run -- ethanol summary
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
```

## Visual Learning

Visuals live in [visuals/](visuals/README.md):

- [Course map](visuals/mermaid/course-map.md)
- [Graph algorithms](visuals/mermaid/graph-algorithms.md)
- [Models, contracts, and flows](visuals/mermaid/model-contracts-flows.md)
- [Traits and functions](visuals/mermaid/traits-and-functions.md)
- [Molecule data model](visuals/plantuml/molecule-data-model.puml)
- [Traits and molecular behavior](visuals/plantuml/traits-and-functions.puml)
- [Ownership and borrowing contract](visuals/plantuml/ownership-borrowing-contract.puml)
- [Molecule design flow](visuals/plantuml/molecule-design-flow.puml)
- [Quantum simulation loop](visuals/plantuml/quantum-simulation-loop.puml)
- [Memory address flow](visuals/plantuml/memory-address-flow.puml)

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
