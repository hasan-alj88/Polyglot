---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.-:Text.Diff"
metadata_instance: "%-:Text.Diff:N"
---

# -Text.Diff

Compares two text strings and produces a list of line-level differences. Pure computation — no file access or side effects.

## Definition

```aljam3
{N} =Text.Diff
   [%] .Kind << #NativeKind.Computation
   [%] .Rust << "TextDiff"
   [%] .description << "Compare two texts and produce line-level diffs"
   (-) <original#string
   (-) <modified#string
   (-) >diffs#TextDiffs
   (-) >stats#DiffStats
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<original` | `#string` | Original text to compare against |
| `<modified` | `#string` | Modified text to compare with |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>diffs` | `#TextDiffs` | Array of [[pglib/types/TextDiff\|#TextDiff]] entries (line, op, content) |
| `>stats` | `#DiffStats` | Summary counts ([[pglib/types/DiffStats\|#DiffStats]]: additions, deletions, unchanged) |

## Errors

| Error | When |
|-------|------|
| `!Text.Diff.EmptyInput` | Either `<original` or `<modified` is empty |

## Permissions

None (pure computation).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| `%definition.-:Text.Diff` | Definition schema | Pipeline definition metadata |
| `%-:Text.Diff:N` | Instance | Runtime instance metadata |

## Related

- [[pglib/types/TextDiff\|#TextDiff]] -- diff entry type
- [[pglib/types/DiffStats\|#DiffStats]] -- summary statistics type
- [[pglib/collectors/Into/Text.Merge\|*Into.Text.Merge]] -- merge diffs back into text
- [[pglib/expanders/ForEach/Text.Lines\|=ForEach.Text.Lines]] -- expand text into lines
