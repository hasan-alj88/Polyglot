# Old Code Examples Archive

**Archived:** 2025-11-25
**Reason:** Non-compliant with v0.0.2 syntax

---

## Files Archived

### 1. variable-states-examples.pg
**Original Location:** `docs/variable-states-examples.pg`
**Violations:** 9 occurrences of `#Variables.States` (should be `#PgVar.States`)
**Purpose:** Basic variable states demonstration examples

### 2. variable-states-advanced-examples.pg  
**Original Location:** `docs/variable-states-advanced-examples.pg`
**Violations:** 46+ occurrences of `#Variables.States`, 1 map reference
**Purpose:** Advanced variable states patterns and use cases

### 3. test-file-processing-pipeline.pg
**Original Location:** `/test-file-processing-pipeline.pg` (project root)
**Violations:** 12 occurrences of `#Variables.States`, 1 missing trigger prefix
**Purpose:** File processing pipeline test/demo

### 4. test-state-aware-code.pg
**Original Location:** `/test-state-aware-code.pg` (project root)
**Violations:** 8 occurrences of `#Variables.States`, 1 missing trigger prefix
**Purpose:** State-aware code patterns test/demo

---

## Syntax Issues Summary

### Issue 1: Wrong Reserved Enumeration
```polyglot
# ❌ Old (invalid):
#Variables.States.Ready

# ✅ New (v0.0.2):
#PgVar.States.Ready
```

### Issue 2: Missing Trigger Prefix
```polyglot
# ❌ Old (invalid):
[t] T.Cron"0 2 * * *"

# ✅ New (v0.0.2):
[t] |T.Cron"0 2 * * *"
```

### Issue 3: Maps (Removed)
```polyglot
# ❌ Old (invalid):
pg\map{pg\string, pg\int}

# ✅ New (v0.0.2):
pg\serial  # Dynamic key-value structure
```

---

## Replacement Strategy

**New files will be created with v0.0.2 compliant syntax:**
- Correct reserved enumeration names (`#PgVar.States.*`)
- All operator prefixes present (`.`, `#`, `|`, `!`)
- No maps (use `pg\serial` or enumerations)
- Proper type separator (`\`)

**Reference:** See `docs/SYNTAX-COMPLIANCE-AUDIT-2025-11-25.md` for full audit details

---

## Retrieval

These files are preserved for reference to understand the patterns and examples, but should NOT be used as syntax references. All patterns should be reimplemented with v0.0.2 compliant syntax.
