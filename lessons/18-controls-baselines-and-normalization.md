# Lesson 18: Controls, Baselines, And Normalization

## Goal

Students learn how blank, negative-control, and positive-control values turn raw
responses into interpretable normalized values.

## Big Question

What makes a raw assay response meaningful?

## Visuals

- [Controls and normalization](../visuals/mermaid/controls-normalization.md)
- [Typed measurements and assay observations](../visuals/mermaid/typed-measurements.md)
- [Dose-response curves](../visuals/mermaid/dose-response-curves.md)

## Core Analogy

| Control idea | Data-structure idea |
| --- | --- |
| blank | background value |
| negative control | low reference |
| positive control | high reference |
| control set | struct |
| invalid control order | error enum |
| normalized response | transformed value |
| quality window | threshold check |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Ask whether raw response `50` is high or low. |
| 10-25 min | Controls | Add blank, negative, and positive reference cards. |
| 25-40 min | Blank correction | Subtract background by hand. |
| 40-55 min | Normalization | Map candidate response into 0..1 and percent. |
| 55-70 min | Quality | Check whether the control window is large enough. |
| 70-90 min | Code | Run `cargo test normalization` and inspect errors. |

## Classroom Activity

Cards:

```text
blank = 5
negative = 10
positive = 90
candidate = 50
```

Ask:

- What is the blank-corrected candidate?
- What is the control window?
- Where does candidate sit between negative and positive?
- What if positive is not greater than negative?

## University Extension

Inspect:

```text
exercises/rust-molecule-model/src/normalization.rs
```

Discussion prompts:

- Why does `ControlSet::new` return `Result`?
- Should blank correction happen before normalization?
- What happens when control separation is weak?
- Which control fields should be serialized in lab records?
- How would replicate controls change the model?

## Caution

This is a toy normalization model. Real assays require controls, replicates,
protocol details, uncertainty, plate effects, and domain review.
