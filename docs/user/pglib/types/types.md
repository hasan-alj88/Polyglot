---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
---

# Built-in Types (#)

<!-- @c:types -->

pglib structs and enums available in every `.pg` file. No `[@]` import needed. For the full type system specification (annotations, schemas, generics), see [[syntax/types/INDEX|types]].

## Type Hierarchy

```polyglot
RawString (compiler intrinsic) [##Leaf]
└── #String (foundation — .string + .regex) [##Scalar, ###ScalarValue]
    ├── #Int (.regex = signed integers)
    ├── #UnsignedInt (.regex = non-negative integers)
    ├── #Float (.regex = decimals)
    ├── #Sci (.regex = scientific notation)
    ├── #Eng (.regex = engineering notation)
    ├── #Dimension (.regex = dimension values — allows 0D)
    ├── #KeyString (.regex = syntax-safe keys)
    ├── #NestedKeyString (.regex = alias-safe paths)
    └── (user-defined: #emailAddress, #phoneNumber, etc.)

#Boolean (##Enum type — NOT #String) [##Enum, ##Scalar, ###ScalarEnum]

#Map:KeyType:ValueType (generic — sparse, flexible key-value pairs)
#Array:ValueType:Dim (generic — contiguous, rectangular, N-dimensional)
#Dataframe:ColumnEnum:CellType (generic — row-oriented, Array of Map)
#Serial (unconstrained, unlimited depth)

#FieldsDescriptor (##Enum — child field descriptor)
#ActiveKind (##Enum — branch activation classifier)
#Bound (##Int + ##Inf — numeric value or infinity)

#Code:<Lang>.Output (runtime execution output — .stdout, .stderr, .return)
#PyEnv (Python runtime environment handle)
#RsEnv (Rust runtime environment handle)
```

## Category Index

| Category | File | Types |
|----------|------|-------|
| Foundation | [string.md](string.md) | #String |
| Scalar subtypes | [scalars.md](scalars.md) | #Int, #UnsignedInt, #Float, #Sci, #Eng, #Dimension, #KeyString, #NestedKeyString |
| Boolean | [boolean.md](boolean.md) | #Boolean, #None |
| Collections | [collections.md](collections.md) | #Map, #Array, #Dataframe, #Set, #Serial |
| Enums | [enums.md](enums.md) | #OS, #PipelineStatus, #QueueStrategy, #RetriggerStrategy, #QueueState, #KillPropagation, #ResourceTag, #FileAccess, #VarState, #FieldKind, #FieldsDescriptor, #ActiveKind |
| Structs | [structs.md](structs.md) | #path, #Queue |
| Text & Merge | [TextDiff.md](TextDiff.md), [DiffStats.md](DiffStats.md), [MergeConflict.md](MergeConflict.md), [MergeResult.md](MergeResult.md) | #TextDiff, #TextDiffs, #DiffStats, #MergeConflict, #MergeResult |
| Schema types | [Bound.md](Bound.md) | #Bound |
| Runtime | [rt.md](rt.md) | #Code, #PyEnv, #RsEnv |

## Related

- [[syntax/types/INDEX|types]] — full type system specification (annotations, schemas, generics, `##`/`###` prefixes)
- [[data-is-trees|user/concepts/data-is-trees]] — how types relate to the unified tree
- [[concepts/collections/INDEX|collections]] — expand/collect operations on collections
- [[technical/edge-cases/24-datatype-defs|Edge Cases: Datatype Definitions]] — edge cases for type definitions
