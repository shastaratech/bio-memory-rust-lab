# Worksheet: Replicates, Variability, And Uncertainty

## Purpose

Practice summarizing repeated assay responses and interpreting variability.

## Part 1: Replicate Series

| Candidate | Responses |
| --- | --- |
| candidate-a | 40, 41, 42 |
| candidate-b | 20, 50, 80 |

Which candidate looks more consistent?

```text

```

## Part 2: Mean

Compute mean response.

| Candidate | Mean |
| --- | --- |
| candidate-a | |
| candidate-b | |

What does the mean hide?

```text

```

## Part 3: Range

Range:

```text
max - min
```

| Candidate | Min | Max | Range |
| --- | --- | --- | --- |
| candidate-a | | | |
| candidate-b | | | |

## Part 4: Consistency Rule

Rule:

```text
standard deviation <= 2.0 means consistent
```

Which candidate passes?

```text

```

## Part 5: Edge Cases

Why should these inputs be rejected?

| Input | Reason |
| --- | --- |
| empty replicate list | |
| `NaN` response | |
| `101` percent response | |

Why does one replicate not have sample variance?

```text

```

## Part 6: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test replicate
```

Write one test name that passed.

```text

```

## Reflection

How can noisy replicates change the next screening decision?

```text

```
