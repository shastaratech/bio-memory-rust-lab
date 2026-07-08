# Worksheet: Controls, Baselines, And Normalization

## Purpose

Practice converting raw response into a control-aware normalized value.

## Part 1: Control Cards

| Card | Value |
| --- | --- |
| blank | 5 |
| negative control | 10 |
| positive control | 90 |
| candidate | 50 |

## Part 2: Blank Correction

Formula:

```text
corrected = raw - blank
```

Candidate corrected:

```text

```

Negative corrected:

```text

```

Positive corrected:

```text

```

## Part 3: Normalize

Formula:

```text
(corrected candidate - corrected negative) / (corrected positive - corrected negative)
```

Normalized fraction:

```text

```

Normalized percent:

```text

```

## Part 4: Quality Window

Control window:

```text
positive - negative =
```

If the minimum required window is `50`, does this pass?

```text

```

## Part 5: Bad Controls

Why should this be rejected?

```text
blank = 5
negative = 90
positive = 10
```

Answer:

```text

```

## Part 6: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test normalization
```

Write one test name that passed.

```text

```

## Reflection

What control metadata should be stored in a reproducible lab record?

```text

```
