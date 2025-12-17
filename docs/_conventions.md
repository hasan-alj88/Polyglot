# BMAD Documentation Conventions - Polyglot v0.0.4

**Version:** 1.0
**Schema:** bmad-conventions-v1
**Last Updated:** 2025-12-16
**Status:** ✅ Active Standard

---

## Purpose

This document defines standards for maintaining BMAD-optimized documentation in the Polyglot v0.0.4 specification. All contributors must follow these conventions to ensure consistent BMAD agent routing and navigation.

---

## Folder Taxonomy Rules

### Primary Structure

```
v0.0.4/
├── getting-started/    # Entry-level content, complexity: low
├── language/          # Core language specs, complexity: low-high
│   ├── syntax/        # Syntax reference, complexity: medium
│   ├── types/         # Type system specs, complexity: medium
│   ├── control-flow/  # Execution model, complexity: medium
│   └── advanced/      # Advanced features, complexity: high
├── stdlib/            # Standard library API, complexity: low-medium
│   ├── loops/         # Pack/unpack operators
│   ├── utilities/     # Data, datetime, math, string
│   └── wrappers/      # Runtime wrappers
├── guides/            # How-to guides, complexity: low-medium
├── reference/         # Grammar, AI context, complexity: medium
└── _archive/          # Historical content (never deleted)
```

### Folder Creation Rules

**When to create new folders:**
- Concept groups with 5+ related documents
- Distinct complexity levels requiring separation
- Different agent target audiences

**Naming conventions:**
- Use kebab-case: `loop-system.md`, `serial-load-block.md`
- Descriptive, not abbreviated: `datetime/` not `dt/`
- Plural for collections: `utilities/`, `wrappers/`
- Singular for concepts: `syntax/`, `reference/`

### Subdivision Criteria

Create subfolder when:
- ✅ 5+ files share common concept
- ✅ Natural hierarchy exists (parent/child docs)
- ✅ Different agent routing needed

Don't create subfolder when:
- ❌ Only 2-3 files in concept
- ❌ No clear parent/child relationship
- ❌ Same agent routing as parent

---

## BMAD YAML Schema Reference

### Required Fields

Every `.md` file **MUST** include YAML front matter with these fields:

```yaml
---
# --- Identity ---
id: unique-kebab-id        # REQUIRED: unique identifier
shard: false               # REQUIRED: true if part of larger doc set

# --- Classification ---
type: spec                 # REQUIRED: spec|guide|reference|tutorial|concept|api|changelog
topic: Brief Topic         # REQUIRED: 2-4 words, ≤40 chars
summary: Brief summary     # REQUIRED: ≤12 words, ≤80 chars
keywords:                  # REQUIRED: 3-6 items for semantic search
  - keyword1
  - keyword2

# --- BMAD Agent Routing ---
agents:                    # REQUIRED: which BMAD agents need this
  - developer
phase: planning            # REQUIRED: analysis|planning|solutioning|implementation|any
workflow: any              # REQUIRED: greenfield|bugfix|feature|refactor|any
module: bmm                # REQUIRED: bmm|bmb|cis|core|any
complexity: medium         # REQUIRED: low|medium|high

# --- Dependency Chain ---
prereqs:                   # REQUIRED: can be empty []
  - doc-id-1
unlocks:                   # REQUIRED: can be empty []
  - doc-id-2

# --- Relationships ---
related:                   # REQUIRED: can be empty []
  - doc-id-a

# --- Metadata ---
status: stable             # REQUIRED: draft|review|stable|deprecated
updated: 2025-12-16        # REQUIRED: YYYY-MM-DD
version: 0.0.4             # REQUIRED: semver if applicable
tags:                      # REQUIRED: 2-5 tags with # prefix
  - "#syntax"
---
```

### Optional Fields

```yaml
# --- Identity (Optional) ---
shard_of: parent-doc-id    # If shard: true, reference parent

# --- Relationships (Optional) ---
parent: section-index-id   # Parent doc/section
children:                  # Sub-documents if split
  - child-id-1
```

### Field Definitions

| Field | Type | Constraints | Purpose |
|-------|------|-------------|---------|
| `id` | string | kebab-case, unique, ≤40 chars | Document identifier for linking |
| `shard` | boolean | true/false | Part of larger doc set? |
| `shard_of` | string | valid doc ID | Parent document if shard: true |
| `type` | enum | spec, guide, reference, tutorial, concept, api, changelog | Document classification |
| `topic` | string | 2-4 words, ≤40 chars | Quick scan label |
| `summary` | string | ≤12 words, ≤80 chars | Context load summary |
| `keywords` | list | 3-6 items | Semantic search |
| `agents` | list | developer, architect, pm, analyst, ux-designer, tech-writer, scrum-master, game-*, test-* | Target agents |
| `phase` | enum | analysis, planning, solutioning, implementation, any | Workflow stage |
| `workflow` | enum | greenfield, bugfix, feature, refactor, any | Project type |
| `module` | enum | bmm, bmb, cis, core, any | BMAD module |
| `complexity` | enum | low, medium, high | Planning depth hint |
| `prereqs` | list | valid doc IDs or [] | Read-first dependencies |
| `unlocks` | list | valid doc IDs or [] | Enabled documents |
| `related` | list | valid doc IDs or [] | Conceptually linked |
| `parent` | string | valid doc ID | Parent document |
| `children` | list | valid doc IDs | Sub-documents |
| `status` | enum | draft, review, stable, deprecated | Document status |
| `updated` | date | YYYY-MM-DD | Last update date |
| `version` | string | semver or version tag | Document version |
| `tags` | list | 2-5 tags with # prefix | Categorical tags |

### Validation Rules

**On file save:**
- ✅ All required fields present
- ✅ YAML parses without errors
- ✅ `id` is unique across all docs
- ✅ `prereqs`, `unlocks`, `related` reference valid IDs
- ✅ `agents`, `phase`, `workflow`, `module`, `complexity`, `status`, `type` use valid enums
- ✅ `topic` ≤ 40 chars, `summary` ≤ 80 chars
- ✅ `keywords` has 3-6 items
- ✅ `tags` has 2-5 items with # prefix
- ✅ If `shard: true`, `shard_of` is present

---

## BMAD Agent Routing Logic

### Agent Type Mapping

| Agent | Primary Docs | Secondary Docs | Phase | Complexity |
|-------|--------------|----------------|-------|------------|
| **architect** | spec, api, reference | concept | solutioning | high |
| **developer** | guide, tutorial, reference, api | spec | implementation | low-medium |
| **pm** | concept, spec | guide | planning | low |
| **analyst** | concept, reference | spec | analysis | low |
| **ux-designer** | guide, concept | tutorial | planning | low-medium |
| **tech-writer** | guide, tutorial | reference | any | low-medium |
| **scrum-master** | concept, guide | any | any | low |
| **game-architect** | spec, reference | concept | solutioning | high |
| **game-designer** | concept, guide | tutorial | planning | medium |
| **test-architect** | reference, guide, spec | api | solutioning | medium-high |

### Priority Rules

When multiple agents match a document:

1. **Primary match:** `agents` field lists agent explicitly
2. **Type match:** Document `type` matches agent's primary types
3. **Phase match:** Document `phase` matches current workflow phase
4. **Complexity match:** Document `complexity` matches agent's typical depth

**Example:**
```yaml
agents: [developer, architect]
type: spec
phase: solutioning
complexity: high
```

**Routing:** 
- Architect (primary) - matches all criteria
- Developer (secondary) - matches `agents` but complexity mismatch

---

## Linking Protocol

### When to Use prereqs vs related

**Use `prereqs`** when:
- Document B cannot be understood without reading Document A first
- Document A defines concepts/terms used in Document B
- Linear dependency chain exists

**Use `related`** when:
- Documents cover complementary topics
- Reader may benefit from cross-reference
- No strict reading order required

**Example:**

```yaml
# markers.md
unlocks:
  - pipeline-structure
  - control-flow

# pipeline-structure.md
prereqs:
  - markers
  - operators
  - prefix-system
related:
  - control-flow
  - loop-system
```

### Bidirectional Linking Rules

**Rule:** If A links to B, B should link back to A.

```yaml
# File A
unlocks: [B]

# File B (must have)
prereqs: [A]
```

```yaml
# File A
related: [B]

# File B (must have)
related: [A]
```

**Verification:**
```bash
# Check bidirectional links
./scripts/verify_bidirectional_links.sh
```

### `unlocks` Chain Maintenance

**Purpose:** Show what knowledge becomes accessible after reading this doc.

**Rules:**
- Only list immediate unlocks, not transitive
- Include documents that **require** this as prereq
- Keep synchronized with other docs' `prereqs`

**Example:**

```yaml
# core-principles.md
unlocks:
  - markers
  - prefix-system
  - type-system

# markers.md
prereqs:
  - core-principles  # ← Must match!

# prefix-system.md
prereqs:
  - core-principles  # ← Must match!
```

---

## Archive Procedure

### Criteria for Archiving

Archive a document when:
- ✅ Superseded by newer documentation
- ✅ Historical context only (design decisions)
- ✅ Version-specific content no longer applicable
- ✅ Organizational/planning docs (complete)

**Do NOT archive:**
- ❌ Current specifications
- ❌ Active feature documentation
- ❌ Referenced API docs
- ❌ Guides still in use

### Archive Process

1. **Determine archive category:**
   - `_archive/design-history/` - Historical decisions
   - `_archive/old-files/` - Superseded specifications
   - `_archive/meta/` - Organizational documents

2. **Preserve original path:**
   ```
   Original: /core-syntax/old-spec.md
   Archive:  /_archive/old-files/core-syntax/old-spec.md
   ```

3. **Add archival metadata header:**
   ```markdown
   <!-- ARCHIVED: 2025-12-16 | Reason: Superseded by v0.0.4 spec | Superseded by: /language/syntax/markers.md -->
   ```

4. **Update links:**
   - Find all references to archived doc
   - Update to point to superseding document
   - Or mark as "historical reference"

5. **Update _graph.yaml:**
   - Remove from active indexes
   - Add to archived section if needed

### Metadata Requirements

Every archived file must have:

```markdown
<!-- 
ARCHIVED: YYYY-MM-DD 
Reason: Brief reason for archiving
Superseded by: /path/to/new/doc.md (or N/A)
-->
```

---

## Ongoing Edit Conventions

### On Every Edit

When modifying any `.md` file:

1. **Update `updated` field:**
   ```yaml
   updated: 2025-12-16  # ← Update to today
   ```

2. **Review `status` field:**
   ```yaml
   status: stable  # draft → review → stable
   ```

3. **Check relationships:**
   - Are `prereqs` still valid?
   - Should `unlocks` be updated?
   - New `related` docs to add?

4. **Update affected indexes:**
   - Folder `index.md` if structure changed
   - `_tags.md` if tags changed
   - `_graph.yaml` if routing changed

### On New File Creation

When creating a new `.md` file:

1. **Determine correct folder:**
   - Follow taxonomy rules (see above)
   - Check complexity level
   - Verify agent routing

2. **Generate BMAD YAML:**
   - Use template from this doc
   - Fill all required fields
   - Ensure `id` is unique

3. **Establish relationships:**
   - Define `prereqs` (what must be read first?)
   - Define `unlocks` (what does this enable?)
   - Define `related` (what's complementary?)

4. **Update bidirectional links:**
   - If A prereqs B, add A to B's `unlocks`
   - If A related to B, add A to B's `related`

5. **Update navigation files:**
   - Add to folder `index.md`
   - Add to `_tags.md` under appropriate tags
   - Add to `_graph.yaml` indexes

6. **Add breadcrumb navigation:**
   ```markdown
   > [Home](../index.md) / [Language](./index.md) / Markers
   ```

---

## Token Budget Guidelines

### Per-Field Limits

| Field | Max Tokens | Rationale |
|-------|------------|-----------|
| `id` | 4 | Unique reference |
| `topic` | 6 | Quick scan |
| `summary` | 15 | Context load |
| `keywords` | 12 | Semantic search |
| `agents` | 6 | Routing |
| **Total Header** | **~50** | vs ~2000 reading full doc |

### Optimization Tips

**For `topic`:**
- ✅ "Polyglot Loop System"
- ❌ "Complete comprehensive guide to the loop system in Polyglot language"

**For `summary`:**
- ✅ "Unpack and pack operators for iteration control"
- ❌ "This document provides a comprehensive overview of the unpack and pack operator system which allows developers to control iteration and collection operations in loops"

**For `keywords`:**
- ✅ `[loops, iteration, unpack, pack]`
- ❌ `[comprehensive loops guide, how to iterate, unpack operator documentation, pack operator reference]`

---

## Quality Checklist

### Before Commit

- [ ] YAML front matter present and valid
- [ ] All required fields filled
- [ ] `id` is unique
- [ ] `updated` date is current
- [ ] Bidirectional links verified
- [ ] Folder `index.md` updated
- [ ] `_tags.md` updated if tags changed
- [ ] `_graph.yaml` updated if routing changed
- [ ] Breadcrumb navigation present
- [ ] Related docs section at bottom
- [ ] No broken links (`./verify_links.sh`)
- [ ] YAML parses (`./validate_yaml.sh`)

### Verification Scripts

```bash
# Validate all YAML front matter
./scripts/validate_yaml.sh

# Check bidirectional links
./scripts/verify_bidirectional_links.sh

# Find broken links
./scripts/verify_links.sh

# Check unique IDs
./scripts/verify_unique_ids.sh
```

---

## Version Control

### Commit Message Format

```
type(scope): Brief description

- Detail 1
- Detail 2

BMAD: [routing-changed|structure-changed|content-only]
```

**Types:**
- `feat` - New document
- `fix` - Correction
- `docs` - Documentation update
- `refactor` - Reorganization
- `archive` - Archiving content

**Examples:**
```
feat(language): Add error-handling.md

- New advanced feature documentation
- Added agent routing for architect
- Updated loop-system.md prereqs

BMAD: routing-changed
```

```
fix(stdlib): Correct foreach-array.md complexity

- Changed complexity: high → medium
- Updated _graph.yaml

BMAD: routing-changed
```

---

## Contact & Support

**Issues:** Report documentation issues at project repository
**Questions:** Consult [Main Index](./index.md) or [Navigation Graph](./_graph.yaml)
**Updates:** Check this file for convention changes

---

**Last Updated:** 2025-12-16
**Schema:** bmad-conventions-v1
**Status:** ✅ Active Standard
