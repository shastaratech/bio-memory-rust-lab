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

    B --> S1["School: atom cards"]
    C --> S2["School: human bond graph"]
    D --> S3["School: owner / reader / editor roles"]
    E --> S4["School: quantum wave cards"]

    B --> U1["University: type invariants"]
    C --> U2["University: adjacency lists and BFS"]
    D --> U3["University: API design and clone cost"]
    E --> U4["University: VQE intuition"]
    F --> U5["University: representation tradeoffs"]
    G --> U6["University: trait-based APIs"]
```

Teaching prompt:

Ask students where they are in the map before and after each class. The map should
make the course feel like a journey from concrete objects to abstract state.
