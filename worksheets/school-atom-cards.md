# Worksheet: Atom Cards

## Goal

Build a molecule from atom cards, then connect the cards to Rust data types.

## Materials

- index cards
- markers
- string or yarn
- sticky notes

## Card Setup

Create one card for each atom.

On each card, write:

| Field | Example |
| --- | --- |
| Atom ID | `0` |
| Element | `O` |
| Formal charge | `0` |
| Aromatic? | `false` |

## Element Cards

Use only the element symbols needed for the starter molecules:

| Element name | Symbol |
| --- | --- |
| Hydrogen | `H` |
| Carbon | `C` |
| Nitrogen | `N` |
| Oxygen | `O` |
| Chlorine | `Cl` |

## Activity 1: Build Water

Water has formula:

```text
H2O
```

Fill in the atom cards:

| Atom ID | Element | Formal charge | Aromatic? |
| --- | --- | --- | --- |
| `0` | | | |
| `1` | | | |
| `2` | | | |

Now add bond strings:

| Bond ID | From atom | To atom | Bond order |
| --- | --- | --- | --- |
| `0` | | | single |
| `1` | | | single |

## Activity 2: Match Rust

The Rust model uses:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

Circle the Rust type that matches each card field:

| Card field | Rust type choices |
| --- | --- |
| Element | `Element`, `String`, `bool` |
| Formal charge | `i8`, `bool`, `Vec<Atom>` |
| Aromatic? | `bool`, `usize`, `BondOrder` |
| Atom ID | `usize`, `Element`, `Molecule` |

## Activity 3: Human Molecule

1. Each student holds one atom card.
2. Bond strings connect the students.
3. One student asks: "Who are atom 0's neighbors?"
4. Students connected to atom 0 raise their cards.

Write the neighbor list:

```text
neighbors(0) = [        ]
```

## Exit Ticket

Answer in one sentence each:

1. Why is an enum safer than writing any element name as text?
2. Why does a bond need two atom IDs?
3. What does the card activity leave out about real molecules?
