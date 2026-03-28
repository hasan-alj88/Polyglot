---
audience: user
type: specification
updated: 2026-03-28
status: stable
---

# *Into.Level — Collect into Serialized Siblings

Gathers key-value outputs from mini-pipelines into serialized siblings at a specific level, accessible one level up from the expand scope.

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

## IO Signature

| Inputs | Output |
|--------|--------|
| `<key`, `<value` | `>Serial` |

## Usage

```polyglot
[r] *Into.Level
   [*] <key << $key
   [*] <value << $value
   [*] >Serial >> $result
```

See also: [[collections#Collect Operators]]
