---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach.Array

Iterates over each item in an array, producing a mini-pipeline per item. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

## Syntax

```aljam3
[=] =ForEach.Array
   (=) <Array << $myArray
   (=) >item >> $item
   ...
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Array` | `#Array` | Array to iterate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>item` | element type | Current array element |

## Variants

- [[pglib/expanders/ForEach/Array/Enumerate|=ForEach.Array.Enumerate]] -- adds positional `>index` output

## Errors

None.

## Permissions

None.

## Related

- [[pglib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
