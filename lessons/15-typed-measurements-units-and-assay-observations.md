# Lesson 15: Typed Measurements, Units, And Assay Observations

## Goal

Students learn why chemistry measurements need units, validation, normalization,
and context before they can be sorted, averaged, or used for feedback.

## Big Question

How can types prevent unit confusion in molecule and assay data?

## Visuals

- [Typed measurements and assay observations](../visuals/mermaid/typed-measurements.md)
- [Screening feedback loop](../visuals/mermaid/screening-feedback-loop.md)
- [Serialization and lab records](../visuals/mermaid/serialization-lab-records.md)

## Core Analogy

| Measurement idea | Rust/data idea |
| --- | --- |
| unit label | enum |
| value plus unit | struct |
| invalid value | error enum |
| common scale | normalized value |
| assay row | observation record |
| dose ordering | sort by normalized concentration |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Ask whether `500` is high or low without a unit. |
| 10-25 min | Units | Convert `nM`, `uM`, and `mM` cards to micromolar. |
| 25-40 min | Types | Build value-plus-unit cards. |
| 40-55 min | Validation | Reject negative, infinite, and out-of-range responses. |
| 55-70 min | Sorting | Sort observations by normalized concentration. |
| 70-90 min | Code | Run `cargo test measurement` and inspect the tests. |

## Classroom Activity

Give students cards:

```text
500 nM
1 uM
0.01 mM
```

Ask them to sort by raw number first. Then convert all values to micromolar and
sort again.

Teaching line:

Sorting only makes sense when the values mean the same kind of thing.

## University Extension

Inspect:

```text
exercises/rust-molecule-model/src/measurement.rs
```

Discussion prompts:

- Why does `Concentration::new` return `Result`?
- Why is `ConcentrationUnit` an enum instead of a string?
- Why should response percent be range-checked?
- Which unit should a database store internally?
- What additional metadata would a real assay observation need?

## Caution

The toy observation is not a full assay model. Real measurements need replicates,
uncertainty, controls, protocol details, instrument metadata, and domain review.
