#!/usr/bin/env python3
"""Generate short silent MP4 presentation videos for Bio Memory Rust Lab."""

from __future__ import annotations

import math
import shutil
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "media" / "videos" / "generated"
FPS = 24
W, H = 1280, 720

NAVY = "#172033"
BLUE = "#355070"
GREEN = "#2f946d"
ORANGE = "#d9902f"
RED = "#c2413b"
LIGHT = "#f8fafc"
MUTED = "#516070"
WHITE = "#ffffff"
PANEL = "#eef2f7"


def font(size: int, bold: bool = False) -> ImageFont.FreeTypeFont:
    candidates = [
        "/System/Library/Fonts/Supplemental/Arial Bold.ttf" if bold else "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/System/Library/Fonts/Supplemental/Helvetica Bold.ttf" if bold else "/System/Library/Fonts/Supplemental/Helvetica.ttf",
        "/Library/Fonts/Arial Unicode.ttf",
    ]
    for candidate in candidates:
        if candidate and Path(candidate).exists():
            return ImageFont.truetype(candidate, size)
    return ImageFont.load_default()


F_TITLE = font(52, True)
F_H2 = font(34, True)
F_BODY = font(27)
F_SMALL = font(21)
F_CODE = font(23)


def ease(t: float) -> float:
    t = max(0, min(1, t))
    return t * t * (3 - 2 * t)


def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * ease(t)


def frame_base(title: str, subtitle: str = "") -> Image.Image:
    img = Image.new("RGB", (W, H), LIGHT)
    d = ImageDraw.Draw(img)
    d.rectangle((0, 0, W, 78), fill=NAVY)
    d.text((40, 20), "Bio Memory Rust Lab", fill=WHITE, font=F_SMALL)
    d.text((315, 18), title, fill=WHITE, font=F_SMALL)
    if subtitle:
        d.text((40, 105), subtitle, fill=MUTED, font=F_BODY)
    return img


def draw_text(d: ImageDraw.ImageDraw, xy, text: str, fill=NAVY, font_obj=F_BODY, anchor=None):
    d.text(xy, text, fill=fill, font=font_obj, anchor=anchor)


def draw_card(d, box, title, body, color=BLUE):
    x1, y1, x2, y2 = box
    d.rounded_rectangle(box, radius=20, fill=WHITE, outline=color, width=4)
    d.text((x1 + 24, y1 + 22), title, fill=color, font=F_H2)
    lines = wrap(body, 28)
    y = y1 + 78
    for line in lines:
        d.text((x1 + 24, y), line, fill=NAVY, font=F_SMALL)
        y += 30


def wrap(text: str, width: int) -> list[str]:
    words = text.split()
    lines: list[str] = []
    cur: list[str] = []
    for word in words:
        if sum(len(w) for w in cur) + len(cur) + len(word) > width and cur:
            lines.append(" ".join(cur))
            cur = [word]
        else:
            cur.append(word)
    if cur:
        lines.append(" ".join(cur))
    return lines


def draw_atom(d, xy, label, color=BLUE, r=34):
    x, y = xy
    d.ellipse((x - r, y - r, x + r, y + r), fill=WHITE, outline=color, width=5)
    d.text((x, y), label, fill=color, font=F_H2, anchor="mm")


def draw_bond(d, a, b, color=MUTED, width=6):
    d.line((a[0], a[1], b[0], b[1]), fill=color, width=width)


def draw_arrow(d, start, end, color=BLUE, width=4):
    d.line((start[0], start[1], end[0], end[1]), fill=color, width=width)
    ang = math.atan2(end[1] - start[1], end[0] - start[0])
    length = 16
    for delta in (math.pi * 0.82, -math.pi * 0.82):
        p = (end[0] + length * math.cos(ang + delta), end[1] + length * math.sin(ang + delta))
        d.line((end[0], end[1], p[0], p[1]), fill=color, width=width)


def render_video(name: str, duration: float, draw_func):
    OUT.mkdir(parents=True, exist_ok=True)
    frames = int(duration * FPS)
    with tempfile.TemporaryDirectory(prefix=f"{name}-frames-") as tmp:
        tmp_path = Path(tmp)
        for i in range(frames):
            t = i / max(1, frames - 1)
            img = draw_func(t)
            img.save(tmp_path / f"frame_{i:04d}.png")
        out = OUT / f"{name}.mp4"
        cmd = [
            "ffmpeg",
            "-y",
            "-framerate",
            str(FPS),
            "-i",
            str(tmp_path / "frame_%04d.png"),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-crf",
            "30",
            "-movflags",
            "+faststart",
            str(out),
        ]
        subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return out


def formula_counts(t: float):
    img = frame_base("Video 01 - Formula Counts", "Chemical formulas are compact count tables.")
    d = ImageDraw.Draw(img)
    formulas = [
        ("H2O", [("H", "2"), ("O", "1")], BLUE),
        ("CH4", [("C", "1"), ("H", "4")], GREEN),
        ("C2H6O", [("C", "2"), ("H", "6"), ("O", "1")], ORANGE),
    ]
    for idx, (formula, parts, color) in enumerate(formulas):
        x = 80 + idx * 395
        y = 190
        draw_card(d, (x, y, x + 330, y + 360), formula, "Count atoms by element.", color)
        visible = max(0, min(len(parts), int((t * 4) - idx + 1)))
        yy = y + 155
        for symbol, count in parts[:visible]:
            d.text((x + 55, yy), symbol, fill=color, font=F_H2)
            d.text((x + 180, yy), f"= {count}", fill=NAVY, font=F_H2)
            yy += 60
    d.text((80, 615), "Formula preserves counts. It does not show every bond or 3D shape.", fill=NAVY, font=F_BODY)
    return img


def formula_to_graph(t: float):
    img = frame_base("Video 02 - Formula To Graph", "C2H6O tells counts; bonds tell structure.")
    d = ImageDraw.Draw(img)
    d.text((70, 135), "Formula:", fill=MUTED, font=F_H2)
    d.text((220, 128), "C2H6O", fill=ORANGE, font=F_TITLE)
    d.text((70, 215), "Graph view:", fill=MUTED, font=F_H2)
    atoms = {
        0: ("C", (430, 380)),
        1: ("C", (570, 380)),
        2: ("O", (710, 380)),
        3: ("H", (360, 280)),
        4: ("H", (360, 480)),
        5: ("H", (430, 555)),
        6: ("H", (570, 555)),
        7: ("H", (570, 255)),
        8: ("H", (810, 380)),
    }
    bonds = [(0, 1), (1, 2), (0, 3), (0, 4), (0, 5), (1, 6), (1, 7), (2, 8)]
    show_bonds = int(lerp(0, len(bonds), min(1, t * 1.4)))
    for a, b in bonds[:show_bonds]:
        draw_bond(d, atoms[a][1], atoms[b][1])
    show_atoms = int(lerp(0, len(atoms), min(1, t * 1.8)))
    for i in list(atoms)[:show_atoms]:
        label, xy = atoms[i]
        color = GREEN if label == "C" else ORANGE if label == "O" else BLUE
        draw_atom(d, xy, label, color=color, r=30)
    d.text((70, 610), "Formula = counts. Molecule graph = atoms + bonds.", fill=NAVY, font=F_BODY)
    return img


def rust_data_model(t: float):
    img = frame_base("Video 03 - Rust Data Model", "Enums, structs, vectors, and traits describe molecules.")
    d = ImageDraw.Draw(img)
    boxes = [
        ("Element enum", "H, C, N, O, Cl", (80, 165), BLUE),
        ("Atom struct", "element + charge + flags", (390, 165), GREEN),
        ("Bond struct", "from + to + order", (700, 165), ORANGE),
        ("Molecule struct", "Vec<Atom> + Vec<Bond>", (390, 430), BLUE),
    ]
    for idx, (title, body, pos, color) in enumerate(boxes):
        alpha_t = min(1, max(0, t * 5 - idx))
        if alpha_t <= 0:
            continue
        x, y = pos
        draw_card(d, (x, y, x + 260, y + 155), title, body, color)
    if t > 0.35:
        draw_arrow(d, (340, 240), (385, 240), BLUE)
        draw_arrow(d, (650, 240), (695, 240), BLUE)
        draw_arrow(d, (520, 325), (520, 425), BLUE)
        draw_arrow(d, (830, 325), (650, 430), BLUE)
    if t > 0.65:
        d.rounded_rectangle((845, 450, 1150, 610), radius=18, fill=WHITE, outline=GREEN, width=4)
        d.text((875, 478), "Traits", fill=GREEN, font=F_H2)
        for n, line in enumerate(["Describe", "ChemicalFormula", "MolecularGraph"]):
            d.text((890, 528 + n * 28), line, fill=NAVY, font=F_SMALL)
    return img


def bfs_path(t: float):
    img = frame_base("Video 04 - BFS Path", "Shortest path explores a molecule shell by shell.")
    d = ImageDraw.Draw(img)
    pts = {
        3: (180, 450), 0: (330, 360), 1: (500, 360), 2: (670, 360), 8: (830, 360),
        4: (300, 520), 5: (330, 210), 6: (500, 520), 7: (500, 210)
    }
    bonds = [(3,0), (0,1), (1,2), (2,8), (0,4), (0,5), (1,6), (1,7)]
    path = [3,0,1,2,8]
    for a,b in bonds:
        draw_bond(d, pts[a], pts[b], color="#cbd5e1", width=5)
    step = min(len(path), int(t * (len(path)+1)))
    for i in range(max(0, step-1)):
        draw_bond(d, pts[path[i]], pts[path[i+1]], color=ORANGE, width=9)
    for atom_id, xy in pts.items():
        color = ORANGE if atom_id in path[:step] else BLUE
        draw_atom(d, xy, str(atom_id), color=color, r=28)
    d.text((80, 135), "Command:", fill=MUTED, font=F_H2)
    d.text((245, 136), "cargo run -- ethanol path 3 8", fill=NAVY, font=F_CODE)
    if step >= len(path):
        d.text((80, 620), "Output: [3, 0, 1, 2, 8]", fill=GREEN, font=F_H2)
    else:
        d.text((80, 620), "BFS visits neighbors, records previous atoms, then reconstructs the route.", fill=NAVY, font=F_BODY)
    return img


def cli_flow(t: float):
    img = frame_base("Video 05 - CLI Flow", "A command becomes data, then a graph question, then output.")
    d = ImageDraw.Draw(img)
    stages = [
        ("Student command", "ethanol path 3 8"),
        ("Parse args", "molecule + command"),
        ("Lookup molecule", "C2H6O graph"),
        ("Run algorithm", "shortest_path(3, 8)"),
        ("Print output", "[3, 0, 1, 2, 8]"),
    ]
    y = 290
    for i, (title, body) in enumerate(stages):
        x = 65 + i * 240
        color = [BLUE, GREEN, ORANGE, BLUE, GREEN][i]
        if t * 6 < i:
            continue
        d.rounded_rectangle((x, y, x + 190, y + 135), radius=18, fill=WHITE, outline=color, width=4)
        d.text((x + 18, y + 22), title, fill=color, font=F_SMALL)
        for n, line in enumerate(wrap(body, 14)):
            d.text((x + 18, y + 65 + n * 24), line, fill=NAVY, font=F_SMALL)
        if i < len(stages) - 1 and t * 6 > i + 0.5:
            draw_arrow(d, (x + 195, y + 68), (x + 235, y + 68), BLUE, width=4)
    d.text((90, 150), "cargo run -- ethanol path 3 8", fill=NAVY, font=F_TITLE)
    d.text((90, 560), "Teaching point: CLI boundaries turn text input into typed program behavior.", fill=MUTED, font=F_BODY)
    return img


def feedback_loop(t: float):
    img = frame_base("Video 06 - Screening Feedback Loop", "Generate, validate, score, rank, learn, repeat.")
    d = ImageDraw.Draw(img)
    nodes = [
        ("Generate", (260, 210), BLUE),
        ("Validate", (610, 210), GREEN),
        ("Score", (950, 210), ORANGE),
        ("Rank", (950, 485), BLUE),
        ("Learn", (610, 485), GREEN),
        ("Next round", (260, 485), ORANGE),
    ]
    for idx, (label, xy, color) in enumerate(nodes):
        if t * 7 < idx:
            continue
        x,y = xy
        d.rounded_rectangle((x-105,y-50,x+105,y+50), radius=18, fill=WHITE, outline=color, width=4)
        d.text((x, y), label, fill=color, font=F_H2, anchor="mm")
    edges = [(0,1), (1,2), (2,3), (3,4), (4,5), (5,0)]
    for idx, (a,b) in enumerate(edges):
        if t * 7 < idx + 1.2:
            continue
        draw_arrow(d, nodes[a][1], nodes[b][1], BLUE, width=4)
    d.text((80, 120), "Molecule design as an algorithmic loop", fill=NAVY, font=F_H2)
    d.text((80, 625), "Same mental model supports Rust queues, rankings, maps, and feedback data.", fill=MUTED, font=F_BODY)
    return img


VIDEOS = [
    ("01-formula-counts", 10, formula_counts),
    ("02-formula-to-graph", 11, formula_to_graph),
    ("03-rust-data-model", 10, rust_data_model),
    ("04-bfs-path", 10, bfs_path),
    ("05-cli-flow", 10, cli_flow),
    ("06-screening-feedback-loop", 10, feedback_loop),
]


def write_readme(outputs: list[Path]):
    lines = [
        "# Video Materials",
        "",
        "Short silent MP4 videos for presentations and classroom transitions.",
        "",
        "These are designed to be dropped into PowerPoint, Keynote, Google Slides,",
        "or played directly during a lesson.",
        "",
        "## Videos",
        "",
        "| Video | Use with |",
        "| --- | --- |",
        "| [01-formula-counts.mp4](generated/01-formula-counts.mp4) | Chemistry primer, formulas `H2O`, `CH4`, `C2H6O` |",
        "| [02-formula-to-graph.mp4](generated/02-formula-to-graph.mp4) | Formula versus graph representation |",
        "| [03-rust-data-model.mp4](generated/03-rust-data-model.mp4) | Rust enums, structs, vectors, traits |",
        "| [04-bfs-path.mp4](generated/04-bfs-path.mp4) | Molecule graph traversal and shortest path |",
        "| [05-cli-flow.mp4](generated/05-cli-flow.mp4) | Molecule Explorer CLI capstone |",
        "| [06-screening-feedback-loop.mp4](generated/06-screening-feedback-loop.mp4) | Screening, scoring, feedback, molecule design algorithms |",
        "",
        "## Regenerate",
        "",
        "```bash",
        "PYTHONPATH=/Users/akuksin/.cache/codex-runtimes/codex-primary-runtime/dependencies/python \\",
        "  /Users/akuksin/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3 \\",
        "  tools/generate_video_materials.py",
        "```",
        "",
        "The generator uses Pillow plus `ffmpeg` and writes compact MP4 files.",
        "",
        "## Presentation Notes",
        "",
        "- Use these videos as silent loops or short transitions.",
        "- Narrate over them live, or pair them with the tutor scripts in `instructor-materials/`.",
        "- Keep generated videos short so they remain suitable for Git and slide decks.",
    ]
    (ROOT / "media" / "videos" / "README.md").write_text("\n".join(lines) + "\n")


def write_storyboards():
    text = """# Video Storyboards

## 01 Formula Counts

Show formulas as compact element-count tables. Narration: formulas preserve counts,
not structure.

## 02 Formula To Graph

Reveal ethanol atoms and bonds from `C2H6O`. Narration: graph representation adds
connectivity.

## 03 Rust Data Model

Animate `Element -> Atom -> Bond -> Molecule -> Traits`. Narration: Rust types make
the model explicit.

## 04 BFS Path

Animate path `3 -> 0 -> 1 -> 2 -> 8`. Narration: BFS explores neighbors and records
where it came from.

## 05 CLI Flow

Animate command text through parsing, molecule lookup, algorithm, and output.

## 06 Screening Feedback Loop

Animate generate, validate, score, rank, learn, next round.
"""
    (ROOT / "media" / "videos" / "storyboards" / "README.md").write_text(text)


def main():
    if not shutil.which("ffmpeg"):
        raise SystemExit("ffmpeg is required")
    outputs = []
    for name, duration, draw_func in VIDEOS:
        path = render_video(name, duration, draw_func)
        outputs.append(path)
        print(path.relative_to(ROOT))
    write_readme(outputs)
    write_storyboards()


if __name__ == "__main__":
    main()

