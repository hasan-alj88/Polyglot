---
rule: "4.5"
code: PGE-405
name: Undefined Interpolation Variable
severity: error
---

### Rule 4.5 — Undefined Interpolation Variable
`PGE-405`

**Statement:** Every `{$var}` reference inside a string literal must resolve to a variable declared in the current scope. An undefined variable in an interpolation expression is a compile error. Since all Polyglot data is serialized strings, any in-scope variable of any type can be interpolated — no type restriction applies. Nested interpolation (`{$items{$index}}`) is not supported and is a compile error.
**Rationale:** String interpolation silently producing empty or undefined values at runtime is a common source of bugs. Compile-time validation ensures all interpolated variables exist. The all-serialized-strings data model means type compatibility is inherent — no coercion is needed.
**Detection:** The compiler scans string literals for `{$...}` patterns, extracts variable names, and checks each against the current scope. If a variable is not declared, PGE-405 fires. If nested `{` is detected inside an interpolation, PGE-405 fires with a "nested interpolation not supported" message.

**See also:** PGE-401 (type mismatch)

**VALID:**
```polyglot
[ ] ✓ interpolated variable is in scope
[=] >name#string
[r] >name << "Alice"
[r] >output << "Hello, {$name}!"     [ ] ✓ $name is declared and in scope
```

```polyglot
[ ] ✓ interpolating non-string type — valid (all data is serialized strings)
[=] >count#int
[r] >count << 42
[r] >output << "Total: {$count}"     [ ] ✓ $count is int but interpolation is valid
```

**INVALID:**
```polyglot
[ ] ✗ PGE-405 — undefined variable in interpolation
[r] >output << "Hello, {$userName}!" [ ] ✗ PGE-405 — $userName not declared
```

```polyglot
[ ] ✗ PGE-405 — nested interpolation not supported
[r] >output << "Value: {$items{$index}}" [ ] ✗ PGE-405 — nested interpolation
```

**Open point:** None.
