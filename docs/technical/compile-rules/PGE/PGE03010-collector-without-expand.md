---
rule: "3.11"
code: PGE03010
name: Collector Without Expand
severity: error
---

### Rule 3.11 — Collector Without Expand
`PGE03010`

**Statement:** Every `*Into`/`*Agg` collector must appear within the scope of a `~ForEach` expand operator. A collector outside any expand scope has no source of expanded items and is a compile error.
**Rationale:** Collectors aggregate items produced by iteration. Without an expand, there is no iteration — the collector has no source and cannot function. This is the structural inverse of PGE03009 (expand without collector).
**Detection:** The compiler tracks expand scope depth. When a `*Into`/`*Agg` collector is encountered at scope depth zero (no enclosing expand), PGE03010 fires.

**See also:** PGE03009 (nested expand without collect — the inverse rule), PGE03004 (section-boundary pairing for `[p]`/`[*]`), PGE03008 (collect operator IO mismatch)

**VALID:**
```polyglot
[ ] ✓ collector inside expand scope
[p] ~ForEach.Array
   [~] <Array << $items
   [~] >item >> $item

   [r] =Transform
      [=] <data << $item
      [=] >result >> $transformed

   [r] *Into.Array
      [*] <item << $transformed
      [*] >Array >> $results
```

**INVALID:**
```polyglot
[ ] ✗ PGE03010 — collector with no expand scope
[r] =Transform
   [=] <data << $input
   [=] >result >> $transformed

[r] *Into.Array                              [ ] ✗ PGE03010 — no ~ForEach in scope
   [*] <item << $transformed
   [*] >Array >> $results
```

**Open point:** None.

### See Also

- [[concepts/collections/expand|Expand]] — documents collector-must-be-inside-expand constraint (references PGE03010)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03010 to example scenarios
