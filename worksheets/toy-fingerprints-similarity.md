# Worksheet: Toy Fingerprints And Similarity Search

## Purpose

Practice turning molecule observations into feature bits and similarity scores.

## Part 1: Feature Table

Mark each feature as `1` for yes or `0` for no.

| Feature | Water | Methane | Ethanol |
| --- | --- | --- | --- |
| has carbon | | | |
| has oxygen | | | |
| has halogen | | | |
| has at least five atoms | | | |
| has at least eight atoms | | | |
| has branching atom | | | |
| has oxygen-hydrogen bond | | | |
| has carbon-carbon bond | | | |

## Part 2: Bit Slots

Assign each feature to a bit number.

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

Write the bit pattern for ethanol:

```text

```

## Part 3: Shared And Union Counts

Choose two molecules:

```text
left:
right:
```

Shared yes features:

```text

```

Yes features in either molecule:

```text

```

Similarity:

```text
shared / union =
```

## Part 4: Similarity Search

Use ethanol as the query.

Rank these records:

| Record | Similarity | Rank |
| --- | --- | --- |
| water | | |
| methane | | |
| ethanol | | |

## Part 5: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test fingerprint
```

Write one test name that passed:

```text

```

## Reflection

What does a toy fingerprint keep from the molecule graph?

```text

```

What does it throw away?

```text

```
