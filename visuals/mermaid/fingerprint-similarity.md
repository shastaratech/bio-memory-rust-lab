# Mermaid: Toy Fingerprints And Similarity Search

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Toy fingerprints and similarity search](../rendered/fingerprint-similarity.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Molecule graph<br/>atoms + bonds"] --> B["Feature questions<br/>has O? has C-C bond?"]
    B --> C["ToyFingerprint<br/>u64 bitset"]
    C --> D["Intersection<br/>shared bits"]
    C --> E["Union<br/>bits in either"]
    D --> F["Similarity score<br/>shared / union"]
    E --> F
    F --> G["Sorted matches<br/>nearest records first"]
```

Teaching prompt:

Ask students what information disappears when a molecule becomes a fingerprint.
