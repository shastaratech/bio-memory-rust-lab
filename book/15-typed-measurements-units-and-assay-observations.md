# Chapter 15: Typed Measurements, Units, And Assay Observations

## Big Idea

Numbers in chemistry need units and context.

`500` alone is ambiguous. It might mean:

- 500 nanomolar
- 500 micromolar
- 500 atoms
- 500 percent, which is probably invalid

This chapter teaches how Rust types can protect measurement meaning.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Measurement | A value with a unit and context. |
| Unit | The scale attached to a value, such as `nM`, `uM`, or `mM`. |
| Normalization | Converting values into a common representation. |
| Assay observation | A measured response at a concentration. |
| Range validation | Checking that a value is within allowed limits. |
| Mean response | Average response across observations. |

## Why Units Matter

Compare:

```text
500 nM = 0.5 uM
0.5 uM = 0.5 uM
0.001 mM = 1.0 uM
```

The numbers differ, but they can describe similar scales after conversion.

Programming rule:

> Do not let a bare number pretend to be a complete measurement.

## Rust Model

Open:

```text
exercises/rust-molecule-model/src/measurement.rs
```

Core types:

```rust
enum ConcentrationUnit {
    Nanomolar,
    Micromolar,
    Millimolar,
}

struct Concentration {
    value: f64,
    unit: ConcentrationUnit,
}
```

The enum gives a fixed unit vocabulary. The struct keeps the number and unit
together.

## Validation

A concentration cannot be:

- negative
- `NaN`
- infinity

An assay response percent must be:

```text
0 <= response <= 100
```

This is not chemistry magic. It is basic data hygiene.

## Assay Observation

An observation connects:

- candidate name
- concentration
- response percent

```rust
struct AssayObservation {
    candidate_name: String,
    concentration: Concentration,
    response_percent: f64,
}
```

The response can be normalized:

```text
75 percent -> 0.75
```

The concentration can be normalized:

```text
500 nM -> 0.5 uM
```

## Sorting By Common Unit

Before sorting measurements, convert them to a common unit.

Bad idea:

```text
500 nM, 1 uM, 0.01 mM
```

sorted by raw number gives nonsense.

Better:

```text
0.5 uM, 1 uM, 10 uM
```

Now sorting has a coherent meaning.

## Relation To Previous Modules

| Earlier module | Measurement connection |
| --- | --- |
| molecule records | observations need candidate names |
| fingerprints | scores need measured follow-up |
| screening | labels and responses come from assays |
| serialization | records need units and provenance |
| feedback loops | measured observations guide next round |

## DNA And Memory Connection

DNA sequence data is ordered symbolic information. Assay measurements are numeric
observations with units and context.

Both need representation discipline:

- typed vocabulary
- valid ranges
- stable records
- explicit missing context
- careful interpretation

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test measurement
```

Look for tests that:

- convert `nM`, `uM`, and `mM`
- reject negative and non-finite values
- reject invalid response percentages
- sort observations by normalized concentration
- compute mean response

## Checkpoint

1. Why is a bare number dangerous in chemistry data?
2. Why is an enum useful for units?
3. What does normalization do?
4. Why should sorting use a common unit?
5. What response values are invalid?
6. How do assay observations connect to screening feedback?
7. What context is still missing from the toy observation?
