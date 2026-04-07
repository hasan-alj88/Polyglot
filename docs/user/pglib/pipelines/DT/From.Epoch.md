---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.From.Epoch

Converts epoch seconds to a `#dt` value.

## Definition

```polyglot
{N} =DT.From.Epoch
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromEpoch"
   [%] .description << "Epoch seconds to DateTime"
   [=] <epoch#int
   [=] >dt#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<epoch` | `#int` | --- | Epoch seconds to convert |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>dt` | `#dt` | Resulting DateTime value |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
