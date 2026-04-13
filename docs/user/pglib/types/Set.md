---
audience: automation-builder
type: specification
updated: 2026-04-09
status: retired
redirect: pglib/types/Array
---

# #Set (Retired)

<!-- @c:types -->

`#Set` and `##Set` have been retired. Use `#Array` with `%###Unique << #True` instead.

---

## Replacement

The uniqueness guarantee formerly provided by `##Set` is now a leaf-level property on any collection:

```polyglot
{#} #UniqueStrings
   (#) <#ValueType << #String
   [#] ##Array
      (#) <#ValueType << <#ValueType
   [#] %###Unique << #True
   [#] %##Alias << "unique-strings"
```

This composes `##Array` for ordered, range-indexed storage and adds `%###Unique << #True` to reject duplicates.

---

## Migration

| Former | Now |
|--------|-----|
| `#Set` | `#Array` + `%###Unique << #True` |
| `##Set` | `##Array` + `%###Unique << #True` |
| `#set:string` | custom `{#}` with `##Array` + `%###Unique` |

---

## Related

- [[Array]] -- replacement base type
- [[schemas/Array|##Array]] -- parameterized schema
- [[schemas/INDEX|## Schema Types]] -- retired schemas list
- [[syntax/types/INDEX|types]] -- full type system specification

