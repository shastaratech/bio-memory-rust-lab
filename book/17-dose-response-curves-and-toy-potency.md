# Chapter 17: Dose-Response Curves And Toy Potency

## Big Idea

A dose-response curve connects concentration to measured response.

In real biology and chemistry, dose-response analysis requires careful assay
design, curve fitting, controls, uncertainty, and domain review. In this course,
we use a toy version to teach data structures:

- dose points
- sorted concentration lists
- response ranges
- monotonicity checks
- interpolation
- half-max response estimates

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Dose | Concentration used in an observation. |
| Response | Measured effect at that dose. |
| Dose point | One concentration plus one response. |
| Curve | Ordered dose-response points. |
| Monotonic | Moving in one direction without reversing. |
| Interpolation | Estimating between two known points. |
| Half-max response | The concentration where response crosses 50 percent in this toy model. |
| Potency | Rough idea of how much concentration is needed for a response. |

## Why Curves Matter

A single observation says:

```text
candidate-a at 1 uM -> 50 percent response
```

A curve says more:

```text
0.1 uM -> 10 percent
1.0 uM -> 50 percent
10 uM  -> 80 percent
```

Now we can ask:

- Does response increase with dose?
- What response range do we observe?
- Where does the curve cross 50 percent?
- Is the pattern noisy or surprising?

## Rust Model

Open:

```text
exercises/rust-molecule-model/src/dose_response.rs
```

Core types:

```rust
struct DosePoint {
    concentration: Concentration,
    response_percent: f64,
}

struct DoseResponseCurve {
    candidate_name: String,
    points: Vec<DosePoint>,
}
```

The curve sorts points by concentration in micromolar.

This matters because dose-response questions depend on ordered doses.

## Validation

The toy model rejects:

- fewer than two points
- duplicate concentrations
- non-finite responses
- responses below `0` or above `100`

This keeps the exercise focused on valid data before interpretation.

## Interpolation

Suppose:

```text
1 uM  -> 50 percent
10 uM -> 80 percent
```

A target response of `65 percent` is halfway between `50` and `80`.

The toy linear interpolation estimates:

```text
5.5 uM
```

This is a teaching approximation, not a production pharmacology model.

## Half-Max Estimate

The function:

```rust
estimated_half_max_concentration()
```

asks where the response crosses `50 percent`.

If the curve never crosses `50 percent`, it returns:

```rust
None
```

This connects to `Option`:

> Some questions have no answer for the data you collected.

## Relation To Previous Modules

| Earlier module | Dose-response connection |
| --- | --- |
| typed measurements | each dose needs a unit |
| replicates | each point may come from repeated observations |
| screening | curves improve score interpretation |
| feedback loops | curve shape can change next-round choices |
| serialization | curves need stable record formats |

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test dose_response
```

Look for tests that:

- sort dose points by concentration
- reject invalid curve inputs
- compute response span
- check monotonicity
- estimate half-max concentration
- return `None` when a target response is not crossed

## Checkpoint

1. What is a dose point?
2. Why should dose points be sorted by concentration?
3. What does monotonicity check?
4. What does interpolation estimate?
5. Why can half-max concentration be `None`?
6. Why is this toy interpolation not production curve fitting?
7. How do replicates and units affect dose-response interpretation?
