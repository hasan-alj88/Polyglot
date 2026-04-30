---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
---

# *Into.CSV.Merge

K-way merge collector for CSV data. Applies multiple diff sets against a shared base CSV, producing a merged result with conflict detection. Row-aware: preserves the header row and treats header modifications as errors.

## Syntax

```aljam3
[-] *Into.CSV.Merge
   (*) <diffs << $csvDiffs
   (*) <base << $originalCsv
   (*) <conflict << #MergeStrategy.FirstWins
   (*) >result >> $merged
```

## Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `<diffs` | `#TextDiffs` | Yes | Row-level diff set from a parallel [[aj3lib/pipelines/Text/Diff|=Text.Diff]] call on CSV content |
| `<base` | `#string` | Yes | Original CSV text all diffs were computed against |
| `<conflict` | `#MergeStrategy` | No (default `.FirstWins`) | [[aj3lib/types/MergeStrategy|Conflict resolution strategy]] |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#MergeResult` | [[aj3lib/types/MergeResult|Merged result]]: `.text` (merged CSV string) + `.conflicts` (array of [[aj3lib/types/MergeConflict|#MergeConflict]]) |

## Errors

| Error | When |
|-------|------|
| `!CSV.Merge.HeaderConflict` | Diffs modify the header row (row 0) |
| `!Text.Merge.InvalidLineNumber` | Diff references a row outside the base CSV range |
| `!Text.Merge.EmptyBase` | Base CSV is empty |
| `!Storage.Space` | All overflow options exhausted (RAM + PPTD + batch) |

## PPTD Overflow

When diff count exceeds the `-Q.Overflow.RamLimit` threshold, intermediate merge results spill to PPTD (Parallel Processing Temporary Directory). See [[technical/plan/queue-manager/overflow|PPTD Overflow]] for the full overflow chain and configuration parameters.

## Permissions

| Permission | When |
|------------|------|
| `_File.TempWrite` | Only when overflow spills to disk |

## Related

- [[aj3lib/collectors/Into/INDEX|*Into Data Collectors]]
- [[aj3lib/collectors/Into/Text.Merge|*Into.Text.Merge]] -- general text merge (non-CSV-aware)
- [[aj3lib/pipelines/Text/Diff|=Text.Diff]] -- produces the diffs this collector merges
- [[aj3lib/expanders/ForEach/CSV.Rows|=ForEach.CSV.Rows]] -- expand CSV into rows
- [[aj3lib/types/MergeResult|#MergeResult]] -- output type
- [[aj3lib/types/MergeStrategy|#MergeStrategy]] -- conflict resolution enum
- [[concepts/collections/collect|Collect Operators]]
