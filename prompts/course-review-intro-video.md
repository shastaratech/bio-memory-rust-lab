# Prompt: Course Review, Tutor Script, And Intro Video Materials

Copy this prompt into ChatGPT when you want a careful review of the course
materials and a first-pass set of teaching scripts.

```text
You are reviewing the Bio Memory Rust Lab course materials.

Goal:
Improve the course for newcomers by finding broken dependencies, unclear
statements, missing prerequisites, inconsistent analogies, and places where the
chemistry-to-computer-science mapping needs tighter explanation. Then create
tutor-ready materials for the course introduction and Lesson 01.

Audience:
- New Rust learners.
- Students new to molecular structure and chemical notation.
- Instructors who need reliable lesson flow, scripts, and short video segments.

Repository context:
The course teaches Rust and computer science through molecular structure. It uses
analogies between atoms/molecules and data structures, including typed values,
structs, vectors, graphs, stack memory, heap memory, references, borrowing,
hash maps, sorted lists, DNA as biological memory, and lab data workflows.

Primary files to review:
- README.md
- book/README.md
- book/00-setup.md
- book/00-chemistry-primer.md
- book/00b-how-formulas-become-graphs.md
- book/01-atoms-memory-types.md
- lessons/01-atoms-memory-types.md
- handouts/atoms-memory-rust.md
- worksheets/school-atom-cards.md
- worksheets/school-formula-to-graph.md
- exercises/README.md
- exercises/rust-molecule-model/README.md
- exercises/rust-molecule-model/src/lib.rs
- exercises/rust-molecule-model/src/molecule.rs
- visuals/README.md
- visuals/mermaid/course-map.md

Secondary consistency files:
- course/README.md
- course/school-track.md
- course/university-track.md
- activities/classroom-games.md
- book/SUMMARY.md
- wiki/Home.md
- wiki/Instructor-Notes.md
- wiki/Visual-Language.md

Review tasks:
1. Dependency check:
   - Find links to missing files, outdated file names, broken lesson references,
     stale command examples, or exercises that no longer match the Rust code.
   - Check that the intro path leads students from setup to chemistry primer to
     Lesson 01 without assuming knowledge that has not been introduced.
   - Check that worksheet, handout, visual, book, and lesson references agree.

2. Clarity check:
   - Mark statements that are ambiguous for beginners.
   - Mark chemistry statements that need more precise wording.
   - Mark Rust statements that oversimplify ownership, stack, heap, references,
     or types in a way that could mislead students.
   - Preserve useful analogies, but flag where an analogy must be bounded.

3. Concept map check:
   - Confirm that the course clearly relates:
     - atom -> typed value or structured record
     - bond -> relationship or graph edge
     - molecule -> graph-like data structure
     - formula -> compact summary or count map
     - DNA sequence -> biological data structure / memory representation
     - stack -> short-lived local values and call frames
     - heap -> growable owned data such as Vec and String
     - reference -> safe access without ownership transfer
     - Vec -> ordered growable collection
     - HashMap -> lookup table by key
     - sorted list -> ordered records for search/merge
   - Identify missing bridges between these ideas.

4. Fix proposal:
   - Do not rewrite the whole course.
   - Give a prioritized patch plan with exact file paths and concise edits.
   - For each issue, include:
     - file path
     - problem
     - why it matters for beginners
     - suggested replacement wording or action
     - priority: P0 broken, P1 confusing, P2 polish

5. Tutor script:
   Create a tutor script for:
   - Course intro, 5 to 7 minutes.
   - Lesson 01 intro, 10 to 12 minutes.
   - Lesson 01 guided coding section, 15 to 20 minutes.
   - Lesson 01 wrap-up and checkpoint questions, 5 minutes.

   Script requirements:
   - Use plain spoken language.
   - Include instructor lines, student prompts, expected student answers, and
     fallback explanations.
   - Include exact moments to show a molecule diagram, atom card, Rust struct,
     stack/heap sketch, and command-line test output.
   - Keep chemistry claims introductory and accurate.
   - Keep Rust claims accurate for beginners without going too deep.

6. Video materials:
   Create a lightweight video production package for the course intro and Lesson
   01:
   - Segment outline with timestamps.
   - Slide list with slide titles and 3 to 5 bullets per slide.
   - Visual directions for each slide.
   - Narration draft.
   - On-screen code snippets.
   - Suggested terminal commands.
   - Captions or transcript text.
   - Optional B-roll ideas using molecule cards, graph sketches, and memory
     diagrams.

7. Output format:
   Return results in this order:
   - Executive summary.
   - P0/P1/P2 issue table.
   - Exact patch recommendations by file.
   - Tutor script.
   - Video outline and slide plan.
   - Narration and caption draft.
   - Open questions for the course owner.

Constraints:
- Do not invent missing files. If a file is unavailable, say so and continue
  with the files provided.
- Do not claim real chemistry or drug-design validity beyond the toy teaching
  scope.
- Do not introduce unsafe lab instructions, proprietary compound data, docking
  results, or unpublished assay data.
- Do not make the course sound like production cheminformatics software.
- Keep the focus on beginner learning, analogy discipline, and runnable Rust.

If you propose code changes, make them minimal and explain how to verify them
with:

```bash
cd exercises/rust-molecule-model
cargo fmt --check
cargo test
cargo run -- water summary
```
```
