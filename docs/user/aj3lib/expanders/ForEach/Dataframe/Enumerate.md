---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach.Dataframe.Enumerate

Iterates over each row in a dataframe with its positional index. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

## Syntax

```aljam3
[=] =ForEach.Dataframe.Enumerate
   (=) <Dataframe << $sales
   (=) >index >> $idx
   (=) >row >> $row
   ...
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Dataframe` | `#Dataframe` | Dataframe to iterate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>index` | `#int` | Zero-based row index |
| `>row` | row struct | Compiler-synthesized struct with column fields |

## Errors

None.

## Permissions

None.

## Related

- [[aj3lib/expanders/ForEach/Dataframe|=ForEach.Dataframe]] -- base variant without index
- [[aj3lib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
