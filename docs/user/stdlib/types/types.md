---
audience: user
type: specification
updated: 2026-03-28
status: complete
---

# Built-in Types (#)

<!-- @types -->

Stdlib structs and enums available in every `.pg` file. No `[@]` import needed. For the full type system specification (annotations, schemas, generics), see [[types|user/syntax/types]].

## Type Hierarchy

```
RawString (compiler intrinsic)
└── #String (foundation — .string + .re) [##Scalar, ###Value]
    ├── #Int (.re = signed integers)
    ├── #UnsignedInt (.re = non-negative integers)
    ├── #Float (.re = decimals)
    ├── #Sci (.re = scientific notation)
    ├── #Eng (.re = engineering notation)
    ├── #Dimension (.re = dimension values — allows 0D)
    ├── #KeyString (.re = syntax-safe keys)
    ├── #NestedKeyString (.re = alias-safe paths)
    └── (user-defined: #emailAddress, #phoneNumber, etc.)

#Boolean (independent enum struct — NOT #String) [##Scalar, ###Enum]

#Map:KeyType:ValueType (macro-generated — sparse, homogeneous key-value pairs)
#Array:ValueType:Dim (macro-generated — contiguous, rectangular, N-dimensional — #Map variant)
#Dataframe:ColumnEnum:CellType (macro-generated — row-oriented, Array of Map)
#Serial (schema-free, unlimited depth)
```

## Category Index

| Category | File | Types |
|----------|------|-------|
| Foundation | [string.md](string.md) | #String |
| Scalar subtypes | [scalars.md](scalars.md) | #Int, #UnsignedInt, #Float, #Sci, #Eng, #Dimension, #KeyString, #NestedKeyString |
| Boolean | [boolean.md](boolean.md) | #Boolean, #None |
| Collections | [collections.md](collections.md) | #Map, #Array, #Dataframe, #Serial |
| Enums | [enums.md](enums.md) | #OS, #PipelineStatus, #QueueStrategy, #RetriggerStrategy, #QueueState, #FileAccess, #VarState |
| Structs | [structs.md](structs.md) | #path, #Queue |

## Related

- [[types|user/syntax/types]] — full type system specification (annotations, schemas, generics, `##`/`###` prefixes)
- [[data-is-trees|user/concepts/data-is-trees]] — how types relate to the unified tree
- [[collections|user/concepts/collections]] — expand/collect operations on collections
- [[EDGE-CASES#24. Datatype Definitions|technical/EDGE-CASES]] — edge cases for type definitions
