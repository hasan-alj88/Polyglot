---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
---

# -# — Schema Validation & Field Extraction

> This folder is named `Schema/` because `#` is filesystem-unfriendly. Pipelines use the `-#.*` namespace.

<!-- @c:types -->
<!-- @c:errors -->
The `-#` namespace groups schema-related pipelines. The `=` is the pipeline prefix; `#` is the first name segment (referring to schema/type operations). This follows the same convention as `-W.*` (wrappers), `-Q.*` (queues), and `-T.*` (triggers) -- no prefix collision.

aj3lib pipelines for schema validation, field extraction, format parsing, and dataframe column extraction. No `[@]` import needed. See [[errors#Pipeline Error Associations]] for error contracts.

Types are data trees (GT-1). The `<#type` syntax passes a type definition's `%` metadata tree as pipeline input -- extending `<#` from `{#}` generic type inputs to `{-}` pipeline IO. Works with any tier: `#Config` (struct), `##Scalar` (schema), `###Enum` (property). See [[syntax/types/generic-types#<#type in Pipeline IO]].

## Native Parsers (Compiler Intrinsics)

- [[aj3lib/pipelines/Schema/JSON.Parse|-#.JSON.Parse]] -- Parse JSON string into serial data tree
- [[aj3lib/pipelines/Schema/YAML.Parse|-#.YAML.Parse]] -- Parse YAML string into serial data tree
- [[aj3lib/pipelines/Schema/TOML.Parse|-#.TOML.Parse]] -- Parse TOML string into serial data tree

## Schema Validation

- [[aj3lib/pipelines/Schema/Match|-#.Match]] -- Boolean schema check
- [[aj3lib/pipelines/Schema/Validate|-#.Validate]] -- Detailed validation with error reporting
- [[aj3lib/pipelines/Schema/Describe|-#.Describe]] -- Schema introspection
- [[aj3lib/pipelines/Schema/Coerce|-#.Coerce]] -- Best-effort type conversion

## Field & Column Extraction

- [[aj3lib/pipelines/Schema/Field|-#.Field]] -- Extract single field from data tree by path
- [[aj3lib/pipelines/Schema/Column|-#.Column]] -- Extract column values from row-oriented Dataframe

## Implementation Status

| Pipeline | Status |
|---|---|
| `-#.JSON.Parse` | Deferred (compiler intrinsic) |
| `-#.YAML.Parse` | Deferred (compiler intrinsic) |
| `-#.TOML.Parse` | Deferred (compiler intrinsic) |
| `-#.Match` | Deferred |
| `-#.Validate` | Deferred |
| `-#.Describe` | Deferred |
| `-#.Coerce` | Deferred |
| `-#.Field` | Deferred |
| `-#.Column` | Deferred |

## Related

- [[aj3lib/pipelines/INDEX|aj3lib Pipeline Index]]
- [[aj3lib/pipelines/File/INDEX|-File.* File Pipelines]]
- [[syntax/types/generic-types#<#type in Pipeline IO|<#type in Pipeline IO]]
- [[errors#Pipeline Error Associations|Pipeline Error Associations]]
