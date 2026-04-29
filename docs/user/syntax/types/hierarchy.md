---
audience: automation-builder
type: specification
updated: 2026-04-09
---

# Type Hierarchy

<!-- @syntax/types/INDEX -->

## Live Type Modifier

`live` is a type modifier reserved for `[%]` metadata fields managed by the Aljam3 runtime. Users can read `live` fields via the `%` accessor but never assign to them. The type uses dot notation: `#live.#PipelineStatus`, `#live.int`, `#live.array:error`.

`live` fields are **implicit** on every `{-}` pipeline, `$` variable, and `{#}` struct. They do not need to be declared -- the runtime populates them automatically and updates them in real-time.

See [[metadata]] for the full metadata tree, all `live` field listings, and access patterns.

## Namespaced Types

Types use dot notation for namespaces -- these are fixed schema fields. Namespacing is optional for basic types but available when needed (e.g., referencing enumeration fields from `{#}` definitions).

```aljam3
[ ] Direct type annotation -- most common
[-] $score#int <~ 0

[ ] Fully qualified -- equivalent to the above
[-] $score#String:int <~ 0

[ ] Struct enum field -- must use # outside type annotations
[-] $severity << #Severity.Critical

[ ] Cross-package reference -- @alias#DataName.Field
[-] $status << @alerts#Severity.Error
```

In type annotations (after `#`), nested type refs drop the `#` prefix -- the compiler knows `#` starts a type context. Outside annotations, struct references keep the `#` prefix. See [[identifiers#Serialized Identifiers]] for the full prefix rules.

## Type Hierarchy Summary

```aljam3
RawString (compiler intrinsic) [##Leaf]
+-- #String (foundation -- .string + .regex) [##Scalar, ###ScalarValue]
|   +-- #Int (.regex = signed integers) -- composes ##String
|   +-- #UnsignedInt (.regex = non-negative integers)
|   +-- #Float (.regex = decimals)
|   +-- #Sci (.regex = scientific notation)
|   +-- #Eng (.regex = engineering notation)
|   +-- #Dimension (.regex = dimension values)
|   +-- #KeyString (.regex = syntax-safe keys)
|   +-- #NestedKeyString (.regex = alias-safe paths)
|   +-- #CommaSeparatedList (.regex = comma-separated identifiers)
|   +-- #DataTypeString (.regex = valid {x} definition names)
|   +-- #Email (.regex = email addresses)
|   +-- (user-defined: #phoneNumber, #zipCode, etc.)

#Boolean (##Enum type -- NOT #String) [##Enum, ##Scalar, ###ScalarEnum]
#None (absence of value -- empty string "") [##Scalar, ###None]

#Map:KeyType:ValueType (generic -- sparse key-value pairs)
#Array:ValueType:Dim (generic -- contiguous, rectangular, N-dimensional)
#Dataframe:Columns:CellType (generic -- row-oriented, Array of Map)
#Set:ValueType (generic -- unique value collection)
#Serial (unconstrained, unlimited depth)

#FlexKind (##Enum -- branch flexibility classifier)
#ActiveKind (##Enum -- branch activation classifier)
#Bound (##Int + ##Inf -- numeric value or infinity)
```

## See Also

- [[syntax/types/basic-types|Basic Types]] -- RawString, #String, scalar subtypes, and #Boolean
- [[syntax/types/schema-properties|Schema Properties]] -- `%##` / `%###` property reference
- [[syntax/types/prefix-system|Prefix System]] -- three-tier `#`/`##`/`###` prefix details
