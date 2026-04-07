---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =#.Describe

Schema introspection -- returns a type's full schema (properties, fields, constraints) as a `#serial` data tree. Useful for runtime reflection.

## Definition

```polyglot
{N} =#.Describe
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaDescribe"
   [%] .description << "Schema introspection"
   [=] <#type
   [=] >schema#serial
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

## Related

- [[pglib/pipelines/Schema/INDEX|=# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/Validate|=#.Validate]]
