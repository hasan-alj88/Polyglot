---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Nullable"
---

# ##Nullable Schema (Parameterized)

<!-- @c:types -->

`##Nullable` is a parameterized schema that creates a two-branch structure: `.Ok` holding a value of the specified type, or `.None` representing absence.

## Definition

```polyglot
{#} ##Nullable
   (#) <#ValueType
   [#] %##Active << #ActiveKind.One
   [.] .Ok
      [.] .Value;#ValueType
   [.] .None
```

## Usage

```polyglot
{#} #MaybeInt
   [#] ##Nullable
      (#) <#ValueType << #Int
   [ ] .Ok.Value;int OR .None
```

The compiler validates that exactly one branch is active at any time (`%##Active << .One`). This provides type-safe null handling without sentinel values.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Nullable` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Result|##Result]] -- similar pattern with error branch instead of None
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Active` property
