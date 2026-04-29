---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
---

# =ForEach.CSV.Rows

Iterates over each row in CSV text, producing a mini-pipeline per row. The first row is treated as a header by default, providing field names for each parsed row. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

## Syntax

```aljam3
[=] =ForEach.CSV.Rows
   (=) <csv << $csvText
   (=) <delimiter << ","
   (=) <has_header << #True
   (=) >row >> $row
   (=) >index >> $idx
   ...
```

## Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `<csv` | `#string` | Yes | CSV text to parse |
| `<delimiter` | `#string` | No (default `","`) | Field delimiter character |
| `<has_header` | `#Boolean` | No (default `#True`) | Whether first row is a header providing field names |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>row` | `##Record` | Parsed row with field names from header (or positional keys `_0`, `_1`, ... when no header) |
| `>index` | `#int` | Zero-based row number (excludes header row) |

## Errors

| Error | When |
|-------|------|
| `!CSV.Parse.MalformedRow` | Row has a different field count than the header |
| `!CSV.Parse.Empty` | CSV content is empty |
| `!CSV.Parse.InvalidDelimiter` | Delimiter is empty or multi-character |

## Permissions

None.

## Related

- [[pglib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
- [[pglib/collectors/Into/CSV.Rows|*Into.CSV.Rows]] -- reassemble rows into CSV text
- [[pglib/collectors/Into/CSV.Merge|*Into.CSV.Merge]] -- merge CSV diffs with header preservation
