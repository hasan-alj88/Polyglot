---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Count

Expands a collection and counts the number of items.

## Syntax

```aljam3
[-] =*Agg.Count
   (=) <array << $items
   (*) >count >> $total
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<array` | `#Array` of any | Collection to expand and count |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>count` | `#UnsignedInt` | Number of items |

## Equivalent Expand-Collect

```aljam3
[-] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

   [-] *Agg.Count
      (*) <item << $item
      (*) >count >> $total
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[pglib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[pglib/collectors/Agg/Count|*Agg.Count]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
