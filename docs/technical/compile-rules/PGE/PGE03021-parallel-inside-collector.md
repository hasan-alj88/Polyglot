---
audience: developer
rule: "3.21"
code: PGE03021
name: Parallel Execution Inside Collector
severity: error
---

### Rule 3.21 — Parallel Execution Inside Collector
`PGE03021`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** The `[=]` parallel execution marker is forbidden inside `{*}` collector definitions. All work inside `{*}` must use `[-]` sequential execution.
**Rationale:** Collectors are strictly sequential — they process arrivals one at a time in deterministic order. Allowing parallel execution inside a collector would break the arrival-order guarantees (Ground Rule 2) and complicate release semantics.
**Detection:** The compiler scans all execution lines within `{*}` (including inside `[T]` blocks). Any `[=]` marker triggers PGE03021.

### See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — Ground Rule 6
