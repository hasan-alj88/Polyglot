---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
---

# *Into.Text.Append

Concatenates text fragments from mini-pipelines into a single string, with configurable separator and assembly order. Suitable for simple text reassembly where no merge conflict resolution is needed.

## Syntax

```aljam3
[-] *Into.Text.Append
   (*) <fragment << $processedLine
   (*) <separator << "\n"
   (*) <order << #CollectOrder.ExpandIndex
   (*) >text >> $result
```

## Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `<fragment` | `#string` | Yes | Text fragment to append |
| `<separator` | `#string` | No (default `""`) | Delimiter inserted between fragments |
| `<order` | `#CollectOrder` | No (default `.ExpandIndex`) | Assembly order: [[aj3lib/types/CollectOrder\|.ExpandIndex]] (original position) or `.Arrival` (completion order) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>text` | `#string` | Concatenated result of all fragments |

## Errors

| Error | When |
|-------|------|
| `!Text.Append.EmptyResult` | All fragments were empty |
| `!Storage.Space` | All overflow options exhausted (RAM + PPTD + batch) |

## PPTD Overflow

When fragment count exceeds the `-Q.Overflow.RamLimit` threshold, intermediate results spill to PPTD (Parallel Processing Temporary Directory). See [[technical/plan/queue-manager/overflow\|PPTD Overflow]] for the full overflow chain and configuration parameters.

## Permissions

| Permission | When |
|------------|------|
| `_File.TempWrite` | Only when overflow spills to disk |

## Related

- [[aj3lib/collectors/Into/INDEX\|*Into Data Collectors]]
- [[aj3lib/collectors/Agg/Concatenate\|*Agg.Concatenate]] -- simpler alternative (no ordering, no overflow)
- [[aj3lib/expanders/ForEach/Text.Lines\|=ForEach.Text.Lines]] -- expand text into lines
- [[concepts/collections/collect\|Collect Operators]]
