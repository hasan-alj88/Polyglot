---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.T:Call"
metadata_instance: "%T:Call:N"
---

# -T.Call

Pipeline invoked when called from another pipeline. No additional parameters.

## Definition

```polyglot
{N} -T.Call
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TCall"
   [%] .description << "Pipeline is invoked when called from another pipeline."
   [ ] No additional parameters.
```

## Inputs

| Name | Type | Description |
|------|------|-------------|

## Outputs

| Name | Type | Description |
|------|------|-------------|

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Call` | Compile-time pipeline template |
| Instance | `%T:Call:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
