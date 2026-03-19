---
rule: "1.1"
code: PGW-101
name: Empty Execution Body
severity: warning
split_from: PGE-101
---

### Rule 1.1 — Empty Execution Body
`PGW-101`

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
[ ] ⚠ PGW-101 — empty execution body
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [ ] ⚠ PGW-101 — no [r] calls in body
```

**Diagnostic:** "Pipeline `=Process` has an empty execution body"
