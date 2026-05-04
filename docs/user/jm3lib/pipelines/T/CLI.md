---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.T:CLI"
metadata_instance: "%T:CLI:N"
---

# -T.CLI

Pipeline invoked manually (e.g., from CLI or test harness). No additional parameters.

## Definition

```aljam3
{N} -T.CLI
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TCLI"
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
| Definition | `%definition.T:CLI` | Compile-time pipeline template |
| Instance | `%T:CLI:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
