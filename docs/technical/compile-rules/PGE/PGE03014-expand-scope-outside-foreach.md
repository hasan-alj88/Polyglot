---
audience: developer
rule: "3.14"
code: PGE03014
name: Expand-Scoped Collector Outside ForEach
severity: error
---

### Rule 3.14 — Expand-Scoped Collector Outside ForEach
`PGE03014`

<!-- @u:technical/spec/collector-definitions -->
<!-- @u:concepts/collections/expand -->

**Statement:** A collector with `#CollectorScope.Expand` must be invoked inside an `=ForEach` expand scope. Invocation outside `=ForEach` is a compile error.
**Rationale:** Expand-scoped collectors (`*Into.*`, `*Agg.*`) gather per-item results from mini-pipelines. Without an expand scope, there are no items to collect.
**Detection:** At collector invocation site, the compiler checks the enclosing scope. If no `=ForEach` ancestor exists and the collector's `{*}` definition has `.scope == #CollectorScope.Expand`, PGE03014 fires.

**VALID:**
```polyglot
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item
   [-] ...
   [-] *Agg.Sum                          [ ] ✓ inside =ForEach
      (*) <number << $item.value
      (*) >sum >> $total
```

**INVALID:**
```polyglot
[ ] ✗ PGE03014 — *Agg.Sum used outside =ForEach
[-] *Agg.Sum
   (*) <number << $someValue
   (*) >sum >> $total
```

### See Also

- [[PGE03015-parallel-scope-inside-foreach]] — inverse rule
- [[concepts/collections/collect|Collect Operators]] — scope semantics
