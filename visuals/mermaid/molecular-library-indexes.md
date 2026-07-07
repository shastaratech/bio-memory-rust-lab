# Mermaid: Molecular Library Indexes

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Molecular library indexes](../rendered/molecular-library-indexes.svg)

The editable Mermaid source is below.

```mermaid
flowchart LR
    A["Vec<MoleculeRecord><br/>library shelf"] --> B["loop over records"]
    A --> C["HashMap name -> record"]
    A --> D["HashMap formula -> Vec<name>"]
    A --> E["sorted Vec by atom count"]
    E --> F["binary search range boundary"]
    E --> G["merge with another sorted library"]
    C --> H["direct lookup query"]
    D --> I["grouped formula query"]
```

Teaching prompt:

Ask students which structure they would build for each query before showing the
Rust implementation.
