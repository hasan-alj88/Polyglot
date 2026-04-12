---
audience: pg-coder
type: specification
updated: 2026-04-11
status: draft
---

# *Into.Text.Merge

K-way merge collector that applies multiple diff sets against a shared base text, producing a merged result with conflict detection. Each mini-pipeline produces diffs (via [[pglib/pipelines/Text/Diff\|=Text.Diff]]), and this collector reconciles them into a single output.

## Syntax

```polyglot
[-] *Into.Text.Merge
   (*) <diffs << $myDiffs
   (*) <base << $originalText
   (*) <conflict << #MergeStrategy.FirstWins
   (*) >result >> $merged
```

## Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `<diffs` | `#TextDiffs` | Yes | Diff set from a parallel [[pglib/pipelines/Text/Diff\|=Text.Diff]] call |
| `<base` | `#string` | Yes | Original text all diffs were computed against |
| `<conflict` | `#MergeStrategy` | No (default `.FirstWins`) | [[pglib/types/MergeStrategy\|Conflict resolution strategy]] |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#MergeResult` | [[pglib/types/MergeResult\|Merged result]]: `.text` (merged string) + `.conflicts` (array of [[pglib/types/MergeConflict\|#MergeConflict]]) |

## Errors

| Error | When |
|-------|------|
| `!Text.Merge.InvalidLineNumber` | Diff references a line outside the base text range |
| `!Text.Merge.EmptyBase` | Base text is empty |
| `!Storage.Space` | All overflow options exhausted (RAM + PPTD + batch) |

## Conflict Resolution Strategies

The `<conflict` parameter accepts a [[pglib/types/MergeStrategy\|#MergeStrategy]] enum value:

| Strategy | Behavior |
|----------|----------|
| `.FirstWins` | First job's change wins at conflict points |
| `.LastWins` | Last job's change wins at conflict points |
| `.KeepBoth` | Both changes kept, marked in output |
| `.SkipConflicts` | Non-conflicts applied; conflicts reported in `>result.conflicts` |
| `.DiffOnly` | Produce merged diff record without modifying base text |

## Workflow

The canonical pattern pairs `=Text.Diff` with `*Into.Text.Merge`:

```polyglot
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

- [[pglib/collectors/Into/INDEX\|*Into Data Collectors]]
- [[pglib/pipelines/Text/Diff\|=Text.Diff]] -- produces the diffs this collector merges
- [[pglib/types/MergeResult\|#MergeResult]] -- output type
- [[pglib/types/MergeStrategy\|#MergeStrategy]] -- conflict resolution enum
- [[pglib/types/MergeConflict\|#MergeConflict]] -- conflict entry type
- [[pglib/types/TextDiff\|#TextDiff]] -- individual diff entry type
- [[pglib/collectors/Into/Text.Append\|*Into.Text.Append]] -- simpler alternative (no conflict resolution)
- [[concepts/collections/collect\|Collect Operators]]
