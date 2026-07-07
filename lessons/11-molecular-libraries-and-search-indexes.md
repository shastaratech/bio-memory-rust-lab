# Lesson 11: Molecular Libraries And Search Indexes

## Goal

Students learn how the same molecule records can be organized as vectors, sorted
lists, hash maps, grouped indexes, and searchable ranges.

## Big Question

How does a molecule library choose the right data structure for each search
question?

## Visuals

- [Molecular library indexes](../visuals/mermaid/molecular-library-indexes.md)
- [Data algorithms](../visuals/mermaid/data-algorithms.md)

## Core Analogy

| Molecule library idea | Data-structure idea |
| --- | --- |
| compound card shelf | vector |
| name label drawer | hash map |
| formula bucket | grouped hash map |
| smallest-to-largest row | sorted list |
| two class lists | merge inputs |
| range question | binary search |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show the same molecule cards in three arrangements. |
| 10-25 min | Records | Add ID, common name, and molecule fields to each card. |
| 25-40 min | Indexes | Build name and formula lookup tables. |
| 40-55 min | Sorting | Sort cards by atom count and explain the key. |
| 55-70 min | Search | Use binary search to find the first card with enough atoms. |
| 70-90 min | Merge | Merge two sorted class lists into one sorted library. |

## Part 1: Records

Code file:

```text
exercises/rust-molecule-model/src/index.rs
```

Start with:

```rust
struct MoleculeRecord {
    id: String,
    common_name: String,
    molecule: Molecule,
}
```

Teaching line:

A molecule graph stores atoms and bonds. A molecule record stores the graph plus
labels that make it searchable.

## Part 2: Vector As Shelf

Ask students to arrange cards as:

```text
water -> methane -> ethanol
```

This is the vector view.

Questions it answers well:

- What is in the library?
- How many records are there?
- Can we loop over every molecule?

## Part 3: Hash Map As Label Drawer

Build a table:

| Name | Record |
| --- | --- |
| water | water card |
| methane | methane card |
| ethanol | ethanol card |

This answers:

```text
Find ethanol.
```

without scanning every card.

## Part 4: Grouped Formula Index

Use formula buckets:

| Formula | Names |
| --- | --- |
| H2O | water-a, water-b |
| CH4 | methane |

Students should notice why the value is a list:

> One formula key can have multiple records.

## Part 5: Sorted Lists And Binary Search

Sort by atom count:

```text
water(3), methane(5), ethanol(9)
```

Then ask:

> Where does atom count >= 5 begin?

Students can check the middle card first. This gives a physical memory of binary
search.

## Part 6: Merge

Give half the class:

```text
water(3), ethanol(9)
```

Give the other half:

```text
methane(5)
```

Ask them to merge into:

```text
water(3), methane(5), ethanol(9)
```

The rule is:

Compare the front card from each list, move the smaller one, repeat.

## University Extension

Ask students to design these APIs:

```rust
fn lookup_by_name(&self) -> HashMap<String, MoleculeRecord>
fn formula_index(&self) -> HashMap<String, Vec<String>>
fn sorted_by_atom_count(&self) -> Vec<MoleculeRecord>
fn first_with_at_least_atoms(sorted: &[MoleculeRecord], min_atoms: usize) -> usize
```

Discussion prompts:

- Which functions clone records?
- Which functions could return references instead?
- Which indexes should be cached in a larger application?
- What happens when two records share a formula?
- What makes binary search invalid?

## Reflection

Molecular informatics is not only drawing structures. It is also deciding how
records are stored, indexed, sorted, merged, and searched.
