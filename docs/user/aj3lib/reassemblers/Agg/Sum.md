---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Sum

Expands a collection and reduces all numeric values to their sum.

## Syntax

```aljam3
[-] =*Agg.Sum
   (=) <array << $values
   (*) >sum >> $total
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<array` | `#Array` of numeric | Collection to expand and sum |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>sum` | numeric | Sum of all items |

## Equivalent Expand-Collect

```aljam3
[-] =ForEach.Array
   (=) <Array << $values
   (=) >item >> $value

   [-] *Agg.Sum
      (*) <number << $value
      (*) >sum >> $total
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[jm3lib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[jm3lib/collectors/Agg/Sum|*Agg.Sum]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
