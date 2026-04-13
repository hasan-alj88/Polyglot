---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Serial.Read.Field"
metadata_instance: "%-:File.Serial.Read.Field:N"
---

# -File.Serial.Read.Field

One-step field extraction: reads a structured data file, parses it, and extracts a single field by tree path. Combines `-File.Serial.Read` + `-#.Field` into a single pipeline.

The `<field` input uses `<` as the path separator (e.g. `"database<host"` extracts the `host` field under `database`).

## Definition

```polyglot
{N} -File.Serial.Read.Field
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileSerialReadField"
   [%] .description << "One-step field extraction: reads file, parses, extracts single field by tree path"
   (-) <path#path
   (-) <field#RawString
   (-) >value#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the structured data file |
| `<field` | `#RawString` | Tree path using `<` separator (e.g. `"database<host"`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>value` | `#serial` | Extracted field value |

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | File does not exist at path |
| `!File.ReadError` | Cannot read file |
| `!File.ParseError` | File content is not valid JSON/YAML/TOML |
| `!Field.NotFound` | Field path does not exist in parsed data |

## Permissions

Requires `File.Read` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Serial.Read.Field` | Compile-time pipeline template |
| Instance | `%-:File.Serial.Read.Field:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/File/INDEX|-File.* File Pipelines]]
- [[pglib/pipelines/File/Serial.Read|-File.Serial.Read]]
- [[pglib/pipelines/Schema/INDEX|-# Base Parsers]]
