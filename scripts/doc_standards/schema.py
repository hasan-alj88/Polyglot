"""Pydantic models for Polyglot documentation standards.

Defines schemas for:
- _index.yaml files (polyglot-doc-index-v1)
- Markdown frontmatter (5 required fields)
"""

from __future__ import annotations

import re
from datetime import date
from enum import Enum
from typing import Optional

from pydantic import BaseModel, Field, field_validator


# --- Enums ---

class Audience(str, Enum):
    USER = "user"
    DEVELOPER = "developer"
    MIXED = "mixed"


class DocStatus(str, Enum):
    DRAFT = "draft"
    REVIEW = "review"
    STABLE = "stable"
    DEPRECATED = "deprecated"


class DocType(str, Enum):
    FEATURE_GUIDE = "feature-guide"
    TUTORIAL = "tutorial"
    ARCHITECTURE = "architecture"
    SPEC = "spec"
    LANGUAGE_SPEC = "language-spec"
    AUDIT = "audit"
    DECISION = "decision"
    REFERENCE = "reference"
    HUB = "hub"


# --- Frontmatter Schema ---

class Frontmatter(BaseModel):
    """Required YAML frontmatter for every .md file under docs/."""

    id: str = Field(max_length=60, pattern=r"^[a-z0-9][a-z0-9-]*[a-z0-9]$")
    audience: Audience
    type: DocType
    status: DocStatus
    updated: date


# --- _index.yaml Schema ---

class SectionEntry(BaseModel):
    """Section-level metadata for selective loading."""

    anchor: str = Field(min_length=1)
    title: str = Field(min_length=1, max_length=120)
    line_range: tuple[int, int]
    summary: str = Field(min_length=1, max_length=200)
    keywords: list[str] = Field(min_length=1, max_length=10)

    @field_validator("line_range")
    @classmethod
    def validate_line_range(cls, v: tuple[int, int]) -> tuple[int, int]:
        start, end = v
        if start < 1:
            raise ValueError("line_range start must be >= 1")
        if end < start:
            raise ValueError("line_range end must be >= start")
        return v


class DependencyRef(BaseModel):
    """Reference to another document by ID."""

    id: str = Field(min_length=1, max_length=60)
    reason: Optional[str] = Field(default=None, max_length=200)


class FileEntry(BaseModel):
    """Metadata for a single document in a directory index."""

    id: str = Field(max_length=60, pattern=r"^[a-z0-9][a-z0-9-]*[a-z0-9]$")
    file: str = Field(min_length=1)
    title: str = Field(min_length=1, max_length=120)
    audience: Audience
    status: DocStatus
    updated: date
    size_kb: int = Field(ge=0, le=50)
    keywords: list[str] = Field(min_length=1, max_length=10)
    summary: str = Field(min_length=1, max_length=200)

    # Optional fields
    type: Optional[DocType] = None
    sections: list[SectionEntry] = Field(default_factory=list)
    prereqs: list[DependencyRef] = Field(default_factory=list)
    unlocks: list[DependencyRef] = Field(default_factory=list)
    related: list[DependencyRef] = Field(default_factory=list)
    parent: Optional[str] = None
    children: list[str] = Field(default_factory=list)

    @field_validator("id")
    @classmethod
    def validate_id_format(cls, v: str) -> str:
        if not re.match(r"^[a-z0-9][a-z0-9-]*[a-z0-9]$", v):
            raise ValueError(
                f"id must be kebab-case (lowercase alphanumeric + hyphens): {v}"
            )
        return v


class SubdirectoryRef(BaseModel):
    """Pointer to a child directory's _index.yaml."""

    path: str = Field(min_length=1)
    description: str = Field(min_length=1, max_length=200)
    index: str = Field(min_length=1)


class DirectoryMeta(BaseModel):
    """Directory-level metadata."""

    path: str = Field(min_length=1)
    audience: Audience
    description: str = Field(min_length=1, max_length=200)
    parent: Optional[str] = None


class DocIndex(BaseModel):
    """Schema for _index.yaml files (polyglot-doc-index-v1)."""

    schema_version: str = Field(
        alias="_schema", default="polyglot-doc-index-v1"
    )
    directory: DirectoryMeta
    files: list[FileEntry] = Field(default_factory=list)
    subdirectories: list[SubdirectoryRef] = Field(default_factory=list)

    model_config = {"populate_by_name": True}


# --- Root Index Extensions ---

class AudienceEntry(BaseModel):
    """Audience routing entry for root index."""

    entry_point: str = Field(min_length=1)
    description: str = Field(min_length=1, max_length=200)


class ProjectMeta(BaseModel):
    """Project-level metadata for root index."""

    name: str = Field(min_length=1)
    version: str = Field(min_length=1)
    doc_count: int = Field(ge=0)
    last_full_reindex: date


class RootDocIndex(DocIndex):
    """Extended schema for the root docs/_index.yaml."""

    project: Optional[ProjectMeta] = None
    audiences: Optional[dict[str, AudienceEntry]] = None
