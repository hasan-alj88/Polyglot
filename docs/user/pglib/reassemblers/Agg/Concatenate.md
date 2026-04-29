---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg.Concatenate

Expands a collection and concatenates all string values into a single string.

## Syntax

```aljam3
[-] =*Agg.Concatenate
   (=) <array << $names
   (*) >result >> $allNames
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<array` | `#Array` of `#String` | Collection to expand and concatenate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#String` | Concatenated string of all items |

## Equivalent Expand-Collect

```aljam3
[-] =ForEach.Array
   (=) <Array << $names
   (=) >item >> $name

   [-] *Agg.Concatenate
      (*) <string << $name
      (*) >result >> $allNames
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[pglib/reassemblers/Agg/INDEX|=*Agg Aggregation Reassemblers]]
- [[pglib/collectors/Agg/Concatenate|*Agg.Concatenate]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
