---
audience: developer
rule: "2.2"
code: PGE02002
name: Declared State Is Unreadable
severity: error
---

### Rule 2.2 — Declared State Is Unreadable
`PGE02002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** A variable in Declared state holds no value. Attempting to read (pull from) a Declared variable is an error. The variable must reach Default or Final before it can be used as a source.
**Rationale:** Reading an uninitialized variable produces undefined behavior. Catching this eliminates entire categories of silent failures where pipelines operate on empty data without any error signal.
**Detection:** At the execution step that attempts to pull from the variable — the runtime checks the variable's state and fires PGE02002 if it is still Declared.

**VALID:**
```polyglot
[ ] ✓ >name assigned to Final before being read
(-) >name#string
[-] >name << "Alice"         [ ] Final — safe to read
[-] -Greet
   (-) <who << >name        [ ] ✓ >name is Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE02002 — >name declared but never assigned
(-) >name#string             [ ] Declared — no value
[-] -Greet
   (-) <who << >name        [ ] ✗ PGE02002 — >name is still Declared
```

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — defines Declared state and references PGE02002
