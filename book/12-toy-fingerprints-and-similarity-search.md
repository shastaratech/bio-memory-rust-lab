# Chapter 12: Toy Fingerprints And Similarity Search

## Big Idea

A molecular fingerprint is a compact feature summary.

In real cheminformatics, fingerprints are carefully designed and validated. In
this course, we use a toy fingerprint to teach a computer science idea:

> Convert a molecule graph into a small set of yes/no features, then compare those
> feature sets with bit operations.

This chapter connects chemistry, memory, and data structures:

- molecule graph -> computed features
- features -> bits
- bits -> compact memory representation
- shared bits -> similarity score
- sorted scores -> nearest matches

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Feature | A yes/no property, such as "has oxygen." |
| Fingerprint | A compact collection of feature bits. |
| Bitset | A group of bits used as a small set. |
| Intersection | Features shared by two fingerprints. |
| Union | Features present in either fingerprint. |
| Similarity | A score comparing shared features against total features. |
| Query molecule | The molecule being searched with. |
| Nearest match | A record with the highest similarity score. |

## From Molecule To Features

Open:

```text
exercises/rust-molecule-model/src/fingerprint.rs
```

The toy feature enum is:

```rust
enum Feature {
    HasCarbon,
    HasOxygen,
    HasHalogen,
    HasAtLeastFiveAtoms,
    HasAtLeastEightAtoms,
    HasBranchingAtom,
    HasOxygenHydrogenBond,
    HasCarbonCarbonBond,
}
```

Each feature is a yes/no question asked about the molecule.

Examples:

| Feature | Chemistry question |
| --- | --- |
| `HasOxygen` | Does any atom have element `O`? |
| `HasHalogen` | Does any atom have element `F`, `Cl`, or `Br`? |
| `HasAtLeastFiveAtoms` | Is the toy graph at least five atoms long? |
| `HasOxygenHydrogenBond` | Is there an `O-H` bond in the graph? |

## Bitset As Memory

A toy fingerprint stores features in a `u64`:

```rust
struct ToyFingerprint {
    bits: u64,
}
```

One bit means one feature.

```text
bit 0: has carbon
bit 1: has oxygen
bit 2: has halogen
...
```

If a bit is `1`, the feature is present. If it is `0`, the feature is absent.

This is compact computer memory: many yes/no answers inside one integer.

## Set Operations With Bits

Two fingerprints can be compared with bit operations.

| Set idea | Bit operation | Meaning |
| --- | --- | --- |
| intersection | `left & right` | features shared by both |
| union | `left | right` | features present in either |
| count | `.count_ones()` | number of active features |

Toy similarity:

```text
shared features / union features
```

This is a beginner version of the idea behind Tanimoto/Jaccard-style similarity.

## Example: Water And Ethanol

Water features:

```text
has oxygen
has oxygen-hydrogen bond
```

Ethanol features:

```text
has carbon
has oxygen
has at least five atoms
has at least eight atoms
has branching atom
has oxygen-hydrogen bond
has carbon-carbon bond
```

Shared toy features:

```text
has oxygen
has oxygen-hydrogen bond
```

The score is not chemistry truth. It is a way to teach how compact feature
summaries support comparison.

## Similarity Search

The library search workflow is:

1. Convert the query molecule into a fingerprint.
2. Convert each library record into a fingerprint.
3. Compare query fingerprint to each record fingerprint.
4. Store `record_id`, `common_name`, and `similarity`.
5. Sort matches from highest score to lowest score.

Rust function:

```rust
fn rank_by_similarity(query: &Molecule, records: &[MoleculeRecord]) -> Vec<FingerprintMatch>
```

Notice the inputs:

- `&Molecule`: borrow the query molecule
- `&[MoleculeRecord]`: borrow a slice of library records
- `Vec<FingerprintMatch>`: return owned search results

## DNA And Fingerprint Analogy

DNA sequence comparison asks how biological sequences relate. Molecular
fingerprint comparison asks how encoded feature summaries relate.

| Biology / chemistry | Computer structure |
| --- | --- |
| DNA base | enum value |
| DNA sequence | vector/string-like ordered data |
| molecule graph | atoms plus bonds |
| molecular feature | boolean question |
| fingerprint | bitset |
| similarity search | scoring and sorting |

Important limit:

DNA is not literally a Rust bitset. A fingerprint is not a full molecule. Both are
representations that preserve some information and discard other information.

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test fingerprint
```

Look for tests that:

- build fingerprints
- list feature labels
- count shared and union features
- rank records by similarity

## Checkpoint

1. What is a molecular feature?
2. Why can yes/no features fit into a bitset?
3. What does intersection count?
4. What does union count?
5. Why does similarity search sort results?
6. Why is a toy fingerprint not proof that two molecules behave the same?
7. How is a fingerprint different from the full molecule graph?
