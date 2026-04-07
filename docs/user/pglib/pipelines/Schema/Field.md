---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =#.Field

Extracts a single field from a `#serial` data tree by path. Like tree access (`$data<key<subkey`) but with error handling for missing paths.

## Definition

```polyglot
{N} =#.Field
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "SchemaField"
   [%] .description << "Extract single field from data tree by path"
   [=] <data#serial
   [=] <path#RawString
   [=] >value#serial
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<data` | `#serial` | Data tree to extract from |
| `<path` | `#RawString` | Tree path using `<` separator (e.g. `"database<host"`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>value` | `#serial` | Extracted value |

## Errors

| Error | When |
|-------|------|
| `!Field.NotFound` | Path doesn't exist in data tree |
| `!Field.PathError` | Invalid path syntax |

## Three Approaches to Field Access

User picks based on needs:

| Approach | When to use |
|----------|-------------|
| `$data<database<host` | Direct tree access -- fast, no error handling on missing field |
| `=#.Field` | Safe extraction -- error handling with `[!]` fallback chains |
| `=File.Serial.Read.Field` | Single step from file to field -- combines read + parse + extract |

## Permissions

None -- pure computation.

## Related

- [[pglib/pipelines/Schema/INDEX|=# Schema Validation & Field Extraction]]
- [[pglib/pipelines/Schema/Column|=#.Column]]
- [[pglib/pipelines/File/Serial.Read.Field|=File.Serial.Read.Field]]
