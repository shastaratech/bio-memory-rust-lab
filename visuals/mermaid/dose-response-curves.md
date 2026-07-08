# Mermaid: Dose-Response Curves

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Dose-response curves](../rendered/dose-response-curves.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Dose points<br/>concentration + response"] --> B["Validate<br/>responses 0..100"]
    B --> C["Sort by concentration<br/>common unit"]
    C --> D["Curve vector<br/>ordered points"]
    D --> E["Check monotonicity<br/>does response increase?"]
    D --> F["Search adjacent points<br/>target crossing"]
    F --> G["Interpolate<br/>toy estimate"]
    G --> H["Option result<br/>Some concentration or None"]
```

Teaching prompt:

Ask students what assumptions are hidden inside interpolation.
