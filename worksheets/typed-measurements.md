# Worksheet: Typed Measurements, Units, And Assay Observations

## Purpose

Practice converting units, validating observations, and sorting by a common scale.

## Part 1: Convert To Micromolar

| Measurement | Micromolar |
| --- | --- |
| 500 nM | |
| 1 uM | |
| 0.01 mM | |

## Part 2: Validate

Mark valid or invalid.

| Value | Valid? | Reason |
| --- | --- | --- |
| `1 uM` | | |
| `-1 uM` | | |
| `NaN uM` | | |
| `75 percent response` | | |
| `125 percent response` | | |

## Part 3: Build Observations

| Candidate | Concentration | Response percent |
| --- | --- | --- |
| candidate-a | 500 nM | 40 |
| candidate-b | 1 uM | 20 |
| candidate-c | 0.01 mM | 80 |

Normalize concentration to micromolar and sort from low to high.

```text

```

## Part 4: Mean Response

Responses:

```text
20, 60
```

Mean:

```text

```

## Part 5: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test measurement
```

Write one test name that passed.

```text

```

## Reflection

What metadata is missing from the toy assay observation?

```text

```
