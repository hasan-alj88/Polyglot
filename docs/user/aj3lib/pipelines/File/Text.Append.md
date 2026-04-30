---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Text.Append"
metadata_instance: "%-:File.Text.Append:N"
---

# -File.Text.Append

Append text content to the end of an existing file.

## Definition

```aljam3
{N} -File.Text.Append
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextAppend"
   [%] .description << "Append text to file"
   (-) <path#path
   (-) <content#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the file to append to |
| `<content` | `#string` | Text content to append |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | File does not exist at path |
| `!File.WriteError` | Cannot write to path (permissions, disk full) |

## Permissions

Requires `File.Write` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Text.Append` | Compile-time pipeline template |
| Instance | `%-:File.Text.Append:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/File/INDEX|-File.* File Pipelines]]
