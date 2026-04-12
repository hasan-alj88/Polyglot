---
audience: developer
rule: "3.19"
code: PGE03019
name: Arrival Index Out of Bounds
severity: error
---

### Rule 3.19 — Arrival Index Out of Bounds
`PGE03019`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** An arrival index expression in `*Arrive` or `*Job.Arrive` must not exceed `N` (the total count). A literal index known to exceed the possible range at compile time is a compile error.
**Rationale:** Indexing beyond the number of registered arrivals is unreachable code. The compiler can detect this when the index is a constant.
**Detection:** When the index is a literal integer, the compiler compares it against the statically known count (if determinable). If it exceeds `N`, PGE03019 fires. Variable-interpolated indices (`{$n}`) are checked at runtime.

### See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — index expressions (Ground Rule 3)
