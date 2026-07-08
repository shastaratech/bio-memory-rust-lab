# Exercises

For step-by-step self-study, use the [book](../book/README.md). This file is the
shorter exercise list.

## Exercise 1: Element Enum

Create an enum for the elements needed to represent water, methane, ethanol, and
coumarin at a simplified graph level.

Questions:

1. Which elements are required?
2. Which elements can you leave out for now?
3. Why is an enum safer than a free-form string?

## Exercise 2: Atom Struct

Create this Rust struct:

```rust
struct Atom {
    element: Element,
    formal_charge: i8,
    aromatic: bool,
}
```

Then create atoms for:

- neutral carbon
- neutral oxygen
- positively charged nitrogen
- aromatic carbon

## Exercise 3: Bond Validation

Write a function that checks whether every bond points to valid atom indices.

Starter:

```rust
fn validate_bond_indices(molecule: &Molecule) -> bool {
    for bond in &molecule.bonds {
        if bond.from >= molecule.atoms.len() || bond.to >= molecule.atoms.len() {
            return false;
        }
    }

    true
}
```

Reflection:

How is this like checking whether every bond in a molecular drawing points to a real
atom?

## Exercise 4: Neighbors

Write a function that returns all atoms directly bonded to a given atom.

Starter:

```rust
fn neighbors(molecule: &Molecule, atom_id: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for bond in &molecule.bonds {
        if bond.from == atom_id {
            result.push(bond.to);
        } else if bond.to == atom_id {
            result.push(bond.from);
        }
    }

    result
}
```

Extension:

Convert the molecule to an adjacency list:

```rust
type AdjacencyList = Vec<Vec<usize>>;
```

Then compare which representation is easier for repeated neighbor lookup.

## Exercise 5: Molecule Design as Search

Imagine a simple molecule-design loop:

1. Start with a scaffold.
2. Add or replace one substituent.
3. Reject invalid structures.
4. Score the remaining structures.
5. Keep the best candidates.
6. Repeat.

Map each step to an algorithmic idea:

| Design step | Algorithm idea |
| --- | --- |
| Scaffold | initial state |
| Substituent replacement | state transition |
| Chemistry rules | constraints |
| Docking/selectivity score | objective function |
| Keep best candidates | ranking / priority queue |
| Repeat | search loop |

Question:

Where would Rust types prevent bad data before the scoring step?

## Exercise 6: Quantum State Caution

Answer in two sentences:

1. Why is a quantum state not the same as a classical memory value?
2. Why are quantum computers interesting for molecular simulation?

## Exercise 7: Graph Explorer

For a printable student worksheet, use the
[Molecule Explorer CLI handout](../handouts/molecule-explorer-cli.md).

Run the Rust molecule model:

```bash
cd rust-molecule-model
cargo run -- water summary
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
```

Then modify `src/main.rs` so it builds methane or ethanol.

Questions:

1. What is the formula?
2. Which atom has the most neighbors?
3. What is the shortest path between two hydrogens?
4. How many connected components does the molecule have?

University extension:

Explain the runtime tradeoff between scanning all bonds with `neighbors` and building
an adjacency list with `adjacency_list`.

## Exercise 8: Traits as Molecular Capabilities

Open:

```text
rust-molecule-model/src/molecule.rs
```

Find these traits:

- `Describe`
- `ChemicalFormula`
- `MolecularGraph`

Tasks:

1. Add `Describe` for `BondOrder`.
2. Write `fn degree(graph: &impl MolecularGraph, atom_id: usize) -> usize`.
3. Write `fn is_isolated(graph: &impl MolecularGraph, atom_id: usize) -> bool`.
4. Explain why `formula()` can be a default trait method.

## Exercise 9: DNA As Typed Sequence Data

Open:

```text
rust-molecule-model/src/dna.rs
```

Find:

- `Base`
- `DnaStrand`
- `Mutation`

Tasks:

1. Parse `ACGTTAGC` into a `DnaStrand`.
2. Compute its complement.
3. Borrow a gene region as a slice.
4. Apply one substitution, one insertion, and one deletion.
5. Explain which values are stack-friendly, heap-backed, or borrowed.

## Exercise 10: Molecule Design As Search

Open:

```text
rust-molecule-model/src/design.rs
```

Find:

- `Substituent`
- `Candidate`
- `generate_design_round`
- `score_candidates`
- `rank_candidates`

Tasks:

1. Generate candidates from water or ethanol.
2. Score each candidate.
3. Build a score lookup map.
4. Rank candidates by score.
5. Keep the top candidates as the next search frontier.

## Exercise 11: Molecular Library Indexes

Open:

```text
rust-molecule-model/src/index.rs
```

Find:

- `MoleculeRecord`
- `MoleculeLibrary`
- `lookup_by_name`
- `formula_index`
- `sorted_by_atom_count`
- `merge_by_atom_count`
- `first_with_at_least_atoms`

Tasks:

1. Build the toy library.
2. List molecule names from the vector view.
3. Find ethanol through the name hash map.
4. Group records by formula.
5. Sort records by atom count.
6. Merge two already sorted record lists.
7. Use binary search to find where `atom_count >= 5` begins.

## Exercise 12: Toy Fingerprints And Similarity Search

Open:

```text
rust-molecule-model/src/fingerprint.rs
```

Find:

- `Feature`
- `ToyFingerprint`
- `FingerprintMatch`
- `rank_by_similarity`

Tasks:

1. Build a fingerprint for water, methane, and ethanol.
2. List feature labels for one molecule.
3. Count shared features between two fingerprints.
4. Count union features between two fingerprints.
5. Compute similarity as `shared / union`.
6. Rank toy library records by similarity to ethanol.
7. Explain what information the fingerprint throws away.

## Exercise 13: Screening Results And Feedback Loops

Open:

```text
rust-molecule-model/src/screening.rs
```

Find:

- `Prediction`
- `ExperimentalLabel`
- `ScreeningDecision`
- `ConfusionMatrix`
- `testing_queue`
- `uncertain_predictions`

Tasks:

1. Convert prediction scores into `Test` or `Defer`.
2. Add active/inactive labels to tested candidates.
3. Build a confusion matrix.
4. Compute precision, recall, and accuracy.
5. Build a testing queue sorted by score.
6. Find predictions near the threshold.
7. Explain why labels are feedback, not just output.
