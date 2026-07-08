# Lesson 14: Serialization And Reproducible Lab Records

## Goal

Students learn how structured molecule summaries become stable text records, how
parsers validate those records, and why reproducibility needs schema and
provenance.

## Big Question

How can a molecule model be written down so another program or person can read it
back correctly?

## Visuals

- [Serialization and lab records](../visuals/mermaid/serialization-lab-records.md)
- [Screening feedback loop](../visuals/mermaid/screening-feedback-loop.md)

## Core Analogy

| Lab-record idea | Data-structure idea |
| --- | --- |
| field | struct member |
| schema | contract |
| row | record |
| file line | serialized value |
| parser | validator plus converter |
| parse error | explicit failure |
| round trip | reproducibility check |

## Timing

Recommended length: 75-90 minutes.

| Time | Segment | Activity |
| --- | --- | --- |
| 0-10 min | Hook | Show a molecule summary and ask what must be saved. |
| 10-25 min | Schema | Decide fields for a toy lab record. |
| 25-40 min | Serialize | Turn a record card into a pipe-separated line. |
| 40-55 min | Parse | Split a line and validate each field. |
| 55-70 min | Errors | Diagnose bad records. |
| 70-90 min | Reproduce | Run round-trip tests and make a Markdown report row. |

## Part 1: Choose Fields

Start with:

```text
water: formula H2O, 3 atoms, 2 bonds
```

Ask students what should be saved:

- ID
- name
- formula
- atom count
- bond count
- notes

Teaching line:

A schema is a promise about what each field means.

## Part 2: Serialize

Record:

```text
rec-001, water, H2O, 3, 2, starter example
```

Serialized line:

```text
rec-001|water|H2O|3|2|starter example
```

Data-structure idea:

The struct is good for computation. The text line is good for storage and
exchange.

## Part 3: Parse And Validate

Bad inputs:

```text
rec-001|water|H2O
rec-001||H2O|3|2|missing name
rec-001|water|H2O|three|2|bad number
```

Students identify the error before reading the Rust code.

## Part 4: Round Trip

Flow:

```text
LabRecord -> text line -> LabRecord
```

Question:

> Did we get the same record back?

## University Extension

Inspect:

```text
exercises/rust-molecule-model/src/lab_record.rs
```

Discussion prompts:

- Why is a stable header useful?
- Why does the toy writer reject the separator inside field values?
- Which parse errors should include the field name?
- When would a real project use CSV, JSON, SDF, or a database instead?
- What provenance fields are missing from the toy record?

## Caution

This is a teaching format. Real chemistry records need stronger schemas,
metadata, units, provenance, audit trails, and domain-specific validation.
