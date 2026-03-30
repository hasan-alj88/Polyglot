---
audience: user
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->

## User-Defined Struct

User-defined structs declare fixed fields with `[.]`. The `##` schema is optional — the compiler infers it from field declarations.

```polyglot
{#} #Person
   [.] .name#string
   [.] .age#int
```

Fixed fields use the `.` accessor:

```polyglot
[r] $userName#string << $user.name
[r] $userAge#int << $user.age
```

The `<` accessor is for flexible children only. Fixed fields (`.`) and flexible children (`:`) are distinct — a struct with only `[.]` fields has no flexible children to access via `<`.

## See Also

- [[syntax/types/structs|Struct Types]] — struct definitions, enum vs value fields, and level rules
- [[syntax/types/flexible-fields|Typed Flexible Fields]] — structs with `:` flexible levels
- [[concepts/collections/serial|#Serial]] — schema-free alternative when struct shape is unknown
