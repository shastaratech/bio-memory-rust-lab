# Chapter 08: DNA, Biological Memory, and Data Structures

## Big Idea

DNA is a chemical data structure.

It is not computer memory, and it is not Rust code. But it is a physical molecule
that stores sequence information through a constrained alphabet:

```text
A C G T
```

That makes DNA a useful bridge between chemistry, biology, and computer science.

This chapter compares:

- DNA sequence and computer strings
- base pairs and enum variants
- genes and slices
- codons and fixed-size chunks
- mutations and edits
- stack, heap, references, and pointers

## What You Already Know

From earlier chapters:

- an atom can be modeled as a typed record
- a molecule can be modeled as atoms plus bonds
- a formula preserves counts but not connectivity
- a graph preserves connections
- Rust data has ownership, borrowing, and memory location

Now we add one more biological idea:

> Some molecules carry information because their sequence matters.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| DNA | A polymer whose base sequence stores biological information. |
| Nucleotide | One DNA unit containing a base, sugar, and phosphate. |
| Base | One of `A`, `C`, `G`, or `T` in DNA. |
| Base pair | A matched pair: `A` with `T`, or `C` with `G`. |
| Gene | A region of DNA that can contribute to a functional product. |
| Codon | Three bases read together during translation. |
| Mutation | A sequence change such as substitution, insertion, or deletion. |
| Pointer | An address-like value used to find data in memory. |
| Reference | A safe borrowed handle to data in Rust. |

## DNA As Chemistry

DNA is a molecule. It has atoms, bonds, shape, charge, and physical behavior.

At the chemistry layer, we could ask:

- What atoms are present?
- Which bonds connect them?
- What is the 3D shape?
- How does it interact with water, proteins, and ions?

At the information layer, we often ask a different question:

> What is the base sequence?

For example:

```text
ACGTTAGC
```

That sequence is not the whole molecule. It is a useful representation of one
important part of the molecule.

## DNA As A Data Structure

A DNA strand can be modeled as a sequence.

Beginner version:

```rust
let dna = "ACGTTAGC";
```

Better typed version:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    A,
    C,
    G,
    T,
}
```

Now invalid base letters are harder to represent.

```rust
let strand = vec![
    Base::A,
    Base::C,
    Base::G,
    Base::T,
    Base::T,
    Base::A,
    Base::G,
    Base::C,
];
```

Teaching line:

`String` is easy to type. `Vec<Base>` is better when correctness matters.

## Complement Rules As Code

DNA base pairing has a simple starter rule:

- `A` pairs with `T`
- `C` pairs with `G`

In Rust:

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

This is chemistry as a constrained mapping.

It is not full molecular biology. It is a small rule encoded as a function.

## A DNA Strand Type

We can wrap the vector in a struct:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct DnaStrand {
    bases: Vec<Base>,
}
```

This gives the sequence one owner:

```rust
let strand = DnaStrand {
    bases: vec![Base::A, Base::C, Base::G, Base::T],
};
```

The `DnaStrand` owns the `Vec<Base>`.

The vector's small control record lives in the variable. The sequence storage is
on the heap because a vector can grow.

## Stack And Heap Comparison

| Data | Likely memory pattern | Why |
| --- | --- | --- |
| `Base::A` | stack-friendly value | tiny fixed-size enum |
| `[Base; 3]` codon | stack-friendly array | fixed length known at compile time |
| `Vec<Base>` strand | heap-backed sequence | length can vary |
| `String` DNA text | heap-backed bytes | length can vary |
| `&DnaStrand` | stack-held reference | borrowed handle to existing data |
| `Box<DnaStrand>` | heap allocation with one owner | useful when a stable heap-owned value is needed |

The stack is good for small, fixed-size values.

The heap is good for flexible data whose size can change or may be large.

References and pointers are not the data itself. They are handles that tell a
program where to find data.

## Pointers, References, And Biological Handles

In a lab notebook, you might write:

```text
sample tube B7
```

That label is not the DNA. It is a way to find the DNA sample.

In a program:

```rust
fn gc_count(strand: &DnaStrand) -> usize {
    strand
        .bases
        .iter()
        .filter(|base| matches!(base, Base::G | Base::C))
        .count()
}
```

`&DnaStrand` is not a copy of the whole strand.

It is borrowed access to an existing strand.

Important caution:

A Rust reference is not a chemical bond. A lab label is not a pointer. These are
analogies about finding and accessing information, not identical physical systems.

## Codons As Fixed-Size Chunks

A codon is three bases read together.

In Rust, a codon can be a fixed-size array:

```rust
type Codon = [Base; 3];

let start: Codon = [Base::A, Base::T, Base::G];
```

This is different from a whole DNA strand:

```rust
let strand: Vec<Base> = vec![Base::A, Base::T, Base::G, Base::C];
```

Comparison:

| Biology idea | Rust representation | Data structure idea |
| --- | --- | --- |
| one base | `Base` | enum value |
| one codon | `[Base; 3]` | fixed-size array |
| DNA strand | `Vec<Base>` | sequence |
| gene region | `&[Base]` | slice |
| genome index | `usize` position | offset into sequence |
| mutation list | `Vec<Mutation>` | edit log |

## Genes As Slices

A gene can be represented as a region of a longer sequence.

In Rust, a slice borrows part of a sequence:

```rust
fn gene_region(strand: &[Base], start: usize, end: usize) -> &[Base] {
    &strand[start..end]
}
```

The slice does not own the bases. It points into existing sequence data with
bounds checked by Rust.

This is a strong memory lesson:

- the genome-like sequence owns the data
- a gene-like slice borrows a region
- the slice must not outlive the data it points into

## Mutations As Edits

We can model sequence changes with an enum:

```rust
enum Mutation {
    Substitute { at: usize, base: Base },
    Insert { at: usize, base: Base },
    Delete { at: usize },
}
```

This is similar to an edit log.

| Mutation | Computer data operation |
| --- | --- |
| substitution | replace one value |
| insertion | insert one value and shift later values |
| deletion | remove one value and shift later values |

The biological effect of a mutation can be simple, subtle, or severe. The data
operation only describes the sequence edit.

## DNA, Computer Memory, And Quantum State

These are different layers of reality:

| Layer | What it stores or represents | Good analogy | Limit |
| --- | --- | --- | --- |
| DNA sequence | biological sequence information | alphabet, string, vector | DNA also has chemistry and structure |
| Computer memory | bits and bytes at addresses | stored symbols | memory is engineered, not evolved chemistry |
| Rust value | typed program data | structured representation | not physical biology |
| Pointer/reference | address-like access | lab label or location handle | not a chemical bond |
| Quantum state | amplitudes of a physical system | state richer than one stored string | not directly readable like RAM |

The important idea is not that these are the same.

The important idea is that each representation answers different questions.

## Try It

Open:

```text
exercises/rust-molecule-model/src/dna.rs
```

Then read this small Rust sketch:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    A,
    C,
    G,
    T,
}

fn complement(base: Base) -> Base {
    match base {
        Base::A => Base::T,
        Base::T => Base::A,
        Base::C => Base::G,
        Base::G => Base::C,
    }
}
```

Then answer:

1. Why is `Base` an enum instead of `char`?
2. Why is `[Base; 3]` a good codon representation?
3. Why is `Vec<Base>` a good strand representation?
4. Why does `&[Base]` fit a gene region?

Run:

```bash
cd exercises/rust-molecule-model
cargo test dna
```

## Common Confusion

### "Is DNA literally code?"

DNA is not software source code. It is a molecule whose sequence participates in
biological processes.

Calling it code can be useful if students remember the limit:

> DNA is chemical information, not a program written for a CPU.

### "Is a gene just an array?"

No. A gene is a biological concept with regulation, context, expression, and
function.

An array or slice can represent a sequence region, but it does not capture all
biology.

### "Is a pointer like a bond?"

No. A pointer or reference helps a program access data. A bond is a chemical
connection between atoms.

Both can be drawn as arrows in a diagram, but they mean different things.

## Checkpoint

1. Why is DNA a useful example of a chemical data structure?
2. What does `Vec<Base>` preserve that a formula does not?
3. What is the difference between a base, codon, strand, and gene region?
4. Why does a long DNA strand usually fit heap-backed data better than stack-only data?
5. What does a Rust reference do when a function borrows a DNA strand?
6. What is one place where the DNA-as-code analogy breaks?
