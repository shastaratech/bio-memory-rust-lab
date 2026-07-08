# Tutor Script: Course Intro And Lesson 01

## Teaching Position

Use this script for a first live session or recorded tutorial. The goal is not to
teach all Rust or all chemistry. The goal is to give beginners a stable mental
model:

- molecules have structure
- programs store structure as typed data
- Rust makes structure and access rules explicit
- analogies are useful, but they have limits

## Materials

- `README.md`
- `book/00-setup.md`
- `book/00-chemistry-primer.md`
- `book/00b-how-formulas-become-graphs.md`
- `book/01-atoms-memory-types.md`
- `lessons/01-atoms-memory-types.md`
- `handouts/atoms-memory-rust.md`
- `worksheets/school-atom-cards.md`
- `visuals/mermaid/course-map.md`
- `visuals/plantuml/molecule-data-model.puml`
- terminal in `exercises/rust-molecule-model`

## Segment 1: Course Intro, 5-7 Minutes

### Instructor Script

Welcome to Bio Memory Rust Lab. This course uses molecular structure as a way to
learn Rust and computer science. We will not pretend that molecules are just code.
Instead, we will use molecules as a disciplined analogy for typed data,
relationships, memory, and algorithms.

Start one level below atoms. A nucleus contains protons and usually neutrons. The
number of protons determines the element: hydrogen has one, carbon has six, and
oxygen has eight. Electrons are much lighter negatively charged particles described
by quantum probability clouds around nuclei, not tiny planets on fixed paths.

Start with one molecule: water. Water has one oxygen atom and two hydrogen atoms.
That formula, `H2O`, is compact. It tells us counts. It does not tell us the full
graph unless we add bonds.

Before we code, define two chemistry words. A bond is a chemical connection between
two atoms. The line we draw is not a tiny stick; it is a symbol for a stable
relationship caused by electron density between atoms. Aromaticity does not mean
smell here. It means a special delocalized bonding pattern in some rings, such as
benzene. In our Rust model, aromaticity is stored as a boolean flag or an aromatic
bond order.

Also define valence. Valence is how many bonds an atom commonly forms. Hydrogen
usually forms one bond, oxygen two, nitrogen three, and carbon four. In graph
language, valence is like a maximum degree rule for an atom node, but it belongs to
atoms, not quarks. Quarks have properties such as electric charge, flavor, spin, and
color charge; they do not have chemical valence.

Use hydrogen and chlorine to show the limit. Both commonly form one bond, but
hydrogen has one proton and one electron while chlorine has 17 protons and 17
electrons. Same valence does not mean same behavior. It means one shared connection
constraint, not identical internal state.

Now look at a program. A Rust program also stores facts in structured ways. An atom
can be represented by fields such as element, charge, and aromaticity. A molecule
can own a list of atoms and a list of bonds. A graph algorithm can ask which atoms
are neighbors.

The course moves from simple typed facts to larger structures: vectors, graphs,
borrowing, DNA sequences, hash maps, sorted lists, screening data, measurements,
replicates, dose-response curves, controls, and normalization.

Keep one rule in mind: analogy helps you start; precision helps you finish. Atoms
are not bytes. Bonds are not Rust references. DNA is not a hard drive. But each
comparison helps us ask better questions about state, structure, and constraints.

### Student Prompt

What information is visible from the formula `H2O`, and what information is still
missing?

### Expected Student Answers

- Visible: two hydrogens and one oxygen.
- Missing: which atoms are bonded, geometry, charges, electron behavior, physical
  context.
- A bond line is a drawing shortcut for a chemical connection, not a physical rod.

### Fallback Explanation

The formula is like a summary. A graph is more detailed because it includes
relationships. Real chemistry is even richer because molecules have 3D geometry and
quantum behavior.

## Segment 2: Lesson 01 Intro, 10-12 Minutes

### Instructor Script

Lesson 01 is about atoms, memory, and types.

In Rust, a type tells the compiler what kind of value something is. That is similar
to how a chemistry model must know whether a symbol means hydrogen, carbon, oxygen,
or something else. If a student types `"chlorene"` by mistake, plain text might
accept it. A Rust enum can reject unsupported element names before the program runs.

Show this code:

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

This is a constrained vocabulary. It does not represent the whole periodic table.
It represents the subset used by this toy model.

Now show an atom:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

Each field has a reason:

- `Element` prevents arbitrary element labels.
- `i8` allows negative, zero, and positive formal charge.
- `bool` is enough for a true/false aromatic flag in this beginner model.

Then show a molecule:

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

The molecule owns two growable lists. One list stores atom records. The other list
stores bond records. A bond stores atom indices, not direct self-references, because
graph-like structures are easier to teach safely this way in Rust.

Pause on `BondOrder::Aromatic`. Say: aromatic bonds are a simplified way to mark
delocalized ring bonding. We are not teaching full aromaticity detection yet; we
are showing that chemistry software often stores this feature because algorithms
need to know about it.

### Student Prompt

Why is `Element::Cl` better than storing `"cl"` as text?

### Expected Student Answer

The enum gives a fixed set of valid choices. It prevents spelling and capitalization
errors inside the clean model.

### Fallback Explanation

Text is flexible but easy to mistype. An enum is less flexible but safer. In early
lessons, safety is more important than supporting every possible element label.

## Segment 3: Lesson 01 Guided Coding, 15-20 Minutes

### Setup

Run:

```bash
cd exercises/rust-molecule-model
cargo test
cargo run -- water summary
```

### Instructor Script

First, we test the project. Tests tell us whether the examples and model still
match the expected behavior.

Now ask Rust to summarize water:

```bash
cargo run -- water summary
```

Expected ideas:

- water has formula `H2O`
- it has 3 atoms
- it has 2 bonds
- its bond indices should validate

Next, inspect the atoms:

```bash
cargo run -- water atoms
```

Connect each terminal line to an atom card:

```text
0: O atom (neutral)
1: H atom (neutral)
2: H atom (neutral)
```

The number is the atom ID. The element comes from the enum. The charge is stored in
the atom struct.

Now inspect bonds:

```bash
cargo run -- water bonds
```

Expected output:

```text
0: single bond: atom 0 <-> atom 1
1: single bond: atom 0 <-> atom 2
```

The bond is an edge in the graph. It connects two atom IDs.

Finally, ask for neighbors:

```bash
cargo run -- water neighbors 0
```

The oxygen atom is connected to both hydrogens. This is the first graph algorithm:
given one node, find connected nodes.

### Student Prompt

What does the molecule own: atoms, bonds, or both?

### Expected Student Answer

Both. The molecule owns `Vec<Atom>` and `Vec<Bond>`.

### Fallback Explanation

The molecule is the container. Atoms and bonds are its internal data. Algorithms
can borrow the molecule to read it without taking it apart.

### Memory Sketch

Draw this:

```text
Stack frame
└── molecule variable
    └── owns heap-backed Vec storage
        ├── atoms: [Atom, Atom, Atom]
        └── bonds: [Bond, Bond]

borrowed read
└── &molecule passed to an algorithm
```

Say:

The exact machine layout is more complex, but this sketch is enough for now. The
stack holds local variables and handles. Growable lists use heap storage. A
reference lets a function inspect data without taking ownership.

## Segment 4: Lesson 01 Wrap-Up, 5 Minutes

### Instructor Script

Today, we used one small molecule to learn several programming ideas:

- an enum is a safe vocabulary
- a struct groups related facts
- a vector stores a growable list
- a molecule can be represented as a graph
- a bond is a graph edge between atom IDs
- a function can borrow a molecule to inspect it

Now we must state the limits. A molecule graph is not the full molecule. It leaves
out geometry, electron density, solvent, temperature, measurement uncertainty, and
many other physical details. That is fine. A beginner model is allowed to be small
if it is honest about what it omits.

### Checkpoint Questions

1. Why is `Element` an enum instead of a string?
2. Why can formal charge use a signed integer?
3. What does `Vec<Atom>` represent?
4. Why does a `Bond` store two atom IDs?
5. What does borrowing let an algorithm do?
6. What does the toy graph model leave out about real molecules?

### Strong Answers

1. It prevents unsupported element labels in the clean model.
2. Charge can be negative, zero, or positive.
3. A growable list of atom records owned by the molecule.
4. A bond connects two atoms in the graph.
5. Read the molecule without taking ownership or copying it.
6. 3D shape, electron behavior, environment, and detailed chemistry.
