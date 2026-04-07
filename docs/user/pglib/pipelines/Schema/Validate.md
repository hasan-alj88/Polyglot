---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:#.Validate"
metadata_instance: "%=:#.Validate:N"
---

# =#.Validate

Detailed validation -- checks data against type and reports all mismatches as a list of human-readable error strings.

## Definition

```polyglot
{N} =#.Validate
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaValidate"
   [%] .description << "Detailed validation with error reporting"
   [=] <data#serial
   [=] <#type
   [=] >valid#bool
   [=] >errors#array:string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<data` | `#serial` | Data to validate |
| `<#type` | type tree | Type definition as data tree input |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>valid` | `#bool` | Overall pass/fail |
| `>errors` | `#array:string` | List of validation error descriptions |

## Errors

None -- always succeeds. Validation failures are reported via `>errors`, not `[!]`.

## Permissions

None -- pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:#.Validate` | Compile-time pipeline template |
| Instance | `%=:#.Validate:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Schema/INDEX|=# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/Match|=#.Match]]
- [[pglib/pipelines/Schema/Coerce|=#.Coerce]]
