# Chapter 18: Controls, Baselines, And Normalization

## Big Idea

Raw assay response is not interpretable by itself.

Controls give the measurement a reference frame:

- blank: background signal
- negative control: low or no expected response
- positive control: high expected response

Normalization turns raw response into a value relative to those controls.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Blank | Background signal to subtract. |
| Negative control | Reference for low response. |
| Positive control | Reference for high response. |
| Control window | Positive control minus negative control. |
| Blank correction | Raw response minus blank response. |
| Normalization | Convert response into a shared scale. |
| Quality window | Minimum separation expected between controls. |

## Why Controls Matter

Raw response:

```text
50
```

This means little without reference points.

With controls:

```text
blank = 5
negative control = 10
positive control = 90
candidate = 50
```

The candidate sits halfway between the negative and positive controls.

Normalized:

```text
50 percent
```

## Rust Model

Open:

```text
exercises/rust-molecule-model/src/normalization.rs
```

Core type:

```rust
struct ControlSet {
    blank_response: f64,
    negative_control_response: f64,
    positive_control_response: f64,
}
```

The constructor checks:

- all values are finite
- positive control is greater than negative control

## Blank Correction

Blank correction subtracts background:

```text
corrected = raw - blank
```

Example:

```text
42 - 5 = 37
```

This teaches a central idea:

> Before comparing signals, remove the signal that should not belong to the
> candidate.

## Normalization To Controls

The toy formula:

```text
(corrected candidate - corrected negative) / (corrected positive - corrected negative)
```

With:

```text
blank = 5
negative = 10
positive = 90
candidate = 50
```

The result is:

```text
0.5
```

As percent:

```text
50 percent
```

## Quality Window

If positive and negative controls are too close, the assay may not separate signal
from background well enough.

Toy rule:

```text
positive - negative >= minimum window
```

This is a basic data-quality gate.

## Relation To Previous Modules

| Earlier module | Control connection |
| --- | --- |
| typed measurements | controls are measured values with context |
| replicates | controls should also have repeated observations |
| dose-response | every curve point should be control-aware |
| screening | decisions depend on normalized evidence |
| serialization | records should store control metadata |

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test normalization
```

Look for tests that:

- create valid controls
- reject bad control order
- subtract blank
- normalize to fraction and percent
- check control-window quality
- reject non-finite raw response

## Checkpoint

1. What is a blank?
2. What is a negative control?
3. What is a positive control?
4. Why is raw response not enough?
5. What does blank correction remove?
6. What does the control window measure?
7. Why should records store control context?
