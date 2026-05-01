---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Copy"
metadata_instance: "%-:File.Copy:N"
---

# -File.Copy

Copy a file from one path to another.

## Definition

```aljam3
{N} -File.Copy
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileCopy"
   [%] .description << "Copy file"
   (-) <source#path
   (-) <destination#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<source` | `#path` | Path to the source file |
| `<destination` | `#path` | Path to the destination file |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | Source file does not exist |
| `!File.CopyError` | Cannot complete copy (permissions, disk full) |

## Permissions

Requires `File.Read` + `File.Write` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Copy` | Compile-time pipeline template |
| Instance | `%-:File.Copy:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/File/INDEX|-File.* File Pipelines]]
- [[jm3lib/pipelines/File/Move|-File.Move]]
