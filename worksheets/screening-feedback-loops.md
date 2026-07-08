# Worksheet: Screening Results And Feedback Loops

## Purpose

Practice turning scores into decisions and measuring prediction quality with
experimental labels.

## Part 1: Threshold Decisions

Threshold:

```text
0.70
```

Fill in `test` or `defer`.

| Candidate | Score | Decision |
| --- | --- | --- |
| candidate-a | 0.92 | |
| candidate-b | 0.81 | |
| candidate-c | 0.44 | |
| candidate-d | 0.39 | |
| candidate-e | 0.74 | |

## Part 2: Add Experimental Labels

| Candidate | Decision | Experimental label |
| --- | --- | --- |
| candidate-a | | active |
| candidate-b | | inactive |
| candidate-c | | inactive |
| candidate-d | | active |
| candidate-e | | unknown |

Which row cannot be used in the confusion matrix yet?

```text

```

## Part 3: Confusion Matrix

Fill in counts.

| | Experiment active | Experiment inactive |
| --- | --- | --- |
| Predicted test | | |
| Predicted defer | | |

True positives:

```text

```

False positives:

```text

```

True negatives:

```text

```

False negatives:

```text

```

## Part 4: Metrics

Precision:

```text
true positives / predicted positives =
```

Recall:

```text
true positives / actual positives =
```

Accuracy:

```text
(true positives + true negatives) / all labeled predictions =
```

## Part 5: Uncertainty Near The Threshold

Circle scores within `0.12` of the threshold.

| Candidate | Score | Near threshold? |
| --- | --- | --- |
| candidate-a | 0.92 | |
| candidate-b | 0.81 | |
| candidate-c | 0.44 | |
| candidate-d | 0.39 | |
| candidate-e | 0.74 | |

Which near-threshold candidate should be inspected first, and why?

```text

```

## Part 6: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test screening
```

Write one test name that passed.

```text

```

## Reflection

Why is a score not experimental truth?

```text

```
