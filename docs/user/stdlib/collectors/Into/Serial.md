---
audience: user
type: specification
updated: 2026-03-28
status: stable
---

# *Into.Serial — Collect into Serial

Gathers key-value outputs from mini-pipelines back into a single serial, accessible one level up from the expand scope.

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

## IO Signature

| Inputs | Output |
|--------|--------|
| `<key`, `<value` | `>Serial` |

## Usage

```polyglot
[r] *Into.Serial
   [*] <key << $key
   [*] <value << $value
   [*] >Serial >> $result
```

See also: [[concepts/collections/collect#Collect Operators]]
