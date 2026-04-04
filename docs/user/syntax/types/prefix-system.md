---
audience: pg-coder
type: specification
updated: 2026-04-04
---

# Prefix System and Tree Access

<!-- @syntax/types/INDEX -->

## Three-Tier Prefix System

The `#` character is not just for type annotations — it participates in a three-tier prefix system that describes types at different levels of abstraction:

| Prefix | Name | Describes | Example |
|--------|------|-----------|---------|
| `#` | Type | Concrete data type | `#Array`, `#String`, `#Person` |
| `##` | Schema | Tree shape — structure and constraints | `##Scalar`, `##Flat`, `##Leaf`, `##Contiguous` |
| `###` | Field | Leaf content nature | `###Value`, `###Enum`, `###ScalarValue`, `###ScalarEnum`, `###None` |

- `#` = **what** — the concrete type a variable holds
- `##` = **shape** — the tree structure: depth, keys, ordering, uniformity
- `###` = **content** — the leaf nature: typed value or enum variant

Schema (`##`) and field (`###`) prefixes are used inside `{#}` type definitions to declare structural properties. They are not used in variable annotations — you always annotate variables with `#`. See [[data-is-trees]] for how these tiers relate to the unified tree.

## The `<` Operator: Tree Child Access

The `<` character is a tree child accessor. It reads a child from a data tree by key:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `$var<key` | Tree child access | `$myArray<0`, `$myMap<name` |
| `$var<key<subkey` | Chained access for nested trees | `$matrix<0<1`, `$df<0<product` |

The `<` operator also appears inside `{M}` macro definitions as a parameter input marker (`[#] <Param`). See [[macro-types]] for details.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths and type annotation rules
- [[syntax/types/schema-properties|Schema Properties]] — `##` and `###` property definitions
- [[syntax/types/macro-types|Macro-Generated Types]] — `<` as macro parameter input marker
