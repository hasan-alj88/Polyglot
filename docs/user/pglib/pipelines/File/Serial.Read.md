---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =File.Serial.Read

Read and parse a structured data file. Format is auto-detected from file extension (.json, .yaml, .toml). Internally delegates to `=#.JSON.Parse`, `=#.YAML.Parse`, or `=#.TOML.Parse` base parsers (see [[pglib/pipelines/Schema/INDEX|pipelines/Schema]]).

## Definition

```polyglot
{N} =File.Serial.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileSerialRead"
   [%] .description << "Read and parse structured data file (JSON/YAML/TOML, auto-detected from extension)"
   [=] <path#path
   [=] >data#serial
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

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/Serial.Write|=File.Serial.Write]]
- [[pglib/pipelines/File/Serial.Read.Field|=File.Serial.Read.Field]]
- [[pglib/pipelines/Schema/INDEX|=# Base Parsers]]
