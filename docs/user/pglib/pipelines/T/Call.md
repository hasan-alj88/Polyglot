---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =T.Call

Pipeline invoked when called from another pipeline. No additional parameters.

## Definition

```polyglot
{N} =T.Call
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

## Related

- [[pglib/pipelines/T/INDEX|=T.* Trigger Pipelines]]
