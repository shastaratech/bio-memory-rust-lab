# Mermaid: Typed Measurements And Assay Observations

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Typed measurements and assay observations](../rendered/typed-measurements.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Raw value<br/>500"] --> B["Attach unit<br/>500 nM"]
    B --> C["Concentration struct<br/>value + unit"]
    C --> D["Validate<br/>finite and non-negative"]
    D --> E["Normalize<br/>micromolar"]
    E --> F["AssayObservation<br/>candidate + dose + response"]
    F --> G["Sort / average / feedback<br/>common meaning"]
```

Teaching prompt:

Ask students where a bare number can create a wrong scientific conclusion.
