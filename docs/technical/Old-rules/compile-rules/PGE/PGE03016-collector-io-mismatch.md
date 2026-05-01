---
audience: developer
rule: "3.16"
code: PGE03016
name: Collector IO Mismatch
severity: error
---

# Rule 3.16 — Collector IO Mismatch
`PGE03016`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** At a collector invocation site, the `(*)` IO ports must match the `{*}` definition's declared IO. Missing required inputs, extra undefined inputs, or type mismatches between invocation and definition are compile errors.
**Rationale:** The `{*}` definition declares a contract. The invocation must satisfy it — same as pipeline IO matching (PGE01010).
**Detection:** The compiler resolves the `{*}` definition for the invoked collector and compares each `(*)` line at the invocation site against the definition's `(*)` declarations. Mismatches trigger PGE03016.

## See Also

- [[PGE01010-pipeline-io-name-mismatch]] — analogous rule for pipeline IO
- [[technical/spec/collector-definitions|Collector Definitions]] — `{*}` IO declarations
