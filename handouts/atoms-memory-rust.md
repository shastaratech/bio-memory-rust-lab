# Student Handout: Molecules, Memory, and Rust

## Big Idea

Programming and chemistry both care about structure.

In chemistry, atoms combine under rules to form molecules. In Rust, values combine
under type and ownership rules to form reliable programs. In computer memory, values
must live somewhere. In quantum mechanics, molecules have states that cannot be fully
treated like ordinary stored bits.

## Vocabulary

| Word | In this lesson |
| --- | --- |
| Type | A rule for what kind of value something is. |
| Scalar | One basic value, like `u8`, `i8`, `bool`, `char`, or `f64`. |
| Struct | A named group of fields. |
| Enum | A fixed set of variants. |
| Vector | A growable list, written `Vec<T>`. |
| Ownership | Rust's rule that a value has one owner responsible for it. |
| Borrowing | Temporary access to a value without taking ownership. |
| Graph | Nodes plus edges; useful for atoms plus bonds. |
| Bond | A chemical connection between two atoms; a graph edge in the toy model. |
| Bond order | A label such as single, double, triple, or aromatic. |
| Aromaticity | A special delocalized bonding pattern in some rings; not about smell. |
| Quantum state | A physical state described by amplitudes, not ordinary stored bits. |

## Atom to Rust Mapping

```rust
let atomic_number: u8 = 6;
let formal_charge: i8 = 0;
let is_aromatic: bool = false;
let mass: f64 = 12.011;
```

Better element labels:

```rust
enum Element {
    H,
    C,
    N,
    O,
    Cl,
}
```

## Molecule Skeleton

```rust
enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}

struct Atom {
    element: Element,
    formal_charge: i8,
    aromatic: bool,
}

struct Bond {
    from: usize,
    to: usize,
    order: BondOrder,
}

struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

## Chemistry Basics

A bond is not a tiny stick between atoms. It is a stable chemical relationship that
comes from electron density shared between nearby atoms. Chemists draw a line
because the line is a compact way to say "these atoms are connected."

Aromaticity is a special bonding pattern in some rings, such as benzene. The
electrons are spread across the ring instead of belonging to one exact single or
double bond. In this toy Rust model, we store that beginner idea with
`aromatic: bool` or `BondOrder::Aromatic`.

## Memory Comparison

| Layer | What it stores or represents |
| --- | --- |
| Rust variable | A typed program value. |
| Stack | Fast scoped storage, often for known-size values. |
| Heap | Flexible storage for data such as growable vectors. |
| Reference | Borrowed access to a value. |
| CPU/OS memory mapping | Translation from program-level addresses to hardware memory. |
| Quantum state | Amplitudes of possible outcomes, not ordinary readable memory. |

## Mini Exercise

Pick one molecule and answer:

1. What enum variants do you need for its elements?
2. How many `Atom` values does it need?
3. How many `Bond` values does it need?
4. Which fields belong inside `Atom`?
5. Which fields belong inside `Bond`?
6. What is one invalid molecule your Rust code should reject?

## Remember

Analogy helps you start. Precision helps you finish.

Atoms are not bytes. Bonds are not references. Qubits are not ordinary memory cells.
But all three domains reward careful thinking about state, structure, and constraints.
