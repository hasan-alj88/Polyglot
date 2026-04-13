---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:String"
---

# ##String Schema (Parameterized)

<!-- @c:types -->

`##String` is a parameterized schema that generates a string type with `.string` and `.regex` fields. The `<regex` parameter sets the validation pattern. All `#String` subtypes compose this schema.

## Definition

```polyglot
{#} ##String
   (#) <regex
   [#] ##Scalar
   [#] ###ScalarValue
```

## Allows

```
#Int [##String with regex "^-?[0-9]+$"]
├── .string  -> "42"       ← matches regex ✓
└── .regex   -> "^-?[0-9]+$"

#Int
├── .string  -> "-7"       ← matches regex ✓
└── .regex   -> "^-?[0-9]+$"
```

## Disallows

```
#Int [##String with regex "^-?[0-9]+$"]
├── .string  -> "hello"    ✗ does not match "^-?[0-9]+$"
└── .regex   -> "^-?[0-9]+$"

#Int [##String]
├── .string  -> "42"
├── .regex   -> "^-?[0-9]+$"
└── .extra   -> "metadata" ✗ ##Scalar (depth 1) — only .string and .regex allowed
```

## Usage

```polyglot
{#} #Int
   [%] %alias << "int,integer,Integer"
   [#] ##String
      (#) <regex << "^-?[0-9]+$"
   [ ] .string#RawString validated by .regex
```

The schema generates `.string#RawString` and `.regex#RawString` fields. The compiler validates that `.string` values match the `.regex` pattern. All `#String:*` scalar subtypes use this schema.

## Used By

<!-- @u:pglib/types/string -->
<!-- @u:pglib/types/scalars -->

- [[string|#String]] foundation type
- All [[scalars|scalar subtypes]] ([[scalars|#Int]], [[scalars|#Float]], etc.)

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
