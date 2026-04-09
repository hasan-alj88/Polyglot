---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg.Concatenate

Reduces mini-pipeline string outputs by joining them into a single string.

## Syntax

```polyglot
[-] *Agg.Concatenate
   (*) <string << $fragment
   (*) >result >> $combined
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<string` | `#String` | String fragment to append |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#String` | Concatenation of all input strings |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
