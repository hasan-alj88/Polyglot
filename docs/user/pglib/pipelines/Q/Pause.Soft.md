---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Pause.Soft"
metadata_instance: "%Q:Pause.Soft:N"
---

# -Q.Pause.Soft

<!-- @d:pglib/pipelines/Q/Job.Pause.Free.CPU -->
**Deprecated.** Replaced by [[pglib/pipelines/Q/Job.Pause.Free.CPU|d:-Q.Job.Pause.Free.CPU]] (`.Wait` variant). Conditions now use `[?]`/`[&]`/`[+]` conditional blocks inside `{Q} #JobRules` definitions instead of pipeline name suffixes.

---

*Original content preserved below for historical reference.*

Direct command: finish current work, then suspend. Frees CPU. Signal: `command.pause.soft`.

## Definition

```polyglot
{N} -Q.Pause.Soft
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QPauseSoft"
   [%] .description << "Finish current work, then suspend. Frees CPU."
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
| `-Q.Pause.Soft.RAM.LessThan` | `<mb#Float` | Soft pause when RAM drops below threshold |
| `-Q.Pause.Soft.CPU.MoreThan` | `<percent#Float` | Soft pause when CPU exceeds threshold |
| `-Q.Pause.Soft.Disk.LessThan` | `<mb#Float` | Soft pause when disk space drops below threshold |
| `-Q.Pause.Soft.GPU.InUse` | (none) | Soft pause when GPU is occupied |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Pause.Soft` | Compile-time pipeline template |
| Instance | `%Q:Pause.Soft:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Pause.Hard|-Q.Pause.Hard]]
- [[pglib/pipelines/Q/Resume|-Q.Resume]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
