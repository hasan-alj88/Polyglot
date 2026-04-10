---
audience: developer
rule: "4.5"
code: PGE04005
name: Undefined Interpolation Variable
severity: error
---

### Rule 4.5 — Undefined Interpolation Variable
`PGE04005`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Every `{$var}` reference inside a string literal must resolve to a variable declared in the current scope. An undefined variable in an interpolation expression is a compile error. Since all Polyglot data is serialized strings, any in-scope variable of any type can be interpolated — no type restriction applies. Nested interpolation (`{$items{$index}}`) is not supported and is a compile error.
**Rationale:** String interpolation silently producing empty or undefined values at runtime is a common source of bugs. Compile-time validation ensures all interpolated variables exist. The all-serialized-strings data model means type compatibility is inherent — no coercion is needed.
**Detection:** The compiler scans string literals for `{$...}` patterns, extracts variable names, and checks each against the current scope. If a variable is not declared, PGE04005 fires. If nested `{` is detected inside an interpolation, PGE04005 fires with a "nested interpolation not supported" message.

**See also:** PGE04001 (type mismatch)

**VALID:**
```polyglot
[ ] ✓ interpolated variable is in scope
(-) >name#string
[-] >name << "Alice"
[-] >output << "Hello, {$name}!"     [ ] ✓ $name is declared and in scope
```

```polyglot
[ ] ✓ interpolating non-string type — valid (all data is serialized strings)
(-) >count#int
[-] >count << 42
[-] >output << "Total: {$count}"     [ ] ✓ $count is int but interpolation is valid
```

**INVALID:**
```polyglot
[ ] ✗ PGE04005 — undefined variable in interpolation
[-] >output << "Hello, {$userName}!" [ ] ✗ PGE04005 — $userName not declared
```

```polyglot
[ ] ✗ PGE04005 — nested interpolation not supported
[-] >output << "Value: {$items{$index}}" [ ] ✗ PGE04005 — nested interpolation
```

**Open point:** None.

### See Also

- [[syntax/operators|Operators]] — string interpolation and undefined variable references
