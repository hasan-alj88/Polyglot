#!/usr/bin/env python3
"""Generate or update _index.yaml for a documentation directory.

Scans a directory for .md files and generates skeleton _index.yaml entries
with:
- id derived from filename
- title from first heading
- sections from headings with line ranges
- size_kb from file size
- keywords placeholder

Usage:
    python scripts/doc-reindex.py docs/Tech/Architecture/     # Print skeleton
    python scripts/doc-reindex.py docs/Tech/Architecture/ --write  # Write to file
    python scripts/doc-reindex.py docs/Tech/Architecture/ --update # Add new files only
"""

from __future__ import annotations

import os
import re
import sys
from datetime import date
from pathlib import Path

import yaml

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DOCS_DIR = PROJECT_ROOT / "docs"

SKIP_FILES = {"README.md", "INDEX.md", "_conventions.md", "DOCUMENTATION-HIERARCHY.md",
              "_changelog.md", "_tags.md"}


def extract_title(path: Path) -> str:
    """Extract the first H1 heading from a markdown file."""
    try:
        for line in path.open(encoding="utf-8"):
            line = line.strip()
            if line.startswith("# ") and not line.startswith("# ---"):
                return line[2:].strip()
    except (OSError, UnicodeDecodeError):
        pass
    return path.stem.replace("-", " ").title()


def extract_sections(path: Path) -> list[dict]:
    """Extract H2 headings with line ranges."""
    sections = []
    current_section = None
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except (OSError, UnicodeDecodeError):
        return []

    for i, line in enumerate(lines, 1):
        if line.startswith("## "):
            if current_section:
                current_section["line_range"][1] = i - 1
                sections.append(current_section)

            title = line[3:].strip()
            anchor = re.sub(r"[^a-z0-9\s-]", "", title.lower())
            anchor = re.sub(r"\s+", "-", anchor).strip("-")

            current_section = {
                "anchor": anchor,
                "title": title,
                "line_range": [i, 0],
                "summary": f"Section: {title}",
                "keywords": [anchor.split("-")[0]] if anchor else ["section"],
            }

    if current_section:
        current_section["line_range"][1] = len(lines)
        sections.append(current_section)

    return sections


def filename_to_id(filename: str) -> str:
    """Convert filename to a kebab-case id."""
    name = Path(filename).stem
    # Already kebab-case in most cases
    return name.lower()


def guess_audience(path: Path) -> str:
    """Guess audience from directory location."""
    rel = path.relative_to(DOCS_DIR)
    parts = rel.parts
    if parts and parts[0] == "User":
        return "user"
    if parts and parts[0] == "Tech":
        return "developer"
    if parts and parts[0] == "Audit":
        return "developer"
    return "mixed"


def generate_index(target_dir: Path) -> dict:
    """Generate a skeleton _index.yaml for a directory."""
    rel_path = target_dir.relative_to(DOCS_DIR)
    audience = guess_audience(target_dir)

    # Find parent index
    parent = None
    if target_dir != DOCS_DIR:
        parent_dir = target_dir.parent
        parent_index = parent_dir / "_index.yaml"
        if parent_index.exists():
            parent = str(parent_index.relative_to(DOCS_DIR))

    index = {
        "_schema": "polyglot-doc-index-v1",
        "directory": {
            "path": str(rel_path),
            "audience": audience,
            "description": f"Documentation for {rel_path}",
            "parent": parent,
        },
        "files": [],
        "subdirectories": [],
    }

    # Scan .md files
    for md_file in sorted(target_dir.glob("*.md")):
        if md_file.name in SKIP_FILES:
            continue
        if md_file.name.startswith("_"):
            continue

        file_id = filename_to_id(md_file.name)
        # Ensure id is at least 2 chars (schema requires start + end alphanumeric)
        if len(file_id) < 2:
            file_id = f"{file_id}-doc"

        size_kb = md_file.stat().st_size // 1024
        title = extract_title(md_file)
        sections = extract_sections(md_file)

        entry = {
            "id": file_id,
            "file": md_file.name,
            "title": title,
            "audience": audience,
            "status": "review",
            "updated": str(date.today()),
            "size_kb": min(size_kb, 50),
            "keywords": [file_id.split("-")[0]],
            "summary": f"TODO: Add summary for {title}",
        }

        if sections:
            entry["sections"] = sections

        index["files"].append(entry)

    # Scan subdirectories
    for subdir in sorted(target_dir.iterdir()):
        if not subdir.is_dir():
            continue
        if subdir.name.startswith(("_", ".")):
            continue

        sub_rel = subdir.relative_to(DOCS_DIR)
        index["subdirectories"].append({
            "path": f"{subdir.name}/",
            "description": f"Documentation for {subdir.name}",
            "index": f"{sub_rel}/_index.yaml",
        })

    return index


def main() -> int:
    args = sys.argv[1:]
    if not args:
        print("Usage: python scripts/doc-reindex.py <directory> [--write|--update]")
        return 1

    target = Path(args[0]).resolve()
    write_mode = "--write" in args
    update_mode = "--update" in args

    if not target.is_dir():
        print(f"Error: {target} is not a directory")
        return 1

    index = generate_index(target)
    index_path = target / "_index.yaml"

    if update_mode and index_path.exists():
        # Load existing and only add new files
        existing = yaml.safe_load(index_path.read_text(encoding="utf-8"))
        existing_ids = {f["id"] for f in existing.get("files", [])}
        new_files = [f for f in index["files"] if f["id"] not in existing_ids]

        if new_files:
            existing.setdefault("files", []).extend(new_files)
            output = yaml.dump(existing, default_flow_style=False, sort_keys=False,
                               allow_unicode=True)
            if write_mode:
                index_path.write_text(output, encoding="utf-8")
                print(f"Updated {index_path} with {len(new_files)} new entries")
            else:
                print(f"# Would add {len(new_files)} new entries to {index_path}")
                print(yaml.dump(new_files, default_flow_style=False, sort_keys=False))
        else:
            print("No new files to add")
        return 0

    output = yaml.dump(index, default_flow_style=False, sort_keys=False,
                       allow_unicode=True)

    if write_mode:
        index_path.write_text(output, encoding="utf-8")
        print(f"Written: {index_path}")
    else:
        print(f"# Skeleton _index.yaml for {target.relative_to(PROJECT_ROOT)}")
        print(output)

    return 0


if __name__ == "__main__":
    sys.exit(main())
