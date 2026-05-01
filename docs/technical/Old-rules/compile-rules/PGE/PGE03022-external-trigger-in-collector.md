---
audience: developer
rule: "3.22"
code: PGE03022
name: External Trigger Source in Collector
severity: error
---

# Rule 3.22 — External Trigger Source in Collector
`PGE03022`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** Inside a `{*}` collector definition, `[T]` blocks must use arrival conditions (`*Arrive`, `*Job.Arrive`) only. External trigger sources (`=T.Call`, `=T.Folder.NewFiles`, etc.) are forbidden.
**Rationale:** Collectors are triggered by arrivals, not external events. The collector's lifecycle is bound to its arrival stream — external triggers would create ambiguous firing semantics.
**Detection:** The compiler checks all `[T]` lines within `{*}`. If the trigger reference is not an arrival condition (`*Arrive` or `*Job.Arrive`), PGE03022 fires.

## See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — Ground Rule 4
