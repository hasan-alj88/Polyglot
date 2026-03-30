---
rule: "1.20"
code: PGE01020
name: Effectless Execution Expression
severity: error
---

### Rule 1.20 — Effectless Execution Expression
`PGE01020`

**Statement:** An execution expression under `[r]`, `[p]`, or `[b]` must be an assignment, pipeline call, chain call, or expand invocation. Bare literals and non-pipeline identifiers produce no effect and are compile errors.
**Rationale:** Execution markers define how work runs — `[r]` is synchronous, `[p]` is parallel, `[b]` is fire-and-forget. A bare literal (`42`, `"string"`) or non-pipeline identifier (`#UserRecord`, `$existingVar`, `@AD`) is not work: it assigns nothing, calls nothing, and produces no side effect. Allowing it would be misleading dead code that passes parsing but wastes an execution slot.
**Detection:** The compiler validates each `exec_expr` after the block element marker. If the expression is not an `assignment_expr`, `pipeline_call`, `chain_call`, or `expand_invocation`, it is rejected. The diagnostic suggests the likely intent (e.g., missing assignment operator, missing pipeline `=` prefix).

**See also:** PGE01016 (unmarked execution line — no marker at all), PGW02002 (unused variable — assigned but never read)

**VALID:**
```polyglot
[ ] ✓ pipeline call under [r]
{=} =Process
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >out#string
   [r] =Transform
      [=] <data << $input
      [=] >result >> $cleaned
   [r] >out << $cleaned
```

```polyglot
[ ] ✓ assignment under [r]
{=} =Assign
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >out#string
   [r] >out << $input
```

```polyglot
[ ] ✓ parallel pipeline calls
{=} =ParallelWork
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] <email#string
   [p] =Account.Create
      [=] <name << $name
   [p] =Mailbox.Provision
      [=] <email << $email
```

**INVALID:**
```polyglot
[ ] ✗ PGE01020 — bare literal under [r]
{=} =BadLiteral
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] 42                                 [ ] ✗ PGE01020 — bare literal, no effect
```

```polyglot
[ ] ✗ PGE01020 — bare string literal under [r]
{=} =BadString
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] "orphaned string"                  [ ] ✗ PGE01020 — bare literal, no effect
```

```polyglot
[ ] ✗ PGE01020 — non-pipeline identifiers under execution markers
{=} =BadIdentifiers
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [r] #UserRecord                        [ ] ✗ PGE01020 — data type, not a pipeline call
   [p] $existingVar                       [ ] ✗ PGE01020 — variable, not a pipeline call
   [b] @AD                               [ ] ✗ PGE01020 — package alias, not a pipeline call
```

**Open point:** None.

### See Also

- [[technical/ebnf/10-execution|Execution EBNF]] — `exec_expr` production (identifier removed)
- [[technical/compile-rules/PGE/PGE01016-unmarked-execution-line|PGE01016]] — missing block element marker (complementary rule)
