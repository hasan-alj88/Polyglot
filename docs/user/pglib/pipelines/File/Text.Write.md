---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Text.Write"
metadata_instance: "%-:File.Text.Write:N"
---

# -File.Text.Write

Write text content to a file, replacing any existing content.

## Definition

```polyglot
{N} -File.Text.Write
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextWrite"
   [%] .description << "Write text to file"
   (-) <path#path
   (-) <content#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the file to write |
| `<content` | `#string` | Text content to write |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | Parent directory does not exist |
| `!File.WriteError` | Cannot write to path (permissions, disk full) |

## Permissions

Requires `File.Write` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Text.Write` | Compile-time pipeline template |
| Instance | `%-:File.Text.Write:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/File/INDEX|-File.* File Pipelines]]
