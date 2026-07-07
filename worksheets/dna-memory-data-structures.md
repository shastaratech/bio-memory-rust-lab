# Worksheet: DNA, Memory, and Data Structures

## Goal

Compare DNA sequence information with Rust data structures and memory choices.

## Part 1: Base Alphabet

Fill in the enum variants:

```rust
enum Base {




}
```

Why is this safer than a free-form `String`?

```text

```

## Part 2: Stack Or Heap?

Mark each value as mostly stack-friendly, heap-backed, or borrowed.

| Value | Stack-friendly | Heap-backed | Borrowed |
| --- | --- | --- | --- |
| `Base::A` | | | |
| `[Base; 3]` | | | |
| `Vec<Base>` | | | |
| `String` | | | |
| `&DnaStrand` | | | |
| `&[Base]` | | | |

## Part 3: Codon

Represent the codon `ATG`:

```rust
type Codon = [Base; 3];

let start: Codon = [

];
```

Why is an array a good fit for exactly three bases?

```text

```

## Part 4: Strand

Represent `ACGTTAGC` as a vector:

```rust
let strand = vec![

];
```

Why is a vector a better fit for a long strand than a fixed-size codon array?

```text

```

## Part 5: Borrowed Gene Region

Complete the function signature:

```rust
fn gene_region(strand:          , start: usize, end: usize) ->          {
    &strand[start..end]
}
```

What owns the bases?

```text

```

What borrows the bases?

```text

```

## Part 6: Mutations

Fill in one example of each mutation:

```rust
enum Mutation {
    Substitute { at: usize, base: Base },
    Insert { at: usize, base: Base },
    Delete { at: usize },
}
```

| Mutation kind | Example |
| --- | --- |
| Substitute | |
| Insert | |
| Delete | |

## Reflection

Answer in two sentences:

1. How is DNA like data?
2. Where does the DNA-as-code analogy break?
