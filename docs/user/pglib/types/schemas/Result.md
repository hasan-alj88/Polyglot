---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Result"
---

# ##Result Schema (Parameterized)

<!-- @c:types -->

`##Result` is a parameterized schema that creates a two-branch structure: `.Ok` holding a success value, or `.Err` holding an error value. Exactly one branch is active at any time.

## Definition

```polyglot
{#} ##Result
   (#) <#OkType
   (#) <#ErrType
   [#] %##Active << #ActiveKind.One
   [.] .Ok
      [.] .Value;#OkType
   [.] .Err
      [.] .Value;#ErrType
```

## Allows

```
#ParseResult [##Result:#Int,!Validation.Type]
├── .Ok                    ✓ active
│   └── .Value -> 42#int
└── .Err                   ○ inactive
│   └── .Value
                            success — .Ok active

#ParseResult [##Result:#Int,!Validation.Type]
├─�� .Ok                    ○ inactive
│   └── .Value
└── .Err                   ✓ active
    └── .Value -> !Validation.Type
                            failure — .Err active
```

## Disallows

```
#ParseResult [##Result:#Int,!Validation.Type]
├── .Ok                    ✓ active
│   └── .Value -> 42#int
└─��� .Err                   ✓ active
    └── .Value -> !Validation.Type
                           ✗ two branches — #One requires exactly one

#ParseResult [##Result:#Int,!Validation.Type]
├���─ .Ok
│   └── .Value -> "hi"#string  ✗ #string — <#OkType is #Int
└── .Err                   ○ inactive
```

## Usage

```polyglot
{#} #ParseResult
   [#] ##Result
      (#) <#OkType << #Int
      (#) <#ErrType << !Validation.Type
   [ ] .Ok.Value;int OR .Err.Value;!Validation.Type
```

The compiler validates that exactly one branch is active at any time (`%##Active << .One`). This provides type-safe error handling at the data level.

## Used By

User-defined result types compose this schema (e.g., `#ParseResult`, `#FetchResult`).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Result` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Nullable|##Nullable]] -- similar pattern with None instead of error
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Active` property
