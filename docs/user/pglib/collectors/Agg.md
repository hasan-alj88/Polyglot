---
audience: pg-coder
type: specification
updated: 2026-03-20
status: draft
---

# *Agg — Reduce to Single Value

Aggregation collectors that reduce mini-pipeline outputs to a single value. The namespace is `*Agg` — the canonical shorthand alias for `*Aggregate`. Always use `*Agg` in code.

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

```polyglot
*Agg
   .Sum
      <number
      >sum
   .Count
      <item
      >count
   .Average
      <number
      >average
   .Max
      <number
      >max
   .Min
      <number
      >min
   .Concatenate
      <string
      >result
```

See also: [collections.md §Collect Operators](../concepts/collections.md)
