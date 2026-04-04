---
rule: "12.4"
code: PGE12004
name: Empty Metadata Alias
severity: error
---

### Rule 12.4 — Empty Metadata Alias
`PGE12004`

**Statement:** A `[%] %alias` declaration must contain at least one `:` alias name. An alias block with no names is a compile error.
**Rationale:** The `%alias` marker exists to declare shorthand names for definitions or fields. An empty alias declaration adds noise without defining any aliases. The EBNF requires at least one alias name.
**Detection:** The compiler checks that each `[%] %alias` block contains at least one `[:] "name"` child line.

**VALID:**
```polyglot
[ ] ✓ alias with names
{#} #UserRecord
   [%] %alias
      [:] "User"
      [:] "Person"
   [.] .name#string
```

**INVALID:**
```polyglot
[ ] ✗ PGE12004 — empty alias declaration
{#} #MyType
   [%] %alias
   [.] .field#string
```

**Diagnostic:** "Empty `%alias` — requires at least one `:` alias name"
