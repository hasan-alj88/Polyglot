---
audience: pg-coder
type: specification
updated: 2026-03-28
status: stable
---

# *Into.Array — Collect into Array

Gathers outputs from mini-pipelines back into a single array, accessible one level up from the expand scope.

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

## IO Signature

| Input | Output |
|-------|--------|
| `<item` | `>Array` |

## Usage

```polyglot
[r] *Into.Array
   [*] <item << $doubled
   [*] >Array >> $results
```

See also: [[concepts/collections/collect#Collect Operators]]
