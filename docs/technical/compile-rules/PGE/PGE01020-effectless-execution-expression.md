---
audience: developer
rule: "1.20"
code: PGE01020
name: Effectless Execution Expression
severity: error
---

# Rule 1.20 — Effectless Execution Expression
`PGE01020`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** An execution expression under `[-]`, `[=]`, or `[b]` must be an assignment, pipeline call, chain call, or expand invocation. Bare literals and non-pipeline identifiers produce no effect and are compile errors.
**Rationale:** Execution markers define how work runs — `[-]` is synchronous, `[=]` is parallel, `[b]` is fire-and-forget. A bare literal (`42`, `"string"`) or non-pipeline identifier (`#UserRecord`, `$existingVar`, `@AD`) is not work: it assigns nothing, calls nothing, and produces no side effect. Allowing it would be misleading dead code that passes parsing but wastes an execution slot.
**Detection:** The compiler validates each `exec_expr` after the block element marker. If the expression is not an `assignment_expr`, `pipeline_call`, `chain_call`, or `expand_invocation`, it is rejected. The diagnostic suggests the likely intent (e.g., missing assignment operator, missing pipeline `=` prefix).

**See also:** PGE01016 (unmarked execution line — no marker at all), PGW02002 (unused variable — assigned but never read)

**VALID:**
```polyglot
[ ] ✓ pipeline call under [-]
{-} -Process
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] -Transform
      (-) <data << $input
      (-) >result >> $cleaned
   [-] >out << $cleaned
```

```polyglot
[ ] ✓ assignment under [-]
{-} -Assign
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] >out << $input
```

```polyglot
[ ] ✓ parallel pipeline calls
{-} -ParallelWork
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <name#string
   (-) <email#string
   [=] -Account.Create
      (-) <name << $name
   [=] -Mailbox.Provision
      (-) <email << $email
```

**INVALID:**
```polyglot
[ ] ✗ PGE01020 — bare literal under [-]
{-} -BadLiteral
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] 42                                 [ ] ✗ PGE01020 — bare literal, no effect
```

```polyglot
[ ] ✗ PGE01020 — bare string literal under [-]
{-} -BadString
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] "orphaned string"                  [ ] ✗ PGE01020 — bare literal, no effect
```

```polyglot
[ ] ✗ PGE01020 — non-pipeline identifiers under execution markers
{-} -BadIdentifiers
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   [-] #UserRecord                        [ ] ✗ PGE01020 — data type, not a pipeline call
   [=] $existingVar                       [ ] ✗ PGE01020 — variable, not a pipeline call
   [b] @AD                               [ ] ✗ PGE01020 — package alias, not a pipeline call
```

**Open point:** None.

## See Also

- [[technical/ebnf/10-execution|Execution EBNF]] — `exec_expr` production (identifier removed)
- [[technical/compile-rules/PGE/PGE01016-unmarked-execution-line|PGE01016]] — missing block element marker (complementary rule)
