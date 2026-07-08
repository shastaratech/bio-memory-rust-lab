# Chapter 00: Atoms, Elements, Bonds, Molecules, And Formulas

## Big Idea

Before we model molecules in Rust, we need a small chemistry vocabulary.

This chapter explains:

- atom
- element
- bond
- bond order
- aromaticity
- molecule
- chemical formula
- why formulas such as `H2O`, `CH4`, and `C2H6O` are useful compact
  representations

You do not need to be a chemist. We are learning enough chemistry to understand the
data structures.

## Atom

An atom is a tiny unit of matter.

For this course, think of an atom as one node in a molecule.

Examples:

- one hydrogen atom
- one oxygen atom
- one carbon atom

In Rust, our toy model represents an atom with:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

That means an atom is not only a letter. It is a record with fields.

## Element

An element is a kind of atom.

Examples:

| Element name | Symbol | Rust enum variant |
| --- | --- | --- |
| Hydrogen | `H` | `Element::H` |
| Carbon | `C` | `Element::C` |
| Nitrogen | `N` | `Element::N` |
| Oxygen | `O` | `Element::O` |
| Chlorine | `Cl` | `Element::Cl` |

In this course, we use an enum:

```rust
pub enum Element {
    H,
    C,
    N,
    O,
    F,
    Cl,
    Br,
}
```

Why?

Because element symbols are a controlled vocabulary. `Element::Cl` is safer than
free text such as `"cl"`, `"CL"`, or `"chlorine"`.

## Bond

A bond is a connection between atoms.

For this course, think of a bond as an edge in a graph.

Water has:

- one oxygen atom
- two hydrogen atoms
- two O-H bonds

In Rust:

```rust
pub struct Bond {
    pub from: usize,
    pub to: usize,
    pub order: BondOrder,
}
```

`from` and `to` are atom IDs. `order` says whether the bond is single, double,
triple, or aromatic in the toy model.

## Bond Types

The word bond means "chemical connection between atoms." In diagrams, chemists draw
bonds as lines:

```text
H - O - H
```

That line is a useful symbol. It is not a tiny physical stick between atoms.

In this course, the Rust model uses bond orders:

| Bond order | Drawing shortcut | Beginner meaning |
| --- | --- | --- |
| `Single` | `-` | one ordinary connection in the toy graph |
| `Double` | `=` | a stronger/more electron-rich connection than a single bond |
| `Triple` | `≡` | a still higher bond order |
| `Aromatic` | ring notation | a special delocalized bonding pattern in some rings |

For example, ethylene is often drawn as:

```text
H2C = CH2
```

The `=` means the two carbon atoms are connected by a double bond.

## What A Bond Means Physically

A chemical bond is not a material string. Atoms contain positively charged nuclei
and surrounding electron clouds. When atoms come close enough, their outer electron
clouds can overlap.

For a simple covalent bond, shared electron density between two nuclei helps hold
the atoms together:

```text
nucleus (+)    shared electron density    nucleus (+)
    ●  <------------- cloud ------------->  ●
```

If we could draw the electron density, we would draw a cloudy region rather than a
straight line:

```text
      .:::::::.
   .::::::::::::.
 ●::::::::::::::::●
   '::::::::::::'
      ':::::::'
```

Chemists still draw lines because they are compact and useful. A line means:

> These two atoms are chemically connected in this representation.

For programming, this is exactly what the beginner graph needs. The graph edge does
not model the full quantum electron distribution; it records that two atom records
are connected.

## Aromaticity

Aromaticity does not mean smell in this course.

Aromaticity is a special electronic state found in some cyclic molecules. Benzene
is the standard beginner example. It is often drawn as a six-membered ring:

```text
      C
   /     \
  C       C
  |       |
  C       C
   \     /
      C
```

In benzene, some electrons are not assigned to one single bond. They are
delocalized around the ring. Beginner translation:

- the electrons are spread over the ring
- the bonds are not best described as purely single or purely double
- the ring has a special stable bonding pattern

That is why chemistry software often stores aromaticity explicitly:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}

pub enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}
```

Libraries such as RDKit, Open Babel, and CDK need aromaticity because algorithms
may use it for similarity search, substructure search, charge handling, and property
prediction. This course uses the same idea at toy scale: `aromatic: bool` and
`BondOrder::Aromatic` are fields that make an important chemical feature visible to
the data structure.

Important limit:

> Aromaticity detection is a real chemistry algorithm. In this course, we store an
> aromatic flag; we do not teach full aromaticity perception.

## Molecule

A molecule is a group of atoms connected by bonds.

Examples:

- water
- methane
- ethanol

In Rust:

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

This is already a data structure:

- `Vec<Atom>` stores the atom records.
- `Vec<Bond>` stores the connections.
- Together they describe a molecular graph.

## Chemical Formula

A chemical formula is a compact count of atoms by element.

It does not show every bond. It does not show 3D shape. It answers one important
question:

> How many atoms of each element are in this molecule?

## How To Read `H2O`

`H2O` means:

- `H2`: two hydrogen atoms
- `O`: one oxygen atom

If no number appears after an element symbol, the count is one.

So:

```text
H2O = 2 hydrogen atoms + 1 oxygen atom
```

Run:

```bash
cd exercises/rust-molecule-model
cargo run -- water summary
```

Expected output includes:

```text
formula: H2O
atoms: 3
bonds: 2
```

## How To Read `CH4`

`CH4` means:

- `C`: one carbon atom
- `H4`: four hydrogen atoms

So:

```text
CH4 = 1 carbon atom + 4 hydrogen atoms
```

Run:

```bash
cargo run -- methane summary
```

Expected output includes:

```text
formula: CH4
atoms: 5
bonds: 4
```

## How To Read `C2H6O`

`C2H6O` means:

- `C2`: two carbon atoms
- `H6`: six hydrogen atoms
- `O`: one oxygen atom

So:

```text
C2H6O = 2 carbon atoms + 6 hydrogen atoms + 1 oxygen atom
```

Run:

```bash
cargo run -- ethanol summary
```

Expected output includes:

```text
formula: C2H6O
atoms: 9
bonds: 8
```

## Why Formulas Are Data Representations

A formula is a compressed representation.

| Formula | What it preserves | What it leaves out |
| --- | --- | --- |
| `H2O` | element counts | exact bond list, 3D shape, electrons |
| `CH4` | element counts | geometry and bond angles |
| `C2H6O` | element counts | which atoms connect to which |

This is the first major modeling lesson:

> A representation keeps some information and leaves other information out.

That is also true in programming.

## Formula Versus Graph

The formula `C2H6O` tells us ethanol has:

- two carbon atoms
- six hydrogen atoms
- one oxygen atom

But it does not tell us how they are connected.

The graph representation stores connectivity:

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

That is why we need both ideas:

- formula for counts
- graph for connections

## Checkpoint

1. What is an atom?
2. What is an element?
3. What is a bond?
4. What is a molecule?
5. What does the `2` mean in `H2O`?
6. What does `CH4` mean?
7. What information does `C2H6O` leave out?
