# Appendix: Command Reference

Run all commands from:

```bash
cd exercises/rust-molecule-model
```

## Build And Test

```bash
cargo fmt
cargo test
```

## Help

```bash
cargo run -- help
```

## List Molecules

```bash
cargo run -- list
```

## Summaries

```bash
cargo run -- water summary
cargo run -- methane summary
cargo run -- ethanol summary
```

## Formulas

```bash
cargo run -- water formula
cargo run -- methane formula
cargo run -- ethanol formula
```

## Atoms And Bonds

```bash
cargo run -- water atoms
cargo run -- water bonds
cargo run -- ethanol atoms
cargo run -- ethanol bonds
```

## Graph Queries

```bash
cargo run -- water neighbors 0
cargo run -- methane path 1 4
cargo run -- ethanol neighbors 1
cargo run -- ethanol path 3 8
cargo run -- ethanol components
```

## Validation

```bash
cargo run -- water validate
cargo run -- methane validate
cargo run -- ethanol validate
```

## Common Errors

Unknown molecule:

```bash
cargo run -- glucose summary
```

Expected behavior:

```text
unknown molecule: glucose
available molecules: water, methane, ethanol
```

Missing atom ID:

```bash
cargo run -- water neighbors
```

Expected behavior:

```text
missing atom id
```

