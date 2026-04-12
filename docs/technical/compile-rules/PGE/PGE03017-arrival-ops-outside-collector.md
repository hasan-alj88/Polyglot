---
audience: developer
rule: "3.17"
code: PGE03017
name: Arrival Operations Outside Collector Context
severity: error
---

### Rule 3.17 — Arrival Operations Outside Collector Context
`PGE03017`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** `*Arrive`, `*Job.Arrive`, `*Job.Release`, and `*Arrive.Job.Release` operations are only valid inside a `{*}` collector definition. Using them outside `{*}` context is a compile error.
**Rationale:** These operations depend on the collector's arrival tracking infrastructure. Outside a `{*}` block, there is no arrival context.
**Detection:** The compiler checks that arrival operations appear within a `{*}` block scope. If not, PGE03017 fires.

### See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — arrival operations (Ground Rules 3-4)
