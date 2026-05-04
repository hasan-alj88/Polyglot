---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:#.TOML.Parse"
metadata_instance: "%-:#.TOML.Parse:N"
---

# -#.TOML.Parse

Parses a TOML string into a `#serial` data tree. Compiler intrinsic -- not user-definable.

## Definition

```aljam3
{N} -#.TOML.Parse
   [%] .Kind << #NativeKind.Compiler
   [%] .Rust << "SchemaTomlParse"
   [%] .description << "Parse TOML string into serial data tree"
   (-) <raw#RawString
   (-) >data#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<raw` | `#RawString` | Raw TOML text |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>data` | `#serial` | Parsed data tree |

## Errors

None -- invalid input handled by calling pipeline (e.g. `-File.Serial.Read` raises `!File.ParseError`).

## Permissions

None -- pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:#.TOML.Parse` | Compile-time pipeline template |
| Instance | `%-:#.TOML.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Schema/INDEX|-# Schema Validation & Field Extraction]]
- [[jm3lib/pipelines/Schema/JSON.Parse|-#.JSON.Parse]]
- [[jm3lib/pipelines/Schema/YAML.Parse|-#.YAML.Parse]]
