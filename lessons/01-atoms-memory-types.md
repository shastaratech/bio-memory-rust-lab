# Lesson 01: Atoms, Memory, and Types

## Goal

Students will learn Rust data types and basic data-structure thinking by modeling
molecules. They will compare three forms of state:

- molecular state: atoms, bonds, geometry, charge, conformation
- classical computer state: bits, bytes, addresses, stack, heap, structs, arrays
- quantum state: amplitudes, superposition, measurement, molecular orbitals

By the end, students should be able to explain why Rust cares about type, ownership,
and borrowing using molecule-design language.

## Visuals

- [Molecule data model](../visuals/plantuml/molecule-data-model.puml)
- [Memory address flow](../visuals/plantuml/memory-address-flow.puml)
- [Course map](../visuals/mermaid/course-map.md)

## Learning Outcomes

Students can:

1. Map Rust scalar types to atom-level facts.
2. Build Rust structs and enums for atoms, bonds, and molecules.
3. Explain stack and heap memory with a small molecule model.
4. Represent a molecule as a graph and describe graph traversal.
5. Distinguish classical memory from quantum state without saying that a qubit is just
   a bigger bit.
6. Describe molecule design as a constrained search algorithm.

## Timing

Recommended length: 90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show one molecule and one Rust struct side by side. Ask: where is the information? |
| 10-25 min | Atoms as typed values | Rust scalar types through atom properties. |
| 25-40 min | Molecules as structs | Build `Atom`, `Bond`, and `Molecule`. |
| 40-55 min | Memory | Stack, heap, references, and ownership. |
| 55-70 min | Algorithms | Molecule as graph: neighbors, paths, rings, scoring. |
| 70-82 min | Quantum state | Why molecular simulation needs quantum ideas. |
| 82-90 min | Reflection | Students state one useful analogy and one place the analogy fails. |

## Core Analogy

| Chemistry concept | Rust / CS concept | Good teaching use | Limit |
| --- | --- | --- | --- |
| Atom | scalar or struct value | A value with properties and constraints | Atoms have physical behavior; data values do not. |
| Element symbol | enum variant | A fixed set of named possibilities | The periodic table is physical, not a programming enum. |
| Atomic number | integer | Countable identity field | Chemical identity is not only an integer in practice. |
| Charge | signed integer | Positive, zero, negative state | Real charge distribution is richer. |
| Bond | edge/reference-like relation | Connects two atom indices | A Rust reference has lifetime rules; a bond has chemistry. |
| Molecule | graph / aggregate type | Nodes plus edges plus metadata | Molecules are 3D quantum systems, not only graphs. |
| Valence rule | type invariant | Invalid structures should be rejected early | Chemical validity is contextual. |
| Conformation | runtime state | Same graph, different geometry | Rust type may not encode all geometry. |
| Binding score | algorithm result | Function over a structure | Docking scores are approximations, not truth. |

## Part 1: Rust Types as Atom Facts

Rust is statically typed: the compiler must know what kind of value each expression
has. Start with atom properties:

```rust
let atomic_number: u8 = 6;      // carbon
let formal_charge: i8 = 0;
let symbol: char = 'C';
let is_aromatic: bool = false;
let mass: f64 = 12.011;
```

Teaching notes:

- `u8` works for atomic number because known elements fit in a small positive integer.
- `i8` works for formal charge because charge can be negative, zero, or positive.
- `bool` works for yes/no flags such as aromaticity.
- `f64` works for approximate measured or computed values such as mass or coordinates.
- `char` is acceptable for a toy element symbol, but real symbols such as `Cl` need a
  string or enum.

## Part 2: Elements as Enums

An enum is a constrained vocabulary. That is exactly what students need when they
should not type arbitrary element labels.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    H,
    C,
    N,
    O,
    F,
    Cl,
    Br,
}
```

Discussion prompt:

What is better about `Element::Cl` than `"cl"`, `"CL"`, or `"chlorine"`?

Expected answer:

The enum makes invalid spelling impossible at compile time for the supported set.
The compiler becomes a small lab-safety assistant for data shape.

## Part 3: Atoms and Bonds as Structs

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}

#[derive(Debug, Clone)]
struct Atom {
    element: Element,
    formal_charge: i8,
    aromatic: bool,
}

#[derive(Debug, Clone)]
struct Bond {
    from: usize,
    to: usize,
    order: BondOrder,
}

#[derive(Debug, Clone)]
struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

Key idea:

The molecule owns its atoms and bonds. A `Bond` stores atom indices instead of direct
references, because graph-like structures are easier to represent safely with stable
IDs than with many self-references.

## Part 4: Classical Memory Through Molecules

Use the old Habr article as a historical systems hook: programmers usually work with
logical or virtual addresses, while hardware and the operating system map those to
physical memory. The article explains x86 concepts such as registers, stack, segments,
descriptors, and the logical-to-linear-to-physical address path.

Student-level version:

- A value in source code is not the same thing as a physical memory cell.
- A variable name is a human-readable handle.
- A pointer or reference is closer to an address-like handle.
- The stack is fast, scoped, and structured like nested lab trays.
- The heap is for data whose size or lifetime is less fixed at compile time.
- Rust tracks ownership so that data has one responsible owner and borrowed access is
  checked.

Rust example:

```rust
fn count_atoms(molecule: &Molecule) -> usize {
    molecule.atoms.len()
}
```

The function borrows the molecule. It can inspect atom count without taking ownership
and without copying the whole molecule.

## Part 5: Data Structures and Algorithms from Molecule Design

Representing a molecule as a graph opens standard algorithms:

| Molecule-design task | Data structure / algorithm |
| --- | --- |
| Find atoms bonded to atom `i` | adjacency list |
| Count connected fragments | graph traversal |
| Detect rings | cycle detection |
| Compare two molecules | graph matching / fingerprints |
| Generate analogs | constrained search |
| Score docking poses | ranking and optimization |
| Choose next compounds to test | active learning loop |

Minimal adjacency function:

```rust
fn neighbors(molecule: &Molecule, atom_id: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for bond in &molecule.bonds {
        if bond.from == atom_id {
            result.push(bond.to);
        } else if bond.to == atom_id {
            result.push(bond.from);
        }
    }

    result
}
```

Discussion:

This is not the fastest possible representation, but it is transparent. Later students
can replace `Vec<Bond>` scanning with an adjacency list or matrix and compare the
algorithmic tradeoff.

## Part 6: Quantum State Without Hype

Molecules are quantum systems. Electrons are not tiny planets orbiting with exact
classical paths. A quantum state stores probability amplitudes, and measurement gives
an outcome according to those amplitudes.

Teaching comparison:

| Question | Classical memory | Quantum state | Molecule analogy |
| --- | --- | --- | --- |
| Basic unit | bit | qubit | electron/state component |
| Can copy freely? | usually yes | no unknown-state cloning | copying a full physical state is not trivial |
| Read without changing? | usually yes | measurement changes available state information | measurement perturbs small systems |
| Best use | exact stored values | quantum computation / simulation | electronic structure and energy |
| Student caution | not all data is just bytes | not a magic memory upgrade | molecule state is richer than a graph |

Use this sentence as the anchor:

Classical memory stores chosen symbols; quantum state represents physical amplitudes.

## Classroom Activity

Give each group a simple molecule: water, methane, ethanol, benzene, or coumarin.

Each group must:

1. Choose Rust types for atom properties.
2. Write an `Element` enum subset.
3. Sketch `Vec<Atom>` and `Vec<Bond>` for the molecule.
4. Identify one invariant, such as "oxygen in water has two single bonds."
5. Pick one algorithm they would run on the molecule graph.
6. Name one thing their graph leaves out, such as 3D geometry or electron density.

## Instructor Debrief

Good student answers will notice:

- Types reduce invalid states.
- Structs group related facts.
- Enums constrain vocabulary.
- Vectors support variable-size molecules.
- Indices are practical for graph ownership in Rust.
- Borrowing lets algorithms read structures without consuming them.
- Classical memory, molecule graphs, and quantum states are different layers of
  representation, not interchangeable realities.

## Homework

Implement the molecule structs and add:

1. `fn atom_count(&self) -> usize`
2. `fn bond_count(&self) -> usize`
3. `fn neighbors(&self, atom_id: usize) -> Vec<usize>`
4. `fn validate_bond_indices(&self) -> bool`
5. Optional: `fn formula(&self) -> String`
