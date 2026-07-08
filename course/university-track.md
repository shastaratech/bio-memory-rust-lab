# University Track

## Tone

Use the molecule metaphor as a bridge into serious systems and scientific computing:
representation, invariants, memory behavior, graph algorithms, and simulation limits.

## Prerequisites

Helpful but not required:

- one programming language
- basic chemistry vocabulary
- arrays, structs/classes, and functions

## Seventeen-Session Seminar

### Session 01: Typed Molecular Data

Core questions:

- What should be an enum, struct, vector, or method?
- Which invalid states should be impossible?
- Which invalid states are better caught at runtime?

Implementation focus:

- `Element`
- `Atom`
- `Bond`
- `Molecule`
- `validate_bond_indices`

### Session 02: Molecules as Graphs

Core questions:

- When is `Vec<Bond>` enough?
- When do we need an adjacency list?
- How do traversal algorithms change the questions we can ask?

Implementation focus:

- adjacency list construction
- neighbor lookup
- shortest path
- connected components

### Session 03: Ownership and Design Pipelines

Core questions:

- When should an algorithm borrow a molecule?
- When should it mutate a molecule?
- When should it clone?
- How do API choices encode scientific workflow assumptions?

Implementation focus:

- borrowed scoring functions
- mutating analog generation
- immutable scaffold data
- clone cost discussion

### Session 04: Quantum State and Simulation

Core questions:

- What does a graph representation leave out?
- Why do electronic structure problems become hard?
- What does VQE optimize, at a high level?
- Why are quantum computers interesting but not magic accelerators?

Implementation focus:

- state vector intuition
- Hamiltonian as an energy operator
- classical scoring vs quantum energy estimation
- limits of toy models

### Session 05: Rust Data Structures for Molecules

Core questions:

- What should be stored as an enum, struct, vector, graph, or count table?
- What questions should be cheap to answer?
- What invalid states can the representation prevent?

Implementation focus:

- `Element`
- `Atom`
- `Bond`
- `Molecule`
- formula counts
- storage representation vs graph view

### Session 06: Traits and Functions for Molecular Behavior

Core questions:

- Which behaviors belong in traits?
- Which methods should have default implementations?
- What does a function promise through its parameters and return type?
- How can algorithms accept `&impl MolecularGraph` instead of only `&Molecule`?

Implementation focus:

- `Describe`
- `ChemicalFormula`
- `MolecularGraph`
- `Option<Vec<usize>>`
- generic functions over molecular graph behavior

### Session 07: Molecule Explorer CLI Capstone

Core questions:

- Where is the boundary between the library and the binary?
- Which failures should return `Result`, `Option`, or a printed message?
- How do command strings become testable command data?
- Which behavior belongs in unit tests before the CLI prints anything?

Implementation focus:

- command parsing
- molecule lookup
- CLI error handling
- `degree` command
- new toy molecule plus formula test

### Session 08: DNA, Biological Memory, and Data Structures

Core questions:

- When is a DNA sequence best represented as `String`, `Vec<Base>`, or `&[Base]`?
- Which values are stack-friendly, heap-backed, or borrowed?
- How do codons, gene regions, and mutations map to arrays, slices, and enums?
- Where does the DNA-as-code analogy help, and where does it break?

Implementation focus:

- `Base` enum
- `[Base; 3]` codon type
- `DnaStrand { bases: Vec<Base> }`
- borrowed gene regions as slices
- `Mutation` enum for substitutions, insertions, and deletions

### Session 09: Visualizing Molecules and Data Algorithms

Core questions:

- When should molecule data use `Vec`, sorted `Vec`, `HashMap`, graph adjacency,
  or 3D coordinates?
- Which algorithmic operations are sorting, merging, counting, and lookup?
- Why is a 3D visualization useful even when graph algorithms use atom indices?
- What does each representation make cheap or expensive?

Implementation focus:

- atom vectors and bond vectors
- sorted formula-count output
- `HashMap<Element, usize>` for counting
- merge-on-key workflows for labels or scores
- 3D teaching coordinates for visualization

### Session 10: Designing Molecules as Data Search

Core questions:

- How do scaffold, substituent, candidate, constraint, feature, score, and ranking
  map to data structures?
- When should candidates be cloned, borrowed, filtered, indexed, or sorted?
- Which stage needs a graph, hash map, sorted vector, or queue?
- Why does a toy score not establish real chemical usefulness?

Implementation focus:

- `Substituent` enum
- `Candidate` struct
- validation filters
- `HashMap<String, f64>` score lookup
- sorted `Vec<Candidate>` ranking
- queue/frontier for iterative search

### Session 11: Molecular Libraries And Search Indexes

Core questions:

- When should a molecule library scan a vector, use a hash map, or sort records?
- How should one-key-to-many-record relationships be represented?
- What must be true before binary search is valid?
- When is merging sorted library outputs better than sorting from scratch?

Implementation focus:

- `MoleculeRecord` struct
- `MoleculeLibrary` vector storage
- `HashMap<String, MoleculeRecord>` name lookup
- `HashMap<String, Vec<String>>` formula buckets
- sorted `Vec<MoleculeRecord>` by atom count
- merge and binary-search boundary functions

### Session 12: Toy Fingerprints And Similarity Search

Core questions:

- What molecule information can be represented as yes/no feature bits?
- How do bitsets model compact sets in memory?
- How do intersection and union become a similarity score?
- What information does a fingerprint discard from the full molecule graph?

Implementation focus:

- `Feature` enum
- `ToyFingerprint { bits: u64 }`
- bit insertion and membership checks
- shared and union feature counts
- `rank_by_similarity` over borrowed molecule-library records
- sorted nearest-match output

### Session 13: Screening Results And Feedback Loops

Core questions:

- How should a score become a test/defer decision?
- Why are experimental labels optional in the data model?
- Which errors are false positives and false negatives?
- How do precision, recall, and accuracy answer different questions?
- Which candidates should the next feedback round prioritize?

Implementation focus:

- `Prediction` with optional experimental label
- `ScreeningDecision` thresholding
- `ConfusionMatrix`
- precision, recall, and accuracy with zero-denominator handling
- sorted testing queue
- near-threshold uncertainty selection

### Session 14: Serialization And Reproducible Lab Records

Core questions:

- What is the difference between structured data and serialized data?
- What schema guarantees does a parser enforce?
- Which parsing failures should be represented explicitly?
- What does a round-trip test prove and not prove?
- Which provenance fields are missing from a toy record?

Implementation focus:

- `LabRecord` struct
- stable header and field order
- `to_record_line` serialization
- `from_record_line` parsing
- `LabRecordParseError`
- Markdown report row rendering

### Session 15: Typed Measurements, Units, And Assay Observations

Core questions:

- Why is a bare `f64` not enough for chemistry measurements?
- Which units should be represented as enum variants?
- When should constructors return `Result`?
- Why should sorting normalize units first?
- What metadata is missing from a toy assay observation?

Implementation focus:

- `ConcentrationUnit`
- `Concentration`
- `MeasurementError`
- `AssayObservation`
- unit conversion to micromolar
- response range validation
- observation sorting and mean response

### Session 16: Replicates, Variability, And Uncertainty

Core questions:

- Why is one measurement not enough?
- What does mean hide about variability?
- Why does sample variance require at least two values?
- When is coefficient of variation useful?
- How should noisy replicates affect feedback decisions?

Implementation focus:

- `ReplicateSeries`
- response-vector validation
- mean, min, max, and range
- sample variance and standard deviation
- coefficient of variation
- consistency checks under a spread threshold

### Session 17: Dose-Response Curves And Toy Potency

Core questions:

- Why should dose points be sorted by normalized concentration?
- What makes a curve monotonic?
- When can an adjacent-point search find a target crossing?
- Why does interpolation return `Option<f64>`?
- What does a toy half-max estimate omit from real curve fitting?

Implementation focus:

- `DosePoint`
- `DoseResponseCurve`
- sorted dose vectors
- duplicate concentration validation
- response span
- monotonicity check
- linear interpolation for target crossing

## Assessment Ideas

- Code review: students explain representation choices in their Rust model.
- Algorithm lab: implement shortest path and connected components.
- Design memo: compare string-based element labels with enum-based labels.
- Reflection: explain why quantum state cannot be copied and read like normal data.
- Design memo: compare DNA sequence as `String`, `Vec<Base>`, and borrowed slices.
- Design memo: choose data structures for visualization, lookup, ranking, and merging.
- Design memo: choose data structures for a scaffold-to-analog design round.
- Design memo: choose indexes for a toy molecule library and justify query costs.
- Design memo: compare full molecule graphs with compact fingerprints and state
  what each representation can and cannot answer.
- Design memo: evaluate a toy screen and recommend a next experiment based on
  false positives, false negatives, and uncertain scores.
- Design memo: design a minimal reproducible record schema and name the metadata
  required before it could support real lab use.
- Design memo: choose typed measurement representations for concentration,
  response, units, replicates, and assay provenance.
- Design memo: summarize replicate evidence and recommend whether to trust,
  repeat, or deprioritize a candidate.
- Design memo: compare toy interpolation with real dose-response requirements
  such as replicates, controls, uncertainty, and fitted models.

## Capstone Rubric

| Criterion | Strong evidence |
| --- | --- |
| Rust type design | Invalid labels and invalid bonds are constrained or detected. |
| Algorithmic thinking | Graph operations are correct and tested. |
| Scientific humility | The project states what its model leaves out. |
| Communication | Code, diagrams, and explanation agree with each other. |

## Visuals to Use

- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)
- [Ownership and borrowing contract](../visuals/plantuml/ownership-borrowing-contract.puml)
- [Memory address flow](../visuals/plantuml/memory-address-flow.puml)
- [Quantum simulation loop](../visuals/plantuml/quantum-simulation-loop.puml)
- [Traits and molecular behavior](../visuals/plantuml/traits-and-functions.puml)
- [Molecular library indexes](../visuals/mermaid/molecular-library-indexes.md)
- [Toy fingerprints and similarity search](../visuals/mermaid/fingerprint-similarity.md)
- [Screening feedback loop](../visuals/mermaid/screening-feedback-loop.md)
- [Serialization and lab records](../visuals/mermaid/serialization-lab-records.md)
- [Typed measurements](../visuals/mermaid/typed-measurements.md)
- [Replicates and uncertainty](../visuals/mermaid/replicates-uncertainty.md)
- [Dose-response curves](../visuals/mermaid/dose-response-curves.md)

For university students, treat diagrams as design artifacts. Ask what each diagram
forces into the model and what it hides.

## Implementation Lab

1. Refactor a concrete molecule API into traits.
2. Write generic functions over `&impl MolecularGraph`.
3. Compare returning owned `Vec<usize>` with iterator-based APIs.
4. Decide which methods deserve default trait implementations.

Use the [Traits API Design worksheet](../worksheets/university-traits-api-design.md)
for this lab.

## First Project

Use the [Molecule Explorer CLI](../projects/molecule-explorer-cli.md) as the first
capstone. Have students refactor the argument parsing into testable `Result`-based
functions.

Use [Lesson 07](../lessons/07-molecule-explorer-cli-capstone.md) for the in-class
capstone sequence.

Use the [CLI Extension worksheet](../worksheets/university-cli-extension.md) for
the implementation lab.

## Biological Memory Lab

Use [Lesson 08](../lessons/08-dna-biological-memory-data-structures.md), the
[DNA handout](../handouts/dna-memory-data-structures.md), and the
[DNA worksheet](../worksheets/dna-memory-data-structures.md). Ask students to
defend each memory choice: stack-friendly enum, fixed codon array, heap-backed
strand vector, borrowed slice, or owned mutation log.

## Visualization And Algorithms Lab

Use [Lesson 09](../lessons/09-visualizing-molecules-and-data-algorithms.md), the
[3D molecule viewer](../visuals/html/molecule-3d-viewer.html), and the
[visual data algorithms worksheet](../worksheets/visualizing-data-algorithms.md).
Ask students to design one storage representation, one lookup representation, one
sorted report, and one merge operation for the same molecule.

## Molecule Design Lab

Use [Lesson 10](../lessons/10-designing-molecules-as-data-search.md), the
[molecule design worksheet](../worksheets/molecule-design-data-search.md), and the
[molecule design algorithms diagram](../visuals/mermaid/molecule-design-algorithms.md).
Ask students to model one full round: scaffold, options, candidates, constraints,
features, scores, ranked list, and next frontier.
