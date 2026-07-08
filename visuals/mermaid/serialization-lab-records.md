# Mermaid: Serialization And Lab Records

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Serialization and lab records](../rendered/serialization-lab-records.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Molecule graph<br/>atoms + bonds"] --> B["LabRecord struct<br/>schema fields"]
    B --> C["Serialize<br/>pipe-separated text"]
    C --> D["Store or share<br/>lab note / report"]
    D --> E["Parse<br/>split and validate"]
    E --> F["LabRecord struct<br/>read back"]
    E --> G["Parse error<br/>field count, empty field, bad number"]
    F --> H["Round-trip check<br/>same structured record?"]
```

Teaching prompt:

Ask students what scientific context is missing from the toy record.
