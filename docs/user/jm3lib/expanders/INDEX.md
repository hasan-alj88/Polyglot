---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# Expander Operators (=)

Expanders iterate over collection elements, producing a mini-pipeline per item. Each mini-pipeline runs independently and feeds results into a collector.

All expanders use the `=` prefix. Invocation uses `[=]` (parallel) or `[-]` (sequential) execution markers. Expander IO lines use `(=)`.

No `[@]` import needed.

## 1. Memory Expanders (`=ForEach.{}`)
Iterate over in-memory Data Trees. The namespace defines which part of the topology is traversed.
- `=ForEach.Level` — Expands branches at a specific depth.
- `=ForEach.Leaves` — (Default) Exhaustive depth-first search to scalar terminals.
- `=ForEach.Branches` — Yields only structural nodes without underlying data.

## 2. File Streaming Expanders (`=File.{Format}.{}`)
Stream data directly from an authorized file (`_FileReadHandle`) into native Data Trees.
- `=File.Text.Lines`, `=File.Binary.Bytes` — Raw data format generators.
- `=File.CSV.Rows`, `=File.CSV.Columns` — `##Uniform` record generators.
- `=File.JSON.Nodes`, `=File.JSON.Leaves` — `#Serial` heterogenous generators.

## 3. Database Streaming Expanders (`=DB.{}`)
Stream data directly from an authorized DB connection (`_DBQueryHandle`).
- `=DB.Table.Rows`, `=DB.Query.Rows` — `##Uniform` record generators.
- `=DB.Collection.Documents` — `#Serial` NoSQL generators.

## Related

- [[jm3lib/INDEX|jm3lib Namespace Registry]]
- [[jm3lib/collectors/INDEX|Collector Operators]] -- pair with expanders to collect results
- [[concepts/collections/expand|Expand Operators]] -- conceptual overview
