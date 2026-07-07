# Lesson 10: Designing Molecules as Data Search

## Goal

Students learn molecule design as a beginner-friendly search workflow connected to
data structures: vectors, graphs, queues, hash maps, filters, sorted lists, and
ranking.

## Big Question

How do data structures help us explore possible molecules?

## Visuals

- [Molecule design algorithms](../visuals/mermaid/molecule-design-algorithms.md)
- [Molecule design flow](../visuals/plantuml/molecule-design-flow.puml)
- [Data algorithms](../visuals/mermaid/data-algorithms.md)
- [3D molecule viewer](../visuals/html/molecule-3d-viewer.html)

## Core Analogy

| Molecule design idea | Data-structure idea |
| --- | --- |
| scaffold | starting state |
| substituent list | vector of options |
| attachment point | atom ID / graph node |
| generated analog | candidate state |
| constraint | filter predicate |
| feature | computed value |
| score | value used for ranking |
| top candidates | sorted list / top slice |
| next round | search frontier |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show a scaffold and ask what can change. |
| 10-25 min | Options | Build a vector of substituent cards. |
| 25-40 min | Candidates | Generate analog cards by combining scaffold plus option. |
| 40-55 min | Constraints | Reject invalid candidates with rule cards. |
| 55-70 min | Scoring | Score and sort candidate cards. |
| 70-90 min | Search | Choose top candidates for the next round. |

## Part 1: Scaffold And Options

Code file:

```text
exercises/rust-molecule-model/src/design.rs
```

Use a simple scaffold:

```text
C-C-O
```

Use a small option list:

```text
H, F, Cl, Br
```

Data structure:

```rust
let options = vec![Substituent::H, Substituent::F, Substituent::Cl, Substituent::Br];
```

Teaching line:

A vector is a tray of choices the loop can visit one by one.

## Part 2: Generate Candidates

For each option, make a candidate.

```text
scaffold + H  -> candidate 1
scaffold + F  -> candidate 2
scaffold + Cl -> candidate 3
scaffold + Br -> candidate 4
```

Data structure:

```rust
Vec<Candidate>
```

## Part 3: Validate

Reject candidates that break rules.

Examples:

- unsupported element
- bond index points outside atom list
- disconnected molecule
- too many atoms for this toy exercise

Data structure idea:

Validation is a filter.

## Part 4: Score And Rank

Give each candidate a toy score, then sort.

Data structure idea:

- `HashMap<candidate_name, score>` for lookup
- sorted `Vec<Candidate>` for ranking
- top `N` slice for choosing the next round

## School Version

Use cards:

- one scaffold card
- four substituent cards
- candidate cards
- rule cards
- score stickers
- ranked row on a table

Students physically move candidates through stations:

```text
generate -> validate -> score -> sort -> choose
```

## University Version

Ask students to design types:

```rust
enum Substituent
struct Candidate
enum RejectReason
struct DesignRound
```

Then ask:

- Which data should be cloned?
- Which functions should borrow?
- Which structures need stable IDs?
- Which outputs need sorting?
- Which lookups need hash maps?

## Reflection

Molecule design is not just drawing new molecules.

It is a data workflow: represent, generate, validate, score, rank, and learn from
the next round.
