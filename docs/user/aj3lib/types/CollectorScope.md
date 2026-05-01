---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:CollectorScope"
metadata_instance: "%#:CollectorScope:N"
---

# #CollectorScope Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.jm3` file. Declares the invocation context of a `{*}` collector definition.

---

## Definition

```aljam3
{#} #CollectorScope
   [%] .description << "Invocation context for collector definitions"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "collectorscope"
   [.] .Expand
   [.] .Parallel
```

---

## Variants

| Variant | Description |
|---------|-------------|
| `.Expand` | Collector operates inside `=ForEach` scope — gathers per-item results from mini-pipelines. Used by `*Into.*` and `*Agg.*`. Invocation outside `=ForEach` triggers PGE03014. |
| `.Parallel` | Collector operates at pipeline `[=]` parallel boundary — synchronizes parallel calls. Used by `*All`, `*First`, `*Second`, `*Nth`, `*Ignore`. Invocation inside `=ForEach` triggers PGE03015. |

---

## Related

- [[CollectorCategory]] — reconciliation category (Into, Agg, Sync, Race, Discard)
- [[OverflowStrategy]] — overflow behavior for collector definitions
- [[enums]] — other jm3lib enum types
- [[technical/spec/collector-definitions\|Collector Definitions]] — `{*}` block specification
