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

## Suggested Capstone Session

Length: 90 minutes.

1. Start with `cargo run -- list` and have students predict available molecules.
2. Run `summary`, `atoms`, `bonds`, `neighbors`, `path`, `components`, and
   `validate` on water, methane, and ethanol.
3. Draw the CLI flow from terminal input to molecule function.
4. Add the `degree` command as the smallest extension.
5. Add ammonia only if the group is ready to edit molecule lookup and tests.
6. Close with presentations: command, Rust function, molecule question, and model
   limitation.

## Suggested Biological Memory Session

Length: 75-90 minutes.

1. Start with `ACGTTAGC` and ask whether it is chemistry, data, or both.
2. Convert base cards into a Rust `Base` enum.
3. Compare `Base`, `[Base; 3]`, `Vec<Base>`, `String`, `&DnaStrand`, and `&[Base]`.
4. Use sticky notes to mark a borrowed gene region inside a longer strand.
5. Model substitutions, insertions, and deletions as enum variants.
6. Close by naming where the DNA-as-code analogy helps and where it breaks.

## Suggested Visualization And Algorithms Session

Length: 75 minutes.

1. Open the local 3D molecule viewer and rotate ethanol.
2. Ask what the 3D view shows that a formula does not.
3. Put atom cards in a row to represent a vector.
4. Use buckets labeled `C`, `H`, `O`, and `N` to represent hash-map counting.
5. Sort formula-count cards into stable output order.
6. Merge atom-ID cards with label cards and discuss missing IDs.
7. Close by asking which representation makes each molecule question cheap.

## Suggested Molecule Design Session

Length: 90 minutes.

1. Choose one scaffold card and one attachment atom.
2. Put substituent cards in a vector-like row.
3. Generate candidate cards by combining scaffold plus substituent.
4. Apply validation rule cards and reject invalid candidates.
5. Compute one toy feature and one toy score for each valid candidate.
6. Sort candidates into a ranked list.
7. Choose the top candidates as the next search frontier.
8. Close by naming why a toy score is not real-world molecule evidence.

## Diagram Routine

Use this three-step loop:

1. Show the diagram.
2. Act it out or trace it by hand.
3. Open the Rust code or lesson text that implements the same idea.

Best starting diagrams:

- Course map
- Molecule data model
- Graph algorithms
- Ownership and borrowing contract
- Traits and functions
- Quantum simulation loop

## Good Discussion Prompts

- What does the compiler prevent when we use `Element::Cl` instead of `"cl"`?
- Why is a `Vec<Atom>` a better starter model than a fixed-size array?
- Why do bonds store atom indices in the starter model?
- What does a molecule graph leave out?
- Why is a qubit not just a larger bit?

## Public Repo Safety

Do not add raw molecular datasets, unpublished assay data, proprietary compound
structures, model weights, credentials, or tokens.
