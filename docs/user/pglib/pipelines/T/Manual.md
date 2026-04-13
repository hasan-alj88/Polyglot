---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.T:Manual"
metadata_instance: "%T:Manual:N"
---

# -T.Manual

Pipeline invoked manually (e.g., from CLI or test harness). No additional parameters.

## Definition

```polyglot
{N} -T.Manual
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TManual"
   [%] .description << "Pipeline is invoked manually (e.g., from CLI or test harness)."
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
| Definition | `%definition.T:Manual` | Compile-time pipeline template |
| Instance | `%T:Manual:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
