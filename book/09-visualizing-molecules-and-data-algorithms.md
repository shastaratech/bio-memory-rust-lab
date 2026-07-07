# Chapter 09: Visualizing Molecules and Data Algorithms

## Big Idea

The same molecule can be viewed through several data structures.

Each view makes a different question easier:

- a 3D view shows approximate shape
- an array or vector stores atoms in order
- a linked list teaches traversal, but is rarely the best beginner molecule model
- a sorted list makes ordered reports easy
- a hash map makes lookup and counting fast
- sorting and merging turn raw records into useful summaries

## Read First

- Chapter 05: [Rust Data Structures for Molecules](05-data-structures.md)
- Chapter 08: [DNA, Biological Memory, and Data Structures](08-dna-biological-memory-data-structures.md)
- Visual: [3D Molecule Viewer](../visuals/html/molecule-3d-viewer.html)
- Visual: [Graph Algorithms](../visuals/mermaid/graph-algorithms.md)

## Open The 3D Viewer

Open this file in a browser:

```text
visuals/html/molecule-3d-viewer.html
```

Choose water, methane, or ethanol.

The viewer stores each molecule as:

```text
atoms: element + 3D position
bonds: pairs of atom indices
```

This is still a model. The coordinates are simplified teaching coordinates, not a
replacement for experimental or computed molecular geometry.

## 3D View Versus Graph View

| View | Keeps | Leaves out |
| --- | --- | --- |
| formula | element counts | bonds, shape, atom IDs |
| graph | atoms and bonds | real geometry, energy, electron density |
| 3D teaching view | atoms, bonds, approximate coordinates | true conformational ensemble |
| quantum model | physical state and amplitudes | easy visual intuition |

Teaching line:

A visual model is still a data structure. It keeps selected information so a
student can answer selected questions.

## Array Or Vector: Ordered Atom Storage

In Rust, the starter molecule uses:

```rust
atoms: Vec<Atom>
```

This is like an array that can grow.

Good for:

- atom IDs by position
- iterating over all atoms
- indexing atom `0`, atom `1`, atom `2`

Example:

```rust
for (id, atom) in molecule.atoms().iter().enumerate() {
    println!("{id}: {}", atom.describe());
}
```

Tradeoff:

Indexing is easy, but inserting in the middle can shift later positions.

## List: Sequential Traversal

A list teaches the idea of moving from one item to the next.

Conceptual list:

```text
Atom 0 -> Atom 1 -> Atom 2 -> end
```

Good for:

- teaching traversal
- queues and stacks
- edit histories

Usually not ideal for the starter molecule model because graph questions need
neighbors, paths, and repeated lookup. A vector plus bond list is simpler and more
transparent.

## Sorted List: Ordered Reports

A sorted list keeps records in order.

For formula output, the model sorts element counts:

```rust
counts.sort_by_key(|(element, _)| element.sort_key());
```

This turns internal counts into stable output:

```text
C2H6O
```

Good for:

- predictable reports
- comparing outputs in tests
- printing formulas consistently

Tradeoff:

Maintaining sorted order during many updates can cost extra work.

## Hash Map: Fast Lookup And Counting

A hash map stores key-value pairs.

For molecule formulas:

```text
Element -> count
```

Conceptual Rust:

```rust
use std::collections::HashMap;

let mut counts: HashMap<Element, usize> = HashMap::new();

for atom in molecule.atoms() {
    *counts.entry(atom.element).or_insert(0) += 1;
}
```

Good for:

- counting elements
- finding records by name
- mapping atom IDs to labels
- merging summaries from multiple molecules

Tradeoff:

Hash maps do not naturally preserve a chemistry-friendly print order. You often
count with a map, then sort keys for output.

## Sorting Atoms Or Counts

Sorting arranges values by a rule.

Examples:

| Sort target | Sort key | Why |
| --- | --- | --- |
| element counts | element order | stable formula output |
| atoms | atom ID | reproducible display |
| atoms | element symbol | group similar atoms |
| bonds | `from`, then `to` | easier graph debugging |
| molecules | atom count | compare size |
| scores | docking score | rank candidates |

Sorting is not just cosmetic. It makes reports reproducible and comparisons easier.

## Merging Data

Merging combines two data sources.

Examples:

| Merge task | Inputs | Output |
| --- | --- | --- |
| formula counts | atom list + element counts | updated counts |
| molecule list | known molecules + student molecule | larger catalog |
| annotation merge | atom IDs + labels | labeled atoms |
| score merge | molecule names + scores | ranked result table |
| DNA edit merge | original sequence + mutations | edited sequence |

Conceptual merge:

```rust
for atom in molecule.atoms() {
    add_atom_to_counts(atom, &mut counts);
}
```

University extension:

Ask whether merge should overwrite conflicts, reject conflicts, or keep both values.

## Choosing The Structure

| Question | Good first structure |
| --- | --- |
| What atoms exist in order? | `Vec<Atom>` |
| What bonds exist? | `Vec<Bond>` |
| Who are this atom's neighbors? | adjacency list |
| How many of each element? | `HashMap<Element, usize>` or `Vec<(Element, usize)>` |
| How do I print a stable formula? | sorted list of counts |
| How do I rank candidates? | sorted vector |
| How do I combine annotations? | hash map by atom ID |
| How do I show shape? | atoms with 3D coordinates |

## Try It

Open the 3D viewer and answer:

1. Which atom is easiest to find visually?
2. Which question is easier in the CLI than in 3D?
3. Which question is easier in 3D than in a formula string?
4. What data structure would you use to count elements?
5. What data structure would you use to rank molecule scores?

## Checkpoint

1. Why is `Vec<Atom>` a good starter model?
2. Why might a hash map be better for element counting?
3. Why sort formula counts before printing?
4. What does a 3D view add to a graph?
5. What does a 3D view still leave out?
6. Give one example of a merge operation in molecule data.
