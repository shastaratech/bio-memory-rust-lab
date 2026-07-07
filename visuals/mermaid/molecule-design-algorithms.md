# Mermaid: Molecule Design Algorithms

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Molecule design algorithms](../rendered/molecule-design-algorithms.svg)

The editable Mermaid source is below.

## Design Round

```mermaid
flowchart LR
    A["scaffold"] --> B["Vec<Substituent>"]
    B --> C["generate candidates"]
    C --> D["validate / filter"]
    D --> E["compute features"]
    E --> F["score"]
    F --> G["sort by score"]
    G --> H["top candidates"]
    H --> I["next design round"]
```

## Candidate Data Flow

```mermaid
flowchart TD
    A["Candidate"] --> B{"valid bonds?"}
    B -- "no" --> X["reject: invalid graph"]
    B -- "yes" --> C{"allowed elements?"}
    C -- "no" --> Y["reject: unsupported element"]
    C -- "yes" --> D["compute formula counts"]
    D --> E["compute graph features"]
    E --> F["score candidate"]
    F --> G["ranked list"]
```

## Data Structures In Design

```mermaid
flowchart TD
    A["Design question"] --> B{"Need candidate order?"}
    B -- "yes" --> C["Vec<Candidate>"]
    B -- "no" --> D{"Need lookup by name?"}
    D -- "yes" --> E["HashMap<String, Candidate>"]
    D -- "no" --> F{"Need connectivity?"}
    F -- "yes" --> G["Molecule graph"]
    F -- "no" --> H{"Need best candidates?"}
    H -- "yes" --> I["sorted Vec or priority queue"]
    H -- "no" --> J["simple struct fields"]
```

## Search Frontier

```mermaid
flowchart LR
    A["current top candidates"] --> B["expand each candidate"]
    B --> C["new candidate frontier"]
    C --> D["filter"]
    D --> E["score"]
    E --> F["keep top N"]
    F --> A
```

Teaching prompt:

Ask students to say which data structure owns each stage of the design round.
