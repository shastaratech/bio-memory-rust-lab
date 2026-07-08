# Review Notes: Intro And Lesson 01

## Executive Summary

The intro path is usable for beginners: `README.md` points to setup, course map,
Lesson 01, handout, exercises, visuals, project, worksheets, and production
prompts. Lesson 01 has a strong concept map across chemistry, Rust types, memory,
graphs, and quantum-state limits.

The most important teaching requirement is to keep the analogy bounded. The current
materials mostly do this well. Instructors should repeatedly say that atom cards
and graph records are a toy representation, not complete chemistry.

## Dependency Check

| Priority | Area | Status | Note |
| --- | --- | --- | --- |
| P0 | Intro file path | Clear | `README.md` points to setup, Lesson 01, handout, exercises, visuals, and worksheets. |
| P0 | Lesson 01 code names | Clear | Lesson references match `Element`, `BondOrder`, `Atom`, `Bond`, and `Molecule`. |
| P0 | First commands | Clear | `cargo test`, `cargo run -- water summary`, `atoms`, and `bonds` are supported. |
| P1 | Prerequisite flow | Acceptable | Beginners should read the chemistry primer and formula-to-graph bridge before Lesson 01. |
| P1 | Visual dependency | Acceptable | Some visuals are source diagrams; instructors may need rendering tools or screenshots. |
| P2 | Media workflow | Improved | Added tutor script and video package in `instructor-materials/`. |

## Clarity Notes

| Priority | Issue | Recommendation |
| --- | --- | --- |
| P1 | Stack/heap explanation can become too concrete. | Use "beginner sketch" language; avoid promising exact machine layout. |
| P1 | Bond/reference analogy can mislead. | Say a bond is graph-like, not a Rust reference. |
| P1 | Chemistry model is intentionally incomplete. | Repeat that the toy model omits 3D geometry, electron density, environment, and many validity rules. |
| P2 | Enum vocabulary is limited. | Say the enum is the supported subset, not the periodic table. |
| P2 | Quantum comparison should stay brief. | Use it as a boundary-setting concept, not a full quantum lesson. |

## Suggested Instructor Framing

Use this opening frame:

```text
We are not reducing chemistry to code. We are using a small code model to learn how
structure, constraints, memory, and algorithms work.
```

Use this analogy boundary:

```text
An atom card is a record. A real atom is a physical system. The record is useful
because it lets us ask beginner-sized programming questions.
```

## Verification Commands

Run before recording:

```bash
cd exercises/rust-molecule-model
cargo fmt --check
cargo test
cargo run -- water summary
cargo run -- water atoms
cargo run -- water bonds
cargo run -- water neighbors 0
```

