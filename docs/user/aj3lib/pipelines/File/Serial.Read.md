---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Serial.Read"
metadata_instance: "%-:File.Serial.Read:N"
---

# -File.Serial.Read

Read and parse a structured data file. Format is auto-detected from file extension (.json, .yaml, .toml). Internally delegates to `-#.JSON.Parse`, `-#.YAML.Parse`, or `-#.TOML.Parse` base parsers (see [[aj3lib/pipelines/Schema/INDEX|pipelines/Schema]]).

## Definition

```aljam3
{N} -File.Serial.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileSerialRead"
   [%] .description << "Read and parse structured data file (JSON/YAML/TOML, auto-detected from extension)"
   (-) <path#path
   (-) >data#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the structured data file |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>data` | `#serial` | Parsed data tree |

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | File does not exist at path |
| `!File.ReadError` | File exists but cannot be read (permissions, locked) |
| `!File.ParseError` | File content is not valid JSON/YAML/TOML |

## Permissions

Requires `File.Read` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Serial.Read` | Compile-time pipeline template |
| Instance | `%-:File.Serial.Read:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/File/INDEX|-File.* File Pipelines]]
- [[aj3lib/pipelines/File/Serial.Write|-File.Serial.Write]]
- [[aj3lib/pipelines/File/Serial.Read.Field|-File.Serial.Read.Field]]
- [[aj3lib/pipelines/Schema/INDEX|-# Base Parsers]]
