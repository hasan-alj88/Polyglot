---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach.Map

Iterates over each key-value pair in a `#Map`. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

No `.Enumerate` variant -- Map is unordered (`%Ordered = #False`), so a positional index would be misleading.

## Syntax

```aljam3
[=] =ForEach.Map
   (=) <Map << $prices
   (=) >key >> $ticker
   (=) >item >> $price
   ...
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Map` | `#Map` | Map to iterate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>key` | `#KeyString` | Current map key |
| `>item` | value type | Current map value |

## Errors

None.

## Permissions

None.

## Related

- [[jm3lib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
