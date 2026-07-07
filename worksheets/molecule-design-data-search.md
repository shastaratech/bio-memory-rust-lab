# Worksheet: Molecule Design As Data Search

## Goal

Design a toy molecule campaign using data structures.

## Part 1: Scaffold

Choose one starter scaffold:

```text
water / methane / ethanol / other:
```

Draw it as a graph:

```text



```

What data structure stores the scaffold in this course?

```text

```

## Part 2: Options Vector

Choose four possible substituents:

| Option ID | Substituent |
| --- | --- |
| `0` | |
| `1` | |
| `2` | |
| `3` | |

Write the Rust shape:

```rust
let options = vec![

];
```

## Part 3: Generate Candidates

Fill in candidate names.

| Candidate | Scaffold | Substituent | Attachment atom |
| --- | --- | --- | --- |
| | | | |
| | | | |
| | | | |
| | | | |

Which data structure stores all candidates?

```text

```

## Part 4: Constraints

Write three rules.

| Rule | What data does it inspect? | Keep or reject? |
| --- | --- | --- |
| | | |
| | | |
| | | |

Which step is this?

```text
map / filter / sort / merge
```

## Part 5: Features And Scores

Choose three features.

| Feature | Data structure needed |
| --- | --- |
| atom count | |
| bond count | |
| element count | |

Write a toy score:

```text
score =
```

## Part 6: Rank Candidates

Fill in scores, then sort from best to worst.

| Rank | Candidate | Score |
| --- | --- | --- |
| 1 | | |
| 2 | | |
| 3 | | |
| 4 | | |

Which data structure is useful after scoring?

```text

```

## Part 7: Lookup And History

Choose keys and values.

| Purpose | Key | Value |
| --- | --- | --- |
| score lookup | | |
| candidate lookup | | |
| reject reason lookup | | |

How would you store previous design rounds?

```text

```

## Reflection

Answer in two sentences:

1. How is molecule design like search?
2. Why does a toy score not prove a real molecule is useful?
