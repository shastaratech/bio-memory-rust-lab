# Worksheet: Formula To Graph

## Goal

Turn a chemical formula into atom counts, then turn those atoms into a graph.

## Part 1: Read The Formula

Fill in the atom counts.

| Formula | Carbon | Hydrogen | Oxygen | Total atoms |
| --- | --- | --- | --- | --- |
| `H2O` | | | | |
| `CH4` | | | | |
| `C2H6O` | | | | |

Remember:

If an element symbol has no number after it, the count is one.

## Part 2: Formula Is Not Enough

`C2H6O` can describe ethanol:

```text
C - C - O
```

It can also describe dimethyl ether:

```text
C - O - C
```

Answer:

```text
What is the same?


What is different?


```

## Part 3: Draw Water As A Graph

Use three nodes:

- one oxygen
- two hydrogens

Draw the graph:

```text



```

Fill in the bond table:

| Bond ID | From atom | To atom |
| --- | --- | --- |
| `0` | | |
| `1` | | |

## Part 4: Check With Rust

Run:

```bash
cd exercises/rust-molecule-model
cargo run -- water atoms
cargo run -- water bonds
cargo run -- water neighbors 0
```

Write the neighbor output:

```text

```

## Part 5: Draw Ethanol From Output

Run:

```bash
cargo run -- ethanol atoms
cargo run -- ethanol bonds
```

Choose any three connected atoms from ethanol and draw that small graph:

```text



```

## Reflection

Answer in one sentence:

Why does the Rust model need `Vec<Bond>` instead of only a formula string?
