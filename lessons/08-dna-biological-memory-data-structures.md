# Lesson 08: DNA, Biological Memory, and Data Structures

## Goal

Students compare DNA sequence information with computer data structures and Rust
memory concepts. They learn how base sequences can be represented as code while
keeping the biological analogy precise.

## Big Question

How can a molecule carry information, and how can a program represent that
information safely?

## Visuals

- [Memory address flow](../visuals/plantuml/memory-address-flow.puml)
- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)
- [Course map](../visuals/mermaid/course-map.md)

## Core Comparison

| Biology / chemistry | Rust / CS | Teaching use | Limit |
| --- | --- | --- | --- |
| DNA base | enum variant | fixed alphabet | bases are chemical parts, not symbols only |
| DNA strand | `Vec<Base>` or `String` | sequence data | DNA has shape and chemistry |
| codon | `[Base; 3]` | fixed-size chunk | translation needs biological machinery |
| gene region | slice `&[Base]` | borrowed region | genes are more than coordinates |
| mutation | enum edit operation | substitution, insertion, deletion | effect depends on biological context |
| lab sample label | pointer-like handle | location/access analogy | a label is not a memory address |
| borrowed strand | `&DnaStrand` | read without copying | reference is not a chemical bond |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show `ACGTTAGC` and ask: molecule, data, or both? |
| 10-25 min | DNA alphabet | Convert bases into a Rust enum. |
| 25-40 min | Memory locations | Compare `Base`, `[Base; 3]`, `Vec<Base>`, `String`, and `&DnaStrand`. |
| 40-55 min | Slices and pointers | Model a gene region as borrowed sequence access. |
| 55-70 min | Mutations | Represent substitutions, insertions, and deletions as enum variants. |
| 70-90 min | Debrief | Identify useful analogies and false shortcuts. |

## Part 1: DNA Alphabet As Enum

Code file:

```text
exercises/rust-molecule-model/src/dna.rs
```

Start with the simplest representation:

```rust
let dna = "ACGTTAGC";
```

Then show why a typed representation is safer:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    A,
    C,
    G,
    T,
}
```

Prompt:

What invalid values can appear in a `String` that cannot appear in `Vec<Base>`?

Expected answers:

- lowercase ambiguity
- accidental letters such as `X`
- spaces or punctuation
- unsupported symbols

## Part 2: Stack, Heap, And Sequence Size

Use this table:

| Rust value | Memory idea | DNA analogy |
| --- | --- | --- |
| `Base::A` | tiny fixed-size value | one base card |
| `[Base; 3]` | fixed-size array | one codon card |
| `Vec<Base>` | heap-backed growable sequence | a strand with variable length |
| `String` | heap-backed text bytes | quick text representation |
| `&[Base]` | borrowed slice | a gene region view |
| `&DnaStrand` | borrowed reference | read access to one owned strand |

Teaching line:

The stack is for small, scoped, known-size values. The heap is for flexible data
such as growable sequences. A reference is a safe way to access data without owning
or copying it.

## Part 3: Complement Function

```rust
fn complement(base: Base) -> Base {
    match base {
        Base::A => Base::T,
        Base::T => Base::A,
        Base::C => Base::G,
        Base::G => Base::C,
    }
}
```

Ask:

- What chemistry rule is encoded?
- What does the compiler check?
- What biological details are missing?

## Part 4: Gene Region As Slice

```rust
fn gene_region(strand: &[Base], start: usize, end: usize) -> &[Base] {
    &strand[start..end]
}
```

Discussion:

- the whole strand owns the data
- the slice borrows a region
- Rust checks that borrowed access stays valid
- a gene region in biology is more than a slice, but a slice is a useful sequence
  representation

## Part 5: Mutations As Data

```rust
enum Mutation {
    Substitute { at: usize, base: Base },
    Insert { at: usize, base: Base },
    Delete { at: usize },
}
```

Connect to data-structure edits:

| Mutation | Sequence operation |
| --- | --- |
| substitution | replace |
| insertion | insert and shift |
| deletion | remove and shift |

## School Version

Use cards:

- `A`, `C`, `G`, `T` cards
- codon envelopes that hold three cards
- long string on the floor as a DNA strand
- sticky notes for gene start and end positions

Students physically borrow a region by standing at the start and end indices.

## University Version

Ask students to design:

```rust
enum Base
struct DnaStrand
type Codon = [Base; 3]
enum Mutation
```

Then discuss:

- when `String` is acceptable
- when `Vec<Base>` is safer
- when a slice is better than cloning
- how mutation operations affect indexing
- what a real bioinformatics library would need beyond this toy model

## Reflection

DNA is a chemical molecule and an information-bearing sequence.

Computer memory is engineered storage with addresses, stack frames, heap
allocations, pointers, and references.

Rust code can model DNA sequence information, but the model must say what it keeps
and what it leaves out.
