---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:#.Coerce"
metadata_instance: "%-:#.Coerce:N"
---

# -#.Coerce

Best-effort type conversion -- takes data and a target type, keeps fields that match, reports fields that don't. Always succeeds (never throws).

## Definition

```aljam3
{N} -#.Coerce
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaCoerce"
   [%] .description << "Best-effort type conversion"
   (-) <data#serial
   (-) <#type
   (-) >result#serial
   (-) >dropped#array:string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<data` | `#serial` | Data to coerce |
| `<#type` | type tree | Target type |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#serial` | Coerced data (matching fields only) |
| `>dropped` | `#array:string` | List of field paths that couldn't be coerced |

## Errors

None -- always succeeds.

## Permissions

None -- pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:#.Coerce` | Compile-time pipeline template |
| Instance | `%-:#.Coerce:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Schema/INDEX|-# Schema Validation & Field Extraction]]
- [[jm3lib/pipelines/Schema/Match|-#.Match]]
- [[jm3lib/pipelines/Schema/Validate|-#.Validate]]
