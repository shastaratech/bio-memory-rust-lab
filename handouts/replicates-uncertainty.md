# Handout: Replicates, Variability, And Uncertainty

## Core Idea

Repeated measurements show whether a response is stable or noisy.

```text
candidate-a: 40, 41, 42
candidate-b: 20, 50, 80
```

Both are response series. They do not carry the same uncertainty.

## Key Summaries

| Summary | Meaning |
| --- | --- |
| mean | average response |
| min | lowest response |
| max | highest response |
| range | max minus min |
| sample variance | squared spread around mean |
| standard deviation | spread in response units |
| coefficient of variation | standard deviation divided by mean |

## Edge Rules

Reject:

- empty replicate lists
- `NaN` or infinite values
- response percent below `0`
- response percent above `100`

Single-replicate series:

- has a mean
- has no sample variance
- cannot estimate spread

## Interpretation Rule

A mean is not enough. Always ask:

- How many replicates?
- How spread out are they?
- Were the values valid?
- What controls and conditions produced them?
