---
audience: automation-builder
type: specification
updated: 2026-04-09
---

# Prefix System and Tree Access

<!-- @syntax/types/INDEX -->

## Three-Tier Prefix System

The `#` character is not just for type annotations — it participates in a three-tier prefix system that describes types at different levels of abstraction:

| Prefix | Name | Describes | Example |
|--------|------|-----------|---------|
| `#` | Type | Concrete data type | `#Array`, `#String`, `#Person` |
| `##` | Schema | Tree shape -- structure and constraints | `##Scalar`, `##Flat`, `##Leaf`, `##Array` |
| `###` | Field | Leaf content nature | `###Value`, `###Enum`, `###ScalarValue`, `###ScalarEnum`, `###None` |

- `#` = **what** — the concrete type a variable holds
- `##` = **shape** — the tree structure: depth, keys, ordering, uniformity
- `###` = **content** — the leaf nature: typed value or enum variant

Schema (`##`) and field (`###`) prefixes are used inside `{#}` type definitions to declare structural properties. They are not used in variable annotations — you always annotate variables with `#`. See [[data-is-trees]] for how these tiers relate to the unified tree.

## `%` — Metadata and the Tree Address System

The `%` prefix means **metadata**. Every `%` reference is a **relative path** into the metadata tree — the compiler resolves it to an absolute address in the tree. For example, `%##Depth.Max` inside a `{#} #Array` definition is a relative reference that resolves to the absolute path `%definition.#:Array.%##Depth.Max`.

This means the prefix tiers (`##`, `###`) and the metadata prefix (`%`) work together:

| You write | Resolves to |
|-----------|-------------|
| `[#] %##Depth.Max << 1` | `%definition.#:{Type}.%##Depth.Max -> 1` |
| `[#] %##Fields << #Range` | `%definition.#:{Type}.%##Fields -> #Range` |
| `[#] %###Kind << #FieldKind.Value` | `%definition.#:{Type}.%###Kind -> #FieldKind.Value` |

`%##` and `%###` are the metadata forms of `##` and `###` — they address structural properties stored in the metadata tree. The `%` makes explicit that these are **compile-time metadata entries**, not runtime data.

## `##` Schemas — Named Property Bundles

A `##` schema is syntactic sugar for defining a reusable bundle of `%##` properties. When a type composes `[#] ##Flat`, the compiler expands it into the individual `%##` assignments that `##Flat` defines:

```aljam3
{#} ##Flat
   [#] %##Depth.Max << 1

[ ] Composing ##Flat:
[#] ##Flat
[ ] ...is equivalent to writing:
[#] %##Depth.Max << 1
```

A more complex example — `##Enum` bundles four properties:

```aljam3
{#} ##Enum
   [#] ##Flat
   [#] %##Active << #ActiveKind.One
   [#] %###Kind << #FieldKind.Enum

[ ] Composing ##Enum expands to:
[#] %##Depth.Max << 1       (from ##Flat)
[#] %##Active << #ActiveKind.One
[#] %###Kind << #FieldKind.Enum
```

Schemas exist so that common structural patterns have names — `##Array`, `##Record`, `##Enum` — instead of requiring every type to repeat the same `%##` assignments. The `%##` properties are the ground truth; `##` schemas are a convenience layer over them.

## The `<` Operator: Tree Child Access

The `<` character is a tree child accessor. It reads a child from a data tree by key:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `$var<key` | Tree child access | `$myArray<0`, `$myMap<name` |
| `$var<key<subkey` | Chained access for nested trees | `$matrix<0<1`, `$df<0<product` |

The `<` operator also appears inside `{#}` generic type definitions as a parameter input marker (`(#) <Param`). See [[generic-types]] for details.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths and type annotation rules
- [[syntax/types/schema-properties|Schema Properties]] — `##` and `###` property definitions
- [[syntax/types/generic-types|Generic Types]] — `<` as generic parameter input marker
