# Mermaid: Replicates And Uncertainty

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Replicates and uncertainty](../rendered/replicates-uncertainty.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Replicate responses<br/>40, 44, 46"] --> B["Validate<br/>finite and 0..100"]
    B --> C["Vector of values<br/>ReplicateSeries"]
    C --> D["Mean<br/>center"]
    C --> E["Range<br/>max - min"]
    C --> F["Sample std dev<br/>spread"]
    D --> G["Interpretation<br/>signal plus uncertainty"]
    E --> G
    F --> G
    G --> H["Feedback decision<br/>trust, repeat, or inspect"]
```

Teaching prompt:

Ask students why two candidates can have similar means but different decisions.
