# Reference Notes

## Habr Memory Article

Old reference provided for this session:

- Habr: [Организация памяти](https://habr.com/en/articles/128991/) by `ztarlitz`,
  published September 23, 2011.

Finding:

I could not verify an original English version of this article. The `/en/` URL still
serves Russian text, and searches for distinctive phrases, the author name, and the
logical-to-linear-to-physical address wording did not reveal a matching English
original. Treat the Habr article as a Russian-language memory-organization reference,
not as a translation from a known English source.

How it is used in the lesson:

- historical hook for x86 memory organization
- vocabulary bridge for registers, stack, segment descriptors, and address translation
- not a primary source for modern Rust memory-safety teaching

Recommended English primary source for the same systems topic:

- Intel: [Intel 64 and IA-32 Architectures Software Developer Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

## Rust References

- Rust Book: [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- Rust Book: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- Rust Reference: [Types](https://doc.rust-lang.org/reference/types.html)

Lesson alignment:

- scalar types map to atom-level properties
- compound types map to molecules and molecule components
- references and borrowing map to safe temporary access during algorithms

## Quantum References

- IBM: [What is quantum computing?](https://www.ibm.com/think/topics/quantum-computing)
- IBM: [What is a qubit?](https://www.ibm.com/think/topics/qubit)
- IBM Quantum: [Learning](https://quantum.cloud.ibm.com/learning)

Lesson alignment:

- quantum state is introduced carefully as amplitude-based physical state
- qubits are not taught as ordinary memory upgrades
- molecular simulation is presented as a natural reason to care about quantum state

## Diagramming References

- PlantUML: [official site](https://plantuml.com/)

Lesson alignment:

- Mermaid is used for diagrams that render directly in GitHub Markdown.
- PlantUML is used for UML-style source files covering structures, contracts, and flows.

## Project Context References

Local project references:

- `memory/current.md`
- `playbooks/molecular-docking/README.md`
- `playbooks/molecular-docking/_shared/compute-profile.yml`

This lesson should not include raw molecular datasets, proprietary structures,
credentials, or assay results. It is safe to keep as public educational material.
