# Lesson 13: Screening Results And Feedback Loops

## Goal

Students learn how scores become test/defer decisions, how experimental labels
measure prediction quality, and how feedback loops improve later rounds.

## Big Question

How do we know whether a molecular screen is helping or misleading us?

## Visuals

- [Screening feedback loop](../visuals/mermaid/screening-feedback-loop.md)
- [Toy fingerprints and similarity search](../visuals/mermaid/fingerprint-similarity.md)
- [Molecule design algorithms](../visuals/mermaid/molecule-design-algorithms.md)

## Core Analogy

| Screening idea | Data-structure idea |
| --- | --- |
| score | floating-point value |
| threshold | boundary |
| test/defer | enum decision |
| experimental result | optional label |
| selected candidates | queue |
| prediction quality | confusion matrix |
| next round | feedback loop |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Ask whether the highest score is always the best experiment. |
| 10-25 min | Threshold | Turn score cards into test/defer piles. |
| 25-40 min | Labels | Add experimental active/inactive labels. |
| 40-55 min | Matrix | Fill a confusion matrix. |
| 55-70 min | Metrics | Compute precision, recall, and accuracy. |
| 70-90 min | Feedback | Choose what the next round should inspect. |

## Part 1: Score Cards

Give students candidate cards:

```text
candidate-a 0.92
candidate-b 0.81
candidate-c 0.44
candidate-d 0.39
candidate-e 0.74
```

Choose threshold:

```text
0.70
```

Students sort cards into:

- test
- defer

## Part 2: Experimental Labels

Add labels:

```text
candidate-a active
candidate-b inactive
candidate-c inactive
candidate-d active
candidate-e unknown
```

Teaching line:

A prediction becomes evidence only when compared with measurement.

## Part 3: Confusion Matrix

Draw:

| | Experiment active | Experiment inactive |
| --- | --- | --- |
| Predicted test | | |
| Predicted defer | | |

Students place labeled cards into the four boxes.

## Part 4: Metrics

Ask:

- Precision: when we tested, how often were we right?
- Recall: of active molecules, how many did we catch?
- Accuracy: how often did test/defer match the label?

Do not let students treat one metric as the whole story.

## Part 5: Feedback Loop

Ask:

> Which candidate should the next experiment teach us about?

Discuss:

- high scores
- false positives
- false negatives
- candidates near the threshold
- missing labels

## University Extension

Ask students to inspect:

```text
exercises/rust-molecule-model/src/screening.rs
```

Discussion prompts:

- Why does `Prediction` use `Option<ExperimentalLabel>`?
- Which metrics can have zero denominators?
- Why does `testing_queue` sort before returning a queue?
- Why might uncertain predictions be valuable?
- What changes if active/inactive becomes a continuous measurement?

## Caution

This module teaches screening logic, not real assay interpretation. Real
experimental decisions require chemistry, biology, uncertainty, controls, and
safety review.
