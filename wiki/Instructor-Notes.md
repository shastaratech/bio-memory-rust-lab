# Instructor Notes

## Teaching Rule

Use analogy as a bridge, not as a proof.

Atoms can help students understand Rust types, ownership, graph structure, and
constraints. But atoms are not bytes, bonds are not Rust references, and qubits
are not ordinary memory cells.

## Suggested First Session

Length: 90 minutes.

1. Show a molecule and a Rust struct side by side.
2. Map atom facts to scalar types.
3. Build `Element`, `Atom`, `Bond`, and `Molecule`.
4. Explain stack, heap, references, and ownership through molecule ownership.
5. Treat molecules as graphs.
6. Close with quantum state as a distinct representation layer.

## Make It Interesting

For school students:

- Keep the room moving. Alternate cards, string graphs, drawing, and code.
- Let students be atoms and pass messages through bonds.
- Give Rust a personality: strict lab notebook, not angry compiler.
- End with a mystery: "What does our molecule graph still not know?"

For university students:

- Make every analogy pay rent with an implementation decision.
- Ask whether an invariant belongs in a type, constructor, validator, or test.
- Compare bond-list scanning with adjacency-list lookup.
- Ask when clone-heavy code is acceptable and when it becomes scientific debt.

## Suggested Second Session

Length: 90 minutes.

1. Start with the human bond graph.
2. Write the same graph as `Vec<Bond>`.
3. Convert it to an adjacency list.
4. Run shortest path by hand.
5. Run `cargo test` and inspect the graph tests.
6. Discuss what graph models leave out: 3D shape, energy, and electron density.

## Good Discussion Prompts

- What does the compiler prevent when we use `Element::Cl` instead of `"cl"`?
- Why is a `Vec<Atom>` a better starter model than a fixed-size array?
- Why do bonds store atom indices in the starter model?
- What does a molecule graph leave out?
- Why is a qubit not just a larger bit?

## Public Repo Safety

Do not add raw molecular datasets, unpublished assay data, proprietary compound
structures, model weights, credentials, or tokens.
