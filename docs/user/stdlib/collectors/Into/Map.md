---
audience: user
type: specification
updated: 2026-03-28
status: stable
---

# *Into.Map — Collect into Map

Gathers key-value outputs from mini-pipelines back into a single `#Map`, accessible one level up from the expand scope. Mirrors `*Into.Serial` (key+value inputs).

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

**Duplicate keys:** If two mini-pipelines produce the same key, this is a runtime error. Each key in the resulting Map must be unique.

## IO Signature

| Inputs | Output |
|--------|--------|
| `<key`, `<value` | `>Map` |

## Usage

```polyglot
[r] *Into.Map
   [*] <key << $ticker
   [*] <value << $adjusted
   [*] >Map >> $adjustedPrices
```

See also: [[concepts/collections/collect#Collect Operators]]
