---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Max

Expands a collection and produces the maximum numeric value.

## Syntax

```aljam3
[-] =*Agg.Max
   (=) <array << $values
   (*) >max >> $highest
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<array` | `#Array` of numeric | Collection to expand and find maximum |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>max` | numeric | Maximum value across all items |

## Equivalent Expand-Collect

```aljam3
[-] =ForEach.Array
   (=) <Array << $values
   (=) >item >> $value

   [-] *Agg.Max
      (*) <number << $value
      (*) >max >> $highest
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[aj3lib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[aj3lib/collectors/Agg/Max|*Agg.Max]] -- standalone collector equivalent
- [[aj3lib/reassemblers/Agg/Min|=*Agg.Min]] -- minimum counterpart
- [[concepts/collections/reassemble|Reassemble Operators]]
