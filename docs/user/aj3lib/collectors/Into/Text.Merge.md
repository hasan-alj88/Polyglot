---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
---

# *Into.Text.Merge

K-way merge collector that applies multiple diff sets against a shared base text, producing a merged result with conflict detection. Each mini-pipeline produces diffs (via [[jm3lib/pipelines/Text/Diff\|=Text.Diff]]), and this collector reconciles them into a single output.

## Syntax

```aljam3
[-] *Into.Text.Merge
   (*) <diffs << $myDiffs
   (*) <base << $originalText
   (*) <conflict << #MergeStrategy.FirstWins
   (*) >result >> $merged
```

## Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `<diffs` | `#TextDiffs` | Yes | Diff set from a parallel [[jm3lib/pipelines/Text/Diff\|=Text.Diff]] call |
| `<base` | `#string` | Yes | Original text all diffs were computed against |
| `<conflict` | `#MergeStrategy` | No (default `.FirstWins`) | [[jm3lib/types/MergeStrategy\|Conflict resolution strategy]] |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#MergeResult` | [[jm3lib/types/MergeResult\|Merged result]]: `.text` (merged string) + `.conflicts` (array of [[jm3lib/types/MergeConflict\|#MergeConflict]]) |

## Errors

| Error | When |
|-------|------|
| `!Text.Merge.InvalidLineNumber` | Diff references a line outside the base text range |
| `!Text.Merge.EmptyBase` | Base text is empty |
| `!Storage.Space` | All overflow options exhausted (RAM + PPTD + batch) |

## Conflict Resolution Strategies

The `<conflict` parameter accepts a [[jm3lib/types/MergeStrategy\|#MergeStrategy]] enum value:

| Strategy | Behavior |
|----------|----------|
| `.FirstWins` | First job's change wins at conflict points |
| `.LastWins` | Last job's change wins at conflict points |
| `.KeepBoth` | Both changes kept, marked in output |
| `.SkipConflicts` | Non-conflicts applied; conflicts reported in `>result.conflicts` |
| `.DiffOnly` | Produce merged diff record without modifying base text |

## Workflow

The canonical pattern pairs `=Text.Diff` with `*Into.Text.Merge`:

```aljam3
[=] =ForEach.Text.Lines
   (=) <text << $originalText
   (=) >line >> $line
   (=) >index >> $idx

   [-] -ProcessLine
      (-) <line << $line
      (-) >modified >> $processed

   [-] -Text.Diff
      (-) <original << $line
      (-) <modified << $processed
      (-) >diffs >> $lineDiffs

   [-] *Into.Text.Merge
      (*) <diffs << $lineDiffs
      (*) <base << $originalText
      (*) <conflict << #MergeStrategy.SkipConflicts
      (*) >result >> $merged
```

## PPTD Overflow

When diff count exceeds the `-Q.Overflow.RamLimit` threshold, intermediate merge results spill to PPTD (Parallel Processing Temporary Directory). See [[technical/plan/queue-manager/overflow\|PPTD Overflow]] for the full overflow chain and configuration parameters.

## Permissions

| Permission | When |
|------------|------|
| `_File.TempWrite` | Only when overflow spills to disk |

## Related

- [[jm3lib/collectors/Into/INDEX\|*Into Data Collectors]]
- [[jm3lib/pipelines/Text/Diff\|=Text.Diff]] -- produces the diffs this collector merges
- [[jm3lib/types/MergeResult\|#MergeResult]] -- output type
- [[jm3lib/types/MergeStrategy\|#MergeStrategy]] -- conflict resolution enum
- [[jm3lib/types/MergeConflict\|#MergeConflict]] -- conflict entry type
- [[jm3lib/types/TextDiff\|#TextDiff]] -- individual diff entry type
- [[jm3lib/collectors/Into/Text.Append\|*Into.Text.Append]] -- simpler alternative (no conflict resolution)
- [[concepts/collections/collect\|Collect Operators]]
