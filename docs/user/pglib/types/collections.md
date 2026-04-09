---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
---

# Collection Types

<!-- @types -->

Polyglot provides five collection types: `#Map`, `#Array`, `#Dataframe`, `#Set`, and `#Serial`. All are available in every `.pg` file without `[@]` import.

`#Map`, `#Array`, `#Dataframe`, and `#Set` are generic types with `[#] <param` inputs that compose parameterized `##` schemas. `#Serial` is a plain `{#}` definition (no parameters needed). See [[syntax/types/INDEX|types]] for the full type hierarchy and schema property definitions.

| Type | Description | File |
|------|-------------|------|
| `#Map` | Sparse key-value pairs (generic) | [[Map]] |
| `#Array` | Contiguous rectangular collection (generic) | [[Array]] |
| `#Dataframe` | Row-oriented table (generic) | [[Dataframe]] |
| `#Set` | Collection of unique values (generic) | [[Set]] |
| `#Serial` | Unconstrained collection, unlimited depth | [[Serial]] |

## Usage

The `:` separator binds positionally to `[#] <param` inputs. Users use the type names directly:

```polyglot
[-] $scores#array:int <~ {...}
[-] $lookup#map:string:int <~ {...}
[-] $matrix#array:float:2D <~ {...}
[-] $sales#dataframe:SalesColumns:string <~ {}
[-] $tags#set:string <~ {}
```

## Related

- [[string]] -- #String foundation type
- [[scalars]] -- scalar subtypes
- [[structs]] -- #path and #Queue struct types
- [[enums]] -- enum types
- [[syntax/types/INDEX|types]] -- full type system specification
