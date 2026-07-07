# Chapter 10: Designing Molecules as Data Search

## Big Idea

Molecule design can be taught as a data-structure and search problem.

In a real lab, molecule design involves chemistry, biology, measurement, synthesis,
and safety. In this course, we use a toy version:

1. Start with a scaffold.
2. Make small changes.
3. Reject invalid candidates.
4. Score the remaining candidates.
5. Sort or rank them.
6. Pick a few for the next round.

This is enough to teach how data structures support design thinking.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Scaffold | The core molecule pattern kept during a design round. |
| Substituent | A small group swapped onto a scaffold. |
| Analog | A related candidate molecule made by changing part of a scaffold. |
| Constraint | A rule that rejects bad candidates. |
| Feature | A value computed from a candidate, such as atom count or formula. |
| Score | A number used to compare candidates. |
| Ranking | Sorting candidates by score or priority. |
| Search | Trying possible changes under rules. |

## Scaffold As A Starting State

A scaffold is the part you keep fixed while exploring changes.

Programming analogy:

| Molecule design | Data structure idea |
| --- | --- |
| scaffold | initial state |
| attachment point | index or node ID |
| substituent list | array / vector of options |
| candidate analog | copied or modified state |
| design round | loop over options |

Toy example:

```text
scaffold: C-C-O
attachment point: atom 1
substituents: H, F, Cl, Br
```

The beginner idea is:

> Keep the core, vary one part, compare the results.

## Representing Design Options

Use an enum when the option set is controlled:

```rust
enum Substituent {
    H,
    F,
    Cl,
    Br,
}
```

Use a vector when you need a list of options:

```rust
let options = vec![
    Substituent::H,
    Substituent::F,
    Substituent::Cl,
    Substituent::Br,
];
```

Analogy:

The vector is a tray of possible substitutions. The enum keeps the labels clean.

## Candidate Records

A candidate needs data about the molecule and its evaluation.

```rust
struct Candidate {
    name: String,
    molecule: Molecule,
    substituent: Substituent,
    valid: bool,
    score: Option<f64>,
}
```

Why `Option<f64>`?

Some candidates may not have a score yet.

That matters because the design pipeline has stages:

- generated but not validated
- validated but not scored
- scored and ready to rank

## Constraints As Filters

A constraint is a rule that rejects candidates.

Toy constraints:

| Constraint | Data check |
| --- | --- |
| supported element | enum variant exists |
| bond points to real atom | `validate_bond_indices()` |
| atom count under limit | `molecule.atom_count() <= max_atoms` |
| one connected fragment | `connected_components().len() == 1` |
| no unsupported charge | inspect `Atom.formal_charge` |

In code, a filter keeps only candidates that pass:

```rust
let kept: Vec<Candidate> = candidates
    .into_iter()
    .filter(|candidate| candidate.valid)
    .collect();
```

## Scoring As A Function

A scoring function reads a candidate and returns a number.

```rust
fn toy_score(molecule: &Molecule) -> f64 {
    molecule.atom_count() as f64 - molecule.bond_count() as f64 * 0.1
}
```

The function borrows the molecule:

```rust
&Molecule
```

It does not need to own or mutate the molecule.

Teaching line:

Scoring is a question asked about a candidate, not a reason to destroy the
candidate.

## Ranking As Sorting

Once candidates have scores, sort them.

```rust
candidates.sort_by(|a, b| {
    b.score
        .partial_cmp(&a.score)
        .unwrap_or(std::cmp::Ordering::Equal)
});
```

This is a sorted list.

Good for:

- showing the best candidates first
- taking the top `N`
- making reports reproducible

## Hash Maps For Lookup

Use a hash map when you need lookup by key.

Examples:

| Key | Value |
| --- | --- |
| candidate name | candidate record |
| molecule name | score |
| atom ID | atom annotation |
| substituent | count of generated candidates |

Conceptual Rust:

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, f64> = HashMap::new();
scores.insert("candidate-001".to_string(), 7.4);
```

Hash maps support fast lookup, but they do not naturally preserve ranking order.
Use sorting when order matters.

## Queue For Search

Some design loops explore candidates step by step.

A queue stores what to try next:

```text
front -> candidate A, candidate B, candidate C -> back
```

Use a queue when the algorithm asks:

> What candidate should I expand next?

This connects molecule design to graph search:

- scaffold is the start state
- substitutions are transitions
- candidates are states
- constraints block invalid paths
- scoring ranks useful paths

## Design Pipeline As Data Structures

| Pipeline step | Data structure |
| --- | --- |
| Store scaffold | `Molecule` struct |
| Store options | `Vec<Substituent>` |
| Generate candidates | loop over vector |
| Validate candidates | filter |
| Compute features | functions over `&Molecule` |
| Store scores | `HashMap<String, f64>` or field on `Candidate` |
| Rank candidates | sorted `Vec<Candidate>` |
| Choose next round | top `N` slice |
| Track history | list / vector of rounds |

## Beginner Design Exercise

Open:

```text
exercises/rust-molecule-model/src/design.rs
```

Design a toy molecule campaign:

1. Choose a scaffold from water, methane, or ethanol.
2. Pick one attachment point.
3. List three possible changes.
4. Write one rule that rejects a bad candidate.
5. Write one feature to compute.
6. Write one score.
7. Decide how to rank candidates.

The goal is not chemical realism. The goal is to understand the data flow.

Run:

```bash
cd exercises/rust-molecule-model
cargo test design
```

## Common Confusion

### "Does a high score mean the molecule is good?"

Not by itself.

A toy score teaches ranking. Real molecule design needs experimental reality,
synthesis constraints, biological context, and safety review.

### "Is molecule generation just random?"

It can include random choices, but design becomes useful when choices are guided
by constraints, features, and feedback.

### "Should every candidate be mutated in place?"

Not always.

For teaching, it is often clearer to keep the scaffold unchanged and create new
candidate values. That makes ownership and history easier to explain.

## Checkpoint

1. What is a scaffold?
2. What is a substituent?
3. Why is a vector useful for design options?
4. Why is a hash map useful for candidate scores?
5. Why is sorting useful after scoring?
6. What is one design constraint that can be checked with a data structure?
7. Why does a toy score not prove a molecule is useful in the real world?
