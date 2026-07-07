# Worksheet: University Traits API Design

## Goal

Decide which molecule behaviors belong in methods, traits, and generic functions.

## Starting Point

Open:

```text
exercises/rust-molecule-model/src/molecule.rs
```

Find:

```rust
pub trait Describe
pub trait ChemicalFormula
pub trait MolecularGraph
```

## Part 1: Behavior Inventory

Classify each behavior.

| Behavior | Inherent method | Trait method | Free function | Why? |
| --- | --- | --- | --- | --- |
| Count atoms | | | | |
| Count bonds | | | | |
| Format a formula | | | | |
| Describe an atom | | | | |
| Find neighbors | | | | |
| Find shortest path | | | | |
| Validate bond indices | | | | |

## Part 2: Trait Boundary

`MolecularGraph` currently includes:

```rust
pub trait MolecularGraph {
    fn atom_count(&self) -> usize;
    fn bond_count(&self) -> usize;
    fn neighbors(&self, atom_id: usize) -> Vec<usize>;
    fn shortest_path(&self, start: usize, goal: usize) -> Option<Vec<usize>>;
}
```

Answer:

1. Which methods are essential for graph behavior?
2. Which methods are convenient but not essential?
3. What does returning `Option<Vec<usize>>` communicate?

## Part 3: Write A Generic Function

Write a function signature for degree:

```rust
fn degree(                                      ) -> usize {

}
```

Hint:

```rust
graph.neighbors(atom_id).len()
```

Now write `is_isolated`:

```rust
fn is_isolated(                                ) -> bool {

}
```

## Part 4: API Tradeoff

The starter trait returns:

```rust
Vec<usize>
```

for neighbors.

Discuss:

| Choice | Beginner clarity | Allocation cost | API complexity |
| --- | --- | --- | --- |
| `Vec<usize>` | | | |
| borrowed slice | | | |
| iterator | | | |

## Part 5: Design Memo

Write a short design memo:

```text
I would keep __________________ as an inherent method because:


I would put __________________ in a trait because:


I would not optimize __________________ yet because:


```

## Checkpoint

1. What problem do traits solve in this course?
2. When is a default trait method useful?
3. Why is beginner clarity sometimes worth a small allocation?
