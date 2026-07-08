# Worksheet: Dose-Response Curves And Toy Potency

## Purpose

Practice sorting dose-response points and estimating a target response crossing.

## Part 1: Dose Points

Fill in the table.

| Concentration | Response percent |
| --- | --- |
| 0.1 uM | 10 |
| 1.0 uM | 50 |
| 10 uM | 80 |

What is one dose point?

```text

```

## Part 2: Sort

Sort by concentration:

```text

```

Why should sorting use a common unit?

```text

```

## Part 3: Response Span

Minimum response:

```text

```

Maximum response:

```text

```

Span:

```text

```

## Part 4: Monotonicity

Does response increase as dose increases?

```text

```

## Part 5: Interpolation

Estimate where response crosses `65 percent`.

Known points:

```text
1 uM -> 50 percent
10 uM -> 80 percent
```

Estimated concentration:

```text

```

## Part 6: No Crossing

If the highest response is `30 percent`, can the curve cross `50 percent`?

```text

```

## Part 7: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test dose_response
```

Write one test name that passed.

```text

```

## Reflection

Why is toy interpolation not enough for real dose-response analysis?

```text

```
