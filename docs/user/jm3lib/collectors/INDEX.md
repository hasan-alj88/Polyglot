---
audience: automation-builder
type: specification
updated: 2026-04-11
status: stable
---

# Collector Operators (*)

<!-- @c:glossary#Reconciliation -->
Collectors gather outputs from expanded mini-pipelines or parallel pipeline calls back into a single value or collection. They are the [[glossary#Reconciliation|c:reconciliation]] layer — every parallel job must be reconciled, and the collector determines both what happens to the data (output reconciliation) and when jobs terminate (job reconciliation). See [[concepts/collections/collect#Reconciliation]] for the full model.

All collectors use the `*` prefix. Invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. Collector IO lines use `(*)`.

No `[@]` import needed.

## Data Collectors (Memory Gathering)

Reassemble outputs back into an in-memory Data Tree.

- `*Collect` -- Reassembles items into a Data Tree based on a target `(*) <schema`.

## Aggregation Collectors (Memory Reduction)

Reduce mini-pipeline outputs to a single scalar value.

- [[jm3lib/collectors/Agg/INDEX|*Agg.*]] -- Sum, Count, Average, Max, Min, Concatenate

## Sink Collectors (Streaming Endpoints)

Stream outputs directly to external resources instead of building them in memory.

- `*File.{Format}` -- Streams pipeline outputs directly into serialized file formats (e.g., `*File.CSV.Rows`, `*File.JSON.Nodes`).
- `*DB.{Collection}` -- Streams pipeline outputs directly into a database (e.g., `*DB.Table.Insert`, `*DB.Collection.Insert`).

## Collect-All & Race Collectors

Collect variables from parallel `[=]` pipeline calls (outside expand scopes).

- [[jm3lib/collectors/Sync/INDEX|*All / *First / *Nth]] -- collect-all barrier, race collectors, discard

## Collector Definitions (`{*}`)

Collectors are definable as first-class entities using `{*}` blocks. Existing jm3lib collectors (`*First`, `*All`, etc.) are implemented as `{*}` definitions — collector logic lives in `{*}`, while underlying job management remains native.

- [[technical/spec/collector-definitions|Collector Definitions]] -- `{*}` block specification, ground rules, and examples
- [[technical/ebnf/16-collector-definitions|EBNF §16]] -- formal grammar for `{*}` blocks

## Related

- [[jm3lib/INDEX|jm3lib Namespace Registry]]
- [[concepts/collections/collect|Collect Operators]] -- conceptual overview
- [[concepts/collections/collect#Reconciliation]] -- reconciliation model (output + job lifecycle)
- [[concepts/collections/collect#Compound Collector Strategies]] -- multiple collectors on the same job
