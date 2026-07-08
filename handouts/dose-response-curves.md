# Handout: Dose-Response Curves And Toy Potency

## Core Idea

A dose-response curve is an ordered set of concentration-response points.

```text
0.1 uM -> 10 percent
1.0 uM -> 50 percent
10 uM  -> 80 percent
```

## Data Shapes

| Concept | Rust/data shape |
| --- | --- |
| dose point | `DosePoint` struct |
| curve | sorted `Vec<DosePoint>` |
| target crossing | adjacent-point search |
| no crossing | `Option::None` |
| response range | `max - min` |

## Toy Half-Max Estimate

The toy half-max question is:

```text
Where does response cross 50 percent?
```

If the curve crosses `50`, return a concentration.

If it does not cross `50`, return no estimate.

## Caution

Linear interpolation is a teaching approximation. Real potency estimates need
replicates, controls, uncertainty, curve fitting, and domain review.
