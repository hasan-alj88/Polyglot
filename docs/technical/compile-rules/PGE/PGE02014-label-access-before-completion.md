---
audience: design
rule: "2.14"
code: PGE02014
name: Label Access Before Completion
severity: error
type: spec
updated: 2026-04-09
---

# Rule 2.14 — Label Access Before Completion
`PGE02014`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** Accessing a label's outputs before the labeled operation has completed (in sequential execution order) is a compile error. The compiler verifies that every `$Label>param` or `$Label<param` reference appears strictly after the labeled operation in the execution flow.
**Rationale:** A label's outputs are only populated once the labeled operation finishes. Accessing them beforehand would read uninitialized state, which Aljam3 forbids (see PGE02002). The compiler enforces this statically by analyzing execution ordering. This is Aljam3's compile-time approach to preventing race conditions — in a system where operations may execute concurrently, the compiler proves that data dependencies are satisfied before access, eliminating a class of timing bugs that would only surface at runtime.
**Detection:** The compiler tracks the sequential position of each `($)` label and every accessor that references it. If an accessor appears before or at the same position as the labeled operation, the error is raised.

**VALID:**
```aljam3
[ ] ✓ label accessed after labeled operation completes
[-] -File.Text.Read"/input.txt"
   (-) $Read
[-] -Serial.JSON.Parse
   (-) <raw << $Read>content
```

**INVALID:**
```aljam3
[ ] ✗ PGE02014 — accessing $Read before the labeled operation
[-] -Serial.JSON.Parse
   (-) <raw << $Read>content
[-] -File.Text.Read"/input.txt"
   (-) $Read
```

**Diagnostic:** "Label accessor `$Read>content` used before labeled operation `$Read` has completed — move the access after the labeled call"

**Related:** PGE02002 (Declared State Is Unreadable)
