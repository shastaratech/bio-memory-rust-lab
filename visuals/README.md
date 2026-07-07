# Visual Language

This folder keeps diagram source files and visual teaching patterns for Bio Memory
Rust Lab.

Use two diagram styles:

- Mermaid for diagrams that should render directly on GitHub.
- PlantUML for richer UML-style structures, contracts, state machines, and activity
  flows.

PlantUML's official site describes it as an open-source tool that draws UML and
other diagrams from simple textual descriptions. It supports class, sequence,
activity, component, state, object, mind map, JSON, YAML, and other diagram types:
https://plantuml.com/

## Why Text Diagrams

Text diagrams are good teaching material because students can:

- read the diagram and source side by side
- edit a diagram like code
- review diagrams in pull requests
- connect visual structure to Rust structure
- keep diagrams versioned with the lesson

## Diagram Guide

| What to teach | Best format | Example |
| --- | --- | --- |
| Course path | Mermaid flowchart | `mermaid/course-map.md` |
| Course path fallback | SVG | `rendered/course-map.svg` |
| Data model quick view | Mermaid class/state/flowchart | `mermaid/model-contracts-flows.md` |
| Data algorithms | Mermaid flowchart | `mermaid/data-algorithms.md` |
| Molecule design algorithms | Mermaid flowchart | `mermaid/molecule-design-algorithms.md` |
| Traits and functions | Mermaid class/flowchart | `mermaid/traits-and-functions.md` |
| CLI flow | Mermaid flowchart | `mermaid/cli-flow.md` |
| 3D molecule teaching view | HTML / Three.js | `html/molecule-3d-viewer.html` |
| Rust data model | PlantUML class diagram | `plantuml/molecule-data-model.puml` |
| Rust trait contracts | PlantUML class diagram | `plantuml/traits-and-functions.puml` |
| Borrowing contract | PlantUML state diagram | `plantuml/ownership-borrowing-contract.puml` |
| Molecule design pipeline | PlantUML activity diagram | `plantuml/molecule-design-flow.puml` |
| BFS / shortest path | Mermaid flowchart | `mermaid/graph-algorithms.md` |
| Quantum-classical loop | PlantUML activity diagram | `plantuml/quantum-simulation-loop.puml` |

## Student Rule

Every diagram must answer a question.

Weak:

> Here is a diagram of the system.

Strong:

> Which object owns the atoms? Which functions only borrow? Which steps reject
> invalid molecule data?

## Rendering PlantUML

Options:

1. Use a local PlantUML plugin in an editor.
2. Use the PlantUML online server for classroom preview.
3. Use PlantUML's browser extension for GitHub rendering.
4. Add generated SVGs later if the course needs printable worksheets.

For now, the `.puml` files are the source of truth. Mermaid diagrams render directly
inside GitHub Markdown.

Some browsers, extensions, and GitHub contexts may show Mermaid as a code block
instead of rendering it. When a diagram is important for teaching, keep a rendered
fallback in `visuals/rendered/` and embed it above the editable source.

## Mermaid vs PlantUML

Use Mermaid when the diagram must be visible immediately on GitHub. Use PlantUML
when the lesson needs formal UML syntax, richer state/activity diagrams, or students
are practicing text-to-diagram modeling as a design skill.

Use the standalone HTML viewer when the lesson needs interactive spatial intuition.
The viewer is a teaching model with simplified coordinates, not a chemistry-grade
geometry engine.
