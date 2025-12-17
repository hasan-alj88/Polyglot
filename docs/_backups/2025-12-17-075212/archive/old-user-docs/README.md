# Archived Polyglot v0.0.2 User Documentation

**Archive Date:** December 2025
**Archived Version:** v0.0.2 (superseded by v0.0.3+)
**Current Stable Version:** See [/docs/user/](/docs/user/) for v0.0.3+ documentation

---

## ⚠️ Important: These Are Historical Archives

This folder contains **archived documentation for Polyglot v0.0.2**, which has been superseded by v0.0.3 (current stable) and v0.0.4 (finalized December 2025).

**Do NOT use this documentation for current development.** These files are preserved for:
- Historical reference of language evolution
- Understanding syntax design decisions
- Tracking how v0.0.2 evolved into v0.0.3/v0.0.4

---

## Archive Structure

This archive contains **TWO iterations** of v0.0.2 documentation, showing how syntax evolved during v0.0.2 development:

### 📁 `user-v0.0.2-original/` - Earlier Iteration
**Date:** ~November 2025 (earlier)
**Syntax Characteristics:**
- Uses `Fixed` keyword for fixed inputs: `[i] Fixed .api_key: pg\string`
- Uses `Default` keyword for default values: `[i] Default .timeout: pg\int`
- Numbered file prefixes: `00-overview.md`, `01-database-schema.md`, `02-type-system.md`
- Maps still being considered (later removed)

**Why "original"?** This is the earlier/original version of v0.0.2 docs, preserved to show initial syntax before revisions.

**File Count:** 66 markdown files

### 📁 `user-v0.0.2-revised/` - Later Iteration
**Date:** ~November-December 2025 (later)
**Syntax Characteristics:**
- Dropped `Fixed`/`Default` keywords
- Uses operators: `<<` for fixed, `<~` for default with value
- Plain file names: `overview.md`, `type-system.md`
- Maps removed from specification
- Cleaner operator-based syntax (influenced later versions)

**Why "revised"?** This is the revised/refined version of v0.0.2 showing syntax cleanup.

**File Count:** 67 markdown files

---

## Key Syntax Differences Between Iterations

### Input Parameter Syntax Evolution

**Original (user-v0.0.2-original):**
```polyglot
[i] Fixed .api_key: pg\string << "secret-123"
[i] Default .timeout: pg\int << 30
[i] Required .url: pg\string
```

**Revised (user-v0.0.2-revised):**
```polyglot
[i] .api_key: pg\string << "secret-123"     // Fixed (<<)
[i] .timeout: pg\int <~ 30                  // Default (<~)
[i] .url: pg\string                         // Required
```

**Evolution Impact:**
- Removed keyword clutter (`Fixed`, `Default`, `Required`)
- Made operators convey meaning (`<<` = fixed, `<~` = default)
- This pattern influenced v0.0.3+ syntax refinement

### Type System Changes

**Original:**
- Considered `pg.mutable\type` for mutable types
- Path separator discussion (`/` vs `\` vs `.`)

**Revised:**
- Settled on `pg\type` for regular types
- Removed mutable type complexity for v0.0.2

---

## Relationship to Current Versions

### v0.0.2 → v0.0.3 (Current Stable)
**Major Changes from v0.0.2:**
- Variable prefix remained: `,name`
- Block markers remained: `[|]...[X]`
- Parser implementation started (Epic 1)
- Multi-file compilation added
- Error handling refined

### v0.0.2 → v0.0.4 (Finalized December 2025)
**Major Syntax Overhaul:**
- Variables: `,name` → **`$name`**
- Blocks: `[|]...[X]` → **`{|}...{x}`**
- IO markers: `[i]`/`[o]` → **`[|] <param`** / **`[|] >param`**
- Types: `pg\string` → **`:pg.string`**
- Reserved enums: `#True` → **`#;Boolean;True`**
- Indentation: Removed `\~\` markers, use 3 spaces

**See:** [/docs/specifications/VERSION-INDEX.md](/docs/specifications/VERSION-INDEX.md) for complete version history

---

## Why Keep Both Iterations?

**Historical Value:**
1. **Design Evolution:** Shows how syntax decisions evolved within a single version
2. **Operator Philosophy:** Demonstrates shift from keywords to operators
3. **Lessons Learned:** Documents what worked and what didn't
4. **Decision Context:** Provides context for current syntax choices

**Academic Interest:**
- Language design case study
- Syntax refinement patterns
- Documentation evolution tracking

---

## What to Use Instead

**For Current Development:**
- **User Documentation:** [/docs/user/](/docs/user/) (v0.0.3 stable)
- **Latest Specification:** [/docs/specifications/v0.0.4/](/docs/specifications/v0.0.4/)
- **Implementation Guide:** [/docs/project/prd.md](/docs/project/prd.md) and [/docs/project/epics.md](/docs/project/epics.md)

**For AI Assistants:**
- **AI Context v0.0.4:** [/docs/ai-context/v0.0.4/](/docs/ai-context/v0.0.4/) *(to be created)*
- **NOT this archive** - v0.0.2 is obsolete

---

## Archive Maintenance

**Compression Consideration:**
- Current size: 2.7MB (133 files)
- If space becomes an issue, consider compressing to `user-docs-v0.0.2-archive.tar.gz`
- Extract command: `tar -xzf user-docs-v0.0.2-archive.tar.gz`

**Retention Policy:**
- Keep indefinitely for historical reference
- Do not update or modify (frozen)
- If moving to compressed format, update this README with extraction instructions

---

## Quick Reference: File Organization

### user-v0.0.2-original/ Structure
```
architecture/     - 00-overview.md, 01-database-schema.md, etc.
audit/            - decision-log.md, inconsistencies-log.md
cli/              - 00-workflow.md, 01-compile.md, 02-register.md
examples/         - Code examples for v0.0.2
guides/           - User guides and tutorials
language/         - Language specification (bnf/, types, operators)
packages/         - Package system docs
planning/         - Project planning documents
quick-reference/  - Quick syntax reference
standard-library/ - Standard library documentation
```

### user-v0.0.2-revised/ Structure
```
Same structure as original, but:
- Files use plain names (no numbered prefixes)
- Content reflects revised syntax
- Some content updates/refinements
```

---

## Access and Usage

**To reference historical syntax:**
1. Identify which iteration you need (original vs revised)
2. Navigate to appropriate subfolder
3. Check README.md in that folder for structure
4. Use for historical comparison only

**Citation Format (if needed):**
```
Polyglot v0.0.2 Documentation (Archived)
Iteration: [Original | Revised]
Path: /docs/archive/old-user-docs/user-v0.0.2-[original|revised]/
Archive Date: December 2025
Status: Superseded by v0.0.3+
```

---

**Last Updated:** 2025-12-14
**Maintained By:** Archive (read-only, no updates)

**For Questions:** See current documentation at [/docs/user/](/docs/user/) or [/docs/specifications/](/docs/specifications/)
