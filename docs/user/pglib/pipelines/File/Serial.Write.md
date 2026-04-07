---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =File.Serial.Write

Serialize a data tree to a structured data file. Format is auto-detected from the file extension (.json, .yaml, .toml).

## Definition

```polyglot
{N} =File.Serial.Write
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileSerialWrite"
   [%] .description << "Serialize data tree to file"
   [=] <path#path
   [=] <data#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the file to write |
| `<data` | `#serial` | Data tree to serialize |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | Parent directory does not exist |
| `!File.WriteError` | Cannot write to path (permissions, disk full) |

## Permissions

Requires `File.Write` capability.

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/Serial.Read|=File.Serial.Read]]
- [[pglib/pipelines/Schema/INDEX|=# Base Parsers]]
