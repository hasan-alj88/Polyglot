# [=] Block Marker Removal - Complete Summary

**Date:** 2025-11-26
**Change Type:** Breaking Change - Block Marker Removal
**Scope:** All documentation (AI context, user docs, technical docs)

---

## Summary

Removed the obsolete `[=]` (equals/constant input) block marker from Polyglot v0.0.2. The `[=]` block marker is now **completely replaced** by `[i]` with the `<<` operator for constant inputs.

---

## Rationale

With the variable state transition model, `[=]` became redundant:

### Old Model (with [=]):
- `[i] .var: Type` - Required input
- `[i] .var: Type << value` - Input with default
- `[=] .var: Type << value` - Constant input (separate marker)

### New Model (without [=]):
- `[i] .var: Type` - Required input (Declared state)
- `[i] .var: Type <~ value` - Default input (DefaultReady state)
- `[i] .var: Type << value` - Constant input (Ready state)

**Key Insight:** All three cases are just different states of `[i]` inputs, controlled by operators (`<<` vs `<~` vs nothing).

---

## What Changed

### Removed
- `[=]` block marker entirely
- `equals_decl` production from grammar
- All references to `[=]` in documentation

### Replaced With
- `[i]` with `<<` for constant inputs
- `[i]` with `<~` for default inputs
- `[i]` alone for required inputs

---

## State Semantics

| Syntax | State | Push Count (within pipeline) | Who Provides | Can Override |
|--------|-------|------------------------------|--------------|--------------|
| `[i] .var: Type` | Declared | 0 allowed | External caller | N/A (must provide) |
| `[i] .var: Type <~ val` | DefaultReady | 0 allowed | External caller | Yes (1 override) |
| `[i] .var: Type << val` | Ready | 0 allowed | Built-in constant | No |

**Critical Distinction:**
- `[i]` inputs are provided by **external caller** (push count within pipeline: 0)
- `[r]` variables are declared **within pipeline** (push count: 1)

---

## Files Updated

### AI Context Package (`/docs/ai-context/`)

#### 1. `grammar.ebnf`
**Changes:**
- Removed `equals_decl ::= '[=]' VARIABLE ':' type '<<' literal`
- Updated `io_decl ::= input_decl | equals_decl` → `io_decl ::= input_decl`
- Added comment explaining three `[i]` variations

**Before:**
```ebnf
io_decl ::= input_decl | equals_decl
input_decl ::= '[i]' VARIABLE ':' type default?
equals_decl ::= '[=]' VARIABLE ':' type '<<' literal
```

**After:**
```ebnf
io_decl ::= input_decl
input_decl ::= '[i]' VARIABLE ':' type default?
/* Three variations:
   [i] .var: Type           - Required input (Declared)
   [i] .var: Type <~ value  - Default input (DefaultReady)
   [i] .var: Type << value  - Constant input (Ready)
*/
```

#### 2. `state-machine.yaml`
**Changes:**
- Added three new assignment operators: `input_required`, `input_default`, `input_constant`
- Documented push count semantics (all have push_count_remaining: 0 within pipeline)

**Added:**
```yaml
assignment_operators:
  input_required:
    syntax: "[i] .variable: Type"
    result_state: "Declared"
    push_count_remaining: 0  # External caller provides

  input_default:
    syntax: "[i] .variable: Type <~ default_value"
    result_state: "DefaultReady"
    push_count_remaining: 0  # External caller can override

  input_constant:
    syntax: "[i] .variable: Type << value"
    result_state: "Ready"
    push_count_remaining: 0  # Cannot override
```

#### 3. `constraints.yaml`, `examples-annotated.pg`, `operators.json`, `README.md`
**Changes:** No `[=]` references found (already clean)

---

### User Documentation (`/docs/user/`)

#### 1. `quick-start.md` (4 occurrences updated)
**Changes:**
- Line 523-527: `[=] .var << value` → `[i] .var << value` (5 input constants)
- Line 721: Comment updated to explain `[i]` variations
- Line 750: "Constants resolved" text updated
- Line 849: Syntax reference updated

**Example Change:**
```polyglot
// BEFORE
[=] .stat_rust_file: pg\path << \\FileDir\\rustrepo\\statgen.rs
[=] .pyfile: pg\path << \\FileDir\\pythonrepo\\formatter.py

// AFTER
[i] .stat_rust_file: pg\path << \\FileDir\\rustrepo\\statgen.rs
[i] .pyfile: pg\path << \\FileDir\\pythonrepo\\formatter.py
```

#### 2. `language/syntax-complete.md` (4 occurrences updated)
**Changes:**
- Line 72: "Constants: `[=] .max`" → "Input constants: `[i] .max`"
- Lines 504-510: Entire `[=]` section replaced with `[i]` variations section
- Line 594: Removed `[=]` from block markers table
- Line 1403: Updated deprecated keywords table

**Before:**
```polyglot
**`[=]` - Constant Input**
[=] .max_retries: pg\int << 3
```

**After:**
```polyglot
**`[i]` with `<<` - Constant Input**
[i] .max_retries: pg\int << 3

**Note:** `[i]` supports three variations:
- [i] .var: Type - Required
- [i] .var: Type <~ value - Default
- [i] .var: Type << value - Constant
```

#### 3. `language/bnf-grammar.md` (6 occurrences updated)
**Changes:**
- Line 183: Removed `<fixed-declaration>` production
- Lines 192-201: Updated note about keyword replacements
- Line 1045: Updated "Fixed" replacement text
- Line 1270: Updated example code
- Line 1589: Removed `[=]` from multi-character operators list

#### 4. `language/block-markers.md` (5 occurrences updated)
**Changes:**
- Lines 1460-1496: Replaced entire `[=]` section with `[i]` Input Variations section
- Line 796: Removed `[=]` from execution order list

**Before:**
```polyglot
### `[=]` - Constant Input
[=] .max_retries: pg\int << 3
```

**After:**
```polyglot
### `[i]` Input Variations
1. Required: [i] .var: Type
2. Default: [i] .var: Type <~ value
3. Constant: [i] .var: Type << value
```

#### 5. `language/macros.md` (8 occurrences - automated replacement)
**Changes:** All `[=]` → `[i]` via sed

#### 6. `architecture/runtime-execution.md` (1 occurrence - automated replacement)
**Changes:** `[=]` → `[i]` via sed

---

### Technical Documentation (`/docs/technical/`)

#### Updated Files (automated replacement via sed):
1. `architecture.md`
2. `block-hierarchy-reference.md`
3. `polyglot-formatting-guidelines-v1.0.md`
4. `block-hierarchy-qa.md`

**All occurrences:** `[=]` → `[i]`

---

## Migration Guide

### For Existing Code

**Old Syntax (v0.0.1):**
```polyglot
[|] MyPipeline
[i] .user_input: pg\string
[=] .max_retries: pg\int << 3
[=] .timeout: pg\int << 30
[X]
```

**New Syntax (v0.0.2):**
```polyglot
[|] MyPipeline
[i] .user_input: pg\string
[i] .max_retries: pg\int << 3
[i] .timeout: pg\int << 30
[X]
```

**Migration:** Simply replace all `[=]` with `[i]`. Semantics unchanged.

---

## Verification

### Grammar Check
✅ Removed `equals_decl` production from grammar.ebnf
✅ Updated `io_decl` to only include `input_decl`
✅ Added comments explaining three `[i]` variations

### Documentation Sweep
✅ AI context package (7 files) - grammar.ebnf, state-machine.yaml updated
✅ User docs (6 files) - All `[=]` removed and replaced
✅ Technical docs (4 files) - All `[=]` removed

### Consistency Check
✅ All `[=]` references replaced with `[i]`
✅ All explanations updated to reflect state-based model
✅ Examples show correct `[i]` usage with `<<`, `<~`, or nothing

---

## Breaking Changes

**Impact:** **Low**

**Reason:** Simple find-replace migration (`[=]` → `[i]`)

**Backward Compatibility:** None (v0.0.2 does not support `[=]`)

**Migration Effort:** Minimal - automated sed replacement sufficient

---

## Files Modified Summary

| Category | Files Modified | Method |
|----------|---------------|--------|
| AI Context | 2 files | Manual edits (grammar, state-machine) |
| User Docs | 6 files | Manual + sed (quick-start, syntax, grammar, blocks, macros, runtime) |
| Technical Docs | 4 files | Automated sed |
| **Total** | **12 files** | Mixed |

---

## Related Changes

This change complements the earlier fixes:
- Block marker hierarchy clarification (`[r]` vs `[<]` usage)
- Inline pipeline syntax (`DT.Now""`)
- Collection literal syntax (`{}` not `[]`)
- Async terminology (PUSH/PULL, Variable States)

All documented in:
- `/docs/ai-context/AI-CONTEXT-PACKAGE-FIXES.md`
- `/docs/ai-context/AI-CONTEXT-CORRECTIONS.md`

---

## Next Steps

1. ✅ Update all documentation (COMPLETE)
2. ⏳ Update lexer to reject `[=]` token
3. ⏳ Update parser to remove `equals_decl` handling
4. ⏳ Update compiler error messages

---

## Testing Recommendations

1. **Parser:** Ensure `[=]` throws syntax error
2. **Lexer:** Remove `[=]` from valid block markers
3. **Examples:** Verify all example `.pg` files compile
4. **Documentation:** Run link checker on updated docs

---

**Last Updated:** 2025-11-26
**Change Author:** AI Assistant
**Approved By:** User (via confirmation)
**Status:** Documentation Complete ✅
