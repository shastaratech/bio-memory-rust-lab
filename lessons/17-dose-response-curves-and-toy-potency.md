# Lesson 17: Dose-Response Curves And Toy Potency

## Goal

Students learn how concentration-response points become an ordered curve and how a
toy interpolation can estimate where a response crosses a target.

## Big Question

How does a series of measurements become a curve that supports a decision?

## Visuals

- [Dose-response curves](../visuals/mermaid/dose-response-curves.md)
- [Typed measurements and assay observations](../visuals/mermaid/typed-measurements.md)
- [Replicates and uncertainty](../visuals/mermaid/replicates-uncertainty.md)

## Core Analogy

| Dose-response idea | Data-structure idea |
| --- | --- |
| dose point | struct |
| curve | sorted vector |
| concentration order | sort key |
| target crossing | search between adjacent points |
| interpolation | estimate between records |
| missing crossing | `Option::None` |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show one response, then show a three-point curve. |
| 10-25 min | Dose points | Build concentration-response cards. |
| 25-40 min | Sorting | Sort by normalized concentration. |
| 40-55 min | Monotonicity | Check whether response increases with dose. |
| 55-70 min | Interpolation | Estimate where response crosses 50 percent. |
| 70-90 min | Code | Run `cargo test dose_response` and inspect edge cases. |

## Classroom Activity

Give students cards:

```text
0.1 uM -> 10 percent
1.0 uM -> 50 percent
10 uM  -> 80 percent
```

Ask:

- What is the lowest dose?
- What is the highest response?
- Where does response cross 50 percent?
- What if the curve only reaches 30 percent?

## University Extension

Inspect:

```text
exercises/rust-molecule-model/src/dose_response.rs
```

Discussion prompts:

- Why does the curve sort points during construction?
- Why are duplicate concentrations rejected?
- Why does interpolation return `Option<f64>`?
- What assumptions does linear interpolation make?
- What would a real dose-response model need beyond this toy example?

## Caution

This is a data-structure lesson, not a validated potency model. Real
dose-response analysis needs replicates, controls, uncertainty, curve fitting,
and domain review.
