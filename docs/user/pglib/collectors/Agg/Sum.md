---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg.Sum

Reduces mini-pipeline numeric outputs to their sum.

## Syntax

```polyglot
[r] *Agg.Sum
   [*] <number << $value
   [*] >sum >> $total
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<number` | numeric | Value to accumulate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>sum` | numeric | Sum of all input values |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
