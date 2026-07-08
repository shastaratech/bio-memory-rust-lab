# Lesson 12: Toy Fingerprints And Similarity Search

## Goal

Students learn how a molecule graph can be summarized as feature bits and compared
with simple set operations.

## Big Question

How can a compact memory representation help search for molecules with similar
features?

## Visuals

- [Toy fingerprints and similarity search](../visuals/mermaid/fingerprint-similarity.md)
- [Molecular library indexes](../visuals/mermaid/molecular-library-indexes.md)

## Core Analogy

| Molecule idea | Data-structure idea |
| --- | --- |
| feature question | boolean |
| feature collection | set |
| compact feature set | bitset |
| shared features | intersection |
| total observed features | union |
| closest records | sorted score list |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show two molecule cards and ask "same or different how?" |
| 10-25 min | Features | Turn molecule observations into yes/no cards. |
| 25-40 min | Bits | Place feature cards into numbered bit slots. |
| 40-55 min | Compare | Count shared and union feature cards. |
| 55-70 min | Search | Score one query against a small molecule library. |
| 70-90 min | Code | Run `cargo test fingerprint` and inspect the tests. |

## Part 1: Feature Cards

Use cards:

- has carbon
- has oxygen
- has halogen
- has at least five atoms
- has at least eight atoms
- has branching atom
- has oxygen-hydrogen bond
- has carbon-carbon bond

Students mark each card yes/no for water, methane, and ethanol.

Teaching line:

A fingerprint is not the molecule. It is a compact summary of chosen questions.

## Part 2: Bits

Map each feature to one bit:

```text
bit 0 -> has carbon
bit 1 -> has oxygen
bit 2 -> has halogen
```

If the molecule has that feature, the bit is `1`.

If not, the bit is `0`.

## Part 3: Similarity

Use this classroom formula:

```text
similarity = shared yes cards / yes cards in either molecule
```

This teaches intersection and union before naming formal similarity metrics.

## Part 4: Search

Ask:

> If ethanol is the query, which library record is most similar?

Data flow:

```text
query molecule -> query fingerprint
library records -> record fingerprints
compare -> score list
sort -> nearest matches
```

## University Extension

Discuss:

- Why does `ToyFingerprint` store a `u64`?
- What happens when there are more than 64 features?
- Which features should be cached in a larger library?
- Why does `rank_by_similarity` borrow the query and records?
- What does a fingerprint discard from the full graph?

## Caution

This is not a production chemistry fingerprint and not a molecule recommendation
engine. It is a small, inspectable model for learning bitsets, similarity, and
search.
