---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Pause.Hard"
metadata_instance: "%Q:Pause.Hard:N"
---

# -Q.Pause.Hard

<!-- @d:pglib/pipelines/Q/Job.Pause.Free.RAM -->
<!-- @d:pglib/pipelines/Q/Job.Pause.Free.All -->
**Deprecated.** Replaced by [[pglib/pipelines/Q/Job.Pause.Free.RAM|d:-Q.Job.Pause.Free.RAM]] (CPU+RAM) and [[pglib/pipelines/Q/Job.Pause.Free.All|d:-Q.Job.Pause.Free.All]] (everything to disk). The new system provides five granular resource-freeing levels. Conditions now use `[?]`/`[&]`/`[+]` conditional blocks inside `{Q} #JobRules` definitions.

---

*Original content preserved below for historical reference.*

Direct command: immediate suspend. Frees CPU+RAM. Signal: `command.pause.hard`.

## Definition

```polyglot
{N} -Q.Pause.Hard
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QPauseHard"
   [%] .description << "Immediate suspend. Frees CPU+RAM."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Conditional Variants

Pause when a resource condition is met. Used as nested `[Q]` lines in `{Q}` definitions or pipeline `[Q]` sections.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `-Q.Pause.Hard.RAM.LessThan` | `<mb#Float` | Hard pause when RAM drops below threshold |
| `-Q.Pause.Hard.CPU.MoreThan` | `<percent#Float` | Hard pause when CPU exceeds threshold |
| `-Q.Pause.Hard.Disk.LessThan` | `<mb#Float` | Hard pause when disk space drops below threshold |
| `-Q.Pause.Hard.GPU.InUse` | (none) | Hard pause when GPU is occupied |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Pause.Hard` | Compile-time pipeline template |
| Instance | `%Q:Pause.Hard:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Pause.Soft|-Q.Pause.Soft]]
- [[pglib/pipelines/Q/Resume|-Q.Resume]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
