# School Track

## Tone

Make it feel like a design studio, not a lecture. Students are not memorizing Rust
syntax first; they are learning to ask: "What is this thing, what can it connect to,
and what rules keep it from becoming nonsense?"

## The Story

You are a molecule architect. Your building blocks are atoms. Your connectors are
bonds. Your blueprint is data. Rust is the strict but helpful lab notebook that
refuses to let you write "chlorene" when you meant chlorine.

## 60-Minute Version

| Time | Activity | What students do |
| --- | --- | --- |
| 0-5 | Mystery object | See a molecule drawing and guess what information is hidden in it. |
| 5-15 | Atom cards | Hold cards for H, C, N, O, Cl and list each atom's properties. |
| 15-25 | Type spell | Turn atom facts into `u8`, `i8`, `bool`, and enum variants. |
| 25-35 | Bond string graph | Stand up as atoms and connect with string as bonds. |
| 35-45 | Human memory | Act out stack, heap, owner, and borrower roles. |
| 45-55 | Tiny Rust | Read or edit the molecule model and run `cargo run`. |
| 55-60 | Exit ticket | One good analogy, one place the analogy breaks. |

## 90-Minute Version

Add:

- graph traversal race: pass a token from one atom to another through bonds
- invalid molecule challenge: students invent bad data, then design a rule to catch it
- quantum wave cards: students hold hidden cards and reveal one measured outcome

## Props

- index cards with element symbols
- string or yarn for bonds
- sticky notes for charges and aromatic flags
- paper "memory addresses"
- two baskets labeled stack and heap
- colored cards labeled owner, immutable borrower, mutable borrower

## Code Moment

Show this small example and ask students what could go wrong if `Element` were a
plain string:

```rust
enum Element {
    H,
    C,
    N,
    O,
    Cl,
}
```

Expected student answer:

Strings can be misspelled. Enums give the program a fixed vocabulary.

## Teacher Moves

- Ask "What rule did Rust protect?" more often than "What syntax did you type?"
- Let students physically act out ownership before explaining the borrow checker.
- Treat wrong analogies as useful: "Where does this comparison stop working?"
- Keep quantum state humble. Say: "This is not magic memory; it is a different kind
  of physical state."

## Final Challenge

Build water, methane, or ethanol in Rust. Then answer:

1. Who owns the atoms?
2. How do we find an atom's neighbors?
3. What invalid bond should the program reject?

## Visuals to Use

- [Course map](../visuals/mermaid/course-map.md)
- [Graph algorithms](../visuals/mermaid/graph-algorithms.md)
- [Molecule design flow](../visuals/plantuml/molecule-design-flow.puml)
- [Traits and functions](../visuals/mermaid/traits-and-functions.md)

For school students, show the diagram first, then act it out, then show code.

## Extra Lesson Pair

Use Lessons 05 and 06 as a studio day:

1. Students choose containers for atom cards, bond strings, and formula counts.
2. Students act out traits as role cards: `Describe`, `ChemicalFormula`, and
   `MolecularGraph`.
3. Students run the Rust crate and find the matching code.
