---
audience: automation-builder
type: specification
updated: 2026-03-30
---

# Typed Flexible Fields

<!-- @syntax/types/INDEX -->

## Typed Flexible Fields

A struct can have levels that use flexible (`:`) fields. At such a level, the `[:] :*#Type` syntax declares that ALL `:` siblings share the same type — a typed dictionary. The `:*` wildcard means "collectively, every key at this level has this type."

```aljam3
{#} #Registry
   [.] .builtins
      [.] .http#Handler
      [.] .grpc#Handler
   [.] .plugins
      [:] :*#Handler
```

Here `.plugins` has flexible children. Every `:key` under `.plugins` must be `#Handler`. Users can push `:myPlugin`, `:anotherPlugin`, etc. — all constrained to `#Handler`.

### Schema Enforcement on New Keys

When a typed flexible level references a struct type, all new keys inherit that struct's schema:

```aljam3
{#} #SubStruct
   [.] .level4#string

{#} #Example
   [.] .level1
      [.] .level2
         [:] :*#SubStruct
```

To create a new key `:new` alongside existing `:level3`: push to `#Example.level1.level2:new.level4` — the compiler knows `:new` is `#SubStruct`, so `.level4#string` is enforced.

### Constraints

- **No extra levels** — a flexible field's children are fully defined by its type annotation. You cannot insert additional levels between the flexible field and its typed children.
- **Separator homogeneity is per-level** — siblings at the same level must share a separator, but different levels may differ (e.g., `.` at level 1, `:` at level 2). See [[technical/compile-rules/PGE/PGE05001-sibling-separator-homogeneity|PGE05001]].
- **Multi-level flexibility** — a struct can have multiple flexible levels (each level independently homogeneous):

```aljam3
{#} #DeepFlex
   [.] .config
      [:] :*#Section

{#} #Section
   [:] :*#Setting

{#} #Setting
   [.] .value#string
   [.] .default#string
```

## See Also

- [[syntax/types/structs|Struct Types]] — fixed-field struct definitions and enum vs value fields
- [[syntax/types/schema-properties|Schema Properties]] — `%##Children.Type` and depth constraints
- [[concepts/collections/map|#Map]] — flexible key-value collections using `<` access
- [[technical/compile-rules/PGE/PGE05001-sibling-separator-homogeneity|PGE05001]] — sibling separator homogeneity (per-level rule)
