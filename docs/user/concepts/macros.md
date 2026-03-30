---
audience: user
type: concept
updated: 2026-03-30
---

# Macros

<!-- @glossary:Polyglot Code -->

Macros generate data type definitions at compile time. Rather than writing separate `{#}` blocks for every combination of parameters (e.g., every array dimension and element type), a single `{M}` macro produces them all.

You typically consume macro-generated types directly — `#array:int:1D`, `#Map:String`, `#Dataframe` — without needing to interact with the macro system itself.

## How It Works

A `{#}` block invokes a macro with `[M]`:

```polyglot
{#} ##Int
   [M] #String.Subtype
      [#] <Name << "Int"
      [#] <Alias << "int"
      [#] <Regex << "^-?[0-9]+$"
```

The `[M] #String.Subtype` line runs the macro and fills the type definition. The result is a fully formed `{#}` block — identical to writing one by hand.

## Stdlib Macros

| Macro | Generates | See |
|-------|-----------|-----|
| `#String.Subtype` | Scalar subtypes (`##Int`, `##Float`, `##Dimension`, etc.) | [[stdlib/types/scalars\|Scalars]] |
| `#Array` | Array types for all dimension/element combinations | [[syntax/types/arrays\|Arrays]] |
| `#Map` | Map types keyed by string subtypes | [[stdlib/types/collections\|Collections]] |
| `#Dataframe` | Dataframe types with row/column structure | [[stdlib/types/collections\|Collections]] |

## Related

- [[syntax/types/macro-types\|Macro-Generated Types]] — full syntax reference: `{M}` definitions, `[M]` invocation, dispatch, bootstrap layers
- [[syntax/blocks#Definition Blocks\|blocks]] — `{M}` block element reference
