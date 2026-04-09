---
audience: developer
rule: "8.5"
code: PGE08005
name: Unresolved Step Reference
severity: error
---

### Rule 8.5 — Unresolved Step Reference
`PGE08005`

**Statement:** In chain execution, step references in `(-)` IO lines and `[!]` error blocks must resolve to an existing step. A numeric index that exceeds the chain length, or a name that matches zero steps, is a compile error. This is distinct from PGE08004 (ambiguity, which matches >1 step) — PGE08005 fires when a reference matches zero steps.
**Rationale:** Referencing a nonexistent step is always a bug — a typo in the name or an index from a previous chain version that no longer applies. Catching this at compile time prevents silent wiring failures.
**Detection:** The compiler resolves each step reference against the chain's step list. If a numeric index is >= the number of steps, or a name-based reference matches no step's leaf or extended path, PGE08005 fires.

**See also:** PGE08004 (ambiguous step reference — matches >1), PGE07002 (chain error scoping)

**VALID:**
```polyglot
[ ] ✓ valid numeric indices for a 2-step chain
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path         [ ] ✓ index 0 exists
   (-) <1.rows#string >> >content     [ ] ✓ index 1 exists
```

```polyglot
[ ] ✓ valid name-based reference
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >Read.path#path << $path      [ ] ✓ "Read" matches step 0
   (-) <CSV.rows#string >> >content   [ ] ✓ "CSV" matches step 1
```

**INVALID:**
```polyglot
[ ] ✗ PGE08005 — numeric index out of bounds
[-] -File.Text.Read->-Text.Parse.CSV    [ ] 2 steps: index 0 and 1
   (-) >5.path#path << $path              [ ] ✗ PGE08005 — index 5 doesn't exist
```

```polyglot
[ ] ✗ PGE08005 — name matches zero steps
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >Format.input#string << $text      [ ] ✗ PGE08005 — no step named "Format"
```

```polyglot
[ ] ✗ PGE08005 — error reference to nonexistent step
[-] -File.Text.Read->-Text.Parse.CSV
   [!] .3!File.NotFound                   [ ] ✗ PGE08005 — index 3 doesn't exist
```

```polyglot
[ ] ✗ PGE08005 — error reference with unmatched name
[-] -File.Text.Read->-Text.Parse.CSV
   [!] .Format!File.NotFound              [ ] ✗ PGE08005 — no step named "Format"
```

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE08005 in step addressing rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08005 in step addressing table

**Open point:** None.
