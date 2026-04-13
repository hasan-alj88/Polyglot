---
audience: automation-builder
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
   (*) <separator << "\n"
   (*) >result >> $combined
```

## Inputs

| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `<string` | `#String` | Yes | — | String fragment to append |
| `<separator` | `#String` | No | `""` | Separator inserted between fragments |

Separator is optional — omitting preserves existing concatenation behavior (empty-string join).

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#String` | Concatenation of all input strings, joined by separator |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Agg/INDEX|*Agg Aggregation Collectors]]
- [[concepts/collections/collect|Collect Operators]]
