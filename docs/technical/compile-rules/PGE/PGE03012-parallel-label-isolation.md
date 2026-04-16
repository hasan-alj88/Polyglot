---
audience: designer
rule: "3.12"
code: PGE03012
name: Parallel Label Isolation
severity: error
type: spec
updated: 2026-04-09
---

### Rule 3.12 — Parallel Label Isolation
`PGE03012`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/collections -->
<!-- @u:syntax/operators -->

**Statement:** Accessing a label from a `[=]` parallel operation outside the collection scope violates pull isolation. Parallel label outputs must be collected before access, the same as any parallel variable. Directly referencing `$ParallelLabel>param` after a `[=]` block without going through a collector (`[*]`) is a compile error.
**Rationale:** Parallel operations spawn multiple concurrent instances. A label inside a parallel scope refers to one instance per parallel branch — there is no single deterministic value to read. The collector must aggregate or select results before they become available to sequential code, maintaining Polyglot's pull isolation guarantee.
**Detection:** The compiler checks that every accessor of a `($)` label defined inside a `[=]` parallel block either (a) appears within the same parallel branch or (b) appears inside a `[*]` collector scope that collects from that parallel block. Any accessor outside both contexts is rejected.

**VALID:**
```polyglot
[ ] ✓ parallel label accessed inside collector scope
[p] -Transform.Each
   [=] $items
      [-] -Parse.Record
         (-) $Parse
         (-) <raw << $item
      (-) >result << $Parse>output
   [*] *All
      [*] >> >results
```

**INVALID:**
```polyglot
[ ] ✗ PGE03012 — accessing parallel label outside collection
[p] -Transform.Each
   [=] $items
      [-] -Parse.Record
         (-) $Parse
         (-) <raw << $item
[-] -Next.Step
   (-) <data << $Parse>output
```

**Diagnostic:** "Label accessor `$Parse>output` references a parallel-scoped label — collect parallel results before accessing them"

**Related:** PGE03003 (Pull Isolation Until Collection), PGE03001 (No Push Across Parallel Boundaries)
