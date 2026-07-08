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

## 10. Fingerprint Feature Bits

Give students feature cards:

- has carbon
- has oxygen
- has halogen
- has at least five atoms
- has at least eight atoms
- has branching atom
- has oxygen-hydrogen bond
- has carbon-carbon bond

Activities:

- mark each feature card `1` or `0` for water, methane, and ethanol
- place yes/no cards into numbered bit slots
- count shared yes cards between two molecules
- count yes cards present in either molecule
- compute `shared / union`
- sort molecule cards by similarity to ethanol

Rust reveal:

```rust
struct ToyFingerprint {
    bits: u64,
}
```

Data-structure prompt:

- boolean: one feature answer
- bitset: compact set of feature answers
- intersection: shared features
- union: features seen in either molecule
- sorted list: nearest matches first

Important caution:

A fingerprint is a compressed summary. It keeps selected features and discards the
full atom-and-bond graph.

## 11. Screening Feedback Matrix

Give students prediction cards with candidate names and scores.

Activities:

- choose a threshold
- sort cards into `test` and `defer`
- reveal active/inactive experimental labels
- place cards into a confusion matrix
- compute precision, recall, and accuracy
- identify near-threshold candidates
- choose what the next experiment should test

Rust reveal:

```rust
struct Prediction {
    candidate_name: String,
    score: f64,
    label: Option<ExperimentalLabel>,
}
```

Data-structure prompt:

- enum: active/inactive label
- option: label may be missing
- threshold: numeric boundary
- queue: selected candidates to test
- matrix: count prediction outcomes
- feedback loop: next-round planning

Important caution:

Experimental labels are evidence from a specific setup. They should be interpreted
with controls, uncertainty, and domain knowledge.

## 12. Lab Record Relay

Give each group a molecule summary card.

Activities:

- choose a stable field order
- write a serialized row with separators
- swap rows with another group
- parse the received row into fields
- check required fields and number fields
- diagnose one intentionally broken row
- render one Markdown report row

Rust reveal:

```rust
struct LabRecord {
    record_id: String,
    molecule_name: String,
    formula: String,
    atom_count: usize,
    bond_count: usize,
    notes: String,
}
```

Data-structure prompt:

- struct: typed record in memory
- text line: stored representation
- schema: field contract
- parser: validator and converter
- error enum: explicit failure
- round trip: reproducibility check

Important caution:

A record without context is not reproducible enough for real science. Students
should ask what provenance, units, conditions, and validation are missing.

## 13. Unit Conversion Relay

Give students concentration cards with mixed units.

Activities:

- sort cards by raw number and notice the mistake
- attach unit labels to every value
- convert all values to micromolar
- reject negative and non-finite values
- reject response cards outside `0..100`
- sort assay observations by normalized concentration
- compute a mean response

Rust reveal:

```rust
struct Concentration {
    value: f64,
    unit: ConcentrationUnit,
}
```

Data-structure prompt:

- enum: fixed unit vocabulary
- struct: value and unit stay together
- result: constructor can fail
- normalized value: common comparison scale
- observation record: candidate, dose, response

Important caution:

Units prevent one class of mistake, but real assay interpretation also needs
replicates, uncertainty, controls, protocols, and provenance.

## 14. Replicate Spread Cards

Give students repeated response cards for two candidates.

Activities:

- compute mean for each candidate
- compute min, max, and range
- compare tight and noisy replicate sets
- discuss why one replicate cannot estimate spread
- apply a standard-deviation consistency threshold
- decide whether to trust, repeat, or inspect the result

Rust reveal:

```rust
struct ReplicateSeries {
    candidate_name: String,
    responses_percent: Vec<f64>,
}
```

Data-structure prompt:

- vector: repeated observations
- mean: center summary
- range: simple spread summary
- option: sample variance may be unavailable
- threshold: consistency rule

Important caution:

Statistics summarize evidence. They do not replace experimental design, controls,
or scientific judgment.

## 15. Dose-Response Curve Cards

Give students concentration-response cards.

Activities:

- normalize all concentrations to the same unit
- sort cards by concentration
- draw the response curve by hand
- check whether response increases with dose
- find adjacent cards that surround a target response
- estimate the crossing point by interpolation
- discuss why real curve fitting needs more than this

Rust reveal:

```rust
struct DosePoint {
    concentration: Concentration,
    response_percent: f64,
}
```

Data-structure prompt:

- struct: one dose-response point
- sorted vector: curve order
- window scan: adjacent-point search
- option: target crossing may not exist
- interpolation: estimate between known records

Important caution:

Toy interpolation teaches data flow. It is not a validated potency estimate.
