---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Min

Expands a collection and produces the minimum numeric value.

## Syntax

```aljam3
[-] =*Agg.Min
   (=) <array << $values
   (*) >min >> $lowest
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<array` | `#Array` of numeric | Collection to expand and find minimum |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>min` | numeric | Minimum value across all items |

## Equivalent Expand-Collect

```aljam3
[-] =ForEach.Array
   (=) <Array << $values
   (=) >item >> $value

   [-] *Agg.Min
      (*) <number << $value
      (*) >min >> $lowest
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[aj3lib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[aj3lib/collectors/Agg/Min|*Agg.Min]] -- standalone collector equivalent
- [[aj3lib/reassemblers/Agg/Max|=*Agg.Max]] -- maximum counterpart
- [[concepts/collections/reassemble|Reassemble Operators]]
