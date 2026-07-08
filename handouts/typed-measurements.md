# Handout: Typed Measurements, Units, And Assay Observations

## Core Rule

A chemistry number needs a unit and context.

```text
500
```

is not enough.

```text
500 nM
```

is better, but still needs assay context.

## Unit Conversion

| Unit | Micromolar conversion |
| --- | --- |
| `nM` | multiply by `0.001` |
| `uM` | multiply by `1.0` |
| `mM` | multiply by `1000.0` |

## Typed Shape

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

## Validation

Reject:

- negative concentrations
- `NaN` and infinity
- response percent below `0`
- response percent above `100`

## Reproducibility Rule

Before comparing, sorting, averaging, or storing measurements:

1. Keep value and unit together.
2. Convert to a common unit.
3. Validate allowed ranges.
4. Preserve assay context and provenance.
