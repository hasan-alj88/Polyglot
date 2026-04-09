---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:String"
---

# ##String Schema (Parameterized)

<!-- @types -->

`##String` is a parameterized schema that generates a string type with `.string` and `.regex` fields. The `<regex` parameter sets the validation pattern. All `#String` subtypes compose this schema.

## Definition

```polyglot
{#} ##String
   [#] <regex
   [#] << ##Scalar
   [#] << ###ScalarValue
```

## Usage

```polyglot
{#} #Int
   [%] %alias << "int,integer,Integer"
   [#] << ##String
      [#] <regex << "^-?[0-9]+$"
   [ ] .string#RawString validated by .regex
```

The schema generates `.string#RawString` and `.regex#RawString` fields. The compiler validates that `.string` values match the `.regex` pattern. All `#String:*` scalar subtypes use this schema.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:String` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Scalar|##Scalar]] -- depth 1 constraint (composed by ##String)
- [[string]] -- `#String` foundation type
- [[scalars]] -- scalar subtypes using ##String
