---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =T.Manual

Pipeline invoked manually (e.g., from CLI or test harness). No additional parameters.

## Definition

```polyglot
{N} =T.Manual
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

## Related

- [[pglib/pipelines/T/INDEX|=T.* Trigger Pipelines]]
