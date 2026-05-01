---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:Aljam3"
metadata_instance: "%W:Aljam3:N"
---

# -W.Aljam3

Pure Aljam3 Code — no external runtime, no setup/cleanup.

## Definition

```aljam3
{N} -W.Aljam3
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WAljam3"
   [%] .description << "Pure Aljam3 Code — no external runtime, no setup/cleanup."
   [ ] Calls -DoNothing for both [\] and [/].
   [ ] Every pipeline requires [W]; this no-op confirms intent.
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:Aljam3` | Compile-time pipeline template |
| Instance | `%W:Aljam3:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/W/INDEX|-W.* Wrappers]]
