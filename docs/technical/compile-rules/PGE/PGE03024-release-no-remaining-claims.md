---
audience: developer
rule: "3.24"
code: PGE03024
name: Release With No Remaining Claims
severity: error
---

### Rule 3.24 — Release With No Remaining Claims
`PGE03024`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** `*Job.Release` or `*Arrive.Job.Release` on a job that has no remaining claims from this collector is a compile error. A job cannot be released twice by the same collector.
**Rationale:** Double-release would corrupt the compound reconciliation model — the Trigger Monitor tracks claims per collector. Releasing a claim that doesn't exist is always a logic error.
**Detection:** The compiler tracks release operations within each `{*}` block. If a code path releases the same job index twice (statically determinable), PGE03024 fires.

### See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — compound reconciliation (Ground Rule 5)
- [[concepts/collections/collect#Compound Collector Strategies]] — claim semantics
