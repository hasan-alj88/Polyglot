---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Result"
---

# ##Result Schema (Parameterized)

<!-- @types -->

`##Result` is a parameterized schema that creates a two-branch structure: `.OK` holding a success value, or `.Err` holding an error value. Exactly one branch is active at any time.

## Definition

```polyglot
{#} ##Result
   [#] <#OkType
   [#] <#ErrType
   [#] %##Active << #ActiveKind.One
```

## Usage

```polyglot
{#} #ParseResult
   [#] << ##Result
      [#] <#OkType << #Int
      [#] <#ErrType << !Validation.Type
   [ ] .OK.Value#Int OR .Err.Value#!Validation.Type
```

The compiler validates that exactly one branch is active at any time (`%##Active << .One`). This provides type-safe error handling at the data level.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Result` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Nullable|##Nullable]] -- similar pattern with None instead of error
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Active` property
