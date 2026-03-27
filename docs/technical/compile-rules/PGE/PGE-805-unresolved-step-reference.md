---
rule: "8.5"
code: PGE-805
name: Unresolved Step Reference
severity: error
---

### Rule 8.5 — Unresolved Step Reference
`PGE-805`

**Statement:** In chain execution, step references in `[=]` IO lines and `[!]` error blocks must resolve to an existing step. A numeric index that exceeds the chain length, or a name that matches zero steps, is a compile error. This is distinct from PGE-804 (ambiguity, which matches >1 step) — PGE-805 fires when a reference matches zero steps.
**Rationale:** Referencing a nonexistent step is always a bug — a typo in the name or an index from a previous chain version that no longer applies. Catching this at compile time prevents silent wiring failures.
**Detection:** The compiler resolves each step reference against the chain's step list. If a numeric index is >= the number of steps, or a name-based reference matches no step's leaf or extended path, PGE-805 fires.

**See also:** PGE-804 (ambiguous step reference — matches >1), PGE-702 (chain error scoping)

**VALID:**
```polyglot
[ ] ✓ valid numeric indices for a 2-step chain
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path         [ ] ✓ index 0 exists
   [=] <1.rows#string >> >content     [ ] ✓ index 1 exists
```

```polyglot
[ ] ✓ valid name-based reference
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >Read.path#path << $path      [ ] ✓ "Read" matches step 0
   [=] <CSV.rows#string >> >content   [ ] ✓ "CSV" matches step 1
```

**INVALID:**
```polyglot
[ ] ✗ PGE-805 — numeric index out of bounds
[r] =File.Text.Read=>=Text.Parse.CSV    [ ] 2 steps: index 0 and 1
   [=] >5.path#path << $path              [ ] ✗ PGE-805 — index 5 doesn't exist
```

```polyglot
[ ] ✗ PGE-805 — name matches zero steps
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >Format.input#string << $text      [ ] ✗ PGE-805 — no step named "Format"
```

```polyglot
[ ] ✗ PGE-805 — error reference to nonexistent step
[r] =File.Text.Read=>=Text.Parse.CSV
   [!] .3!File.NotFound                   [ ] ✗ PGE-805 — index 3 doesn't exist
```

```polyglot
[ ] ✗ PGE-805 — error reference with unmatched name
[r] =File.Text.Read=>=Text.Parse.CSV
   [!] .Format!File.NotFound              [ ] ✗ PGE-805 — no step named "Format"
```

**Open point:** None.
