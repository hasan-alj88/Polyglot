# Documentation Reorganization Plan
**Date:** 2025-12-24
**Planner:** Scribe Documentation Architect
**Scope:** Restructure docs/ to match canonical Scribe architecture

---

## Target Structure

```
docs/
├── Agile/          # Project management, PRDs, sprints, architecture decisions
├── User/           # User-facing documentation (language users/programmers)
├── Tech/           # Developer documentation (Polyglot implementers/contributors)
├── Audit/          # Audit history and validation reports (keep as-is)
├── archive/        # Historical/deprecated content (keep as-is)
└── INDEX.md        # Master index
```

---

## Current State Analysis

**Target directories exist but are EMPTY:**
- `docs/Agile/` - 0 files
- `docs/Tech/` - 0 files
- `docs/Users/` - 0 files (note: plural, should be singular)

**Content is scattered across 14 directories:**
- `project/` - 16 MD files (PRDs, epics, stories, tickets)
- `technical/` - 16 MD files (architecture, implementation specs)
- `specifications/` - 4 MD files + 3 subdirs (v0.0.4 spec)
- `stdlib/` - 2 MD files + 3 subdirs (standard library docs)
- `language/` - 1 MD file + 4 subdirs (language syntax)
- `examples/` - 0 MD + 1 subdir (code examples)
- `getting-started/` - 1 MD file
- `reference/` - 2 MD files
- `ai-context/` - 1 MD + 1 subdir
- `Architecture/` - 0 files (empty)
- `_patches/` - 0 MD + 2 subdirs

---

## Reorganization Mapping

### → Agile/ (Project Management & Planning)

**Move from `project/`:**
- All PRD, epic, story, and ticket files
- Brainstorming backlogs
- Technology stack decisions
- Change requests

**Move from `Architecture/` (merge into Agile):**
- All architecture decision records
- System design documents

**Move from `technical/` (architectural subset):**
- `architecture.md` - Overall system architecture
- Any ADR (Architecture Decision Records)

**Move from `_patches/`:**
- Patch documentation and version updates

**Expected structure:**
```
Agile/
├── prds/           # Product Requirements Documents
├── epics/          # Epic definitions
├── stories/        # User stories and tickets
├── sprints/        # Sprint planning and retrospectives
├── architecture/   # Architecture decisions and designs
├── tech-stack/     # Technology decisions
└── patches/        # Version patches and updates
```

---

### → User/ (Language Users/Programmers)

**Move from `language/`:**
- All language syntax documentation
- Control flow, types, operators, etc.

**Move from `stdlib/`:**
- All standard library documentation
- Utilities, loops, wrappers

**Move from `specifications/`:**
- v0.0.4 specification (the canonical language spec)
- Design history and decisions

**Move from `examples/`:**
- All code examples
- Tutorial examples

**Move from `getting-started/`:**
- Quick start guides
- Core principles
- Beginner tutorials

**Move from `reference/`:**
- Grammar reference
- AI context (useful for users too)

**Expected structure:**
```
User/
├── getting-started/     # Tutorials, quick starts
├── language/            # Language syntax and semantics
│   ├── syntax/
│   ├── types/
│   ├── control-flow/
│   └── advanced/
├── stdlib/              # Standard library reference
│   ├── loops/
│   ├── utilities/
│   └── wrappers/
├── examples/            # Code examples
├── reference/           # Grammar, quick reference
└── specifications/      # v0.0.4 language specification
```

---

### → Tech/ (Polyglot Implementers/Contributors)

**Move from `technical/`:**
- All implementation-specific documentation
- Parser design, lexer specs
- Internal architecture
- Developer guides

**Move from `ai-context/`:**
- AI context for development
- Correction logs
- Package fixes

**Create new:**
- Contributing guide
- Development setup
- Build system documentation
- Testing documentation

**Expected structure:**
```
Tech/
├── implementation/      # Parser, lexer, compiler internals
├── architecture/        # Internal system architecture
├── development/         # Dev setup, contributing guides
├── testing/            # Test architecture and strategies
└── ai-context/         # AI development context
```

---

## File Movement Plan

### Phase 1: Agile Migration (16 files)

```bash
docs/Agile/                    → docs/Agile/prds/
docs/Agile/epics.md            → docs/Agile/epics/
docs/Agile/stories/            → docs/Agile/stories/
docs/Agile/tickets/            → docs/Agile/tickets/
docs/Agile/examples/           → docs/Agile/reference/
docs/Tech/implementation/technical/architecture.md   → docs/Agile/architecture/
docs/_patches/                   → docs/Agile/patches/
```

### Phase 2: User Migration (~200+ files)

```bash
docs/User/language/                   → docs/User/language/
docs/User/stdlib/                     → docs/User/stdlib/
docs/User/specifications/             → docs/User/specifications/
docs/User/examples/                   → docs/User/examples/
docs/User/getting-started/            → docs/User/getting-started/
docs/User/reference/                  → docs/User/reference/
```

### Phase 3: Tech Migration (15 files)

```bash
docs/Tech/implementation/technical/                  → docs/Tech/implementation/
docs/Tech/ai-context/                 → docs/Tech/ai-context/
```

### Phase 4: Cleanup

```bash
# Remove empty directories
rmdir docs/project
rmdir docs/Architecture
rmdir docs/language
rmdir docs/stdlib
rmdir docs/specifications
rmdir docs/examples
rmdir docs/getting-started
rmdir docs/reference
rmdir docs/technical
rmdir docs/_patches

# Rename Users → User (singular)
mv docs/Users docs/User-temp
# (will be populated from migrations)
rmdir docs/Users
mv docs/User-temp/* docs/User/ (if anything exists)
```

---

## Top-Level Files Reorganization

**Keep at root:**
- `INDEX.md` - Master index (will be regenerated)
- `README.md` - Project documentation entry point

**Move to Agile:**
- `BMAD-REORGANIZATION-COMPLETE.md` → `Agile/reorganization-history/`
- `_conventions.md` → `Agile/conventions/`

**Move to Audit:**
- `AUDIT-INCONSISTENCIES.md` → `Audit/checks/` (already exists there)
- `AUDIT-TODO.md` → `Audit/checks/` (already exists there)
- `_changelog.md` → `Audit/history/`

**Archive:**
- `DOCUMENTATION-HIERARCHY.md` → `archive/pre-reorganization/`
- `INDEX-NEW.md` → `archive/pre-reorganization/`
- `index.md` (lowercase) → `archive/pre-reorganization/`
- `_tags.md` → `archive/pre-reorganization/`

---

## Risk Assessment

### Low Risk
- Moving files within `docs/` (all internal to docs)
- Existing target directories are empty (no conflicts)

### Medium Risk
- **Broken internal links** (already 733 known broken links)
- Cross-references between documents will break

### Mitigation
1. **Pre-backup:** Create backup in `docs/_backups/2025-12-24-reorganization/`
2. **Incremental:** Move in phases (Agile → User → Tech)
3. **Link tracking:** Generate mapping file for link updates
4. **Validation:** Run doc-validate after each phase

---

## Post-Reorganization Tasks

1. **Update all internal links** - Use find/replace with mapping table
2. **Regenerate INDEX.md** - New structure-aware master index
3. **Update README.md** - Reflect new structure
4. **Run doc-validate** - Ensure no new issues introduced
5. **Update .gitignore** - If needed for new structure
6. **Notify team** - Document the new structure

---

## Estimated Impact

**Files to move:** ~250+ markdown files
**Directories to create:** ~15 new subdirectories
**Directories to remove:** ~10 empty directories
**Links to update:** ~733+ broken links (existing) + new breaks from moves

**Time estimate:**
- Reorganization: 30 minutes (automated)
- Link updates: 2-4 hours (semi-automated)
- Validation: 30 minutes
- **Total: 3-5 hours**

---

## Approval Required

This plan requires user approval before execution.

**Questions for user:**
1. Approve the 3-tier structure (Agile/User/Tech)?
2. Approve the directory mapping plan?
3. Should we handle link updates now or separately?
4. Any specific directories that should be organized differently?

---

**Status:** AWAITING APPROVAL
**Next Step:** Present to user for confirmation
