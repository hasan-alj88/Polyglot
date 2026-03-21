---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# *Into — Collect into Collection

Collectors that gather outputs from mini-pipelines back into a single collection, accessible one level up from the expand scope.

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

```
*Into
   .Array
      <item
      >Array
   .Serial
      <key
      <value
      >Serial
   .Level
      <key
      <value
      >Serial
```

See also: [collections.md §Collect Operators](../concepts/collections.md)
