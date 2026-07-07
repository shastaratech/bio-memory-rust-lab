# Mermaid: Graph Algorithms

## Neighbor Lookup

```mermaid
flowchart LR
    A["atom_id"] --> B["scan bonds"]
    B --> C{"bond touches atom_id?"}
    C -- "yes" --> D["push other endpoint"]
    C -- "no" --> E["skip"]
    D --> F["neighbors"]
    E --> B
```

## Breadth-First Search

```mermaid
flowchart TD
    A["start atom"] --> B["queue = [start]"]
    B --> C{"queue empty?"}
    C -- "yes" --> Z["no path"]
    C -- "no" --> D["pop front"]
    D --> E{"is goal?"}
    E -- "yes" --> F["reconstruct path"]
    E -- "no" --> G["visit unvisited neighbors"]
    G --> H["record previous atom"]
    H --> I["push neighbors"]
    I --> C
```

## Connected Components

```mermaid
flowchart TD
    A["all atoms"] --> B["pick unvisited atom"]
    B --> C["run BFS"]
    C --> D["collect reached atoms"]
    D --> E["one component"]
    E --> F{"unvisited atoms left?"}
    F -- "yes" --> B
    F -- "no" --> G["all fragments found"]
```

Teaching prompt:

Have students run the same algorithm by hand with string bonds before they read the
Rust function.

