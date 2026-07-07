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

