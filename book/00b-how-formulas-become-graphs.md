# Chapter 00b: How Formulas Become Graphs

## Big Idea

A chemical formula tells us what atoms are present.

A molecular graph tells us how those atoms are connected.

Rust needs the graph because many useful questions are connectivity questions:

- Which atoms are directly bonded?
- Is there a path between two atoms?
- Is the molecule one connected piece?
- Does every bond point to a real atom?

## What You Already Know

From the chemistry primer:

- `H2O` means two hydrogen atoms and one oxygen atom.
- `CH4` means one carbon atom and four hydrogen atoms.
- `C2H6O` means two carbon atoms, six hydrogen atoms, and one oxygen atom.
- A formula is compact, but it leaves out bond connections.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Formula | A count of atoms by element. |
| Structure | The way atoms are connected. |
| Graph | Nodes plus edges. |
| Node | One atom in the molecule graph. |
| Edge | One bond in the molecule graph. |
| Bond order | The model's label for single, double, triple, or aromatic bonds. |
| Aromaticity | A special delocalized bonding pattern in some rings, stored as a flag in the toy model. |
| Isomer | A molecule with the same formula as another molecule, but a different structure. |

## Formula Is Not Structure

The formula `C2H6O` tells us the atom counts:

```text
C2H6O = 2 carbon atoms + 6 hydrogen atoms + 1 oxygen atom
```

It does not tell us where the oxygen atom connects.

That matters because the same formula can describe different structures.

For example:

- ethanol has the shape `C-C-O`
- dimethyl ether has the shape `C-O-C`

Both have formula `C2H6O`.

They are not the same molecule.

## Why This Matters For Rust

If we store only a formula string:

```rust
let formula = "C2H6O";
```

we can answer:

> How many atoms of each element are present?

But we cannot answer:

> Which atoms are bonded to atom 1?

For that, we need atoms and bonds.

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

This structure turns the molecule into a graph:

- each `Atom` is a node
- each `Bond` is an edge
- each bond stores two atom IDs
- each bond also stores a bond order, such as single or double

## Water As A Graph

Water has formula `H2O`.

The formula gives us:

```text
O, H, H
```

The graph adds the connections:

```text
H - O - H
```

The Rust model stores that as:

```rust
vec![
    Atom::neutral(Element::O),
    Atom::neutral(Element::H),
    Atom::neutral(Element::H),
]
```

and:

```rust
vec![
    Bond::new(0, 1, BondOrder::Single),
    Bond::new(0, 2, BondOrder::Single),
]
```

The atom IDs are:

| Atom ID | Element |
| --- | --- |
| `0` | oxygen |
| `1` | hydrogen |
| `2` | hydrogen |

The bonds say:

- atom `0` connects to atom `1`
- atom `0` connects to atom `2`

## Run The Model

From the repository root:

```bash
cd exercises/rust-molecule-model
cargo run -- water formula
cargo run -- water atoms
cargo run -- water bonds
cargo run -- water neighbors 0
```

Expected neighbor output:

```text
[1, 2]
```

That means atom `0`, the oxygen, is bonded to atoms `1` and `2`.

## Ethanol As A Graph

Ethanol has formula `C2H6O`.

Run:

```bash
cargo run -- ethanol atoms
cargo run -- ethanol bonds
```

Then run:

```bash
cargo run -- ethanol path 3 8
```

Expected output:

```text
[3, 0, 1, 2, 8]
```

Read this as:

> Start at atom 3, move through atoms 0, 1, and 2, and end at atom 8.

The formula alone cannot answer that path question. The graph can.

## Common Confusion

### "If the formula is correct, is the molecule correct?"

Not always.

A formula can have the right atom counts while the bonds are wrong.

### "Are Rust references the same as chemical bonds?"

No.

A bond is part of the molecule model. A Rust reference is temporary program access
to a value.

They are both about relationships, but they are not the same kind of relationship.

### "Why not store bonds inside atoms?"

That is possible, but this beginner model keeps atoms and bonds in separate vectors.

This makes the representation easy to inspect:

- one list for atom records
- one list for bond records

Later, we can build an adjacency list when graph queries need to be faster.

## Try It

Choose water, methane, or ethanol.

1. Write its formula.
2. Count its atoms.
3. Draw its graph with atom IDs.
4. Write one bond as `from -> to`.
5. Run the matching CLI command to check your answer.

Useful commands:

```bash
cargo run -- water summary
cargo run -- methane bonds
cargo run -- ethanol neighbors 1
```

## Checkpoint

1. What information does a formula preserve?
2. What information does a formula leave out?
3. Why can two molecules have the same formula but different structures?
4. In the Rust model, what is a graph node?
5. In the Rust model, what is a graph edge?
6. Why does `path` need bonds, not just a formula?
