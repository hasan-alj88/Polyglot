---
audience: developer
rule: "7.3w"
code: PGW07003
name: Missing Fallback Message
severity: warning
---

# Rule 7.3w — Missing Fallback Message
`PGW07003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** When a pipeline author defines an output fallback inside a `[!] >>` raise block (`(-) >output << value`) but does not include a `(>) %FallbackMessage` line, the compiler emits a warning. Without a message, callers can silently override the fallback without understanding the author's intent. Suppress the warning with `(>) %FallbackMessage << ""` to explicitly allow silent overrides.
**Rationale:** Pipeline-defined fallbacks represent deliberate design decisions. If the author does not document why the fallback exists, callers who override it via `(>) !>` will not receive PGW07002, losing the safety net. This warning ensures authors either document their fallback reasoning or explicitly opt out.
**Detection:** The compiler checks each `(-) >output << value` line inside `[!] >>` raise blocks. If no `(>) %FallbackMessage` line follows (indented under the output fallback), PGW07003 fires.

**See also:**
- [PGW07002 — Caller Overrides Pipeline Fallback](PGW07002-caller-overrides-pipeline-fallback.md) — caller-side warning when overriding documented fallbacks

**VALID:**
```aljam3
[ ] ✓ Fallback with %FallbackMessage — documented intent
{-} -ValidateUser
   (-) <name#string
   (-) >validated#string
   (-) >status#string
   (-) !Validation.Empty
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [?] $name =? ""
      [!] >> !Validation.Empty
         (!) .Message << "Name is required"
         (!) >status << "invalid"
            (>) %FallbackMessage << "Pipeline returns invalid status on empty input"
   [?] *?
      [-] >validated << $name
      [-] >status << "ok"
```

```aljam3
[ ] ✓ Fallback with empty %FallbackMessage — intentionally allows silent override
{-} -ValidateUserPermissive
   (-) <name#string
   (-) >status#string
   (-) !Validation.Empty
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [?] $name =? ""
      [!] >> !Validation.Empty
         (!) .Message << "Name is required"
         (!) >status << "invalid"
            (>) %FallbackMessage << ""
   [?] *?
      [-] >status << "ok"
```

**WARNING:**
```aljam3
[ ] ⚠ PGW07003 — output fallback without %FallbackMessage
{-} -ValidateUserBad
   (-) <name#string
   (-) >status#string
   (-) !Validation.Empty
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [?] $name =? ""
      [!] >> !Validation.Empty
         (!) .Message << "Name is required"
         (!) >status << "invalid"             [ ] ⚠ PGW07003 — missing %FallbackMessage
   [?] *?
      [-] >status << "ok"
```

**Open point:** None.
