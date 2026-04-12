---
audience: pg-coder
type: specification
updated: 2026-04-12
---

# %## and %### Properties

<!-- @c:syntax/types/schema-properties -->

`%##` and `%###` properties are the ground-truth metadata entries that describe a type's tree structure. Each property is a relative path into the metadata tree -- `%##Fields` inside `{#} #Array` resolves to `%definition.#:Array.%##Fields`. `##` schemas are named bundles of these properties; the properties themselves are what the compiler enforces.

## Branch-Level Properties (`%##`)

These describe how children behave at the level where they are declared:

| Property | Type | Purpose |
|----------|------|---------|
| [[properties/Fields\|%##Fields]] | `#FieldsDescriptor` or `##Enum` ref | How children are indexed |
| [[properties/Schema\|%##Schema]] | list of `##` | Structural schemas children must satisfy |
| [[properties/TerminalType\|%##TerminalType]] | type ref | Type all terminal branches must be (any depth) |
| [[properties/Active\|%##Active]] | `#ActiveKind` | Which branches must be present |
| [[properties/Ordered\|%##Ordered]] | `#Boolean` | Insertion order preserved? |
| [[properties/Sorted\|%##Sorted]] | `#Boolean` | Sorted by key? |
| [[properties/Gap\|%##Gap]] | `#Boolean` | Gaps allowed in keys? |
| [[properties/Count\|%##Count]] | `#Bound` | Cardinality bounds |
| [[properties/Propagate\|%##Propagate]] | `#Boolean` | Recursive application to nested levels |
| [[properties/Level\|%##Level.N]] | scope | Per-level overrides |

## Tree-Level Properties (`%##`)

These describe the whole type tree:

| Property | Type | Purpose |
|----------|------|---------|
| [[properties/Depth-Max\|%##Depth.Max]] | `#Bound` | Maximum tree depth |
| [[properties/Alias\|%##Alias]] | `#NestedKeyString` | Lowercase shorthand name |

## Leaf-Level Properties (`%###`)

These describe leaf content constraints:

| Property | Type | Purpose |
|----------|------|---------|
| [[properties/Kind\|%###Kind]] | `#FieldKind` | Value data or enum identity? |
| [[properties/Type\|%###Type]] | type ref | Uniform leaf type constraint |
| [[properties/Unique\|%###Unique]] | `#Boolean` | Leaf values must be distinct? |

## Related

- [[syntax/types/schema-properties|Schema Properties]] -- full specification with inference rules and error codes
- [[schemas/INDEX|## Schema Types]] -- named bundles of these properties
- [[field-types/INDEX|### Field Types]] -- leaf content classification
- [[syntax/types/prefix-system|Prefix System]] -- how `%` maps to metadata tree addresses
