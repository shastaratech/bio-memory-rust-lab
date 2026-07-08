# Chapter 16: Replicates, Variability, And Uncertainty

## Big Idea

One measurement is not enough to understand uncertainty.

Replicates are repeated observations under similar conditions. They help students
see whether a response is stable or noisy.

This chapter connects assay observations to statistics:

- repeated responses
- mean response
- min, max, and range
- sample variance
- sample standard deviation
- coefficient of variation
- consistency checks

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Replicate | A repeated measurement. |
| Series | A list of replicate values. |
| Mean | Average response. |
| Range | Maximum minus minimum. |
| Variance | Average squared spread around the mean. |
| Standard deviation | Spread measured in the original unit. |
| Coefficient of variation | Standard deviation divided by mean. |
| Uncertainty | Limits on how confidently we interpret a value. |

## Why Replicates Matter

Compare:

```text
candidate-a: 40, 41, 42
candidate-b: 20, 50, 80
```

Both may have a similar average scale, but the second candidate is much noisier.

Teaching rule:

> A mean without variability can hide important information.

## Rust Model

Open:

```text
exercises/rust-molecule-model/src/replicate.rs
```

Core type:

```rust
struct ReplicateSeries {
    candidate_name: String,
    responses_percent: Vec<f64>,
}
```

This stores repeated response percentages for one candidate.

The constructor rejects:

- empty series
- `NaN` or infinite values
- response values outside `0..100`

## Mean, Range, And Spread

For:

```text
40, 44, 46
```

Mean:

```text
(40 + 44 + 46) / 3 = 43.33
```

Range:

```text
46 - 40 = 6
```

Standard deviation:

```text
about 3.06
```

The mean tells the center. The spread tells how much the replicates disagree.

## Sample Variance

Sample variance uses `n - 1` in the denominator.

Why?

With a small sample, we estimate variability from limited observations. The
`n - 1` denominator is a standard correction for sample variance.

This is not the place to prove statistics. The teaching goal is:

> The formula should match the question and the data collection process.

## Consistency Checks

A toy consistency rule:

```text
standard deviation <= allowed limit
```

Example:

```text
candidate-a: 40, 41, 42 -> consistent if limit is 2
candidate-b: 20, 50, 80 -> not consistent if limit is 2
```

This mirrors scientific judgment:

- strong signal with low spread is easier to trust
- high spread requires caution
- a single replicate cannot estimate spread

## Relation To Previous Modules

| Earlier module | Replicate connection |
| --- | --- |
| typed measurements | each replicate needs valid units and ranges |
| screening feedback | labels and scores need evidence quality |
| serialization | replicate summaries need reproducible records |
| molecule design | noisy results can change next-round choices |

## DNA And Memory Connection

DNA sequence comparison often asks whether symbols match. Measurements ask how
numeric observations vary.

Both need structure:

- a list of values
- validation rules
- summary functions
- careful interpretation

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test replicate
```

Look for tests that:

- build replicate series
- reject invalid values
- compute mean, range, variance, and standard deviation
- handle single-replicate edge cases
- check consistency against a spread limit

## Checkpoint

1. What is a replicate?
2. Why can a mean hide noisy data?
3. What does range measure?
4. What does standard deviation measure?
5. Why does a single replicate not have sample variance?
6. Why should invalid response values be rejected before statistics?
7. How can variability change the next screening decision?
