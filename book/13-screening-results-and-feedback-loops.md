# Chapter 13: Screening Results And Feedback Loops

## Big Idea

A similarity score or model score is not the same thing as experimental truth.

After a molecule is scored, a real workflow must decide:

1. Which candidates should be tested?
2. Which candidates should wait?
3. Which predictions were correct after experiment?
4. Which predictions were wrong?
5. What should the next round learn from the results?

This chapter teaches screening as a data workflow with thresholds, labels,
confusion matrices, queues, and feedback loops.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Prediction | A model or rule-based score for a candidate. |
| Threshold | The score boundary used to choose `test` or `defer`. |
| Experimental label | A measured outcome such as active or inactive. |
| True positive | Predicted test, experimentally active. |
| False positive | Predicted test, experimentally inactive. |
| True negative | Predicted defer, experimentally inactive. |
| False negative | Predicted defer, experimentally active. |
| Precision | Of tested predictions, how many were active? |
| Recall | Of active molecules, how many did we catch? |
| Feedback loop | Using measured outcomes to improve the next round. |

## Why Scores Need Labels

A score can rank candidates, but it does not prove biology or chemistry.

Toy prediction:

```text
candidate-a score 0.92 -> test
candidate-b score 0.81 -> test
candidate-c score 0.44 -> defer
```

Experimental labels arrive later:

```text
candidate-a active
candidate-b inactive
candidate-c inactive
```

The prediction became useful only after it was compared with measurement.

## Rust Model

Open:

```text
exercises/rust-molecule-model/src/screening.rs
```

Core types:

```rust
enum ExperimentalLabel {
    Active,
    Inactive,
}

enum ScreeningDecision {
    Test,
    Defer,
}

struct Prediction {
    candidate_name: String,
    score: f64,
    label: Option<ExperimentalLabel>,
}
```

Why `Option<ExperimentalLabel>`?

Some candidates have not been tested yet.

That matters because a candidate can move through states:

```text
scored -> selected -> tested -> labeled -> used as feedback
```

## Thresholds

A threshold turns a score into a decision.

```text
score >= 0.70 -> test
score <  0.70 -> defer
```

Data-structure analogy:

| Screening idea | Computer idea |
| --- | --- |
| score | `f64` value |
| threshold | boundary value |
| test/defer | enum decision |
| candidates to test | queue |
| measured result | optional label |

## Confusion Matrix

A confusion matrix compares predictions with experimental labels.

| | Experiment active | Experiment inactive |
| --- | --- | --- |
| Predicted test | true positive | false positive |
| Predicted defer | false negative | true negative |

This matrix is not about blame. It is a debugging table for scientific decisions.

## Precision, Recall, And Accuracy

Precision:

```text
true positives / predicted positives
```

Question:

> When we chose to test, how often was the candidate active?

Recall:

```text
true positives / actual positives
```

Question:

> Of the active candidates, how many did the screen catch?

Accuracy:

```text
(true positives + true negatives) / all labeled predictions
```

Question:

> How often did test/defer match the experimental label?

Each metric answers a different question.

## Testing Queue

After thresholding, the selected candidates can be sorted by score and placed into
a queue:

```text
front -> candidate-a, candidate-b, candidate-e -> back
```

This is not just programming. It mirrors lab planning:

- limited budget
- limited time
- limited assay capacity
- need for a clear order

## Uncertainty Near The Threshold

Candidates near the threshold deserve attention.

Example:

```text
threshold = 0.70
candidate-e = 0.74
candidate-b = 0.81
candidate-c = 0.44
```

If the uncertainty margin is `0.12`, candidate-e and candidate-b are close enough
to inspect.

Why this matters:

The most informative next experiment is not always the highest-scoring molecule.
Sometimes it is the molecule that can teach the model where its boundary is weak.

## Feedback Loop

Screening is a loop:

```text
score candidates
choose threshold
test selected candidates
collect labels
measure errors
revise features, scores, or thresholds
repeat
```

This connects back to earlier modules:

- molecule design generates candidates
- fingerprints create compact features
- library indexes organize records
- screening scores choose what to test
- experimental labels update the next round

## DNA And Memory Connection

DNA comparison, fingerprints, and screening all involve stored information.

| Biology / chemistry | Computer structure |
| --- | --- |
| DNA sequence | ordered biological information |
| molecule graph | structured record |
| fingerprint | compact feature memory |
| prediction score | numeric estimate |
| experimental label | measured value |
| feedback loop | updated data pipeline |

The analogy has limits. Biology is not reducible to a matrix. But matrices help
students see where a model is right, wrong, useful, and risky.

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test screening
```

Look for tests that:

- convert scores to decisions
- build a confusion matrix
- compute precision, recall, and accuracy
- build a testing queue
- find uncertain predictions near a threshold

## Checkpoint

1. What does a threshold do?
2. Why is an experimental label optional?
3. What is a false positive?
4. What is a false negative?
5. How are precision and recall different?
6. Why might a candidate near the threshold be useful?
7. Why is a score not experimental truth?
