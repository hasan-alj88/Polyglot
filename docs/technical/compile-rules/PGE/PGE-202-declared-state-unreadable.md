---
rule: "2.2"
code: PGE-202
name: Declared State Is Unreadable
severity: error
---

### Rule 2.2 — Declared State Is Unreadable
`PGE-202`

**Statement:** A variable in Declared state holds no value. Attempting to read (pull from) a Declared variable is an error. The variable must reach Default or Final before it can be used as a source.
**Rationale:** Reading an uninitialized variable produces undefined behavior. Catching this eliminates entire categories of silent failures where pipelines operate on empty data without any error signal.
**Detection:** At the execution step that attempts to pull from the variable — the runtime checks the variable's state and fires PGE-202 if it is still Declared.

**VALID:**
```polyglot
[ ] ✓ >name assigned to Final before being read
[=] >name#string
[r] >name << "Alice"         [ ] Final — safe to read
[r] =Greet
   [=] <who << >name        [ ] ✓ >name is Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE-202 — >name declared but never assigned
[=] >name#string             [ ] Declared — no value
[r] =Greet
   [=] <who << >name        [ ] ✗ PGE-202 — >name is still Declared
```

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — defines Declared state and references PGE-202
