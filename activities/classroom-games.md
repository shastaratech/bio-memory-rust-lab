# Classroom Games and Studio Activities

These activities are designed to work for school students and can be deepened for
university students.

## 1. Atom Cards

Give each student an atom card:

```text
Element: C
Atomic number: 6
Charge: 0
Aromatic: false
```

Ask:

- Which fields are numbers?
- Which fields are yes/no?
- Which fields should come from a fixed list?

Rust reveal:

```rust
struct Atom {
    element: Element,
    formal_charge: i8,
    aromatic: bool,
}
```

## 2. Bond String Graph

Students stand as atoms. String between students represents bonds.

Run graph questions:

- Who are carbon's neighbors?
- Can oxygen reach chlorine?
- Is the class one molecule or two fragments?
- What path connects atom 2 to atom 7?

University extension:

Write the adjacency list on the board as the students stand:

```text
0: [1, 2]
1: [0, 3]
2: [0]
3: [1]
```

## 3. Human Stack and Heap

Use two baskets labeled `stack` and `heap`.

Roles:

- owner: holds the molecule card
- immutable borrower: may read the molecule
- mutable borrower: may change the molecule, but must be alone

Rule:

Many readers or one writer, not both.

Discussion:

This is not because Rust is fussy. It is because two unsynchronized edits to the
same molecule can produce nonsense.

## 4. The Invalid Molecule Trial

Students invent broken molecular data:

- bond points to atom 99
- atom symbol is `Xx`
- negative atom count
- same bond duplicated three times

Then they decide:

- compile-time prevention with enum/type
- runtime validation with a function
- ignored because the toy model is too small

## 5. Graph Relay

Put atom IDs on students. A message must travel from atom A to atom B only through
bonded neighbors.

Questions:

- How many steps did the shortest path take?
- Did anyone receive the message twice?
- How would a program avoid getting stuck in a loop?

University extension:

Connect this to breadth-first search and a visited set.

## 6. Quantum Measurement Cards

Give a student a hidden card with two amplitudes:

```text
Outcome A: high chance
Outcome B: low chance
```

The class can know the rule, but not the result until the card is revealed.

Teacher line:

Quantum state is not an ordinary stored answer. It is a physical state that gives
measurement outcomes with probabilities.

Important caution:

Do not tell students a qubit "stores both 0 and 1 like a bigger bit." That shortcut
creates confusion later.

## 7. DNA Sequence Memory

Give students cards labeled `A`, `C`, `G`, and `T`.

Activities:

- build a DNA strand as a line of base cards
- group every three cards into codons
- mark a gene region with two sticky notes labeled `start` and `end`
- copy a short region onto a whiteboard as a borrowed slice
- apply one mutation card: substitute, insert, or delete

Rust reveal:

```rust
enum Base {
    A,
    C,
    G,
    T,
}

type Codon = [Base; 3];
```

Memory prompt:

- one base is like a small stack-friendly enum value
- one codon is like a fixed-size array
- a long strand is like heap-backed `Vec<Base>`
- a gene region is like a borrowed slice `&[Base]`

Important caution:

DNA is chemical information, not software source code. A Rust model can represent
the sequence, but not the whole physical biology.

## 8. Molecule Design Stations

Create five stations around the room:

1. scaffold
2. substituent options
3. validation
4. scoring
5. ranking

Students move candidate cards through the stations.

At the scaffold station, students choose a starter graph.

At the substituent station, students draw one option from a vector-like row of
cards.

At the validation station, students apply rule cards:

- supported element
- valid bond indices
- one connected graph
- atom count under the class limit

At the scoring station, students compute a toy score.

At the ranking station, students sort candidate cards from best to worst.

Rust reveal:

```rust
struct Candidate {
    name: String,
    molecule: Molecule,
    valid: bool,
    score: Option<f64>,
}
```

Data-structure prompt:

- vector: store substituent options
- graph: store scaffold connectivity
- filter: reject invalid candidates
- hash map: look up scores by name
- sorted list: rank candidates
- queue: hold candidates for the next design round

Important caution:

This is a toy design workflow. Real molecule design needs chemistry, synthesis,
measurement, biological context, and safety review.

## 9. Molecule Library Index Cards

Give students molecule cards for water, methane, and ethanol.

Activities:

- place cards on a shelf to represent `Vec<MoleculeRecord>`
- make name labels that point directly to one card
- make formula buckets, including two water cards in the `H2O` bucket
- sort cards by atom count
- find where `atom_count >= 5` begins
- merge two already sorted student lists

Rust reveal:

```rust
struct MoleculeRecord {
    id: String,
    common_name: String,
    molecule: Molecule,
}
```

Data-structure prompt:

- vector: ordered record shelf
- hash map: direct name lookup
- hash map of vectors: formula buckets
- sorted list: range and ranking questions
- binary search: find a boundary in sorted data
- merge: combine sorted library outputs

Important caution:

A molecule library is a computational model. It can search encoded records, but it
does not prove chemical usefulness or biological behavior.
