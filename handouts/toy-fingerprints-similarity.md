# Handout: Toy Fingerprints And Similarity Search

## Core Idea

A toy fingerprint turns molecule observations into bits.

```text
feature question -> yes/no -> bit
```

Many bits together form a compact feature set.

## Feature Examples

| Bit | Feature |
| --- | --- |
| 0 | has carbon |
| 1 | has oxygen |
| 2 | has halogen |
| 3 | has at least five atoms |
| 4 | has at least eight atoms |
| 5 | has branching atom |
| 6 | has oxygen-hydrogen bond |
| 7 | has carbon-carbon bond |

## Compare Two Fingerprints

| Operation | Meaning |
| --- | --- |
| intersection | features shared by both |
| union | features present in either |
| similarity | shared count divided by union count |

Classroom formula:

```text
similarity = shared yes features / yes features in either molecule
```

## Search Workflow

```text
query molecule
  -> query fingerprint
  -> compare with library fingerprints
  -> score matches
  -> sort highest to lowest
```

## Caution

A fingerprint is not the molecule. It is a compact representation that keeps some
features and discards others.

Use it here to learn bitsets, set operations, ranking, and search. Do not treat
the toy score as real chemical or biological evidence.
