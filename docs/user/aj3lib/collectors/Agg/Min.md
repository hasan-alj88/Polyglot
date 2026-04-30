---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg.Min

Reduces mini-pipeline numeric outputs to the minimum value.

## Syntax

```aljam3
[-] *Agg.Min
   (*) <number << $value
   (*) >min >> $lowest
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<number` | numeric | Candidate value |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>min` | numeric | Minimum of all input values |

## Errors

None.

## Permissions

None.

## Related

- [[aj3lib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
