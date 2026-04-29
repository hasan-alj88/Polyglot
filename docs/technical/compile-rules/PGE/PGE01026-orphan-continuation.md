---
audience: developer
rule: "1.26"
code: PGE01026
name: Orphan Continuation Line
severity: error
---

# Rule 1.26 — Orphan Continuation Line
`PGE01026`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/collections -->
<!-- @u:syntax/operators -->

**Statement:** A `[~]` continuation line must follow a preceding incomplete expression that it continues. A `[~]` at the start of a block, or after a complete statement, has no expression to continue and is a compile error.
**Rationale:** `[~]` exists to break long expressions across lines. Without a preceding incomplete expression, it is meaningless. This prevents accidental `[~]` usage and ensures clear expression boundaries.
**Detection:** The compiler tracks whether the previous line ended with an incomplete expression (e.g., an open string interpolation, a pipeline call awaiting IO). If `[~]` appears with no such context, PGE01026 fires.

**VALID:**
```aljam3
[ ] ✓ continuation of long string interpolation
[-] $msg#string << "Hello {$firstName}"
   [~] " {$lastName}, welcome to"
   [~] " {$appName}!"
```

**INVALID:**
```aljam3
[ ] ✗ PGE01026 — [~] with no preceding expression
{-} -Bad
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [~] "orphan continuation"
```

**Diagnostic:** "Orphan `[~]` — no preceding expression to continue"
