---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Average

Expands a collection and reduces all numeric values to their arithmetic mean.

## Syntax

```polyglot
[-] =*Agg.Average
   (=) <array << $values
   (*) >average >> $mean
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<array` | `#Array` of numeric | Collection to expand and average |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>average` | `#Float` | Arithmetic mean of all items |

## Equivalent Expand-Collect

```polyglot
[-] =ForEach.Array
   (=) <Array << $values
   (=) >item >> $value

   [-] *Agg.Average
      (*) <number << $value
      (*) >average >> $mean
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[pglib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[pglib/collectors/Agg/Average|*Agg.Average]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
