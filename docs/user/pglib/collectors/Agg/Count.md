---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg.Count

Reduces mini-pipeline outputs to a count of items received.

## Syntax

```polyglot
[-] *Agg.Count
   (*) <item << $value
   (*) >count >> $total
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<item` | any | Item to count |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>count` | `#int` | Number of items received |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
