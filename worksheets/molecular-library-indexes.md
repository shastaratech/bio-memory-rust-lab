# Worksheet: Molecular Library Indexes

## Purpose

Practice choosing data structures for molecule library questions.

## Part 1: Make Records

Fill in the table.

| ID | Common name | Formula | Atom count |
| --- | --- | --- | --- |
| mol-001 | water | H2O | 3 |
| mol-002 | methane | CH4 | 5 |
| mol-003 | ethanol | C2H6O | 9 |

Question:

Which column should be stable even if the common name changes?

Answer:

```text

```

## Part 2: Vector View

Write the records in teaching order.

```text
[                    ]
```

What question does this view answer well?

```text

```

## Part 3: Name Hash Map

Fill the lookup table.

| Key | Value |
| --- | --- |
| water | |
| methane | |
| ethanol | |

Why is this better than scanning every record when the library is large?

```text

```

## Part 4: Formula Buckets

Add a second water record:

```text
mol-004, water-copy, H2O, 3 atoms
```

Fill the grouped index.

| Formula key | Names |
| --- | --- |
| H2O | |
| CH4 | |
| C2H6O | |

Why is the value a list instead of one string?

```text

```

## Part 5: Sorted List

Sort by atom count.

```text
smallest ->                                       -> largest
```

Where does `atom_count >= 5` begin?

```text

```

## Part 6: Merge

Merge these two sorted lists.

```text
left:  water(3), ethanol(9)
right: methane(5)
```

Output:

```text

```

Rule used:

```text

```

## Part 7: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test index
```

Write one test name that passed.

```text

```

## Reflection

DNA, molecule libraries, and Rust structures all store information, but they do
not store it for the same reason.

Write one useful similarity and one important difference.

Similarity:

```text

```

Difference:

```text

```
