---
rule: "6.10"
code: PGE-610
name: Empty Conditional Scope
severity: error
---

### Rule 6.10 — Empty Conditional Scope
`PGE-610`

**Statement:** Every `[?]` branch must contain at least one executable statement (`[r]`, `[p]`, `[b]`, or nested `[?]`). A `[?]` branch with only comments or no body at all is a compile error. Use `[r] =DoNothing` to explicitly mark an intentionally empty branch.
**Rationale:** An empty conditional branch is almost always an authoring mistake — the developer intended to add logic but forgot. Unlike an empty pipeline body (PGW-101, warning), a conditional branch is a targeted decision point: if the developer wrote the condition, they intended an action. Requiring `=DoNothing` makes the "do nothing" intent explicit and self-documenting.
**Detection:** The compiler checks the body of every `[?]` branch (including `[?] *?` catch-all). If no executable statement is found, PGE-610 fires.

**VALID:**
```polyglot
[ ] ✓ each branch has an executable statement
[?] $age =? #FileAge.Old
   [r] =File.Delete
      [=] <path << $file
[?] $age =? #FileAge.Unknown
   [r] =DoNothing
[?] *?
   [r] =DoNothing
```

```polyglot
[ ] ✓ comments alongside executable statement are fine
[?] $status =? #PipelineStatus.Running
   [ ] Pipeline is still running — wait
   [r] =DoNothing
[?] *?
   [r] =HandleComplete
```

**INVALID:**
```polyglot
[ ] ✗ PGE-610 — catch-all branch has only a comment, no executable
[?] $age =? #FileAge.Old
   [r] =File.Delete
      [=] <path << $file
[?] *?
   [ ] File is not old enough — skip
```

```polyglot
[ ] ✗ PGE-610 — branch is completely empty
[?] $mode =? "debug"
[?] *?
   [r] =Process
```

**See also:**
- [PGW-101 — Empty Execution Body](../PGW/PGW-101-empty-execution-body.md) — warning for empty pipeline body (different scope)
- [PGE-601 — Conditional Must Be Exhaustive](PGE-601-conditional-must-be-exhaustive.md) — exhaustiveness rule

### See Also

- [[user/concepts/conditionals|Conditionals]] — empty branch rules reference PGE-610
