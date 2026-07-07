# Handout: Molecular Libraries As Search Indexes

## One Molecule, Many Views

A molecule record can be placed into several data structures.

| View | Rust shape | Best question |
| --- | --- | --- |
| Shelf | `Vec<MoleculeRecord>` | What records exist? |
| Name lookup | `HashMap<String, MoleculeRecord>` | Find ethanol. |
| Formula buckets | `HashMap<String, Vec<String>>` | Which records have `H2O`? |
| Size order | sorted `Vec<MoleculeRecord>` | Show smallest to largest. |
| Search boundary | binary search over sorted records | Where does atom count >= 5 begin? |

## Record Shape

```rust
struct MoleculeRecord {
    id: String,
    common_name: String,
    molecule: Molecule,
}
```

The `Molecule` is the graph of atoms and bonds. The record adds labels that make
the molecule searchable.

## Main Rule

Choose the data structure from the question:

- Need all records in order? Use a vector.
- Need direct lookup? Build a hash map.
- Need one key to many records? Use a hash map of vectors.
- Need range or ranking? Sort first.
- Need to combine sorted sources? Merge.
- Need fast boundary search? Binary search a sorted list.

## DNA And Chemistry Connection

DNA is biological information encoded in a chemical structure. A molecule library
is computational information encoded in Rust values.

Useful analogy:

```text
sequence -> ordered data
gene slice -> borrowed range
molecule record -> struct
compound library -> vector
formula lookup -> hash map
ranked candidates -> sorted list
```

Limit:

DNA is active biological chemistry. A Rust molecule record is a model used for
computation and teaching.
