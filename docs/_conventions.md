# Polyglot Documentation Conventions

**Version:** 2.0
**Last Updated:** 2026-03-12
**Status:** Active Standard
**Replaces:** BMAD Documentation Conventions v1.0

---

## Purpose

This document defines the ground rules for all Polyglot project documentation. Every contributor -- human or AI -- must follow these conventions to ensure structure, order, and consistency across the documentation tree.

---

## 1. Audiences

All documentation is written for one of three audiences. The audience determines what to include and what to omit.

| Audience | What They Need | What to Exclude |
|----------|---------------|-----------------|
| **User** | What is this? How do I use it? Syntax, examples, patterns. | Implementation details, architecture, compiler internals. |
| **Developer** | Architecture, how it works, technical internals, parser/compiler details. | Simplified tutorials (those belong in User docs). |
| **AI** | Structured indexes (`_index.yaml`) to find relevant content by keyword, section, and dependency. | AI does not read docs directly -- it reads indexes first, then loads targeted sections. |

Every document must declare its audience in frontmatter. Use `mixed` only when a document genuinely serves both users and developers (rare).

---

## 2. Document Structure

### 2.1 Top-Level Directories

```
docs/
├── User/              # User documentation (audience: user)
│   ├── getting-started/
│   ├── language/
│   │   ├── syntax/
│   │   ├── types/
│   │   ├── control-flow/
│   │   └── advanced/
│   ├── examples/
│   ├── reference/
│   ├── specifications/
│   └── stdlib/
│
├── Tech/              # Technical documentation (audience: developer)
│   ├── Architecture/
│   ├── Polyglot/      # Language internals
│   │   ├── Syntax/
│   │   ├── Parser/
│   │   └── Compiler/
│   ├── QueueManager/
│   └── Runner/
│
├── Audit/             # Documentation health (audience: developer)
│   ├── documentation/  # Audit violations, fixes
│   └── decisions/      # Decisions that may require doc re-sync
│
├── archive/           # Historical content (read-only)
├── _templates/        # Document templates
└── _index.yaml        # Root index for AI navigation
```

### 2.2 Directory Rules

- Every directory under `docs/` gets an `_index.yaml` file (see Section 5)
- Use kebab-case for directory and file names: `queue-manager.md`, not `QueueManager.md`
  - Exception: top-level directories (`User/`, `Tech/`, `Audit/`) use PascalCase for readability
- Create subdirectories when 5+ files share a common concept
- Do not create subdirectories for fewer than 3 files

### 2.3 Where to Put New Documentation

```
New document?
│
├─ Is it for language users (syntax, tutorials, examples)?
│  └─ YES → User/
│
├─ Is it about architecture, parser, compiler, or internals?
│  └─ YES → Tech/
│
├─ Is it an audit finding or a decision record?
│  └─ YES → Audit/
│
└─ Historical or superseded content?
   └─ YES → archive/
```

---

## 3. File Rules

### 3.1 Size Limit: 50 KB Maximum

**Every file under `docs/` must be < 50 KB (51,200 bytes).**

This is enforced by the pre-commit validation script (`scripts/doc-validate.py`).

**When a file exceeds or approaches 50 KB:**

1. **Split by concept**, not by size. Each resulting file must be a self-contained topic.
2. **Never** create "Part 1 / Part 2" continuations.
3. Create a **hub document** (< 5 KB) at the original path that links to all split-out subtopics.
4. Each split file gets its own `_index.yaml` entry with full metadata.
5. Update all incoming cross-references to point to specific sub-documents.

**Example -- splitting a large architecture doc:**

```
Before:  Tech/Architecture/architecture.md (68 KB)

After:   Tech/Architecture/
           overview.md          (hub, ~3 KB, links to all below)
           parser-arch.md       (~15 KB)
           ir-representation.md (~12 KB)
           storage-layer.md     (~18 KB)
           queue-system.md      (~14 KB)
```

### 3.2 Required Frontmatter

Every `.md` file under `docs/` (except `_templates/` and `README.md`) must have YAML frontmatter with these 5 required fields:

```yaml
---
id: unique-kebab-id
audience: user | developer | mixed
type: feature-guide | tutorial | architecture | spec | language-spec | audit | decision | reference | hub
status: draft | review | stable | deprecated
updated: YYYY-MM-DD
---
```

| Field | Constraints | Purpose |
|-------|-------------|---------|
| `id` | kebab-case, unique across all docs, max 60 chars | Document identifier for cross-referencing |
| `audience` | `user`, `developer`, or `mixed` | Who this document is for |
| `type` | See enum above | Document classification |
| `status` | `draft`, `review`, `stable`, `deprecated` | Lifecycle state |
| `updated` | `YYYY-MM-DD` format | Last modification date |

### 3.3 File Naming

- Use kebab-case: `variables-lifecycle.md`, `queue-manager.md`
- Descriptive, not abbreviated: `error-handling.md` not `err-hdl.md`
- Plural for collections: `examples/`, `patterns/`
- Singular for concepts: `syntax/`, `reference/`

---

## 4. Cross-Referencing

### 4.1 Reference Syntax

**Link to another file:**
```markdown
See [Queue Manager Architecture](../Tech/Architecture/queue-manager.md)
```

**Link to a specific section:**
```markdown
See [Priority Algorithm](../Tech/Architecture/queue-manager.md#priority-algorithm)
```

The anchor after `#` is the standard markdown heading anchor (lowercase, hyphens for spaces).

### 4.2 Required vs Optional References

**Required references** go in a `Prerequisites` section at the top:
```markdown
## Prerequisites

You **must** read [Pipeline Structure](path#section) before this document.
```

**Optional references** go in a `See Also` section at the bottom:
```markdown
## See Also

- [Runner Internals](path) -- deeper dive into execution
- [Async Model](path#overview) -- background on async design
```

### 4.3 Bidirectional Linking

In `_index.yaml` files, cross-references must be bidirectional:
- If A lists B in `prereqs`, then B must list A in `unlocks`
- If A lists B in `related`, then B must list A in `related`

This is validated by the pre-commit hook.

In markdown body text, bidirectional links are recommended but not strictly enforced.

### 4.4 Section Anchors

Every major section heading becomes a linkable anchor. Use descriptive headings that produce clean anchors:

```markdown
## Priority Algorithm          → #priority-algorithm
## Variable Lifecycle States   → #variable-lifecycle-states
```

Avoid headings that produce ambiguous anchors (e.g., multiple `## Overview` in one file).

---

## 5. YAML Index Files (`_index.yaml`)

### 5.1 Purpose

Every directory under `docs/` has an `_index.yaml` file. These are the primary mechanism for AI tools to discover and selectively load content without scanning the filesystem.

### 5.2 Schema

```yaml
_schema: polyglot-doc-index-v1

# Directory-level metadata
directory:
  path: Tech/Architecture          # relative to docs/
  audience: developer              # user | developer | mixed
  description: "System architecture and component design"
  parent: Tech/_index.yaml         # path to parent index (null at root)

# File entries
files:
  - id: queue-manager-arch
    file: queue-manager.md
    title: "Queue Manager Architecture"
    audience: developer
    status: stable
    updated: 2026-03-12
    size_kb: 34
    keywords: [queue, scheduling, priority, async]
    summary: >
      Priority-based queue manager that schedules pipeline
      execution across runtime wrappers.

    # Section-level entries for selective loading
    sections:
      - anchor: overview
        title: "Overview"
        line_range: [1, 45]
        summary: "High-level queue manager purpose and design goals"
        keywords: [queue, design, goals]

      - anchor: priority-algorithm
        title: "Priority Algorithm"
        line_range: [46, 120]
        summary: "How pipeline priority is calculated and resolved"
        keywords: [priority, algorithm, scheduling]

    # Dependency graph
    prereqs:
      - id: pipeline-structure
        reason: "Defines pipeline concepts used throughout"
    unlocks:
      - id: runner-internals
    related:
      - id: async-execution-model

# Subdirectory pointers
subdirectories:
  - path: decisions/
    description: "Architecture Decision Records"
    index: Tech/Architecture/decisions/_index.yaml
```

### 5.3 Key Fields

| Field | Required | Purpose |
|-------|----------|---------|
| `_schema` | Yes | Schema version identifier |
| `directory.path` | Yes | Path relative to `docs/` |
| `directory.audience` | Yes | Default audience for this directory |
| `directory.parent` | Yes | Parent `_index.yaml` path (null at root) |
| `files[].id` | Yes | Unique doc ID for cross-referencing |
| `files[].sections[]` | No | Section-level metadata for partial loading |
| `files[].sections[].line_range` | Yes (if section) | `[start, end]` line numbers for selective reads |
| `prereqs[].reason` | Yes (if prereq) | Why this dependency exists |
| `subdirectories[]` | No | Child directory pointers |

### 5.4 Why Section-Level `line_range`?

AI tools can use line ranges to request partial file reads (e.g., read lines 46-120), avoiding loading entire documents. Anchors alone require parsing the full file to find the section.

### 5.5 Maintaining Indexes

- Update `_index.yaml` when adding, removing, or restructuring files
- Run `scripts/doc-reindex.py` to generate skeleton entries for new files
- The pre-commit hook validates index consistency

---

## 6. Smart Loading Protocol

### 6.1 For AI Tools

When searching for relevant documentation:

1. Start at `docs/_index.yaml` (root index)
2. Read the root index to understand top-level structure
3. Based on query, choose the relevant audience directory
4. Read that directory's `_index.yaml`
5. Scan `files[].keywords` and `files[].summary` for relevance
6. Check `size_kb` to decide loading strategy:

| File Size | Strategy |
|-----------|----------|
| <= 10 KB | Load full file |
| <= 30 KB | Full file or sections based on query specificity |
| > 30 KB | Load only matching section(s) by `line_range` |
| Hub document | Always load full (small by design) |

7. Follow `prereqs` only if the current query requires foundational context
8. Follow `related` only if initial load does not answer the query

### 6.2 Section-Level Loading Example

```yaml
sections:
  - anchor: priority-algorithm
    line_range: [46, 120]
    summary: "How pipeline priority is calculated"
```

To load only this section: read `queue-manager.md` lines 46-120 (using `offset` and `limit` parameters), not the full 34 KB file.

---

## 7. Syntax Notation Conventions

### 7.1 EBNF (Extended Backus-Naur Form)

Use EBNF for all formal grammar definitions. Fence with ` ```ebnf `:

```ebnf
pipeline_def  = "[" , marker , "]" , identifier , body ;
marker        = "r" | "p" | "s" ;
identifier    = letter , { letter | digit | "_" } ;
body          = INDENT , { statement , NEWLINE } , DEDENT ;
```

### 7.2 Railroad Diagrams

Use text-based railroad diagrams for visual grammar representation:

```
pipeline_def:
  ┌───┐   ┌────────┐   ┌───┐   ┌────────────┐   ┌──────┐
──┤ [ ├───┤ marker ├───┤ ] ├───┤ identifier ├───┤ body ├──
  └───┘   └────────┘   └───┘   └────────────┘   └──────┘
               │
         ┌─────┴─────┐
         │ "r" │ "p" │
         │     "s"   │
         └───────────┘
```

### 7.3 When to Use Each

- **EBNF**: Always included in language spec sections. Machine-readable, precise.
- **Railroad diagrams**: Included alongside EBNF for visual learners. Human-readable, intuitive.
- Both are required in the `language-spec-section` template.

---

## 8. Document Templates

Templates live in `docs/_templates/`. Every new document must start from the appropriate template.

| Template | Audience | When to Use |
|----------|----------|-------------|
| `user-feature-guide.md` | user | Documenting a language feature for end users |
| `user-tutorial.md` | user | Step-by-step learning content |
| `tech-architecture.md` | developer | System/component architecture documentation |
| `tech-spec.md` | developer | Technical specifications |
| `language-spec-section.md` | developer | Language feature specification (EBNF + railroad) |
| `audit-record.md` | developer | Documentation audit findings |
| `decision-record.md` | developer | Technical or design decisions |

See `docs/_templates/` for full template contents.

---

## 9. Validation

### 9.1 Pre-Commit Checks

The script `scripts/doc-validate.py` runs on every commit touching `docs/`. It checks:

1. **50 KB limit** -- rejects files exceeding 51,200 bytes
2. **Frontmatter presence** -- every `.md` file has required YAML frontmatter
3. **`_index.yaml` schema** -- validates against pydantic models
4. **Cross-reference validity** -- all `prereqs`, `unlocks`, `related` IDs exist
5. **Bidirectional links** -- prereqs <-> unlocks consistency
6. **Unique IDs** -- no duplicate `id` values across all indexes
7. **Section `line_range` validity** -- referenced lines exist in the file

**Legacy files** (pre-existing without frontmatter) emit warnings, not errors. They must comply when modified.

### 9.2 Validation Scripts

| Script | Purpose |
|--------|---------|
| `scripts/doc-validate.py` | Pre-commit validation (run automatically) |
| `scripts/doc-reindex.py` | Generate/update `_index.yaml` from directory contents |

### 9.3 Running Validation

```bash
# Validate all docs
python scripts/doc-validate.py

# Validate specific directory
python scripts/doc-validate.py docs/Tech/

# Generate skeleton index for a directory
python scripts/doc-reindex.py docs/Tech/Architecture/
```

---

## 10. Editing Guidelines

### 10.1 On Every Edit

1. Update `updated` field in frontmatter to today's date
2. Review `status` field (draft -> review -> stable)
3. Check cross-references still valid
4. Update `_index.yaml` if sections changed (headings, line numbers)

### 10.2 On New File Creation

1. Choose the correct template from `docs/_templates/`
2. Place in the correct directory (see Section 2.3)
3. Fill all required frontmatter fields
4. Add entry to the directory's `_index.yaml`
5. Establish cross-references (prereqs, unlocks, related)
6. Ensure bidirectional links in referenced files' indexes

### 10.3 On File Deletion or Move

1. Update all incoming cross-references
2. Remove entry from old `_index.yaml`, add to new
3. If superseded, mark old file as `status: deprecated` or archive it

---

## 11. Archive Procedure

### 11.1 When to Archive

- Content superseded by newer documentation
- Historical context only (old design decisions)
- Version-specific content no longer applicable

### 11.2 Process

1. Create tarball: `docs-archive-{name}-{date}.tar.gz`
2. Remove archived directory from `docs/`
3. Update `_index.yaml` to remove entries
4. Update any cross-references pointing to archived content

---

## 12. Migration Policy

**New files** created after 2026-03-12 must fully comply with these conventions.

**Existing files** (pre-2026-03-12) are exempt until they are modified. When an existing file is edited:
1. Add required frontmatter
2. Add entry to directory's `_index.yaml`
3. Ensure file is < 50 KB (split if needed)

This ensures gradual migration without requiring a bulk rewrite.

---

*Replaces: BMAD Documentation Conventions v1.0 (2025-12-16)*
*Schema: polyglot-doc-conventions-v2*
