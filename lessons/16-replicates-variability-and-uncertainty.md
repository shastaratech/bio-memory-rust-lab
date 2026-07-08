# Lesson 16: Replicates, Variability, And Uncertainty

## Goal

Students learn why repeated measurements matter and how mean, range, variance,
standard deviation, and consistency checks support scientific interpretation.

## Big Question

How do we know whether repeated assay measurements agree?

## Visuals

- [Replicates and uncertainty](../visuals/mermaid/replicates-uncertainty.md)
- [Typed measurements and assay observations](../visuals/mermaid/typed-measurements.md)
- [Screening feedback loop](../visuals/mermaid/screening-feedback-loop.md)

## Core Analogy

| Replicate idea | Data-structure idea |
| --- | --- |
| repeated response | vector |
| mean | summary function |
| min/max | boundary scan |
| range | spread summary |
| standard deviation | variability estimate |
| consistency rule | threshold check |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show two candidates with similar means but different spread. |
| 10-25 min | Replicates | Build response rows with cards. |
| 25-40 min | Mean/range | Compute center and spread by hand. |
| 40-55 min | Variability | Introduce sample standard deviation conceptually. |
| 55-70 min | Consistency | Apply a standard-deviation limit. |
| 70-90 min | Code | Run `cargo test replicate` and inspect edge cases. |

## Classroom Activity

Give students:

```text
candidate-a: 40, 41, 42
candidate-b: 20, 50, 80
```

Ask:

- Which has the tighter response?
- Which is easier to interpret?
- What does the average hide?

## University Extension

Inspect:

```text
exercises/rust-molecule-model/src/replicate.rs
```

Discussion prompts:

- Why does `ReplicateSeries::new` reject empty data?
- Why does sample variance return `Option<f64>`?
- Why is standard deviation easier to explain than variance?
- When is coefficient of variation useful?
- What uncertainty fields would a real assay record need?

## Caution

This is a teaching summary, not a full statistical assay analysis. Real
experimental interpretation needs replicate design, controls, uncertainty models,
instrument context, and domain review.
