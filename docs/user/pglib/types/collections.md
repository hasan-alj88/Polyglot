---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
---

# Collection Types

<!-- @c:types -->

Polyglot provides three collection schemas and one unconstrained type: `##Record`, `##Array`, `##Dataframe`, and `#Serial`. All are available in every `.pg` file without `[@]` import.

`##Record`, `##Array`, and `##Dataframe` are parameterized schemas that `{#}` types compose via `[#]`. `#Serial` is a plain `{#}` definition (no schema composition needed). See [[syntax/types/INDEX|types]] for the full type hierarchy and schema property definitions.

| Schema / Type | Description | File |
|---------------|-------------|------|
| `##Record` | Enum-keyed value fields (replaces former `#Map`) | [[Map]] |
| `##Array` | Range-indexed ordered collection | [[Array]] |
| `##Dataframe` | Two-level: range rows + ##Record columns | [[Dataframe]] |
| `#Serial` | Unconstrained collection, unlimited depth | [[Serial]] |

## Retired

| Former | Replaced By |
|--------|-------------|
| `#Map` / `##Map` | `##Record` -- enum-keyed records replace sparse key-value maps |
| `#Set` / `##Set` | `##Array` + `%###Unique << #True` -- sets are arrays with uniqueness constraint |

## Usage

Types compose collection schemas using `[#]` schema composition:

```polyglot
[-] $scores#array:int <~ {...}
[-] $matrix#array:float:2D <~ {...}
[-] $sales#dataframe:SalesColumns:string <~ {}
```

## Related

- [[string]] -- #String foundation type
- [[scalars]] -- scalar subtypes
- [[structs]] -- #path and #Queue struct types
- [[enums]] -- enum types
- [[schemas/Fields|%##Fields]] -- field descriptor property
- [[syntax/types/INDEX|types]] -- full type system specification

