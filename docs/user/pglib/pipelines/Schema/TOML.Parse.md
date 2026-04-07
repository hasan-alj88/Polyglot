---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =#.TOML.Parse

Parses a TOML string into a `#serial` data tree. Compiler intrinsic -- not user-definable.

## Definition

```polyglot
{N} =#.TOML.Parse
   [%] .Kind << #NativeKind.Compiler
   [%] .Rust << "SchemaTomlParse"
   [%] .description << "Parse TOML string into serial data tree"
   [=] <raw#RawString
   [=] >data#serial
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

None -- invalid input handled by calling pipeline (e.g. `=File.Serial.Read` raises `!File.ParseError`).

## Permissions

None -- pure computation.

## Related

- [[pglib/pipelines/Schema/INDEX|=# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/JSON.Parse|=#.JSON.Parse]]
- [[pglib/pipelines/Schema/YAML.Parse|=#.YAML.Parse]]
