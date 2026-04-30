---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg.Average

Reduces mini-pipeline numeric outputs to their arithmetic mean.

## Syntax

```aljam3
[-] *Agg.Average
   (*) <number << $value
   (*) >average >> $mean
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<number` | numeric | Value to include in average |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>average` | numeric | Arithmetic mean of all input values |

## Errors

None.

## Permissions

None.

## Related

- [[aj3lib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
