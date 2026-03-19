---
rule: "4.6"
code: PGE-406
name: Undefined Variable Reference
severity: error
---

### Rule 4.6 — Undefined Variable Reference
`PGE-406`

**Statement:** Every `$variable`, `>output`, or `<input` reference used in a push or pull expression must resolve to a declaration in the current scope. A reference that matches no declared variable, IO parameter, or field is a compile error. This covers all assignment operators (`<<`, `>>`, `<~`, `~>`), conditional pulls (`[?]`), and any other expression context. String interpolation references (`{$var}`) are covered separately by PGE-405.
**Rationale:** Undeclared variable references are always bugs — typos, stale names from refactoring, or copy-paste errors. Catching them at compile time prevents silent runtime failures where a pull returns undefined data or a push targets nothing.
**Detection:** The compiler maintains a scope table of all declared variables (`$`), IO inputs (`<`), and IO outputs (`>`) visible at each point in the program. When a reference is encountered, it is looked up in the scope table. If no matching declaration is found, PGE-406 fires.

**See also:** PGE-405 (undefined interpolation variable — same concept inside `{$var}` strings), PGE-202 (declared state is unreadable — variable exists but has no value yet), PGE-110 (pipeline IO name mismatch — undeclared names in pipeline call wiring)

**VALID:**
```polyglot
[ ] ✓ all references resolve to declarations
[=] <name;string
[=] >greeting;string
[r] >greeting << "Hello, {$name}!"
```

```polyglot
[ ] ✓ $variable declared then used
[=] <input;string
[r] $temp;string << <input
[r] >result;string << $temp
```

```polyglot
[ ] ✓ pulling from a declared IO output in a conditional
[=] >status;string
[?] >status
   [?] #Done
      [r] =Log
         [=] <msg;string << "finished"
   [?] *?
      [r] =Log
         [=] <msg;string << "pending"
```

**INVALID:**
```polyglot
[ ] ✗ PGE-406 — $variable never declared
[=] <input;string
[r] >result;string << $neverDeclared      [ ] ✗ PGE-406 — $neverDeclared not in scope
```

```polyglot
[ ] ✗ PGE-406 — typo in variable name
[=] <input;string
[r] $processed;string << <input
[r] >result;string << $procesed           [ ] ✗ PGE-406 — $procesed not declared (typo for $processed)
```

```polyglot
[ ] ✗ PGE-406 — pulling from undeclared output
[?] >undeclaredOutput                     [ ] ✗ PGE-406 — >undeclaredOutput not in scope
   [?] #Done
      [r] =Log
         [=] <msg;string << "done"
   [?] *?
      [r] =Log
         [=] <msg;string << "not done"
```

**Open point:** None.
