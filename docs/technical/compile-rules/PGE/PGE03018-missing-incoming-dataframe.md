---
audience: developer
rule: "3.18"
code: PGE03018
name: Missing Incoming DataFrame
severity: error
---

# Rule 3.18 — Missing Incoming DataFrame
`PGE03018`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** Every `{*}` collector definition must declare `(*) <Incoming#IncomingDataFrame` as an input. Missing this mandatory system input is a compile error.
**Rationale:** `<Incoming#IncomingDataFrame` is the system-provided input that delivers arrival data to the collector. Without it, the collector has no data to process.
**Detection:** After parsing the `{*}` block's IO section, the compiler checks for the presence of `<Incoming#IncomingDataFrame`. If absent, PGE03018 fires.

## See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — Ground Rule 8
- [[pglib/types/IncomingDataFrame|#IncomingDataFrame]] — type definition
