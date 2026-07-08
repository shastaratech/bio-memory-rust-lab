# Mermaid: Screening Feedback Loop

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Screening feedback loop](../rendered/screening-feedback-loop.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Candidates<br/>from design or library"] --> B["Score<br/>model or rule"]
    B --> C["Threshold<br/>test or defer"]
    C --> D["Testing queue<br/>ranked selected candidates"]
    D --> E["Experiment<br/>active or inactive label"]
    E --> F["Confusion matrix<br/>TP FP TN FN"]
    F --> G["Metrics<br/>precision recall accuracy"]
    G --> H["Feedback<br/>revise features, scores, threshold"]
    H --> B
```

Teaching prompt:

Ask students where uncertainty enters the loop and which data structure stores it.
