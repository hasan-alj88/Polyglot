---
audience: pg-coder
type: specification
updated: 2026-03-30
---

# Struct Types

<!-- @syntax/types/INDEX -->
<!-- @blocks -->
<!-- @identifiers -->

## Struct Types

A **struct** is any type with a defined schema, declared with `{#}` (see [[blocks]]). The term "struct" refers to the type category — `{#}` is the declaration syntax. Every struct definition creates a schema template at `%definition.#:{StructName}` and instances at `%#:{StructName}:{n}` — see [[data-is-trees#Schema vs Instance]].

pglib structs with value fields include `#path`, `#Queue`, and `#DateTime` (see [[pglib/types/structs|pglib structs]] and [[pglib/types/datetime/INDEX|datetime]]). Enum types (`#Boolean`, `#OS`, `#PipelineStatus`, `#VarState`) are also `{#}` structs but documented separately — see [[pglib/types/enums|enums]] and [[pglib/types/boolean|boolean]]. User-defined structs follow the same rules.

In type annotations (after `#`), nested type refs drop the `#` prefix — the compiler knows `#` starts a type context:

```polyglot
[-] $user#UserRecord <~ ...
[-] $users#array:UserRecord <~ {}
(-) <incoming#Alert
```

The `#` prefix is only used when **referencing** a struct outside of type annotations (e.g., `#Boolean.True`, `@alias#DataName.EnumField`).

### Struct Level Rules

Each level in a struct must be homogeneous in two ways (see [[identifiers#Serialization Rules]]):

1. **Separator homogeneity** — all siblings at a level must be all fixed (`.`) or all flexible (`:`) — never mixed (PGE05001)
2. **Kind homogeneity** — all siblings at a level must be all enum fields or all value fields — never mixed (PGE05002)

A field whose type is itself a struct defines the later levels. It is invalid to declare sub-levels after a field typed as a struct — the struct's definition already specifies those levels.

```polyglot
{#} #Config
   [.] .timeout#int
   [.] .server#ServerInfo      [ ] later levels defined by #ServerInfo

{#} #ServerInfo
   [.] .host#string
   [.] .port#int
```

## Inline Data Shorthand

Curly braces with comma-separated values create inline data:

```polyglot
[-] $values#array <~ {1, 2, 3, 4, 5}
```

This is shorthand for explicit field assignment: `#data:1 << 1`, `#data:2 << 2`, etc.

## Enum Fields vs Value Fields

In `{#}` struct definitions, fields are either **enum fields** (`###Enum`) or **value fields** (`###Value`). See [[syntax/types/schema-properties#`###` Field Types — Leaf Content|schema properties]] for the formal definition:

| Field Type | Has `#type`? | Has assignment? | `###` Kind | Example |
|------------|-------------|-----------------|-----------|---------|
| Enum | No | No | `###Enum` | `[.] .Critical` |
| Value | Yes | Optional | `###Value` | `[.] .code#int <~ 500` |

**Rules:**
- No type annotation (`#type`) implies an **enum field** (`###Enum`)
- Enum fields always use `[.]` fixed fields
- All siblings at the same level must be the same kind — mixing raises PGE05005
- Enum fields can nest value sub-fields

```polyglot
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Info
   [.] .Other
      [ ] Enum field with nested value sub-fields
      [.] .code#int <~ 500
```

Reference enum values cross-package: `@alias#DataName.EnumField` (e.g., `@Alert#Severity.Critical`). See [[packages#Usage]] for import syntax and [[identifiers#Serialized Identifiers]] for `.` fixed-field navigation.

## See Also

- [[syntax/types/schema-properties|Schema Properties]] — `###Enum` and `###Value` field type definitions
- [[syntax/types/flexible-fields|Typed Flexible Fields]] — structs with `:` flexible levels
- [[concepts/collections/user-struct|User-Defined Struct as Collection]] — struct access patterns with `.` and `<`
