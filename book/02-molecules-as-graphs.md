# Chapter 02: Molecules as Graphs

## Big Idea

A molecule can be represented as a graph:

- atoms are nodes
- bonds are edges
- neighbors are directly bonded atoms
- paths are routes through bonds

This representation lets us use graph algorithms.

## Read First

- Lesson: [Molecules as Graphs](../lessons/02-molecules-as-graphs.md)
- Visual: [Graph Algorithms](../visuals/mermaid/graph-algorithms.md)
- Visual: [Course Map](../visuals/mermaid/course-map.md)

## Open These Files

```text
exercises/rust-molecule-model/src/molecule.rs
visuals/mermaid/graph-algorithms.md
```

Find these methods:

```rust
pub fn neighbors(&self, atom_id: usize) -> Vec<usize>
pub fn adjacency_list(&self) -> Vec<Vec<usize>>
pub fn shortest_path(&self, start: usize, goal: usize) -> Option<Vec<usize>>
pub fn connected_components(&self) -> Vec<Vec<usize>>
```

## Run Neighbor Queries

From the Rust exercise folder:

```bash
cd exercises/rust-molecule-model
cargo run -- ethanol neighbors 1
```

Expected output:

```text
[0, 2, 6, 7]
```

Meaning:

Atom `1` in the ethanol toy model is connected to atoms `0`, `2`, `6`, and `7`.

Print atoms to understand the IDs:

```bash
cargo run -- ethanol atoms
```

## Run A Shortest Path

```bash
cargo run -- ethanol path 3 8
```

Expected output:

```text
[3, 0, 1, 2, 8]
```

Read it as:

Start at atom `3`, go through `0`, then `1`, then `2`, and arrive at `8`.

## Run Connected Components

```bash
cargo run -- ethanol components
```

Expected output is one component because ethanol is one connected molecule:

```text
[[0, 1, 3, 4, 5, 2, 6, 7, 8]]
```

The order can follow the traversal order. The important idea is that all atoms are in
one fragment.

## Code Reading

Find this in `molecule.rs`:

```rust
let mut queue = VecDeque::new();
```

`VecDeque` is used for breadth-first search. It lets the algorithm push new atoms at
the back and pop atoms from the front.

## Try It

1. Run `cargo run -- methane neighbors 0`.
2. Run `cargo run -- methane path 1 4`.
3. Explain the output using the atom and bond lists.

## Checkpoint

1. What is a node in the molecule graph?
2. What is an edge?
3. Why does shortest path return `Option<Vec<usize>>`?
4. What data structure stores the BFS queue?

