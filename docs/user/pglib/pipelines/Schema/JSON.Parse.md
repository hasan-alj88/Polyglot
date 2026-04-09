---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:#.JSON.Parse"
metadata_instance: "%-:#.JSON.Parse:N"
---

# -#.JSON.Parse

Parses a JSON string into a `#serial` data tree. Compiler intrinsic -- not user-definable.

## Definition

```polyglot
{N} -#.JSON.Parse
   [%] .Kind << #NativeKind.Compiler
   [%] .Rust << "SchemaJsonParse"
   [%] .description << "Parse JSON string into serial data tree"
   (-) <raw#RawString
   (-) >data#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<raw` | `#RawString` | Raw JSON text |

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
| Definition | `%definition.-:#.JSON.Parse` | Compile-time pipeline template |
| Instance | `%-:#.JSON.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Schema/INDEX|-# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/YAML.Parse|-#.YAML.Parse]]
- [[pglib/pipelines/Schema/TOML.Parse|-#.TOML.Parse]]
