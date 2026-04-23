---
audience: developer
rule: "3.20"
code: PGE03020
name: Statements Outside Trigger Blocks
severity: error
---

# Rule 3.20 — Statements Outside Trigger Blocks in `{*}`
`PGE03020`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** Inside a `{*}` collector body, only `[-]` variable declarations and `[T]` arrival trigger blocks are valid at the top level. Any other statement (pipeline calls, conditionals, expand/collect, etc.) outside a `[T]` block is a compile error.
**Rationale:** Collectors are trigger-driven — all execution logic must live inside `[T]` blocks that fire on arrivals. Top-level variable declarations (`[-] $acc <~ 0`) are the only exception, as they initialize state before any trigger fires.
**Detection:** The compiler scans top-level lines inside `{*}`. Lines that are neither `[-]` variable declarations nor `[T]` blocks trigger PGE03020.

## See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — Ground Rule 4
