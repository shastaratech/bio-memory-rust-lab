# Video Materials

Short silent MP4 videos for presentations and classroom transitions.

These are designed to be dropped into PowerPoint, Keynote, Google Slides,
or played directly during a lesson.

## Videos

| Video | Use with |
| --- | --- |
| [01-formula-counts.mp4](generated/01-formula-counts.mp4) | Chemistry primer, formulas `H2O`, `CH4`, `C2H6O` |
| [02-formula-to-graph.mp4](generated/02-formula-to-graph.mp4) | Formula versus graph representation |
| [03-rust-data-model.mp4](generated/03-rust-data-model.mp4) | Rust enums, structs, vectors, traits |
| [04-bfs-path.mp4](generated/04-bfs-path.mp4) | Molecule graph traversal and shortest path |
| [05-cli-flow.mp4](generated/05-cli-flow.mp4) | Molecule Explorer CLI capstone |
| [06-screening-feedback-loop.mp4](generated/06-screening-feedback-loop.mp4) | Screening, scoring, feedback, molecule design algorithms |

## Regenerate

```bash
PYTHONPATH=/Users/akuksin/.cache/codex-runtimes/codex-primary-runtime/dependencies/python \
  /Users/akuksin/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3 \
  tools/generate_video_materials.py
```

The generator uses Pillow plus `ffmpeg` and writes compact MP4 files.

## Presentation Notes

- Use these videos as silent loops or short transitions.
- Narrate over them live, or pair them with the tutor scripts in `instructor-materials/`.
- Keep generated videos short so they remain suitable for Git and slide decks.
