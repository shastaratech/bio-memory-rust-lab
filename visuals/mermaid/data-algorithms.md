# Mermaid: Data Algorithms

## Count Elements With A Hash Map

```mermaid
flowchart TD
    A["Vec<Atom>"] --> B["read next atom"]
    B --> C{"element already in map?"}
    C -- "yes" --> D["increment count"]
    C -- "no" --> E["insert count = 1"]
    D --> F{"more atoms?"}
    E --> F
    F -- "yes" --> B
    F -- "no" --> G["HashMap<Element, usize>"]
```

## Sort Counts For Formula Output

```mermaid
flowchart LR
    A["element counts"] --> B["sort by chemistry order"]
    B --> C["format each pair"]
    C --> D["stable formula string"]
```

## Merge Atom Records With Labels

```mermaid
flowchart TD
    A["atoms by atom_id"] --> C["merge on atom_id"]
    B["labels by atom_id"] --> C
    C --> D{"all labels valid?"}
    D -- "yes" --> E["labeled atom table"]
    D -- "no" --> F["report missing atom_id"]
```

## Choose A Representation

```mermaid
flowchart TD
    A["What question is frequent?"] --> B{"Need order?"}
    B -- "yes" --> C["Vec or sorted Vec"]
    B -- "no" --> D{"Need lookup by key?"}
    D -- "yes" --> E["HashMap"]
    D -- "no" --> F{"Need connectivity?"}
    F -- "yes" --> G["Graph / adjacency list"]
    F -- "no" --> H{"Need shape?"}
    H -- "yes" --> I["3D coordinates"]
    H -- "no" --> J["simple struct or enum"]
```

Teaching prompt:

Ask students to name the question first, then pick the data structure.
