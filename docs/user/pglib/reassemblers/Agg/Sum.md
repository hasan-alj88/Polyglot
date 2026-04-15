---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Sum

Expands a collection and reduces all numeric values to their sum.

## Syntax

```polyglot
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

```polyglot
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

- [[pglib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[pglib/collectors/Agg/Sum|*Agg.Sum]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
