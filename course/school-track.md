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
| 45-55 | Tiny Rust | Read or edit the molecule model and run `cargo run -- water summary`. |
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

Printable worksheets:

- [Atom Cards](../worksheets/school-atom-cards.md)
- [Formula To Graph](../worksheets/school-formula-to-graph.md)
- [DNA, Memory, and Data Structures](../worksheets/dna-memory-data-structures.md)
- [Visualizing Molecules and Data Algorithms](../worksheets/visualizing-data-algorithms.md)
- [Molecule Design As Data Search](../worksheets/molecule-design-data-search.md)
- [Molecular Library Indexes](../worksheets/molecular-library-indexes.md)
- [Toy Fingerprints And Similarity Search](../worksheets/toy-fingerprints-similarity.md)
- [Screening Results And Feedback Loops](../worksheets/screening-feedback-loops.md)

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

## First Project

Use the [Molecule Explorer CLI](../projects/molecule-explorer-cli.md) when students
are ready to run commands. Let them predict the output before pressing Enter.

Use [Lesson 07](../lessons/07-molecule-explorer-cli-capstone.md) and the
[CLI handout](../handouts/molecule-explorer-cli.md) as the project day structure.
Keep the first extension to `degree` unless the group is ready to add a new
molecule.

## Biological Memory Extension

Use [Lesson 08](../lessons/08-dna-biological-memory-data-structures.md) when the
class is ready to compare molecule graphs with sequence data. Have students build
base cards for `A`, `C`, `G`, and `T`, group codons into three-card chunks, and
mark a borrowed gene region with start/end sticky notes.

## Visualization And Algorithm Extension

Use [Lesson 09](../lessons/09-visualizing-molecules-and-data-algorithms.md) when
students are ready to compare data structures. Put atom cards in rows for vectors,
arrow-linked cards for lists, alphabetized cards for sorted lists, and element
buckets for hash maps. Then open the [3D viewer](../visuals/html/molecule-3d-viewer.html)
to compare data records with shape.

## Molecule Design Extension

Use [Lesson 10](../lessons/10-designing-molecules-as-data-search.md) when students
are ready to design candidates. Run scaffold, option, validation, scoring, and
ranking stations with cards. Keep the scores playful and make the safety limit
explicit: this is data-structure practice, not real molecule recommendation.

## Molecular Library Index Extension

Use [Lesson 11](../lessons/11-molecular-libraries-and-search-indexes.md) when
students are ready to search a library of molecule cards. Put cards on a shelf for
the vector view, make name labels for hash-map lookup, make formula buckets for
grouped lookup, sort by atom count, and merge two sorted class lists.

## Toy Fingerprint Extension

Use [Lesson 12](../lessons/12-toy-fingerprints-and-similarity-search.md) when
students are ready to compare molecules by feature cards. Turn observations into
yes/no bits, count shared features, count union features, and sort the closest
matches. Keep the caution explicit: a toy fingerprint is a summary, not the
complete molecule.

## Screening Feedback Extension

Use [Lesson 13](../lessons/13-screening-results-and-feedback-loops.md) when
students are ready to compare predictions with measured labels. Let students sort
score cards into test/defer piles, add active/inactive labels, fill a confusion
matrix, and choose which uncertain cards should guide the next round.
