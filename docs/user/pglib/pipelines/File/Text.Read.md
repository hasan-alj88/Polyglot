---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Text.Read"
metadata_instance: "%-:File.Text.Read:N"
---

# -File.Text.Read

Read the full text content of a file at the given path.

## Definition

```aljam3
{N} -File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   (-) <path#path
   (-) >content#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the file to read |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>content` | `#string` | Full text content of the file |

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | File does not exist at path |
| `!File.ReadError` | File exists but cannot be read (permissions, locked) |

## Permissions

Requires `File.Read` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Text.Read` | Compile-time pipeline template |
| Instance | `%-:File.Text.Read:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/File/INDEX|-File.* File Pipelines]]
