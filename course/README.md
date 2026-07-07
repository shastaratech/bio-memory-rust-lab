# Course Map

Bio Memory Rust Lab has two lanes that share the same core metaphor:

- School lane: playful, physical, visual, and short-code-first.
- University lane: formal, implementation-heavy, and connected to algorithms and
  scientific computing.

Both lanes teach the same durable idea:

> A good model keeps enough structure to answer the question, and enough rules to
> prevent nonsense.

For self-study, use the [book](../book/README.md). It gives students step-by-step
reading, commands, expected outputs, and checkpoints.

For printable labs, use the [worksheet pack](../worksheets/README.md).

## Suggested Eleven-Session Arc

| Session | Big question | Core Rust idea | Core science idea |
| --- | --- | --- | --- |
| 01 | What is a molecule made of, as data? | enums, structs, vectors | atoms, bonds, charge |
| 02 | How does a molecule become a network? | graph traversal | neighbors, paths, fragments |
| 03 | Who is allowed to change the molecule? | ownership and borrowing | design pipelines and constraints |
| 04 | Why is a molecule not just a graph? | limits of representation | quantum state and simulation |
| 05 | Which data structure fits each molecule question? | representation choice | molecular records, graphs, counts |
| 06 | What can molecule-like things do? | traits and functions | shared behavior and contracts |
| 07 | How does a model become a tool? | CLI commands and tests | inspectable toy molecule queries |
| 08 | How can a molecule carry information? | sequence data, references, heap storage | DNA bases, codons, genes, mutations |
| 09 | Which data shape fits this question? | arrays, lists, maps, sorting, merging | 3D views and molecular records |
| 10 | How do we explore possible molecules? | search, filters, ranking, queues | scaffolds, substituents, constraints, scores |
| 11 | How do we search a molecule library? | hash indexes, sorted lists, binary search | compound records, formula buckets, library queries |

## School Lane

Use [school-track.md](school-track.md) when students are new to programming,
chemistry, or both. Keep code short and alternate every 10-15 minutes between
conversation, movement, drawing, and typing.

Best outcomes:

- Students can explain types as "rules for values."
- Students can draw a molecule as a graph.
- Students can read a small Rust struct and say what it represents.
- Students understand that quantum state is not ordinary computer memory.

## University Lane

Use [university-track.md](university-track.md) when students can already program
or are ready to learn implementation tradeoffs.

Best outcomes:

- Students can justify a representation choice.
- Students can implement graph operations on molecules.
- Students can explain why Rust references are not the same thing as chemical bonds.
- Students can connect molecule design to constrained search and scoring.
- Students can describe why quantum simulation is useful and hard.
- Students can compare DNA sequence representations with Rust data structures.
- Students can choose arrays, lists, sorted lists, hash maps, graphs, and 3D
  coordinates for different molecule questions.
- Students can describe molecule design as a constrained search over candidate
  data.
- Students can build molecule library indexes for direct lookup, grouped formula
  lookup, sorted range search, and merge.

## Capstone Ideas

- Build the [Molecule Explorer CLI](../projects/molecule-explorer-cli.md).
- Complete the [CLI extension worksheet](../worksheets/university-cli-extension.md).
- Compare `Vec<Bond>` scanning with an adjacency list.
- Write a simple analog generator for a toy scaffold.
- Add a scoring function for "drug-like toy molecules."
- Make a poster explaining "atoms are not bytes, but both need structure."
- Model DNA bases, codons, gene slices, and mutations in Rust.
- Build a visual data-structure report for one molecule.
- Run a toy scaffold/substituent design round and rank candidates.
- Build a toy molecule library index and explain which queries each structure
  makes cheap.

## Visual Support

Use [visuals/](../visuals/README.md) throughout the course:

- start each session with the [course map](../visuals/mermaid/course-map.md)
- use the [molecule data model](../visuals/plantuml/molecule-data-model.puml)
  when introducing Rust structs
- use the [ownership contract](../visuals/plantuml/ownership-borrowing-contract.puml)
  before borrow-checker examples
- use the [graph algorithms](../visuals/mermaid/graph-algorithms.md) for Lesson 02
- use the [data algorithms](../visuals/mermaid/data-algorithms.md) for Lesson 09
- use the [molecule design algorithms](../visuals/mermaid/molecule-design-algorithms.md)
  for Lesson 10
- use the [molecular library indexes](../visuals/mermaid/molecular-library-indexes.md)
  for Lesson 11
- use [models, contracts, and flows](../visuals/mermaid/model-contracts-flows.md)
  when diagrams need to render directly on GitHub
- use [traits and functions](../visuals/mermaid/traits-and-functions.md)
  when introducing reusable behavior
- open the [3D molecule viewer](../visuals/html/molecule-3d-viewer.html) when
  comparing graph connectivity with spatial views
