---
audience: developer
rule: "3.15"
code: PGE03015
name: Parallel-Scoped Collector Inside ForEach
severity: error
---

### Rule 3.15 — Parallel-Scoped Collector Inside ForEach
`PGE03015`

<!-- @u:technical/spec/collector-definitions -->
<!-- @u:concepts/collections/expand -->

**Statement:** A collector with `#CollectorScope.Parallel` must NOT be invoked inside an `=ForEach` expand scope. Invocation inside `=ForEach` is a compile error.
**Rationale:** Parallel-scoped collectors (`*All`, `*First`, `*Second`, `*Nth`, `*Ignore`) synchronize parallel `[=]` pipeline calls. They operate at the pipeline boundary, not inside per-item mini-pipelines.
**Detection:** At collector invocation site, if an `=ForEach` ancestor exists and the collector's `{*}` definition has `.scope == #CollectorScope.Parallel`, PGE03015 fires.

**VALID:**
```polyglot
[=] -Search.A
   (-) >result >> $a
[=] -Search.B
   (-) >result >> $b
(*) *First                                [ ] ✓ outside =ForEach
   (*) << $a
   (*) << $b
   (*) >> $winner
```

**INVALID:**
```polyglot
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item
   [ ] ✗ PGE03015 — *All used inside =ForEach
   (*) *All
      (*) << $someVar
```

### See Also

- [[PGE03014-expand-scope-outside-foreach]] — inverse rule
- [[concepts/collections/collect|Collect Operators]] — scope semantics
