# Handout: Screening Results And Feedback Loops

## Core Idea

A score is a prediction. An experimental label is measured feedback.

Useful screening asks:

```text
score -> decision -> experiment -> label -> metric -> next round
```

## Threshold

```text
score >= threshold -> test
score < threshold  -> defer
```

## Confusion Matrix

| | Experiment active | Experiment inactive |
| --- | --- | --- |
| Predicted test | true positive | false positive |
| Predicted defer | false negative | true negative |

## Metrics

| Metric | Formula | Question |
| --- | --- | --- |
| precision | `TP / (TP + FP)` | When we tested, how often was it active? |
| recall | `TP / (TP + FN)` | Of active candidates, how many did we catch? |
| accuracy | `(TP + TN) / total` | How often did the decision match the label? |

## Feedback Loop

Good follow-up candidates can include:

- high-scoring untested candidates
- false positives
- false negatives
- candidates near the threshold
- candidates with missing labels

## Caution

The toy matrix teaches reasoning about predictions. It does not replace chemistry,
biology, assay controls, or safety review.
