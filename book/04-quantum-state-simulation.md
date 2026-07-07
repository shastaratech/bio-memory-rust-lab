# Chapter 04: Quantum State and Molecular Simulation

## Big Idea

A molecule graph is useful, but it is not the whole molecule. It knows atoms and
bonds. It does not fully know electron density, quantum state, energy, or reactivity.

Quantum simulation is interesting because molecules are quantum systems.

## Read First

- Lesson: [Quantum State and Molecular Simulation](../lessons/04-quantum-state-and-simulation.md)
- Visual: [Quantum Simulation Loop](../visuals/plantuml/quantum-simulation-loop.puml)
- References: [Reference Notes](../refs/sources.md)

## Run The Graph Model

First remind yourself what the toy graph can answer:

```bash
cd exercises/rust-molecule-model
cargo run -- ethanol summary
cargo run -- ethanol path 3 8
```

The model can answer:

- formula
- atom count
- bond count
- neighbors
- paths
- connected components

The model cannot answer:

- exact 3D shape
- electron density
- real reaction behavior
- quantum energy
- docking score

## Quantum-Classical Loop

Open:

```text
visuals/plantuml/quantum-simulation-loop.puml
```

Read the flow:

1. Choose molecular problem.
2. Build simplified Hamiltonian.
3. Choose parameterized trial state.
4. Prepare quantum state.
5. Measure energy terms.
6. Classical optimizer updates parameters.
7. Repeat until energy stops improving.

This is the high-level idea behind VQE.

## Important Guardrail

Do not say:

> A qubit is just a bigger bit.

Say:

> A qubit is a quantum state. Measurement gives outcomes according to amplitudes.

## Self-Study Exercise

Write a short paragraph:

1. What does the Rust molecule graph represent well?
2. What does it leave out?
3. Why might quantum simulation matter for real molecules?

## Checkpoint

1. Why is a molecule graph not a complete molecule?
2. What does VQE try to optimize?
3. Which part of the VQE loop is classical?
4. Which part involves quantum state preparation and measurement?

