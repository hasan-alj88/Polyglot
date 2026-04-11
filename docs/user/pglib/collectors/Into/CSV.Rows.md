---
audience: pg-coder
type: specification
updated: 2026-04-11
status: draft
---

# *Into.CSV.Rows

Collects rows from mini-pipelines into a single CSV text string, with a header row derived from the first collected row's field names.

## Syntax

```polyglot
[-] *Into.CSV.Rows
   (*) <row << $processedRow
   (*) <delimiter << ","
   (*) <order << #CollectOrder.ExpandIndex
   (*) >csv >> $result
```

## Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `<row` | `##Record` | Yes | Row to collect |
| `<delimiter` | `#string` | No (default `","`) | Field delimiter character |
| `<order` | `#CollectOrder` | No (default `.ExpandIndex`) | Assembly order: [[pglib/types/CollectOrder|.ExpandIndex]] (original position) or `.Arrival` (completion order) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>csv` | `#string` | Complete CSV text with header row and all collected rows |

## Errors

| Error | When |
|-------|------|
| `!CSV.Collect.SchemaMismatch` | Row has different fields than the header (first row) |
| `!CSV.Collect.EmptyResult` | Zero rows collected |
| `!Storage.Space` | All overflow options exhausted (RAM + PPTD + batch) |

## PPTD Overflow

When row count exceeds the `-Q.Overflow.RamLimit` threshold, intermediate results spill to PPTD (Parallel Processing Temporary Directory). See [[technical/plan/queue-manager/overflow|PPTD Overflow]] for the full overflow chain and configuration parameters.

## Permissions

| Permission | When |
|------------|------|
| `_File.TempWrite` | Only when overflow spills to disk |

## Related

- [[pglib/collectors/Into/INDEX|*Into Data Collectors]]
- [[pglib/expanders/ForEach/CSV.Rows|=ForEach.CSV.Rows]] -- expand CSV into rows
- [[pglib/collectors/Into/Text.Append|*Into.Text.Append]] -- simpler text-level alternative
- [[concepts/collections/collect|Collect Operators]]
