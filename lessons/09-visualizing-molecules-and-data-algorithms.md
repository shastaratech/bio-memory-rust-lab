# Lesson 09: Visualizing Molecules and Data Algorithms

## Goal

Students connect 3D molecule visualization with common data structures and
algorithms: arrays, vectors, lists, sorted lists, hash maps, sorting, and merging.

## Big Question

Which data shape makes this molecule question easy?

## Visuals

- [3D Molecule Viewer](../visuals/html/molecule-3d-viewer.html)
- [Data Algorithms](../visuals/mermaid/data-algorithms.md)
- [Graph Algorithms](../visuals/mermaid/graph-algorithms.md)
- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)

## Core Comparison

| Structure | Molecule use | Easy question | Tradeoff |
| --- | --- | --- | --- |
| array / vector | ordered atoms | What is atom 3? | inserts can shift positions |
| list | sequential traversal | What comes next? | weak for random access |
| sorted list | formula counts, ranked scores | What is the ordered report? | updates can require resorting |
| hash map | counts, lookup tables | What is the count for this key? | output order is not guaranteed |
| graph | atoms plus bonds | Who connects to whom? | shape still needs coordinates |
| 3D coordinates | spatial view | What does it look like? | geometry may be approximate |

## Timing

Recommended length: 75 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Open the 3D viewer and rotate ethanol. |
| 10-25 min | Storage view | Compare atom vector and bond vector. |
| 25-40 min | Lookup view | Use hash maps for element counts and atom labels. |
| 40-55 min | Ordered view | Sort counts, bonds, or scores for stable output. |
| 55-68 min | Merge view | Combine molecule records with labels or scores. |
| 68-75 min | Debrief | Choose the right structure for each question. |

## Part 1: 3D Visualization

Open:

```text
visuals/html/molecule-3d-viewer.html
```

Ask:

- What does the 3D view make easier to notice?
- What does the graph view make easier to compute?
- What does the formula view hide?

Teaching line:

3D visualization is another representation, not the molecule itself.

## Part 2: Vector Storage

The Rust model stores atoms in a vector:

```rust
atoms: Vec<Atom>
```

This gives stable atom IDs by position during the toy model:

```text
0: first atom
1: second atom
2: third atom
```

## Part 3: Hash Map Counting

Use a hash map when the question is:

> What value belongs to this key?

Examples:

- `Element -> count`
- `atom_id -> label`
- `molecule_name -> score`

## Part 4: Sorting

Sort when output order matters.

Examples:

- formula elements in stable order
- molecule candidates by score
- bonds by endpoint for easier debugging

## Part 5: Merging

Merge when two tables describe the same thing from different angles.

Examples:

- atom records plus atom labels
- molecule names plus docking scores
- sequence plus mutations
- formula counts from several fragments

## School Version

Use cards:

- atom cards in a row for a vector
- linked cards with arrows for a list
- alphabetized cards for a sorted list
- buckets labeled `C`, `H`, `O`, `N` for a hash map
- a 3D ball-and-stick model or the HTML viewer for spatial view

Students answer the same question with different structures:

> How many hydrogens are there?

Then:

> Which atoms are bonded to oxygen?

## University Version

Ask students to design data structures for:

1. fast formula counting
2. stable formula printing
3. repeated neighbor lookup
4. ranking candidate molecules by score
5. merging atom annotations from two tools
6. showing a 3D teaching view

## Reflection

Good engineering starts by asking what question must be cheap, clear, and correct.

For molecules, no single representation does everything.
