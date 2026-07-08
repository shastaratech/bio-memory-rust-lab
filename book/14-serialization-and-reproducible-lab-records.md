# Chapter 14: Serialization And Reproducible Lab Records

## Big Idea

A molecule model becomes useful when it can be written down, read back, checked,
and shared.

Serialization means converting structured data into a stable text or file form.
Deserialization means reading that text back into structured data.

In this course, we use a simple toy lab-record format:

```text
record_id|molecule_name|formula|atom_count|bond_count|notes
```

The goal is not to replace real chemistry file formats. The goal is to teach why
schemas, validation, parsing errors, and reproducible records matter.

## New Vocabulary

| Word | Meaning in this chapter |
| --- | --- |
| Serialization | Turning structured data into text or bytes. |
| Deserialization | Turning text or bytes back into structured data. |
| Schema | A rule for which fields exist and what they mean. |
| Record | One row of structured information. |
| Parser | Code that reads text and checks whether it follows the schema. |
| Round trip | Serialize data, parse it back, and get the same data. |
| Validation | Checking that a record is complete and meaningful. |
| Provenance | Information about where a record came from and why it exists. |

## Why Records Matter

Molecules can be represented in many ways:

- formula
- atom list
- bond list
- graph
- fingerprint
- prediction score
- experimental label
- lab note

A reproducible lab record says:

> Here is exactly what was recorded, using a known schema, so another person or
> program can read it later.

## Rust Model

Open:

```text
exercises/rust-molecule-model/src/lab_record.rs
```

Core type:

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

This is a teaching record, not a full scientific format.

It stores summary fields that students can verify:

| Field | Meaning |
| --- | --- |
| `record_id` | stable label for the row |
| `molecule_name` | human-readable name |
| `formula` | computed formula |
| `atom_count` | graph summary |
| `bond_count` | graph summary |
| `notes` | short provenance or teaching note |

## Serialization

The record line uses `|` as a separator:

```text
rec-002|ethanol|C2H6O|9|8|class demo
```

This is similar to CSV/TSV thinking:

- fields appear in a fixed order
- a separator splits fields
- the parser checks the number and type of fields

The code intentionally rejects values containing `|`.

That keeps the toy format simple and makes the limitation visible.

## Deserialization

The parser checks:

1. Are there exactly six fields?
2. Are required string fields non-empty?
3. Do `atom_count` and `bond_count` parse as numbers?
4. Does the record produce a structured `LabRecord`?

When parsing fails, it returns a specific error:

```rust
enum LabRecordParseError {
    WrongFieldCount,
    EmptyField,
    InvalidNumber,
    UnsupportedSeparator,
}
```

This is better than crashing or silently accepting bad data.

## Round Trip

A round trip means:

```text
LabRecord -> text line -> LabRecord
```

If the record after parsing equals the original record, the basic format is
working.

Round trips are a good beginner test for serialization code.

## Relation To Chemistry Formats

Real cheminformatics uses richer formats such as SMILES, SDF, MOL2, PDB, and
domain-specific metadata tables.

This course avoids raw scientific datasets and production chemistry files. The toy
format is deliberately small so students can inspect every field and every parser
decision.

| Real-world concern | Toy lesson idea |
| --- | --- |
| file format | stable text line |
| schema | fixed field order |
| parser | `from_record_line` |
| writer | `to_record_line` |
| data validation | parse errors |
| provenance | notes field |
| report output | Markdown row |

## DNA And Memory Connection

DNA stores information physically. A lab record stores information
computationally.

| Biology / chemistry | Computer structure |
| --- | --- |
| DNA sequence | ordered biological information |
| molecule graph | structured model |
| formula/counts | computed summary |
| serialized record | text representation |
| parser | rules for reading text |
| lab notebook | provenance and context |

The analogy is limited. DNA is not a file format. But both force us to ask:

- What is stored?
- What is copied?
- What can be read back?
- What context is missing?

## Run The Exercise

```bash
cd exercises/rust-molecule-model
cargo test lab_record
```

Look for tests that:

- create records from molecules
- serialize records to text
- parse records back
- reject bad field counts
- reject missing required fields
- reject invalid number fields
- render Markdown report rows

## Checkpoint

1. What is serialization?
2. What is deserialization?
3. Why does a schema matter?
4. What does a round-trip test prove?
5. What does a round-trip test not prove?
6. Why should parse errors be specific?
7. Why is provenance part of reproducibility?
