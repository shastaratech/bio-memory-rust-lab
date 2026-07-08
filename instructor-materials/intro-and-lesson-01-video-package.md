# Video Package: Course Intro And Lesson 01

## Production Goal

Create two short beginner videos:

1. Course intro: 5-7 minutes.
2. Lesson 01 walkthrough: 20-30 minutes.

The videos should make the course feel concrete without overstating the chemistry.
The core message is: molecules are structured systems, and Rust gives us precise
tools for representing structure.

## Video 1: Course Intro

### Segment Outline

| Time | Segment | Purpose |
| --- | --- | --- |
| 0:00-0:30 | Opening question | Ask where information lives in a molecule and a program. |
| 0:30-1:45 | Physical building blocks | Explain protons, neutrons, nuclei, electrons, and electron clouds. |
| 1:45-2:45 | Water as a first structure | Show `H2O`, atom cards, and a simple graph. |
| 2:45-3:30 | Valence | Explain common bond counts and graph-degree intuition. |
| 3:30-4:30 | Bonds and aromaticity | Explain bond lines, electron density, and aromatic ring flags. |
| 4:30-5:30 | Rust as structured data | Show enum, struct, vector, and graph vocabulary. |
| 5:30-6:45 | Course path | Show setup, Lesson 01, graph lessons, DNA, algorithms, lab data. |
| 6:45-7:45 | Analogy discipline | State where the analogy works and where it fails. |
| 7:45-8:30 | First action | Run tests and inspect `water summary`. |

### Slide Plan

#### Slide 1: What Does A Molecule Remember?

- Formula stores counts.
- Bonds store relationships.
- Geometry and quantum state need richer models.
- Code must choose which layer to represent.

Visual direction: show `H2O`, three atom cards, and two bond lines.

Narration:

Welcome to Bio Memory Rust Lab. We start with a simple question: what does a
molecule remember, and how would a program represent it?

#### Slide 2: Before Atoms Become Data

- Protons and neutrons make up the nucleus.
- Proton count determines the element.
- Electrons are leptons, not quark-built particles.
- Electrons are much lighter and negatively charged.
- Electron shells contain electrons, not protons.
- Electron clouds are probability distributions, not planet-like orbits.

Visual direction: show a nucleus with protons/neutrons and a dotted electron cloud.

Narration:

Before an atom becomes a record in code, it is a physical system. The nucleus gives
the atom its elemental identity and contains almost all atomic mass. Electrons are
leptons, not quark-built particles, and they form quantum probability clouds around
the nucleus. Electron shells are allowed energy levels for electrons; they are not
hard layers and they do not contain protons.

#### Slide 3: Chemistry To Computer Science

- Atom -> typed value or record.
- Bond -> graph edge.
- Molecule -> owned aggregate.
- Formula -> compact summary.
- Algorithm -> question over structure.

Visual direction: two-column mapping table with chemistry on the left and Rust/CS
on the right.

Narration:

The mapping is not perfect, but it is useful. An atom can become a typed record. A
bond can become an edge. A molecule can become a graph-like data structure.

#### Slide 4: What Is A Bond?

- A bond is a chemical connection between atoms.
- The drawn line is a symbol, not a physical stick.
- Physically, bonding comes from electron density between nuclei.
- In code, the beginner model stores this as an edge.

Visual direction: show `H - O - H` beside a cloudy electron-density sketch.

Narration:

When chemists draw a line between atoms, the line is a shortcut. The physical
picture is electron density shared between nuclei. Our Rust model does not simulate
that density; it records the connection as data.

#### Slide 5: What Is Valence?

- Valence is how many bonds an atom commonly forms.
- Hydrogen usually forms 1, oxygen 2, nitrogen 3, carbon 4.
- In a graph, valence is like a degree limit for an atom node.
- Hydrogen and chlorine both form 1 bond, but differ internally.
- Quarks do not have chemical valence.

Visual direction: show methane with carbon connected to four hydrogens.

Narration:

Valence is a beginner rule for how many bonds an atom commonly forms. It is useful
when checking whether a molecular graph looks plausible. It belongs to atoms
because atom-level electrons make chemical bonds. Quarks sit inside protons and
neutrons, so they have different properties, such as color charge, not valence.

Valence also has limits. Hydrogen and chlorine both commonly form one bond, but
hydrogen has one proton and one electron while chlorine has 17 of each in a neutral
atom. Same connection count does not mean same chemical behavior.

#### Slide 6: What Is Aromaticity?

- Aromaticity is not about smell in this lesson.
- It is a special delocalized bonding pattern in some rings.
- Benzene is the standard beginner example.
- Code often stores it as `aromatic: bool` or `BondOrder::Aromatic`.

Visual direction: show a benzene-like hexagon and a boolean field.

Narration:

Aromaticity is a special ring-bonding pattern where some electrons are spread
across the ring. Full aromaticity detection is chemistry. Here, we only need to
understand why software may store an aromatic flag.

#### Slide 7: Rust Makes Structure Explicit

- `enum Element` limits valid labels.
- `struct Atom` groups atom facts.
- `Vec<Atom>` stores growable atom lists.
- `&Molecule` allows borrowed reading.

Visual direction: show a trimmed code block beside atom cards.

On-screen code:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

#### Slide 8: Course Roadmap

- Setup and chemistry primer.
- Atoms, memory, and types.
- Molecules as graphs.
- Ownership, borrowing, traits, and CLI.
- DNA, indexes, screening, measurements, and controls.

Visual direction: use the course map from `visuals/mermaid/course-map.md`.

#### Slide 9: Analogy Helps, Precision Finishes

- Atoms are not bytes.
- Bonds are not Rust references.
- DNA is not a hard drive.
- Quantum state is not ordinary memory.
- Small models are useful when limits are clear.

Visual direction: warning icon plus "Useful / Limit" columns.

#### Slide 10: Your First Command

- Run tests.
- List water atoms.
- List water bonds.
- Ask for neighbors.

On-screen terminal:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- water summary
```

## Video 2: Lesson 01 Walkthrough

### Segment Outline

| Time | Segment | Purpose |
| --- | --- | --- |
| 0:00-1:30 | Lesson target | Explain atoms, memory, and types. |
| 1:30-4:30 | Enum vocabulary | Explain `Element` and why text labels are risky. |
| 4:30-8:00 | Atom struct | Explain fields and scalar types. |
| 8:00-12:00 | Bond and molecule structs | Show graph representation with atom IDs. |
| 12:00-17:00 | Terminal walkthrough | Run water commands and match output to structures. |
| 17:00-21:00 | Stack, heap, and borrowing | Use a careful beginner memory sketch. |
| 21:00-25:00 | Worksheet activity | Build atom cards and neighbor list. |
| 25:00-28:00 | Limits and checkpoint | State what the toy model omits. |

### Slide Plan

#### Slide 1: Lesson 01 Target

- Represent atoms as typed data.
- Represent bonds as graph edges.
- Store molecules as owned vectors.
- Read structures with borrowed access.

Visual direction: course title plus water molecule card sketch.

#### Slide 2: Why Not Just Strings?

- `"Cl"`, `"cl"`, and `"chlorine"` are all text.
- A toy model needs one supported spelling.
- An enum makes invalid labels harder to create.

On-screen code:

```rust
pub enum Element {
    H,
    C,
    N,
    O,
    F,
    Cl,
    Br,
}
```

#### Slide 3: Atom As A Struct

- `element: Element`
- `formal_charge: i8`
- `aromatic: bool`
- Each field has a specific type.

Visual direction: annotate the `Atom` struct with arrows to an atom card.

#### Slide 4: Bond As A Graph Edge

- `from` is one atom ID.
- `to` is another atom ID.
- `order` is single, double, triple, or aromatic.

On-screen code:

```rust
pub struct Bond {
    pub from: usize,
    pub to: usize,
    pub order: BondOrder,
}
```

#### Slide 5: Molecule Owns Lists

- `atoms: Vec<Atom>`
- `bonds: Vec<Bond>`
- Vectors are growable lists.
- Atom IDs index into the atom list.

Visual direction: show a table of atom IDs and a table of bond rows.

#### Slide 6: Run The Model

On-screen terminal:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- water atoms
cargo run -- water bonds
cargo run -- water neighbors 0
```

Narration:

The terminal output is not separate from the model. It is a readable view of the
same atoms and bonds stored in Rust data structures.

#### Slide 7: Memory Sketch

- Stack: local variables and call frames.
- Heap: growable vector storage.
- Owner: the molecule value.
- Borrow: temporary read access.

Visual direction:

```text
molecule
├── atoms -> [O, H, H]
└── bonds -> [(0,1), (0,2)]

count_atoms(&molecule)
```

#### Slide 8: What The Model Leaves Out

- 3D coordinates.
- Electron density.
- Solvent and temperature.
- Measurement uncertainty.
- Many chemistry validity rules.

Narration:

Leaving things out is not a failure if the model says what it leaves out. This
course starts small so the Rust ideas are visible.

## Narration Draft

### Course Intro Narration

Programming and chemistry both care about structure. A molecule is not just a bag
of atoms, and a program is not just a bag of bytes. Structure determines what
questions we can ask.

Start one level deeper than atoms. Protons and neutrons form the nucleus. The
number of protons determines the element. Electrons are much lighter and are
described by quantum probability clouds around nuclei, not fixed planet-like
orbits. Shells are a simplified way to describe allowed electron energy levels
around the nucleus. The electron cloud is not the electron's size; it is a map of
where the electron may be detected.

A chemical bond is a connection between atoms. The line in a molecule drawing is a
symbol, not a tiny stick. Physically, bonding comes from electron density shared
between nuclei. In our beginner code, a bond becomes an edge between atom IDs.

Valence tells us how many bonds an atom commonly forms. Hydrogen commonly forms
one, oxygen two, nitrogen three, and carbon four. In a molecular graph, this acts
like a degree rule for atom nodes. Quarks do not have chemical valence; they belong
to a deeper level of matter and have properties such as electric charge, flavor,
spin, and color charge.

This rule is not enough to identify an element. Hydrogen and chlorine can both form
one bond, yet chlorine has many more protons, electrons, shells, and a stronger pull
on shared electron density.

Aromaticity is another important chemistry word. It does not mean smell here. It
means a special delocalized bonding pattern in some rings. Software often stores
that feature because search and property algorithms may need it.

In this course, we use molecules to learn Rust. An atom becomes a typed record. A
bond becomes a graph edge. A molecule becomes an owned structure that contains
vectors of atoms and bonds.

The analogy is useful, but it has limits. Atoms are physical systems. Rust values
are program data. Bonds are chemical relationships. Rust references are checked
access paths. DNA stores biological information, but it is not the same thing as a
computer hard drive.

That discipline matters. If we use the analogy carefully, we can learn types,
ownership, borrowing, vectors, graphs, hash maps, sorted lists, and lab data
workflows without pretending the toy model is real production chemistry.

Your first job is to compile the model, run the tests, and inspect water. You will
see atoms, bonds, formulas, and graph neighbors as real Rust output.

### Lesson 01 Narration

Lesson 01 starts with types. A type is a rule for what kind of value something is.
Rust wants those rules to be explicit.

For element labels, a plain string is flexible, but it is easy to mistype. A toy
model can use an enum instead. `Element::Cl` means the supported chlorine variant.
The compiler can help reject unsupported labels inside the clean model.

An atom has fields. The element field uses the enum. The formal charge uses a
signed integer because charges can be negative, zero, or positive. The aromatic
flag uses a boolean because this beginner model only asks true or false.

A bond connects two atom IDs. This is a graph edge. A molecule owns a vector of
atoms and a vector of bonds. That gives us enough structure to ask graph questions:
which atoms are connected, how many bonds exist, and whether a path exists between
two atoms.

When we run `cargo run -- water atoms`, the terminal shows the atom list. When we
run `cargo run -- water bonds`, it shows the graph edges. When we run `cargo run --
water neighbors 0`, it asks which atoms are connected to oxygen.

Memory comes next. For beginners, keep the picture simple. A local molecule value
is owned by a variable. Growable vectors use heap storage. A function can borrow
the molecule with `&Molecule` and read it without taking ownership.

This model is useful, but it is not complete chemistry. It leaves out 3D geometry,
electron behavior, solvent, temperature, and many validity rules. That is the point:
small honest models let us learn one idea at a time.

## Caption Draft

### Course Intro Captions

```text
Bio Memory Rust Lab teaches Rust through molecular structure.
A formula like H2O gives atom counts.
A graph adds relationships between atoms.
Rust gives us types, structs, vectors, and borrowing.
An atom can be represented as a typed record.
A bond can be represented as an edge between atom IDs.
A molecule can own vectors of atoms and bonds.
The analogy is useful, but it has limits.
Atoms are not bytes.
Bonds are not Rust references.
DNA is not a hard drive.
Small models are useful when their limits are clear.
Start by running the tests and inspecting water.
```

### Lesson 01 Captions

```text
Lesson 01 is about atoms, memory, and types.
A type is a rule for what kind of value something is.
An enum gives us a constrained vocabulary.
Element::Cl is safer than arbitrary text labels in this toy model.
An Atom groups element, charge, and aromaticity.
A Bond connects two atom IDs.
A Molecule owns vectors of atoms and bonds.
The molecule is a graph-like data structure.
We can run commands to inspect atoms, bonds, and neighbors.
Borrowing lets an algorithm read a molecule without taking ownership.
The toy model leaves out real chemical detail.
That is acceptable because the model is honest and beginner-sized.
```

## B-Roll And Visual Assets

- Hand-drawn water cards: O, H, H.
- String or marker lines connecting oxygen to hydrogens.
- Screen recording of `cargo test`.
- Screen recording of `cargo run -- water atoms`.
- Screen recording of `cargo run -- water bonds`.
- Screen recording of `cargo run -- water neighbors 0`.
- Simple stack/heap sketch on a whiteboard.
- Course map from `visuals/mermaid/course-map.md`.
- Molecule data model from `visuals/plantuml/molecule-data-model.puml`.

## Recording Checklist

- Show file path before showing code.
- Zoom terminal font to readable size.
- Keep code snippets under 12 visible lines.
- State analogy limits explicitly.
- Avoid claiming the toy model validates real chemistry.
- End with one command students can run immediately.
