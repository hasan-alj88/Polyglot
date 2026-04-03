---
audience: pg-coder
type: specification
updated: 2026-03-30
---

# Type Hierarchy

<!-- @syntax/types/INDEX -->

## Live Type Modifier

`live` is a type modifier reserved for `[%]` metadata fields managed by the Polyglot runtime. Users can read `live` fields via the `%` accessor but never assign to them. The type uses dot notation: `#live.#PipelineStatus`, `#live.int`, `#live.array:error`.

`live` fields are **implicit** on every `{=}` pipeline, `$` variable, and `{#}` struct. They do not need to be declared — the runtime populates them automatically and updates them in real-time.

See [[metadata]] for the full metadata tree, all `live` field listings, and access patterns.

## Namespaced Types

Types use dot notation for namespaces — these are fixed schema fields. Namespacing is optional for basic types but available when needed (e.g., referencing enumeration fields from `{#}` definitions).

```polyglot
[ ] Direct type annotation — most common
[r] $score#int <~ 0

[ ] Fully qualified — equivalent to the above
[r] $score#String:int <~ 0

[ ] Struct enum field — must use # outside type annotations
[r] $severity << #Severity.Critical

[ ] Cross-package reference — @alias#DataName.Field
[r] $status << @alerts#Severity.Error
```

In type annotations (after `#`), nested type refs drop the `#` prefix — the compiler knows `#` starts a type context. Outside annotations, struct references keep the `#` prefix. See [[identifiers#Serialized Identifiers]] for the full prefix rules.

## Type Hierarchy Summary

```
RawString (compiler intrinsic — Layer 0)
└── #String (foundation — .string + .regex) [##Scalar, ###Value] (Layer 0)
    ├── ##Int (.regex = signed integers) — macro-generated via {M} #String.Subtype (Layer 1)
    ├── ##UnsignedInt (.regex = non-negative integers) (Layer 1)
    ├── ##Float (.regex = decimals) (Layer 1)
    ├── ##Sci (.regex = scientific notation) (Layer 1)
    ├── ##Eng (.regex = engineering notation) (Layer 1)
    ├── ##Dimension (.regex = dimension values) (Layer 1)
    ├── ##KeyString (.regex = syntax-safe keys) (Layer 1)
    ├── ##NestedKeyString (.regex = alias-safe paths) (Layer 1)
    ├── ##CommaSeparatedList (.regex = comma-separated identifiers) (Layer 1)
    ├── ##DataTypeString (.regex = valid {x} definition names) (Layer 1)
    └── (user-defined: #emailAddress, #phoneNumber, etc.)

#Boolean (independent enum struct — NOT #String) [##Scalar, ###Enum]
#None (absence of value — empty string "") [##Scalar, ###None]

#Map (sparse key-value pairs — macro-generated via {M} #Map) (Layer 2)
#Array (contiguous, rectangular, N-dimensional — macro-generated via {M} #Array) (Layer 2)
#Dataframe (row-oriented table — array of maps, macro-generated via {M} #Dataframe) (Layer 2)
#Serial (schema-free, unlimited depth — plain {#}, no macro)
```

## See Also

- [[syntax/types/basic-types|Basic Types]] — RawString, #String, scalar subtypes, and #Boolean
- [[syntax/types/macro-types|Macro-Generated Types]] — how `{M}` macros generate collection types
- [[syntax/types/prefix-system|Prefix System]] — three-tier `#`/`##`/`###` prefix details
