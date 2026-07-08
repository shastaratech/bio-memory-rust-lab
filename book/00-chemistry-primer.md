# Chapter 00: Protons, Electrons, Atoms, Bonds, Molecules, And Formulas

## Big Idea

Before we model molecules in Rust, we need a small chemistry vocabulary.

This chapter explains:

- quarks
- protons
- neutrons
- atomic nuclei
- electrons
- electron clouds
- electron shells
- atom
- element
- valence
- bond
- bond order
- aromaticity
- molecule
- chemical formula
- why formulas such as `H2O`, `CH4`, and `C2H6O` are useful compact
  representations

You do not need to be a chemist. We are learning enough chemistry to understand the
data structures.

## Physical Building Blocks

Before we talk about atoms as records in Rust, we need one level of physical
foundation. These are physics ideas, not programming ideas, but they explain why
atoms have structure.

### Proton

A proton is a particle found in the atomic nucleus.

Beginner facts:

- it has positive electric charge, written `+1`
- it has a mass of about one atomic mass unit
- it is not elementary; it is made of three quarks
- its quark content is two up quarks and one down quark
- the quarks are held together by the strong nuclear interaction, carried by gluons

Simple sketch:

```text
        proton

       up
        ●
      /   \
    ●------●
   up      down
```

### Quark

A quark is a fundamental particle that helps make up protons, neutrons, and many
other particles. As far as the Standard Model of particle physics tells us, quarks
are not made of smaller known components.

Use this scale ladder:

```text
human
  ↓
cell
  ↓
molecule
  ↓
atom
  ↓
nucleus
  ↓
proton or neutron
  ↓
quark
```

For a long time, scientists treated the proton as if it were indivisible. In the
1960s, high-energy scattering experiments showed that protons and neutrons have
smaller internal components.

A proton contains three valence quarks:

```text
       up
        ●
      /   \
    ●------●
   up      down
```

That is:

- one up quark
- another up quark
- one down quark

A neutron also contains three valence quarks:

- one up quark
- two down quarks

Quarks have fractional electric charge:

| Quark | Electric charge |
| --- | --- |
| up | `+2/3` |
| down | `-1/3` |

This is unusual. It feels natural to expect charge to be `+1`, `-1`, or `0`,
because those are the charges students first meet:

| Particle | Relative electric charge |
| --- | --- |
| proton | `+1` |
| electron | `-1` |
| neutron | `0` |

Electric charge is a fundamental property that determines how particles interact
with electric and magnetic fields. Physicists measure charge in coulombs, but in
introductory chemistry and particle physics it is common to use relative units
where the proton has charge `+1`.

In those units, an up quark has two thirds of the proton's charge, and a down quark
has negative one third of the proton's charge. These fractional charges were found
experimentally and are built into the Standard Model. For this course, do not try
to derive `+2/3` and `-1/3` from a simpler rule. Treat them as measured features of
nature.

So the proton's charge is:

```text
+2/3
+2/3
-1/3
-----
+1
```

And the neutron's charge is:

```text
+2/3
-1/3
-1/3
-----
 0
```

Why do we not see loose `+2/3` or `-1/3` charges in everyday chemistry? Because
quarks are confined. They are observed only inside composite particles such as:

- baryons, which contain three quarks, such as protons and neutrons
- mesons, which contain a quark and an antiquark

The combinations normally produce familiar total charges such as `-1`, `0`, or
`+1`, though some particles can have charges such as `+2` or `-2`.

Money analogy:

```text
+2/3 coin + +2/3 coin + -1/3 coin = +1 coin
+2/3 coin + -1/3 coin + -1/3 coin =  0 coins
```

The analogy is imperfect because money is human-made and quark charge is a physical
property. The useful idea is that fractional parts can combine into a whole-number
total.

Quarks are held together by the strong nuclear interaction. The force carriers are
called gluons, from the word "glue":

```text
quark ===== gluon field ===== quark
```

This is a very simplified diagram. Inside a proton or neutron, quarks and gluons
are involved in a complex quantum field interaction.

One strange feature of quarks is confinement: quarks are not observed alone. If an
experiment tries to pull a quark away, the energy in the field grows until new
quarks are produced and new particles form. A rough analogy is stretching a very
strong elastic band: instead of giving you one loose end, the system creates new
particle pairs.

Physicists know six quark flavors:

| Flavor | Symbol |
| --- | --- |
| Up | `u` |
| Down | `d` |
| Charm | `c` |
| Strange | `s` |
| Top | `t` |
| Bottom | `b` |

Ordinary matter around us is made mostly from up and down quarks plus electrons.
The other quark flavors appear in high-energy environments such as particle
accelerators and cosmic-ray interactions.

What is inside a quark? At present, we do not know of any smaller structure. Some
hypotheses, such as preon models, propose deeper components, but they have not been
experimentally confirmed. For this course, treat quarks as fundamental physical
particles.

### Neutron

A neutron is also found in the atomic nucleus. It is almost as heavy as a proton,
but it has no net electric charge.

Beginner facts:

- it has neutral charge, written `0`
- it is made of three quarks
- its quark content is one up quark and two down quarks

Simple sketch:

```text
       neutron

      down
        ●
      /   \
    ●------●
 down      up
```

### Nucleus

The nucleus is the tiny central region of an atom. It contains protons and, for
most atoms, neutrons.

Examples:

| Atom | Typical nucleus in beginner examples |
| --- | --- |
| Hydrogen | 1 proton |
| Carbon | 6 protons and 6 neutrons |
| Oxygen | 8 protons and 8 neutrons |

The nucleus is much smaller than the whole atom. A useful scale fact is that the
nucleus is roughly 100,000 times smaller than the atom.

### Electron

An electron is an elementary particle in the Standard Model of particle physics.

Beginner facts:

- it has negative electric charge
- it is much lighter than a proton, about 1/1836 of a proton's mass
- as far as current physics knows, it is not made of smaller components

Electrons are essential for chemistry because chemical bonds depend on how electron
density is arranged around atomic nuclei.

### Electron Cloud

The old school picture shows electrons orbiting a nucleus like planets around the
Sun. That picture is useful as a first cartoon, but it is not physically accurate.

In quantum mechanics, an electron is described by a wavefunction. We usually cannot
say:

> The electron is exactly here.

Instead, we calculate the probability of finding the electron in different regions
of space. If we measured many identical atoms many times, the results would form a
probability cloud:

```text
        .......
     .............
   .....  ●  ......
     .............
        .......
```

The `●` is the nucleus. The dots represent regions where the electron is more likely
to be found.

### Electron Shell

This is a common point of confusion:

> Are there protons in the electron shell and also protons in the nucleus?

No. Protons are only in the nucleus. Electron shells contain electrons, not
protons.

A beginner atom sketch looks like this:

```text
          electron cloud
     e-        e-        e-

          ┌────────┐
          │ nucleus│
          │ p+  n  │
          │ p+  n  │
          └────────┘

     e-                 e-
```

where:

- `p+` means proton
- `n` means neutron
- `e-` means electron

The nucleus is the tiny dense center of the atom. For example, an oxygen nucleus
has 8 protons and, in the common oxygen-16 isotope, 8 neutrons.

The nucleus is extremely small compared with the whole atom. A useful analogy is:
if an atom were enlarged to the size of a football stadium, the nucleus would be
roughly like a pea near the center.

An electron shell is the region around the nucleus where electrons occupy allowed
energy levels. It is not a physical shell like an eggshell. It is a simplified way
to describe quantum states available to electrons.

Why do we call them shells? Because electrons occupy different energy levels, and
school diagrams often draw those levels as rings around the nucleus.

For carbon, a simplified shell model is:

```text
nucleus: 6 protons, usually 6 neutrons

1st shell: 2 electrons
2nd shell: 4 electrons
```

Common shell names in the school model:

| Shell number | Shell name | Maximum electrons in the simple model |
| --- | --- | --- |
| 1 | K | 2 |
| 2 | L | 8 |
| 3 | M | 18 |
| 4 | N | 32 |

Examples:

| Atom | Nucleus in common beginner example | Electron shells |
| --- | --- | --- |
| Hydrogen | 1 proton | 1st shell: 1 electron |
| Oxygen | 8 protons, 8 neutrons | 1st shell: 2 electrons; 2nd shell: 6 electrons |
| Chlorine | 17 protons, 18 neutrons | 1st shell: 2; 2nd shell: 8; 3rd shell: 7 |

For chlorine, the 7 electrons in the outer shell are the important beginner fact.
That is why chlorine often forms one chemical bond: it is close to the stable
outer-shell count of 8.

Why do electrons not simply fall into the nucleus? Because an electron is not a
tiny ball orbiting like a planet. In quantum mechanics, an electron occupies a
quantum state described by a wavefunction. It is better to say:

> An electron occupies an allowed quantum state.

not:

> An electron flies around the nucleus on a tiny circular track.

House analogy:

```text
4th floor
3rd floor
2nd floor
1st floor
──────────
foundation
```

The nucleus is like the foundation. Electrons are like residents. Shells are like
floors. The first floor has room for only two residents in the simple model. After
it is filled, additional residents occupy higher floors.

This matters in chemistry because reactions mostly involve electrons in the outer
shell. Inner electrons are held more tightly and usually do not directly participate
in ordinary chemical bonding. The outer-shell electrons are called valence
electrons.

Important limit:

> Shell diagrams are useful beginner models. Real electron states are orbitals with
> shapes such as `s`, `p`, `d`, and `f`, and they are not perfect circles.

### How Electron Clouds Form

Electrons have wave-like behavior, and the positively charged nucleus creates an
electric field. In that field, electrons can occupy only certain stable quantum
states called atomic orbitals.

An `s` orbital is roughly cloud-shaped around the nucleus:

```text
      .......
    ...........
   ..... ● .....
    ...........
      .......
```

A `p` orbital has two lobes:

```text
    ....     ....
  ......  ●  ......
    ....     ....
```

These drawings are not flight paths. They are rough maps of where an electron is
most likely to be found.

### Why Different Elements Exist

The number of protons determines the element.

| Proton count | Element |
| --- | --- |
| 1 | Hydrogen |
| 2 | Helium |
| 6 | Carbon |
| 8 | Oxygen |
| 26 | Iron |
| 79 | Gold |

Add one proton, and you have a different chemical element.

### Where The Particles Come From

In the modern physics picture, quarks, electrons, and neutrinos are fundamental
particles in the Standard Model. We do not currently know whether they are made of
anything smaller.

Early in the universe, matter was extremely hot and energetic. As the universe
cooled, quarks combined into protons and neutrons. Later, nuclei formed, and
electrons became bound to nuclei to form atoms.

### Programming Analogy

Use this analogy carefully:

- quarks are lower-level building blocks inside protons and neutrons, but they are
  physical particles, not programming primitives
- protons and neutrons are like the "core identity" of an atom; the proton count
  determines the element
- the electron cloud is not a fixed list of points; it is a distribution of
  possible measurement outcomes around the nucleus
- chemical bonds form because electron clouds from neighboring atoms interact

Limit:

> Quarks and electrons are not objects sitting in computer memory. Their behavior
> is quantum mechanical. The analogy helps us reason about structure, but physics
> is not literally Rust code.

## Atom

An atom is a tiny unit of matter.

For this course, think of an atom as one node in a molecule.

Examples:

- one hydrogen atom
- one oxygen atom
- one carbon atom

In Rust, our toy model represents an atom with:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}
```

That means an atom is not only a letter. It is a record with fields.

## Element

An element is a kind of atom.

Examples:

| Element name | Symbol | Rust enum variant |
| --- | --- | --- |
| Hydrogen | `H` | `Element::H` |
| Carbon | `C` | `Element::C` |
| Nitrogen | `N` | `Element::N` |
| Oxygen | `O` | `Element::O` |
| Chlorine | `Cl` | `Element::Cl` |

In this course, we use an enum:

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

Why?

Because element symbols are a controlled vocabulary. `Element::Cl` is safer than
free text such as `"cl"`, `"CL"`, or `"chlorine"`.

## Valence

Valence is the ability of an atom to form a certain number of chemical bonds with
other atoms.

Beginner analogy:

> Valence tells us how many "hands" an atom usually has for connecting to other
> atoms.

This is only an analogy, but it is useful for first lessons.

### Hydrogen

Hydrogen usually forms one bond:

```text
H -
```

So a beginner valence for hydrogen is `1`.

### Oxygen

Oxygen commonly forms two bonds:

```text
  |
O
  |
```

Water uses that pattern:

```text
H - O - H
```

The oxygen is connected to two hydrogen atoms, so a beginner valence for oxygen is
`2`.

### Carbon

Carbon commonly forms four bonds, which is one reason carbon can build long chains
and many different molecular shapes.

Methane is the simplest example:

```text
     H
     |
H - C - H
     |
     H
```

Carbon forms four single bonds here, so a beginner valence for carbon is `4`.

Common beginner valences:

| Element | Common beginner valence |
| --- | --- |
| Hydrogen (`H`) | 1 bond |
| Oxygen (`O`) | 2 bonds |
| Nitrogen (`N`) | 3 bonds |
| Carbon (`C`) | 4 bonds |
| Chlorine (`Cl`) | 1 bond |

### Same Valence, Different Elements

Hydrogen and chlorine both commonly form one bond, but they are not chemically
similar in every way. This is the point where valence shows its limits.

| Property | Hydrogen (`H`) | Chlorine (`Cl`) |
| --- | --- | --- |
| Common beginner valence | 1 | 1 |
| Protons in nucleus | 1 | 17 |
| Electrons in neutral atom | 1 | 17 |
| Electron shells | 1 | 3 |
| Approximate atomic mass | about 1 atomic mass unit | about 35.5 atomic mass units |

Why can both form one bond?

For hydrogen, the reason is smallness. Hydrogen has one proton and one electron.
The first electron shell can hold two electrons, so hydrogen commonly shares one
more electron through one chemical bond.

```text
nucleus (+)
    ●
 electron
    ○
```

For chlorine, the reason is different. Chlorine has 17 electrons arranged, in a
simple shell model, as:

```text
2, 8, 7
```

That means chlorine has seven electrons in its outer shell. The common beginner
rule says that this outer shell is close to a stable count of eight, so chlorine
often forms one bond to complete that outer-shell pattern.

Money analogy:

```text
hydrogen: I have 1 dollar and want 2.
chlorine: I have 999,999 dollars and want 1,000,000.
```

Both need "one more," but they are in very different starting states.

The real chemical differences are large:

- hydrogen is very small
- chlorine is larger
- chlorine has many more protons
- chlorine has more electron shells
- chlorine attracts shared electrons more strongly than hydrogen
- chlorine is much heavier

This matters in hydrogen chloride:

```text
H - Cl
```

Hydrogen and chlorine share one pair of electrons, but chlorine attracts that shared
electron density more strongly:

```text
H ----> Cl
```

So the bond is polar. Chlorine carries a small partial negative charge, written
`δ-`, and hydrogen carries a small partial positive charge, written `δ+`.

```text
H(δ+) - Cl(δ-)
```

That polarity is one reason hydrogen chloride behaves very differently from a
simple "one bond plus one bond" graph picture. For example, when hydrogen chloride
dissolves in water, it forms hydrochloric acid.

Key point:

> Valence answers: how many bonds does an atom commonly form?

Valence does not answer:

- how large the atom is
- how strongly it attracts electrons
- how heavy it is
- how many electron shells it has
- how reactive it is
- what substances it forms

Rust analogy:

> Two objects can satisfy the same interface constraint but behave differently
> because their internal state is different.

In this analogy, valence is like an interface limit: how many connections an atom
node commonly supports. Properties such as nuclear charge, mass, electron shells,
and electron attraction are internal state that changes behavior.

### Double And Triple Bonds

Valence counts bond order, not only the number of neighboring atoms.

Carbon dioxide is:

```text
O = C = O
```

Each double bond counts as two. Carbon has two double bonds:

```text
2 + 2 = 4
```

So carbon still has beginner valence `4` in carbon dioxide.

### Why Valence Exists

Valence is related to electrons in the outer electron shell. Atoms form bonds by
sharing or transferring electrons in ways that can make the overall electron
configuration more stable.

Beginner pattern:

- hydrogen often needs one shared electron
- oxygen often forms two bonds
- nitrogen often forms three bonds
- carbon often forms four bonds

This pattern is useful, but it is not the whole story.

### Valence As A Graph Rule

If a molecule is a graph:

- an atom is a node
- a bond is an edge

then valence is similar to a maximum allowed degree for that node, with bond order
included.

In methane:

```text
     H
     |
H - C - H
     |
     H
```

The carbon node has four single-bond edges. Each hydrogen node has one edge.

Programming analogy:

```rust
fn allowed_single_bonds(element: Element) -> usize {
    match element {
        Element::H => 1,
        Element::O => 2,
        Element::N => 3,
        Element::C => 4,
        Element::Cl => 1,
        Element::F | Element::Br => 1,
    }
}
```

This is only a beginner rule. Modern chemistry often uses more precise concepts
such as valence electrons, oxidation state, formal charge, coordination number, and
aromaticity. For more complex molecules, simple valence rules may be incomplete or
misleading.

### Do Quarks Have Valence?

No. Quarks do not have chemical valence.

Valence is a property of atoms because chemical bonds are formed by electrons
around atomic nuclei. Quarks are deep inside protons and neutrons, and they do not
directly form chemical bonds.

Different levels have different properties:

| Level | Useful properties |
| --- | --- |
| Quark | electric charge, mass, spin, flavor, color charge |
| Proton | total electric charge `+1` |
| Neutron | total electric charge `0` |
| Atom | element identity, valence, formal charge, chemical behavior |
| Molecule | bonds, shape, graph structure, reactivity |

Quarks have color charge, not chemical valence. Color charge is not real color; it
is the name physicists use for the charge involved in the strong interaction. A
proton can be sketched as three quarks with different color charges that combine
into a color-neutral state.

The useful hierarchy is:

```text
quark
  ↓
proton or neutron
  ↓
nucleus
  ↓
atom
  ↓
molecule
```

Valence appears at the atom level because atom-level electrons interact with
electrons from other atoms.

## Bond

A bond is a connection between atoms.

For this course, think of a bond as an edge in a graph.

Water has:

- one oxygen atom
- two hydrogen atoms
- two O-H bonds

In Rust:

```rust
pub struct Bond {
    pub from: usize,
    pub to: usize,
    pub order: BondOrder,
}
```

`from` and `to` are atom IDs. `order` says whether the bond is single, double,
triple, or aromatic in the toy model.

## Bond Types

The word bond means "chemical connection between atoms." In diagrams, chemists draw
bonds as lines:

```text
H - O - H
```

That line is a useful symbol. It is not a tiny physical stick between atoms.

In this course, the Rust model uses bond orders:

| Bond order | Drawing shortcut | Beginner meaning |
| --- | --- | --- |
| `Single` | `-` | one ordinary connection in the toy graph |
| `Double` | `=` | a stronger/more electron-rich connection than a single bond |
| `Triple` | `≡` | a still higher bond order |
| `Aromatic` | ring notation | a special delocalized bonding pattern in some rings |

For example, ethylene is often drawn as:

```text
H2C = CH2
```

The `=` means the two carbon atoms are connected by a double bond.

## What A Bond Means Physically

A chemical bond is not a material string. Atoms contain positively charged nuclei
and surrounding electron clouds. When atoms come close enough, their outer electron
clouds can overlap.

For a simple covalent bond, shared electron density between two nuclei helps hold
the atoms together:

```text
nucleus (+)    shared electron density    nucleus (+)
    ●  <------------- cloud ------------->  ●
```

If we could draw the electron density, we would draw a cloudy region rather than a
straight line:

```text
      .:::::::.
   .::::::::::::.
 ●::::::::::::::::●
   '::::::::::::'
      ':::::::'
```

Chemists still draw lines because they are compact and useful. A line means:

> These two atoms are chemically connected in this representation.

For programming, this is exactly what the beginner graph needs. The graph edge does
not model the full quantum electron distribution; it records that two atom records
are connected.

## Aromaticity

Aromaticity does not mean smell in this course.

Aromaticity is a special electronic state found in some cyclic molecules. Benzene
is the standard beginner example. It is often drawn as a six-membered ring:

```text
      C
   /     \
  C       C
  |       |
  C       C
   \     /
      C
```

In benzene, some electrons are not assigned to one single bond. They are
delocalized around the ring. Beginner translation:

- the electrons are spread over the ring
- the bonds are not best described as purely single or purely double
- the ring has a special stable bonding pattern

That is why chemistry software often stores aromaticity explicitly:

```rust
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub aromatic: bool,
}

pub enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}
```

Libraries such as RDKit, Open Babel, and CDK need aromaticity because algorithms
may use it for similarity search, substructure search, charge handling, and property
prediction. This course uses the same idea at toy scale: `aromatic: bool` and
`BondOrder::Aromatic` are fields that make an important chemical feature visible to
the data structure.

Important limit:

> Aromaticity detection is a real chemistry algorithm. In this course, we store an
> aromatic flag; we do not teach full aromaticity perception.

## Molecule

A molecule is a group of atoms connected by bonds.

Examples:

- water
- methane
- ethanol

In Rust:

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

This is already a data structure:

- `Vec<Atom>` stores the atom records.
- `Vec<Bond>` stores the connections.
- Together they describe a molecular graph.

## Chemical Formula

A chemical formula is a compact count of atoms by element.

It does not show every bond. It does not show 3D shape. It answers one important
question:

> How many atoms of each element are in this molecule?

## How To Read `H2O`

`H2O` means:

- `H2`: two hydrogen atoms
- `O`: one oxygen atom

If no number appears after an element symbol, the count is one.

So:

```text
H2O = 2 hydrogen atoms + 1 oxygen atom
```

Run:

```bash
cd exercises/rust-molecule-model
cargo run -- water summary
```

Expected output includes:

```text
formula: H2O
atoms: 3
bonds: 2
```

## How To Read `CH4`

`CH4` means:

- `C`: one carbon atom
- `H4`: four hydrogen atoms

So:

```text
CH4 = 1 carbon atom + 4 hydrogen atoms
```

Run:

```bash
cargo run -- methane summary
```

Expected output includes:

```text
formula: CH4
atoms: 5
bonds: 4
```

## How To Read `C2H6O`

`C2H6O` means:

- `C2`: two carbon atoms
- `H6`: six hydrogen atoms
- `O`: one oxygen atom

So:

```text
C2H6O = 2 carbon atoms + 6 hydrogen atoms + 1 oxygen atom
```

Run:

```bash
cargo run -- ethanol summary
```

Expected output includes:

```text
formula: C2H6O
atoms: 9
bonds: 8
```

## Why Formulas Are Data Representations

A formula is a compressed representation.

| Formula | What it preserves | What it leaves out |
| --- | --- | --- |
| `H2O` | element counts | exact bond list, 3D shape, electrons |
| `CH4` | element counts | geometry and bond angles |
| `C2H6O` | element counts | which atoms connect to which |

This is the first major modeling lesson:

> A representation keeps some information and leaves other information out.

That is also true in programming.

## Formula Versus Graph

The formula `C2H6O` tells us ethanol has:

- two carbon atoms
- six hydrogen atoms
- one oxygen atom

But it does not tell us how they are connected.

The graph representation stores connectivity:

```rust
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

That is why we need both ideas:

- formula for counts
- graph for connections

## Checkpoint

1. What is an atom?
2. What is an element?
3. What is a bond?
4. What is a molecule?
5. What does the `2` mean in `H2O`?
6. What does `CH4` mean?
7. What information does `C2H6O` leave out?
