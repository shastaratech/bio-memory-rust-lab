# Mermaid: Traits and Functions

If GitHub Mermaid rendering is unavailable in your browser, use this rendered SVG:

![Traits and functions](../rendered/traits-and-functions.svg)

The editable Mermaid source is below.

## Behavior Layers

```mermaid
flowchart TD
    A["Element enum"] --> B["Atom struct"]
    A --> C["BondOrder enum"]
    C --> D["Bond struct"]
    B --> E["Molecule struct"]
    D --> E

    E --> F["ChemicalFormula trait"]
    E --> G["MolecularGraph trait"]
    B --> H["Describe trait"]
    D --> H
    E --> H

    F --> I["formula_counts()"]
    F --> J["formula()"]
    G --> K["neighbors()"]
    G --> L["shortest_path()"]
    H --> M["describe()"]
```

## Function Contracts

```mermaid
flowchart LR
    A["function name"] --> B["inputs"]
    B --> C["borrow or own?"]
    C --> D["return type"]
    D --> E{"can fail?"}
    E -- "yes" --> F["Option or Result"]
    E -- "no" --> G["plain value"]
```

## Trait as Capability

```mermaid
classDiagram
    class Describe {
        <<trait>>
        +describe() String
    }

    class ChemicalFormula {
        <<trait>>
        +formula_counts() Vec
        +formula() String
    }

    class MolecularGraph {
        <<trait>>
        +atom_count() usize
        +bond_count() usize
        +neighbors(atom_id) Vec
        +shortest_path(start, goal) Option
    }

    class Atom
    class Bond
    class Molecule

    Atom ..|> Describe
    Bond ..|> Describe
    Molecule ..|> Describe
    Molecule ..|> ChemicalFormula
    Molecule ..|> MolecularGraph
```
