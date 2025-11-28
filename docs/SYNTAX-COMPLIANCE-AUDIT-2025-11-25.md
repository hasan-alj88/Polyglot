# Polyglot Syntax Compliance Audit

**Date:** 2025-11-25
**Auditor:** PM Agent (John)
**Scope:** All .pg files and Polyglot code examples in documentation
**Standard:** v0.0.2 syntax

---

## Executive Summary

**Status:** 🔴 **CRITICAL VIOLATIONS FOUND**

**Total Violations:** 60+ syntax errors across 7 files
**Severity:** High - Non-compliant code will fail lexer/parser

**Primary Issues:**
1. 🔴 **56+ occurrences** - Wrong reserved enumeration name (#Variables.States → #PgVar.States)
2. 🔴 **2 occurrences** - Missing operator prefix on triggers (T.Cron → |T.Cron)
3. ⚠️ **6+ occurrences** - Maps still present (removed in v0.0.2)

---

## Issue 1: Wrong Reserved Enumeration Name (CRITICAL)

### Problem
Using `#Variables.States.*` instead of correct `#PgVar.States.*`

### Impact
- Lexer will not recognize #Variables enumeration
- Parser will fail with "undefined enumeration" error
- All state checking code will break

### Affected Files (56+ violations)

**1. docs/variable-states-advanced-examples.pg (46 violations)**
Lines: 50, 55, 91, 97, 144, 187, 192, 197, 236, 283, 288, 331, 378, 379, 380, 389, 444, 448, 453, 457, 465, 584, 589, 625, 630, ...

**2. docs/variable-states-examples.pg (9 violations)**
Lines: 44, 54, 171, 172, 173, 215, 272, 276, 281

**3. test-file-processing-pipeline.pg (11 violations)**
Lines: 15 (comment), 110, 125, 149, 168, 191, 247, 257, 282, 291, 302, 343, 368

**4. test-state-aware-code.pg (7 violations)**
Lines: 65, 77, 95, 107, 125, 167, 180

### Correct Syntax
```polyglot
# ❌ WRONG (old syntax):
[?] .variable.state =? #Variables.States.Ready

# ✅ CORRECT (v0.0.2):
[?] .variable.state =? #PgVar.States.Ready
```

### Reserved Enumeration Name
Per v0.0.2 specification:
- **Correct:** `#PgVar.States.*` (shortened from "Polyglot Variable")
- **States:** Declared, DefaultReady, Pending, Ready, Faulted, Retrying, Paused, Cached, Dirty

### Fix Strategy
Global find-replace:
- `#Variables.States.Ready` → `#PgVar.States.Ready`
- `#Variables.States.Faulted` → `#PgVar.States.Faulted`
- `#Variables.States.Pending` → `#PgVar.States.Pending`
- `#Variables.States.Declared` → `#PgVar.States.Declared`
- `#Variables.States.DefaultReady` → `#PgVar.States.DefaultReady`
- `#Variables.States.Retrying` → `#PgVar.States.Retrying`
- `#Variables.States.Paused` → `#PgVar.States.Paused`
- `#Variables.States.Cached` → `#PgVar.States.Cached`
- `#Variables.States.Dirty` → `#PgVar.States.Dirty`

---

## Issue 2: Missing Operator Prefix on Triggers (CRITICAL)

### Problem
Triggers missing `|` prefix: `T.Cron` instead of `|T.Cron`

### Impact
- Violates "no keywords" rule (ALL identifiers need operator prefix)
- Lexer will fail to parse trigger declarations
- Pipeline activation will fail

### Affected Files (2 violations)

**1. test-file-processing-pipeline.pg**
```polyglot
# Line 97
[t] T.Cron"0 2 * * *"        # ❌ WRONG

# Should be:
[t] |T.Cron"0 2 * * *"       # ✅ CORRECT
```

**2. test-state-aware-code.pg**
```polyglot
# Line 52
[t] T.Cron"0 */6 * * *"      # ❌ WRONG

# Should be:
[t] |T.Cron"0 */6 * * *"     # ✅ CORRECT
```

### Correct Syntax
```polyglot
[t] |T.Cron"0 2 * * *"       # Daily at 2 AM
[t] |T.Daily                 # Daily trigger (simple)
[t] |T.OnFileCreate          # Event trigger
```

### Fix Strategy
Add `|` prefix to all trigger references in `[t]` blocks.

---

## Issue 3: Maps Present (Removed in v0.0.2)

### Problem
Code uses `pg\map{K,V}` which was removed in v0.0.2

### Impact
- Maps are no longer valid types
- Use `pg\serial` (dynamic key-value) or enumerations instead
- Type system will reject map types

### Affected Files (6+ occurrences)

**1. docs/variable-states-advanced-examples.pg**
```polyglot
# Line 430
[<] .metadata: pg\map{pg\string,pg\string} << {}   # ❌ WRONG

# Should be:
[<] .metadata: pg\serial << {}                      # ✅ CORRECT
```

**2. docs/technical/variable-states-specification.md**
```polyglot
# Line 354
[<] .context: pg\map{pg\string,pg\string}           # ❌ WRONG
```

**3. docs/technical/architecture.md (multiple occurrences)**
- Lines: 2331, 2344, 2352, 2396, 2424, 2437, 2438
- These are in the Collection Types section explaining the constraint

**Action:** 
- Update architecture.md to clarify maps are removed
- Fix code examples to use `pg\serial` or enumerations

---

## Issue 4: Documentation in Markdown Files

### Files to Check for Code Blocks
Need to scan markdown files for ````polyglot` blocks:

**User Documentation:**
- docs/user/language/*.md
- docs/user/examples/*.md
- docs/user/standard-library/*.md

**Technical Documentation:**
- docs/technical/architecture.md ✅ (has map references to update)
- docs/technical/variable-states-specification.md ✅ (has violations)

**Project Documentation:**
- docs/project/agent-sessions/*.md
- Archived brainstorming sessions (in archive/)

---

## Compliance Checklist

### Reserved Enumerations (v0.0.2)
✅ **Correct Names:**
- `#PgVar.States.*` (NOT #Variables.States)
- `#Boolean.True` / `#Boolean.False` (NOT #Bool)
- `#None` (unit type)

### Operator Prefixes (Required for ALL identifiers)
- ✅ Variables: `.variable_name`
- ✅ Enumerations: `#EnumName`
- ✅ Pipelines: `|PipelineName`
- ✅ Triggers: `|T.TriggerName`
- ✅ Errors: `!ErrorType`

### Type System (v0.0.2)
- ✅ Type separator: `\` (backslash) - `pg\string`, `pg\int`
- ❌ Maps removed: Use `pg\serial` or enumerations instead
- ✅ Collections: `pg\array{T}`, `pg\set{T}` (no nested collections directly)

### Assignment Operators
- ✅ Schema-only (Declared): `[<] .var: Type`
- ✅ Default (DefaultReady): `[<] .var: Type <~ default`
- ✅ Constant (Ready): `[<] .var: Type << value`
- ✅ Push: `[<] .dest << .source`
- ✅ Pull: `[>] .source >> .dest`

---

## Fix Priority

### P0 - Critical (Block Implementation)
1. ✅ **#Variables.States → #PgVar.States** (56+ occurrences, 4 files)
2. ✅ **Add | prefix to triggers** (2 occurrences, 2 files)

### P1 - High (Documentation Accuracy)
3. ⚠️ **Remove map references** (6+ occurrences, 3 files)
4. ⚠️ **Scan markdown files** for code blocks with violations

### P2 - Medium (Cleanup)
5. ⏳ **Update archived brainstorming** sessions (if they contain code)
6. ⏳ **Validate lexer-validation-samples/** (15 files)

---

## Recommended Actions

### Immediate (Today)
1. **Fix P0 violations** in all 4 .pg files (56+ fixes)
2. **Update architecture.md** map references
3. **Fix variable-states-specification.md** map usage

### This Week
4. **Scan all markdown files** for polyglot code blocks
5. **Validate lexer-validation-samples** folder (15 test files)
6. **Update examples in user docs** (examples/*.md)

### Optional
7. Review archived brainstorming sessions for old syntax
8. Create automated syntax validator script

---

## Files Requiring Fixes

### High Priority (.pg files with violations)
1. ✅ `docs/variable-states-advanced-examples.pg` - 46+ violations
2. ✅ `docs/variable-states-examples.pg` - 9 violations
3. ✅ `test-file-processing-pipeline.pg` - 12 violations
4. ✅ `test-state-aware-code.pg` - 8 violations

### Medium Priority (documentation files)
5. ⚠️ `docs/technical/variable-states-specification.md` - 1+ violations
6. ⚠️ `docs/technical/architecture.md` - Map references (clarification needed)

### Low Priority (validation samples - need review)
7. ⏳ `docs/project/lexer-validation-samples/*.pg` (15 files)

---

## Next Steps

**User Decision Required:**

1. **Approve automatic fix** for #Variables.States → #PgVar.States (56+ occurrences)?
2. **Approve automatic fix** for trigger prefixes (2 occurrences)?
3. **Review map strategy:** Remove entirely or document as "removed in v0.0.2"?
4. **Scan markdown files** now or defer?

**Estimated Fix Time:**
- P0 fixes: ~15 minutes (automated find-replace)
- P1 fixes: ~30 minutes (manual review)
- Complete audit: ~2 hours

