---
audience: developer
rule: "8.13"
code: PGE08013
name: Nested Inline Data
severity: error
---

# Rule 8.13 — Nested Inline Data
`PGE08013`

<!-- @u:EBNF:inline_data -->

**Statement:** Inline data literals (`{...}`) cannot contain other inline data literals. The `inline_data` production uses `inline_value` (not `value_expr`), which excludes nested `{}`. Nested collection construction requires building inner collections first via pipeline calls, then composing them.

**Rationale:** Nested inline data creates parse ambiguity — `{{1}}` could be interpreted as a nested singleton array or as unrelated brace groupings. Flat literals are unambiguous; complex structures use explicit pipeline composition. This also prevents unbounded nesting depth (`{{{...}}}`) which would require recursive type-depth validation at parse time.

**Detection:** The parser encounters `{` inside an `inline_data` context. PGE08013 fires immediately.

**VALID:**
```aljam3
[ ] ✓ flat inline data
[-] $nums#array:int << {1, 2, 3, 4, 5}
[-] $names#array:string << {"Alice", "Bob", "Charlie"}
(-) >results#array:string ~> {}
```

**INVALID:**
```aljam3
[ ] ✗ PGE08013 — nested inline data
[-] $matrix#array:array:int << {{1, 2, 3}, {4, 5, 6}}

[ ] ✗ PGE08013 — deeply nested
[-] $deep << {{{1}}}

[ ] ✗ PGE08013 — mixed nesting
[-] $mixed << {1, {2, 3}, 4}
```

**Open point:** None.

## See Also

- [[technical/ebnf/08-expressions|EBNF §8.3 — Inline Data]] — `inline_value` production
- [[technical/edge-cases/08-expressions|EC-8.8 — Nested inline data (INVALID)]]
