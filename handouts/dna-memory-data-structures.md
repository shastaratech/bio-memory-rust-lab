# Student Handout: DNA, Memory, and Data Structures

## Big Idea

DNA is a molecule that carries sequence information.

Computer memory stores bits and bytes. Rust gives those stored values types,
owners, references, stack locations, and heap allocations.

The analogy is useful if we keep it precise:

DNA is chemical information. Rust code is a representation of selected parts of
that information.

## Vocabulary

| Word | In this handout |
| --- | --- |
| DNA | A molecule whose base sequence carries biological information. |
| Base | One of `A`, `C`, `G`, or `T`. |
| Codon | Three bases read as a unit. |
| Strand | A sequence of bases. |
| Gene region | A borrowed view into part of a longer sequence. |
| Stack | Fast memory for scoped, usually fixed-size values. |
| Heap | Flexible memory for values whose size can vary or be large. |
| Pointer | Address-like access to data. |
| Reference | Rust's safe borrowed access to data. |

## DNA To Rust Mapping

| Biology idea | Rust representation | Data structure |
| --- | --- | --- |
| base | `Base` enum | fixed vocabulary |
| codon | `[Base; 3]` | fixed-size array |
| strand | `Vec<Base>` | growable sequence |
| quick sequence text | `String` | heap-backed bytes |
| gene region | `&[Base]` | slice |
| mutation | `Mutation` enum | edit operation |

## Base Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    A,
    C,
    G,
    T,
}
```

Why use an enum?

It prevents unsupported base symbols inside the typed model.

## Complement Rule

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

This function encodes a small chemistry rule.

## Memory Comparison

| Value | Memory idea |
| --- | --- |
| `Base::A` | small stack-friendly value |
| `[Base; 3]` | fixed-size codon array |
| `Vec<Base>` | heap-backed strand storage |
| `String` | heap-backed text representation |
| `&DnaStrand` | borrowed reference to an owned strand |
| `&[Base]` | borrowed slice into a sequence |

## Mutation As Code

```rust
enum Mutation {
    Substitute { at: usize, base: Base },
    Insert { at: usize, base: Base },
    Delete { at: usize },
}
```

This describes sequence edits. It does not predict the full biological effect.

## Remember

- DNA is not literally software source code.
- A gene is not just an array.
- A Rust reference is not a chemical bond.
- A model is useful when it says what information it keeps.

## Mini Exercise

Represent this sequence:

```text
ATGC
```

as:

1. a `String`
2. a `Vec<Base>`
3. one borrowed slice for the first three bases

Then answer:

Which representation is easiest to type? Which one prevents bad base letters?
