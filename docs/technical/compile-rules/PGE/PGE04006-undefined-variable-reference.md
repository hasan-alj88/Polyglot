---
audience: developer
rule: "4.6"
code: PGE04006
name: Undefined Variable Reference
severity: error
---

### Rule 4.6 — Undefined Variable Reference
`PGE04006`

**Statement:** Every `$variable`, `>output`, or `<input` reference used in a push or pull expression must resolve to a declaration in the current scope. A reference that matches no declared variable, IO parameter, or field is a compile error. This covers all assignment operators (`<<`, `>>`, `<~`, `~>`), conditional pulls (`[?]`), and any other expression context. String interpolation references (`{$var}`) are covered separately by PGE04005.
**Rationale:** Undeclared variable references are always bugs — typos, stale names from refactoring, or copy-paste errors. Catching them at compile time prevents silent runtime failures where a pull returns undefined data or a push targets nothing.
**Detection:** The compiler maintains a scope table of all declared variables (`$`), IO inputs (`<`), and IO outputs (`>`) visible at each point in the program. When a reference is encountered, it is looked up in the scope table. If no matching declaration is found, PGE04006 fires.

**See also:** PGE04005 (undefined interpolation variable — same concept inside `{$var}` strings), PGE02002 (declared state is unreadable — variable exists but has no value yet), PGE01010 (pipeline IO name mismatch — undeclared names in pipeline call wiring)

**VALID:**
```polyglot
[ ] ✓ all references resolve to declarations
(-) <name#string
(-) >greeting#string
[-] >greeting << "Hello, {$name}!"
```

```polyglot
[ ] ✓ $variable declared then used
(-) <input#string
[-] $temp#string << <input
[-] >result#string << $temp
```

```polyglot
[ ] ✓ pulling from a declared IO output in a conditional
(-) >status#string
[?] >status
   [?] #Done
      [-] -Log
         (-) <msg#string << "finished"
   [?] *?
      [-] -Log
         (-) <msg#string << "pending"
```

**INVALID:**
```polyglot
[ ] ✗ PGE04006 — $variable never declared
(-) <input#string
[-] >result#string << $neverDeclared      [ ] ✗ PGE04006 — $neverDeclared not in scope
```

```polyglot
[ ] ✗ PGE04006 — typo in variable name
(-) <input#string
[-] $processed#string << <input
[-] >result#string << $procesed           [ ] ✗ PGE04006 — $procesed not declared (typo for $processed)
```

```polyglot
[ ] ✗ PGE04006 — pulling from undeclared output
[?] >undeclaredOutput                     [ ] ✗ PGE04006 — >undeclaredOutput not in scope
   [?] #Done
      [-] -Log
         (-) <msg#string << "done"
   [?] *?
      [-] -Log
         (-) <msg#string << "not done"
```

**Open point:** None.
