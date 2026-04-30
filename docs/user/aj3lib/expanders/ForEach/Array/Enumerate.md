---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach.Array.Enumerate

Iterates over each item in an array with its positional index. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

## Syntax

```aljam3
[=] =ForEach.Array.Enumerate
   (=) <Array << $myArray
   (=) >index >> $idx
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
| `>index` | `#int` | Zero-based positional index |
| `>item` | element type | Current array element |

## Errors

None.

## Permissions

None.

## Related

- [[aj3lib/expanders/ForEach/Array|=ForEach.Array]] -- base variant without index
- [[aj3lib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
