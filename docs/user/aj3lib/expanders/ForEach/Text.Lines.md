---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
---

# =ForEach.Text.Lines

Iterates over each line in a text string, producing a mini-pipeline per line. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

## Syntax

```aljam3
[=] =ForEach.Text.Lines
   (=) <text << $myText
   (=) >line >> $line
   (=) >index >> $idx
   ...
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<text` | `#string` | Text to split into lines |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>line` | `#string` | Current line content |
| `>index` | `#int` | Zero-based line number |

## Errors

| Error | When |
|-------|------|
| `!Text.Lines.Empty` | Input text is empty |

## Permissions

None.

## Related

- [[jm3lib/expanders/ForEach/INDEX\|=ForEach Expanders]]
- [[concepts/collections/expand\|Expand Operators]]
- [[jm3lib/collectors/Into/Text.Append\|*Into.Text.Append]] -- reassemble lines by concatenation
- [[jm3lib/collectors/Into/Text.Merge\|*Into.Text.Merge]] -- reassemble lines by diff merge
