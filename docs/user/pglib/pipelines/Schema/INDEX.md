---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =# — Schema Validation & Field Extraction

> This folder is named `Schema/` because `#` is filesystem-unfriendly. Pipelines use the `=#.*` namespace.

<!-- @types -->
<!-- @errors -->
The `=#` namespace groups schema-related pipelines. The `=` is the pipeline prefix; `#` is the first name segment (referring to schema/type operations). This follows the same convention as `=W.*` (wrappers), `=Q.*` (queues), and `=T.*` (triggers) -- no prefix collision.

pglib pipelines for schema validation, field extraction, format parsing, and dataframe column extraction. No `[@]` import needed. See [[errors#Pipeline Error Associations]] for error contracts.

Types are data trees (GT-1). The `<#type` syntax passes a type definition's `%` metadata tree as pipeline input -- extending `<#` from `{M}` macro type inputs to `{=}` pipeline IO. Works with any tier: `#Config` (struct), `##Scalar` (schema), `###Enum` (property). See [[syntax/types/macro-types#<#type in Pipeline IO]].

## Native Parsers (Compiler Intrinsics)

- [[pglib/pipelines/Schema/JSON.Parse|=#.JSON.Parse]] -- Parse JSON string into serial data tree
- [[pglib/pipelines/Schema/YAML.Parse|=#.YAML.Parse]] -- Parse YAML string into serial data tree
- [[pglib/pipelines/Schema/TOML.Parse|=#.TOML.Parse]] -- Parse TOML string into serial data tree

## Schema Validation

- [[pglib/pipelines/Schema/Match|=#.Match]] -- Boolean schema check
- [[pglib/pipelines/Schema/Validate|=#.Validate]] -- Detailed validation with error reporting
- [[pglib/pipelines/Schema/Describe|=#.Describe]] -- Schema introspection
- [[pglib/pipelines/Schema/Coerce|=#.Coerce]] -- Best-effort type conversion

## Field & Column Extraction

- [[pglib/pipelines/Schema/Field|=#.Field]] -- Extract single field from data tree by path
- [[pglib/pipelines/Schema/Column|=#.Column]] -- Extract column values from row-oriented Dataframe

## Implementation Status

| Pipeline | Status |
|---|---|
| `=#.JSON.Parse` | Deferred (compiler intrinsic) |
| `=#.YAML.Parse` | Deferred (compiler intrinsic) |
| `=#.TOML.Parse` | Deferred (compiler intrinsic) |
| `=#.Match` | Deferred |
| `=#.Validate` | Deferred |
| `=#.Describe` | Deferred |
| `=#.Coerce` | Deferred |
| `=#.Field` | Deferred |
| `=#.Column` | Deferred |

## Related

- [[pglib/pipelines/INDEX|pglib Pipeline Index]]
- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[syntax/types/macro-types#<#type in Pipeline IO|<#type in Pipeline IO]]
- [[errors#Pipeline Error Associations|Pipeline Error Associations]]
