---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:#.YAML.Parse"
metadata_instance: "%-:#.YAML.Parse:N"
---

# -#.YAML.Parse

Parses a YAML string into a `#serial` data tree. Compiler intrinsic -- not user-definable.

## Definition

```aljam3
{N} -#.YAML.Parse
   [%] .Kind << #NativeKind.Compiler
   [%] .Rust << "SchemaYamlParse"
   [%] .description << "Parse YAML string into serial data tree"
   (-) <raw#RawString
   (-) >data#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<raw` | `#RawString` | Raw YAML text |

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
| Definition | `%definition.-:#.YAML.Parse` | Compile-time pipeline template |
| Instance | `%-:#.YAML.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Schema/INDEX|-# Schema Validation & Field Extraction]]
- [[aj3lib/pipelines/Schema/JSON.Parse|-#.JSON.Parse]]
- [[aj3lib/pipelines/Schema/TOML.Parse|-#.TOML.Parse]]
