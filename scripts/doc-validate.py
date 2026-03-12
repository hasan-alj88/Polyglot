#!/usr/bin/env python3
"""Polyglot documentation validation script.

Validates documentation files against project conventions:
- 50KB file size limit
- Required YAML frontmatter
- _index.yaml schema compliance
- Cross-reference validity
- Unique document IDs
- Section line_range validity

Usage:
    python scripts/doc-validate.py              # Validate all docs
    python scripts/doc-validate.py docs/Tech/   # Validate specific directory
    python scripts/doc-validate.py --strict     # Treat warnings as errors
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path
from typing import Optional

import yaml
from pydantic import ValidationError

# Add project root to path
PROJECT_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(PROJECT_ROOT / "scripts"))

from doc_standards.schema import DocIndex, Frontmatter

# --- Constants ---

MAX_FILE_SIZE = 51200  # 50 KB
DOCS_DIR = PROJECT_ROOT / "docs"
SKIP_DIRS = {"_templates", "archive", "__pycache__"}
SKIP_FILES = {"README.md", "INDEX.md", "_conventions.md", "DOCUMENTATION-HIERARCHY.md",
              "_changelog.md", "_tags.md"}


# --- Result Tracking ---

class ValidationResult:
    def __init__(self) -> None:
        self.errors: list[str] = []
        self.warnings: list[str] = []

    def error(self, msg: str) -> None:
        self.errors.append(msg)

    def warn(self, msg: str) -> None:
        self.warnings.append(msg)

    @property
    def ok(self) -> bool:
        return len(self.errors) == 0

    def summary(self) -> str:
        lines = []
        for e in self.errors:
            lines.append(f"  ERROR: {e}")
        for w in self.warnings:
            lines.append(f"  WARN:  {w}")
        return "\n".join(lines)


# --- Validators ---

def check_file_size(path: Path, result: ValidationResult) -> None:
    """Check that file is under 50KB."""
    size = path.stat().st_size
    if size > MAX_FILE_SIZE:
        size_kb = size / 1024
        result.error(f"{path.relative_to(PROJECT_ROOT)}: {size_kb:.1f} KB exceeds 50 KB limit")


def parse_frontmatter(path: Path) -> Optional[dict]:
    """Extract YAML frontmatter from a markdown file."""
    try:
        content = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return None

    if not content.startswith("---"):
        return None

    end = content.find("---", 3)
    if end == -1:
        return None

    try:
        return yaml.safe_load(content[3:end])
    except yaml.YAMLError:
        return None


def check_frontmatter(path: Path, result: ValidationResult, strict: bool = False) -> None:
    """Validate frontmatter on .md files."""
    rel = path.relative_to(DOCS_DIR)

    # Skip certain files
    if rel.name in SKIP_FILES:
        return
    if any(part in SKIP_DIRS for part in rel.parts):
        return

    fm = parse_frontmatter(path)
    if fm is None:
        if strict:
            result.error(f"{rel}: missing or invalid YAML frontmatter")
        else:
            result.warn(f"{rel}: missing frontmatter (legacy file)")
        return

    try:
        Frontmatter(**fm)
    except ValidationError as e:
        errors = "; ".join(err["msg"] for err in e.errors())
        if strict:
            result.error(f"{rel}: invalid frontmatter: {errors}")
        else:
            result.warn(f"{rel}: invalid frontmatter: {errors}")


def check_index_schema(path: Path, result: ValidationResult) -> Optional[DocIndex]:
    """Validate _index.yaml against pydantic schema."""
    rel = path.relative_to(PROJECT_ROOT)
    try:
        content = path.read_text(encoding="utf-8")
        data = yaml.safe_load(content)
    except (yaml.YAMLError, OSError) as e:
        result.error(f"{rel}: invalid YAML: {e}")
        return None

    if data is None:
        result.error(f"{rel}: empty _index.yaml")
        return None

    try:
        return DocIndex(**data)
    except ValidationError as e:
        for err in e.errors():
            loc = " -> ".join(str(x) for x in err["loc"])
            result.error(f"{rel}: {loc}: {err['msg']}")
        return None


def collect_all_ids(docs_path: Path) -> dict[str, Path]:
    """Collect all document IDs from all _index.yaml files."""
    ids: dict[str, Path] = {}
    for index_path in docs_path.rglob("_index.yaml"):
        try:
            data = yaml.safe_load(index_path.read_text(encoding="utf-8"))
        except (yaml.YAMLError, OSError):
            continue
        if not data or "files" not in data:
            continue
        for f in data["files"]:
            if "id" in f:
                doc_id = f["id"]
                if doc_id in ids:
                    pass  # Duplicate detection happens in check_unique_ids
                ids[doc_id] = index_path
    return ids


def check_unique_ids(docs_path: Path, result: ValidationResult) -> None:
    """Check that all document IDs are unique across all indexes."""
    seen: dict[str, Path] = {}
    for index_path in docs_path.rglob("_index.yaml"):
        try:
            data = yaml.safe_load(index_path.read_text(encoding="utf-8"))
        except (yaml.YAMLError, OSError):
            continue
        if not data or "files" not in data:
            continue
        for f in data["files"]:
            if "id" not in f:
                continue
            doc_id = f["id"]
            if doc_id in seen:
                rel1 = seen[doc_id].relative_to(PROJECT_ROOT)
                rel2 = index_path.relative_to(PROJECT_ROOT)
                result.error(f"duplicate id '{doc_id}' in {rel1} and {rel2}")
            else:
                seen[doc_id] = index_path


def check_cross_refs(docs_path: Path, result: ValidationResult) -> None:
    """Validate cross-references and bidirectional linking."""
    all_ids = collect_all_ids(docs_path)

    # Check all prereqs/unlocks/related reference valid IDs
    for index_path in docs_path.rglob("_index.yaml"):
        try:
            data = yaml.safe_load(index_path.read_text(encoding="utf-8"))
        except (yaml.YAMLError, OSError):
            continue
        if not data or "files" not in data:
            continue

        rel = index_path.relative_to(PROJECT_ROOT)
        for f in data["files"]:
            file_id = f.get("id", "unknown")
            for ref_type in ("prereqs", "unlocks", "related"):
                for ref in f.get(ref_type, []):
                    ref_id = ref.get("id") if isinstance(ref, dict) else ref
                    if ref_id and ref_id not in all_ids:
                        result.error(
                            f"{rel}: {file_id}.{ref_type} references "
                            f"unknown id '{ref_id}'"
                        )


def check_line_ranges(docs_path: Path, result: ValidationResult) -> None:
    """Validate that section line_ranges are valid."""
    for index_path in docs_path.rglob("_index.yaml"):
        try:
            data = yaml.safe_load(index_path.read_text(encoding="utf-8"))
        except (yaml.YAMLError, OSError):
            continue
        if not data or "files" not in data:
            continue

        index_dir = index_path.parent
        rel_index = index_path.relative_to(PROJECT_ROOT)

        for f in data["files"]:
            if "sections" not in f:
                continue
            file_path = index_dir / f["file"]
            if not file_path.exists():
                result.warn(f"{rel_index}: file '{f['file']}' not found")
                continue

            try:
                line_count = sum(1 for _ in file_path.open(encoding="utf-8"))
            except OSError:
                continue

            for section in f["sections"]:
                if "line_range" not in section:
                    continue
                start, end = section["line_range"]
                if end > line_count:
                    result.warn(
                        f"{rel_index}: {f['id']} section '{section.get('anchor', '?')}' "
                        f"line_range [{start}, {end}] exceeds file length ({line_count} lines)"
                    )


# --- Main ---

def validate(target: Optional[Path] = None, strict: bool = False) -> ValidationResult:
    """Run all validation checks."""
    result = ValidationResult()
    docs_path = target or DOCS_DIR

    if not docs_path.exists():
        result.error(f"Path does not exist: {docs_path}")
        return result

    # 1. File size checks
    for md_file in docs_path.rglob("*.md"):
        check_file_size(md_file, result)

    # 2. Frontmatter checks
    for md_file in docs_path.rglob("*.md"):
        check_frontmatter(md_file, result, strict=strict)

    # 3. Index schema checks
    for index_file in docs_path.rglob("_index.yaml"):
        check_index_schema(index_file, result)

    # 4. Unique ID checks
    check_unique_ids(DOCS_DIR, result)

    # 5. Cross-reference checks
    check_cross_refs(DOCS_DIR, result)

    # 6. Line range checks
    check_line_ranges(DOCS_DIR, result)

    return result


def main() -> int:
    args = sys.argv[1:]
    strict = "--strict" in args
    args = [a for a in args if a != "--strict"]

    target = Path(args[0]) if args else None

    print("Polyglot Documentation Validator")
    print("=" * 40)

    result = validate(target, strict=strict)

    if result.errors or result.warnings:
        print(result.summary())
        print()

    error_count = len(result.errors)
    warn_count = len(result.warnings)

    if strict:
        total = error_count + warn_count
        print(f"Result: {total} issues ({error_count} errors, {warn_count} warnings)")
        return 1 if total > 0 else 0
    else:
        print(f"Result: {error_count} errors, {warn_count} warnings")
        return 1 if error_count > 0 else 0


if __name__ == "__main__":
    sys.exit(main())
