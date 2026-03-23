---
rule: "7.3w"
code: PGW-703
name: Missing Fallback Message
severity: warning
---

### Rule 7.3w — Missing Fallback Message
`PGW-703`

**Statement:** When a pipeline author defines an output fallback inside a `[!] >>` raise block (`[=] >output << value`) but does not include a `[>] %FallbackMessage` line, the compiler emits a warning. Without a message, callers can silently override the fallback without understanding the author's intent. Suppress the warning with `[>] %FallbackMessage << ""` to explicitly allow silent overrides.
**Rationale:** Pipeline-defined fallbacks represent deliberate design decisions. If the author does not document why the fallback exists, callers who override it via `[>] <!` will not receive PGW-702, losing the safety net. This warning ensures authors either document their fallback reasoning or explicitly opt out.
**Detection:** The compiler checks each `[=] >output << value` line inside `[!] >>` raise blocks. If no `[>] %FallbackMessage` line follows (indented under the output fallback), PGW-703 fires.

**See also:**
- [PGW-702 — Caller Overrides Pipeline Fallback](PGW-702-caller-overrides-pipeline-fallback.md) — caller-side warning when overriding documented fallbacks

**VALID:**
```polyglot
[ ] ✓ Fallback with %FallbackMessage — documented intent
{=} =ValidateUser
   [=] <name;string
   [=] >validated;string
   [=] >status;string
   [=] !Validation.Empty
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty
         [=] .Message << "Name is required"
         [=] >status << "invalid"
            [>] %FallbackMessage << "Pipeline returns invalid status on empty input"
   [?] *?
      [r] >validated << $name
      [r] >status << "ok"
```

```polyglot
[ ] ✓ Fallback with empty %FallbackMessage — intentionally allows silent override
{=} =ValidateUserPermissive
   [=] <name;string
   [=] >status;string
   [=] !Validation.Empty
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty
         [=] .Message << "Name is required"
         [=] >status << "invalid"
            [>] %FallbackMessage << ""
   [?] *?
      [r] >status << "ok"
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-703 — output fallback without %FallbackMessage
{=} =ValidateUserBad
   [=] <name;string
   [=] >status;string
   [=] !Validation.Empty
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $name =? ""
      [!] >> !Validation.Empty
         [=] .Message << "Name is required"
         [=] >status << "invalid"             [ ] ⚠ PGW-703 — missing %FallbackMessage
   [?] *?
      [r] >status << "ok"
```

**Open point:** None.
