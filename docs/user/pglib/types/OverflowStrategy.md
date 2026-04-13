---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:OverflowStrategy"
metadata_instance: "%#:OverflowStrategy:N"
---

# #OverflowStrategy Enum

<!-- @c:types -->
<!-- @u:technical/plan/queue-manager/overflow -->

Runtime `##Enum` type available in every `.pg` file. Controls how a `{*}` collector handles intermediate data that exceeds RAM capacity, participating in the PPTD (Parallel Processing Temporary Directory) overflow system ([[technical/plan/queue-manager/overflow|u:overflow]]).

---

## Definition

```polyglot
{#} #OverflowStrategy
   [%] .description << "Overflow behavior for collector definitions"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "overflowstrategy"
   [.] .InMemoryOnly
   [.] .Append
   [.] .Merge
   [.] .Custom
```

---

## Variants

| Variant | Behavior | Use case |
|---------|----------|----------|
| `.InMemoryOnly` | No PPTD — `!Storage.Space` if RAM exceeded | `*First`, `*Nth`, `*All`, `*Ignore` |
| `.Append` | Per-job temp files; concatenated on collect | `*Into.Text.Append`, `*Into.CSV.Rows` |
| `.Merge` | Per-job diff files; k-way merged on collect | `*Into.Text.Merge`, `*Into.CSV.Merge` |
| `.Custom` | Collector-defined overflow logic | User-defined collectors |

**Rule:** Collectors with `.overflow` != `.InMemoryOnly` must declare `(*) !Storage.Space` in their IO — missing triggers PGE03023.

---

## Related

- [[CollectorCategory]] — reconciliation category (Into, Agg, Sync, Race, Discard)
- [[CollectorScope]] — invocation context (Expand vs Parallel)
- [[enums]] — other pglib enum types
- [[technical/spec/collector-definitions\|Collector Definitions]] — `{*}` block specification
- [[technical/plan/queue-manager/overflow\|Overflow System]] — PPTD three-tier overflow chain
