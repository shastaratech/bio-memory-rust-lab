# Setup

## Goal

By the end of setup, you can compile, test, and run the Rust molecule model.

## What You Need

- Git
- Rust and Cargo
- A terminal
- A text editor

Check Rust:

```bash
rustc --version
cargo --version
```

If those commands do not work, install Rust from:

https://www.rust-lang.org/tools/install

## Get The Repo

If you do not have the repo yet:

```bash
git clone https://github.com/shastaratech/bio-memory-rust-lab.git
cd bio-memory-rust-lab
```

If you already have it:

```bash
cd bio-memory-rust-lab
git pull
```

## Compile And Test

Go to the Rust exercise:

```bash
cd exercises/rust-molecule-model
```

Compile and run tests:

```bash
cargo test
```

You should see tests pass. The exact number may grow as the course improves, but the
important line is:

```text
test result: ok
```

## Run The CLI

List available toy molecules:

```bash
cargo run -- list
```

Expected output:

```text
water, methane, ethanol
```

Run a summary:

```bash
cargo run -- water summary
```

Expected output includes:

```text
water: H2O molecule with 3 atoms and 2 bonds
formula: H2O
valid bond indices: true
```

## Files To Know

From the repo root:

| File | Why it matters |
| --- | --- |
| `exercises/rust-molecule-model/src/lib.rs` | Exports the reusable Rust library. |
| `exercises/rust-molecule-model/src/main.rs` | Command-line interface. |
| `exercises/rust-molecule-model/src/molecule.rs` | Main data model and algorithms. |
| `visuals/mermaid/course-map.md` | Course path diagram. |
| `visuals/rendered/course-map.svg` | Rendered course map fallback. |

## Checkpoint

Answer before continuing:

1. What command runs tests?
2. What command lists toy molecules?
3. Which file contains `struct Molecule`?

