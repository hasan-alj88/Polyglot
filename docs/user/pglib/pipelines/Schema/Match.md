---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =#.Match

Boolean schema check -- does this data match this type? Returns `#True`/`#False`, no side effects.

## Definition

```polyglot
{N} =#.Match
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaMatch"
   [%] .description << "Boolean schema check"
   [=] <data#serial
   [=] <#type
   [=] >match#bool
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<data` | `#serial` | Data to check |
| `<#type` | type tree | Type definition as data tree input |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>match` | `#bool` | Whether data matches the type's schema |

## Errors

None -- always succeeds.

## Permissions

None -- pure computation.

## Related

- [[pglib/pipelines/Schema/INDEX|=# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/Validate|=#.Validate]]
- [[pglib/pipelines/Schema/Coerce|=#.Coerce]]
