# Worksheet: Serialization And Reproducible Lab Records

## Purpose

Practice turning structured molecule summaries into stable text records and
checking parser errors.

## Part 1: Record Fields

Fill in the record for water.

| Field | Value |
| --- | --- |
| record_id | rec-001 |
| molecule_name | water |
| formula | |
| atom_count | |
| bond_count | |
| notes | starter example |

## Part 2: Serialize

Use this schema:

```text
record_id|molecule_name|formula|atom_count|bond_count|notes
```

Write the serialized line:

```text

```

## Part 3: Parse

Split this line into fields:

```text
rec-002|ethanol|C2H6O|9|8|class demo
```

| Field | Value |
| --- | --- |
| record_id | |
| molecule_name | |
| formula | |
| atom_count | |
| bond_count | |
| notes | |

## Part 4: Diagnose Errors

For each line, name the error.

| Line | Error |
| --- | --- |
| `rec-001|water|H2O` | |
| `rec-001||H2O|3|2|missing name` | |
| `rec-001|water|H2O|three|2|bad number` | |
| `rec-001|water|H2O|3|2|bad | separator` | |

## Part 5: Round Trip

Explain this flow:

```text
LabRecord -> text line -> LabRecord
```

What should be true at the end?

```text

```

## Part 6: Code Check

Run:

```bash
cd exercises/rust-molecule-model
cargo test lab_record
```

Write one test name that passed.

```text

```

## Reflection

What provenance field would you add to make the record more useful?

```text

```
