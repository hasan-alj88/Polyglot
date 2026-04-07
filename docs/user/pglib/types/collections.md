---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# Collection Types

<!-- @types -->

Polyglot provides four collection types: `#Map`, `#Array`, `#Dataframe`, and `#Serial`. All are available in every `.pg` file without `[@]` import.

`#Map`, `#Array`, and `#Dataframe` are defined as `{M}` type macros that generate `{#}` definitions at compile time. `#Serial` is a plain `{#}` definition (no macro needed). See [[syntax/types/INDEX|types]] for the full type hierarchy and schema property definitions.

| Type | Description | File |
|------|-------------|------|
| `#Map` | Sparse key-value pairs (macro, two overloads) | [[Map]] |
| `#Array` | Contiguous rectangular collection (macro, inherits #Map) | [[Array]] |
| `#Dataframe` | Row-oriented table (macro, two overloads) | [[Dataframe]] |
| `#Serial` | Unconstrained collection, unlimited depth | [[Serial]] |

## Usage

The `:` separator binds positionally to macro inputs. Users use the generated type names directly:

```polyglot
[r] $scores#array:int <~ {...}
[r] $lookup#map:string:int <~ {...}
[r] $matrix#array:float:2D <~ {...}
[r] $sales#dataframe:SalesColumns:string <~ {}
```

## Related

- [[string]] -- #String foundation type
- [[scalars]] -- scalar subtypes (## schemas)
- [[structs]] -- #path and #Queue struct types
- [[enums]] -- #FieldKind enum (used by `%##Leafs.Kind`)
- [[syntax/types/INDEX|types]] -- full type system specification
- [[macros]] -- {M} type macro definitions
