# Lesson 02: Molecules as Graphs

## Goal

Students learn that a molecule can be represented as a graph: atoms are nodes and
bonds are edges. This unlocks algorithms such as neighbor lookup, traversal,
connected-component detection, and shortest paths.

## Hook

Draw ethanol as a chemical sketch. Then redraw it as numbered dots and lines.

Ask:

- What did we keep?
- What did we lose?
- What questions are easier now?

Expected answers:

We kept connectivity. We lost 3D shape, electron density, bond lengths, and much of
the chemistry. Neighbor and path questions became easier.

## School Version

Students become atoms in a human molecule.

1. Give each student an atom ID.
2. Connect bonded students with string.
3. Ask one atom to name its neighbors.
4. Ask whether two atoms are in the same molecule.
5. Pass a token through the shortest route.

Vocabulary:

- node: an atom
- edge: a bond
- neighbor: directly bonded atom
- path: a route through bonds
- component: a separate fragment

## University Version

Start with a bond list:

```rust
vec![
    Bond::new(0, 1, BondOrder::Single),
    Bond::new(1, 2, BondOrder::Single),
    Bond::new(1, 3, BondOrder::Single),
]
```

Convert it into an adjacency list:

```text
0: [1]
1: [0, 2, 3]
2: [1]
3: [1]
```

Compare:

| Representation | Strength | Weakness |
| --- | --- | --- |
| `Vec<Bond>` | simple, compact, easy to serialize | neighbor lookup scans all bonds |
| adjacency list | fast repeated neighbor lookup | extra memory, must stay in sync |
| adjacency matrix | constant-time edge checks | wasteful for sparse molecules |

## Rust Lab

Students run:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run
```

Then inspect:

- `neighbors`
- `adjacency_list`
- `shortest_path`
- `connected_components`

## Algorithm Walkthrough

Breadth-first search asks:

1. Where am I now?
2. Which neighbors have I not visited?
3. Which one-step-farther paths should I try next?
4. Did I reach the goal?

This is like exploring a molecule shell by shell from a starting atom.

## Reflection

A graph is powerful because it is smaller than reality. It is dangerous for the same
reason. A graph can answer connectivity questions, but it cannot fully answer shape,
energy, or reactivity questions.

