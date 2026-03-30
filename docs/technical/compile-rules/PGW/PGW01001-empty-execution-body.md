---
rule: "1.1"
code: PGW01001
name: Empty Execution Body
severity: warning
split_from: PGE01001
---

### Rule 1.1 — Empty Execution Body
`PGW01001`

**Statement:** A `{=}` pipeline that has all required sections (`[t]`, `[Q]`, setup/cleanup) but no `[r]` calls in its execution body produces a warning. The pipeline is syntactically valid but performs no work.
**Rationale:** A pipeline with trigger, queue, and setup/cleanup but no execution steps is likely an authoring mistake — the developer probably forgot to add the business logic. This is a warning rather than an error because the pipeline is structurally complete.
**Detection:** After confirming all required sections are present, the compiler checks whether the execution body (between setup and cleanup) contains at least one `[r]` call.

**VALID:**
```polyglot
[ ] ✓ pipeline has execution body
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

**WARNING:**
```polyglot
[ ] ⚠ PGW01001 — empty execution body
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [ ] ⚠ PGW01001 — no [r] calls in body
```

**Diagnostic:** "Pipeline `=Process` has an empty execution body"
