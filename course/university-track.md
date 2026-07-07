# University Track

## Tone

Use the molecule metaphor as a bridge into serious systems and scientific computing:
representation, invariants, memory behavior, graph algorithms, and simulation limits.

## Prerequisites

Helpful but not required:

- one programming language
- basic chemistry vocabulary
- arrays, structs/classes, and functions

## Six-Session Seminar

### Session 01: Typed Molecular Data

Core questions:

- What should be an enum, struct, vector, or method?
- Which invalid states should be impossible?
- Which invalid states are better caught at runtime?

Implementation focus:

- `Element`
- `Atom`
- `Bond`
- `Molecule`
- `validate_bond_indices`

### Session 02: Molecules as Graphs

Core questions:

- When is `Vec<Bond>` enough?
- When do we need an adjacency list?
- How do traversal algorithms change the questions we can ask?

Implementation focus:

- adjacency list construction
- neighbor lookup
- shortest path
- connected components

### Session 03: Ownership and Design Pipelines

Core questions:

- When should an algorithm borrow a molecule?
- When should it mutate a molecule?
- When should it clone?
- How do API choices encode scientific workflow assumptions?

Implementation focus:

- borrowed scoring functions
- mutating analog generation
- immutable scaffold data
- clone cost discussion

### Session 04: Quantum State and Simulation

Core questions:

- What does a graph representation leave out?
- Why do electronic structure problems become hard?
- What does VQE optimize, at a high level?
- Why are quantum computers interesting but not magic accelerators?

Implementation focus:

- state vector intuition
- Hamiltonian as an energy operator
- classical scoring vs quantum energy estimation
- limits of toy models

### Session 05: Rust Data Structures for Molecules

Core questions:

- What should be stored as an enum, struct, vector, graph, or count table?
- What questions should be cheap to answer?
- What invalid states can the representation prevent?

Implementation focus:

- `Element`
- `Atom`
- `Bond`
- `Molecule`
- formula counts
- storage representation vs graph view

### Session 06: Traits and Functions for Molecular Behavior

Core questions:

- Which behaviors belong in traits?
- Which methods should have default implementations?
- What does a function promise through its parameters and return type?
- How can algorithms accept `&impl MolecularGraph` instead of only `&Molecule`?

Implementation focus:

- `Describe`
- `ChemicalFormula`
- `MolecularGraph`
- `Option<Vec<usize>>`
- generic functions over molecular graph behavior

## Assessment Ideas

- Code review: students explain representation choices in their Rust model.
- Algorithm lab: implement shortest path and connected components.
- Design memo: compare string-based element labels with enum-based labels.
- Reflection: explain why quantum state cannot be copied and read like normal data.

## Capstone Rubric

| Criterion | Strong evidence |
| --- | --- |
| Rust type design | Invalid labels and invalid bonds are constrained or detected. |
| Algorithmic thinking | Graph operations are correct and tested. |
| Scientific humility | The project states what its model leaves out. |
| Communication | Code, diagrams, and explanation agree with each other. |

## Visuals to Use

- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)
- [Ownership and borrowing contract](../visuals/plantuml/ownership-borrowing-contract.puml)
- [Memory address flow](../visuals/plantuml/memory-address-flow.puml)
- [Quantum simulation loop](../visuals/plantuml/quantum-simulation-loop.puml)
- [Traits and molecular behavior](../visuals/plantuml/traits-and-functions.puml)

For university students, treat diagrams as design artifacts. Ask what each diagram
forces into the model and what it hides.

## Implementation Lab

1. Refactor a concrete molecule API into traits.
2. Write generic functions over `&impl MolecularGraph`.
3. Compare returning owned `Vec<usize>` with iterator-based APIs.
4. Decide which methods deserve default trait implementations.
