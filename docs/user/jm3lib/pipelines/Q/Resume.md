---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Resume"
metadata_instance: "%Q:Resume:N"
---

# -Q.Resume

<!-- @d:jm3lib/pipelines/Q/Job.Resume -->
**Deprecated.** Replaced by [[jm3lib/pipelines/Q/Job.Resume|d:-Q.Job.Resume]]. Resume now uses pause reason set semantics — "resume" means "this rule no longer wants the Job paused." The Job actually resumes only when no rule wants it paused.

---

*Original content preserved below for historical reference.*

Direct command: move from Suspended Set to Resume Queue. Signal: `command.resume`.

## Definition

```aljam3
{N} -Q.Resume
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QResume"
   [%] .description << "Move from Suspended Set to Resume Queue."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Conditional Variants

Resume when a resource condition recovers. Used as nested `[Q]` lines in `{Q}` definitions or pipeline `[Q]` sections.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `-Q.Resume.RAM.MoreThan` | `<mb#Float` | Resume when RAM recovers above threshold |
| `-Q.Resume.CPU.LessThan` | `<percent#Float` | Resume when CPU drops below threshold |
| `-Q.Resume.Disk.MoreThan` | `<mb#Float` | Resume when disk space recovers |
| `-Q.Resume.GPU.Free` | (none) | Resume when GPU becomes available |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Resume` | Compile-time pipeline template |
| Instance | `%Q:Resume:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Q/Pause.Soft|-Q.Pause.Soft]]
- [[jm3lib/pipelines/Q/Pause.Hard|-Q.Pause.Hard]]
- [[jm3lib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
