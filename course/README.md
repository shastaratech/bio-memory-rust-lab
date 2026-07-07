# Course Map

Bio Memory Rust Lab has two lanes that share the same core metaphor:

- School lane: playful, physical, visual, and short-code-first.
- University lane: formal, implementation-heavy, and connected to algorithms and
  scientific computing.

Both lanes teach the same durable idea:

> A good model keeps enough structure to answer the question, and enough rules to
> prevent nonsense.

## Suggested Four-Session Arc

| Session | Big question | Core Rust idea | Core science idea |
| --- | --- | --- | --- |
| 01 | What is a molecule made of, as data? | enums, structs, vectors | atoms, bonds, charge |
| 02 | How does a molecule become a network? | graph traversal | neighbors, paths, fragments |
| 03 | Who is allowed to change the molecule? | ownership and borrowing | design pipelines and constraints |
| 04 | Why is a molecule not just a graph? | limits of representation | quantum state and simulation |

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

## Capstone Ideas

- Build a command-line molecule explorer.
- Compare `Vec<Bond>` scanning with an adjacency list.
- Write a simple analog generator for a toy scaffold.
- Add a scoring function for "drug-like toy molecules."
- Make a poster explaining "atoms are not bytes, but both need structure."

## Visual Support

Use [visuals/](../visuals/README.md) throughout the course:

- start each session with the [course map](../visuals/mermaid/course-map.md)
- use the [molecule data model](../visuals/plantuml/molecule-data-model.puml)
  when introducing Rust structs
- use the [ownership contract](../visuals/plantuml/ownership-borrowing-contract.puml)
  before borrow-checker examples
- use the [graph algorithms](../visuals/mermaid/graph-algorithms.md) for Lesson 02
- use [models, contracts, and flows](../visuals/mermaid/model-contracts-flows.md)
  when diagrams need to render directly on GitHub
