# Mermaid: Controls And Normalization

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Controls and normalization](../rendered/controls-normalization.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Raw response<br/>candidate signal"] --> B["Blank correction<br/>subtract background"]
    C["ControlSet<br/>blank, negative, positive"] --> B
    B --> D["Control window<br/>positive - negative"]
    D --> E["Normalize<br/>0..1 fraction"]
    E --> F["Percent response<br/>fraction * 100"]
    D --> G["Quality gate<br/>is window large enough?"]
```

Teaching prompt:

Ask students why a raw response cannot be interpreted without reference controls.
