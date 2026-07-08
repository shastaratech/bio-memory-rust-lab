# Handout: Controls, Baselines, And Normalization

## Core Idea

Raw response needs reference values.

```text
blank
negative control
positive control
candidate response
```

## Blank Correction

```text
corrected = raw - blank
```

Blank correction removes background signal.

## Normalize To Controls

```text
(corrected candidate - corrected negative) / (corrected positive - corrected negative)
```

As percent:

```text
fraction * 100
```

## Quality Window

```text
control window = positive control - negative control
```

If the window is too small, the assay may not separate low and high response well
enough for interpretation.

## Caution

Toy normalization teaches the data flow. Real assays require replicate controls,
uncertainty, protocol details, plate effects, and domain review.
