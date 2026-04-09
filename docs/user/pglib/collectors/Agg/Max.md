---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg.Max

Reduces mini-pipeline numeric outputs to the maximum value.

## Syntax

```polyglot
[-] *Agg.Max
   (*) <number << $value
   (*) >max >> $highest
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<number` | numeric | Candidate value |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>max` | numeric | Maximum of all input values |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
