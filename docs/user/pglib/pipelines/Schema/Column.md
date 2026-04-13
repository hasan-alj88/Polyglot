---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:#.Column"
metadata_instance: "%-:#.Column:N"
---

# -#.Column

Extracts all values for a given column across all rows of a row-oriented Dataframe, returning them as an array. Solves the column access pattern that row-oriented storage doesn't natively provide.

## Definition

```polyglot
{N} -#.Column
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaColumn"
   [%] .description << "Extract column values from row-oriented Dataframe"
   (-) <data#dataframe
   (-) <column#enum
   (-) >values#array
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<data` | `#dataframe` | Source dataframe |
| `<column` | `#enum` | Column identifier from the dataframe's column enum |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>values` | `#array` | All values in that column, one per row |

## Errors

| Error | When |
|-------|------|
| `!Field.NotFound` | Column doesn't exist in dataframe's column enum |

## Permissions

None -- pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:#.Column` | Compile-time pipeline template |
| Instance | `%-:#.Column:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Schema/INDEX|-# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/Field|-#.Field]]
- [[concepts/collections/expand#Expand Operators]]
- [[syntax/types/generic-types|Generic Types and Parameterized Schemas]]
