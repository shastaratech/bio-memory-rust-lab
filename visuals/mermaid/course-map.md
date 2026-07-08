# Mermaid: Course Map

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Bio Memory Rust Lab course map](../rendered/course-map.svg)

The editable Mermaid source is below.

```mermaid
flowchart TD
    A["Start: What is structure?"] --> B["Lesson 01: Atoms, Memory, and Types"]
    B --> C["Lesson 02: Molecules as Graphs"]
    C --> D["Lesson 03: Ownership and Borrowing"]
    D --> E["Lesson 04: Quantum State and Simulation"]
    E --> F["Lesson 05: Rust Data Structures for Molecules"]
    F --> G["Lesson 06: Traits and Functions"]
    G --> H["Lesson 07: Molecule Explorer CLI Capstone"]
    H --> I["Lesson 08: DNA, Biological Memory, and Data Structures"]
    I --> J["Lesson 09: Visualizing Molecules and Data Algorithms"]
    J --> K["Lesson 10: Designing Molecules as Data Search"]
    K --> L["Lesson 11: Molecular Libraries and Search Indexes"]
    L --> M["Lesson 12: Toy Fingerprints and Similarity Search"]
    M --> N["Lesson 13: Screening Results and Feedback Loops"]

    B --> S1["School: atom cards"]
    C --> S2["School: human bond graph"]
    D --> S3["School: owner / reader / editor roles"]
    E --> S4["School: quantum wave cards"]
    H --> S7["School: command prediction lab"]
    I --> S8["School: DNA base cards"]
    J --> S9["School: data structure stations"]
    K --> S10["School: design stations"]
    L --> S11["School: library index cards"]
    M --> S12["School: feature bit cards"]
    N --> S13["School: screening matrix cards"]

    B --> U1["University: type invariants"]
    C --> U2["University: adjacency lists and BFS"]
    D --> U3["University: API design and clone cost"]
    E --> U4["University: VQE intuition"]
    F --> U5["University: representation tradeoffs"]
    G --> U6["University: trait-based APIs"]
    H --> U7["University: command parsing and tests"]
    I --> U8["University: sequence memory models"]
    J --> U9["University: sorting, merging, maps"]
    K --> U10["University: constrained search"]
    L --> U11["University: hash indexes and binary search"]
    M --> U12["University: bitsets and similarity ranking"]
    N --> U13["University: metrics and feedback loops"]
```

Teaching prompt:

Ask students where they are in the map before and after each class. The map should
make the course feel like a journey from concrete objects to abstract state.
