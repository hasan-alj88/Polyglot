---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:CollectorCategory"
metadata_instance: "%#:CollectorCategory:N"
---

# #CollectorCategory Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.aj3` file. Classifies the reconciliation strategy of a `{*}` collector definition.

---

## Definition

```aljam3
{#} #CollectorCategory
   [%] .description << "Reconciliation category for collector definitions"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "collectorcategory"
   [.] .Into
   [.] .Agg
   [.] .Sync
   [.] .Race
   [.] .Discard
```

---

## Variants

| Variant | Description |
|---------|-------------|
| `.Into` | Collect into a collection type (`*Into.*`) |
| `.Agg` | Reduce to a single aggregated value (`*Agg.*`) |
| `.Sync` | Wait for all inputs — barrier (`*All`) |
| `.Race` | Select Nth arrival, cancel losers (`*First`, `*Second`, `*Nth`) |
| `.Discard` | Intentionally discard all output (`*Ignore`) |

---

## Related

- [[CollectorScope]] — invocation context (Expand vs Parallel)
- [[OverflowStrategy]] — overflow behavior for collector definitions
- [[enums]] — other pglib enum types
- [[technical/spec/collector-definitions\|Collector Definitions]] — `{*}` block specification
