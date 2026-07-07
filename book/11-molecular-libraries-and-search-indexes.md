# Chapter 11: Molecular Libraries And Search Indexes

## Big Idea

A molecule library is a collection of molecule records plus indexes that help you
ask useful questions quickly.

In chemistry, a library might contain thousands or millions of structures. In this
course, the toy library contains only water, methane, and ethanol. That is enough
to learn the computer science idea:

> The same molecule records can be viewed as an array, a sorted list, a hash map,
> or a grouped index depending on the question.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Record | One molecule plus labels such as ID and common name. |
| Library | A collection of molecule records. |
| Index | A data structure built to answer one lookup question. |
| Primary key | A stable unique ID for a record. |
| Secondary key | Another searchable value, such as name or formula. |
| Query | A question asked of the library. |
| Merge | Combine two sorted lists into one sorted list. |
| Binary search | Search a sorted list by repeatedly cutting the search range in half. |

## Why One Data Structure Is Not Enough

Imagine three questions:

1. Show every molecule in the teaching library.
2. Find ethanol by name.
3. Find every molecule with formula `H2O`.

A vector is good for question 1.

A hash map from name to record is good for question 2.

A hash map from formula to a list of names is good for question 3.

The molecule did not change. The question changed.

## Library Records

Open:

```text
exercises/rust-molecule-model/src/index.rs
```

The core record is:

```rust
struct MoleculeRecord {
    id: String,
    common_name: String,
    molecule: Molecule,
}
```

The molecule graph is the scientific object. The record adds database-like
metadata:

| Field | Why it exists |
| --- | --- |
| `id` | stable key for storage and references |
| `common_name` | human-readable lookup |
| `molecule` | atoms, bonds, formula, and graph algorithms |

## Vector: Keep The Original Collection

A vector keeps records in an order:

```rust
Vec<MoleculeRecord>
```

Good for:

- listing all records
- looping over the library
- preserving teaching order
- building other indexes

Analogy:

A vector is the shelf of molecule cards.

## Hash Map: Fast Lookup By Name

A name index maps a string to a record:

```rust
HashMap<String, MoleculeRecord>
```

Good for:

- `water -> record`
- `methane -> record`
- `ethanol -> record`

Analogy:

A hash map is the label drawer. You jump directly to a named card instead of
reading every card.

## Grouped Index: Formula To Many Records

One formula can point to more than one record:

```rust
HashMap<String, Vec<String>>
```

For example:

```text
H2O -> water-a, water-b
```

That is why the value is a vector.

Beginner rule:

Use `HashMap<Key, Value>` when one key gives one value. Use
`HashMap<Key, Vec<Value>>` when one key can give many values.

## Sorted List: Range And Ranking Questions

Sort records by atom count:

```text
water   3 atoms
methane 5 atoms
ethanol 9 atoms
```

Now you can ask:

> Where do molecules with at least 5 atoms begin?

That kind of question is easier after sorting.

Sorted lists are useful for:

- range queries
- ranking
- deterministic reports
- merging two already sorted sources

## Merge: Combining Sorted Sources

If two classes build two sorted molecule lists, you can merge them without
starting over:

```text
left:  water, ethanol
right: methane
merge: water, methane, ethanol
```

This is the same algorithm idea used in merge sort.

Chemistry analogy:

Two labs may submit already sorted compound lists. A database pipeline can merge
the lists while keeping the order.

## Binary Search: Search By Cutting The List

Binary search only works when the list is sorted by the thing you are searching.

For atom count:

```text
3 atoms | 5 atoms | 9 atoms
```

To find the first molecule with at least 5 atoms, check the middle, then discard
the half that cannot contain the answer.

Rust exercise:

```bash
cd exercises/rust-molecule-model
cargo test index
```

## Data Structure Comparison

| Question | Data structure | Why |
| --- | --- | --- |
| Show every molecule | `Vec<MoleculeRecord>` | simple ordered storage |
| Find by name | `HashMap<String, MoleculeRecord>` | direct lookup |
| Find by formula | `HashMap<String, Vec<String>>` | one formula can match many records |
| Show smallest to largest | sorted `Vec<MoleculeRecord>` | order matters |
| Combine two sorted lists | merge algorithm | avoid re-sorting from scratch |
| Find first record in a range | binary search | use sorted order |

## Relation To DNA And Memory

DNA can be read as stored biological information. A molecule library is stored
computational information.

| Biology / chemistry | Computer structure |
| --- | --- |
| DNA sequence | string-like biological memory |
| gene region | slice / borrowed range |
| molecule record | struct |
| compound library | vector of records |
| formula lookup | hash map index |
| ranked candidates | sorted list |
| search campaign | repeated queries over indexed data |

The analogy has limits: DNA participates in chemistry and biology; a Rust
structure is an encoded model in computer memory. The point is to compare how
information is represented, copied, referenced, searched, and updated.

## Checkpoint

1. What is a molecule record?
2. Why does a molecule library start as a vector?
3. When is a hash map better than scanning every record?
4. Why can formula lookup return more than one molecule name?
5. What must be true before binary search works?
6. How is merging sorted molecule lists different from sorting from scratch?
7. Why is DNA a useful but imperfect analogy for computer data structures?
