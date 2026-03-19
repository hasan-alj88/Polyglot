---
issue: "025"
title: No error for out-of-bounds chain step references
related: PGE-804 (Rule 8.4), PGE-702 (Rule 7.2)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 025 — No error for out-of-bounds chain step references

## Problem

PGE-804 handles ambiguous step references (when a leaf name matches multiple steps). However, no rule handles invalid step references:

### 1. Numeric index out of bounds
```polyglot
[r] =File.Text.Read >> =Text.Parse.CSV    [ ] 2 steps: index 0 and 1
   [=] >5.path;path << $path              [ ] ✗ index 5 doesn't exist
```

### 2. Name reference matching zero steps
```polyglot
[r] =File.Text.Read >> =Text.Parse.CSV
   [=] >Format.input;string << $text      [ ] ✗ no step named "Format"
```

### 3. Invalid in error references
```polyglot
[r] =File.Text.Read >> =Text.Parse.CSV
   [!] .3!File.NotFound                   [ ] ✗ index 3 doesn't exist
   [!] .Format!File.NotFound              [ ] ✗ no step named "Format"
```

These are distinct from PGE-804 (ambiguity) — these are references to nonexistent steps.

## Affected Rules

- `compile-rules/PGE/PGE-804-ambiguous-step-reference.md` (related but different)
- `compile-rules/PGE/PGE-702-chain-error-scoping.md` (error references)

## Proposed Resolution

**Option A — Extend PGE-804:**

Add "unresolved step reference" as a second failure mode of PGE-804. The rule already handles step resolution — an unresolvable reference is a natural extension.

**Option B — New error code PGE-805 (recommended):**

Create `PGE-805 — Unresolved Step Reference` for references that match zero steps. PGE-804 stays focused on ambiguity (matches >1 step). Different error messages help developers distinguish "which one?" from "doesn't exist."

## See also

- [PGE-804 — Ambiguous Step Reference](../compile-rules/PGE/PGE-804-ambiguous-step-reference.md)
- [PGE-702 — Chain Error Scoping](../compile-rules/PGE/PGE-702-chain-error-scoping.md)
