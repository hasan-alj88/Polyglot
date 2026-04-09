---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:#.Describe"
metadata_instance: "%-:#.Describe:N"
---

# -#.Describe

Schema introspection -- returns a type's full schema (properties, fields, constraints) as a `#serial` data tree. Useful for runtime reflection.

## Definition

```polyglot
{N} -#.Describe
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaDescribe"
   [%] .description << "Schema introspection"
   (-) <#type
   (-) >schema#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<#type` | type tree | Type definition to describe |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>schema` | `#serial` | Full schema as data tree |

## Errors

None -- always succeeds.

## Permissions

None -- pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:#.Describe` | Compile-time pipeline template |
| Instance | `%-:#.Describe:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Schema/INDEX|-# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/Validate|-#.Validate]]
