---
audience: pg-coder
type: specification
updated: 2026-04-04
---

# Macro-Generated Types

<!-- @syntax/types/INDEX -->

## Macro-Generated Types

Parameterized types use `{M}` type macros to generate `{#}` definitions at compile time (GT-9). Instead of generic type parameters in `{#}` headers, macros declare inputs with `[#] <Param` and produce complete `{#}` definitions.

### `{M}` Type Macros

A `{M}` block defines a type macro. It declares parameters with `[#] <Param` and contains one or more `{#}` definition blocks that use those parameters:

```polyglot
{M} #String.Subtype
   [#] <Name#RawString
   [#] <Alias#RawString
   [#] <Regex#RawString

   {#} ##{$Name}
      [#] <~ #String
      [#] %##Alias
         [:] << $Alias
         [:] <~ {%This.%name.Last}
      [#] << ##Scalar
      [.] .regex << $Regex
```

`{#}` can define any tier of the prefix system: `#Name` (type), `##Name` (schema), or `###Name` (field). The tier is part of the name, not the block syntax.

### `[M]` Macro Invocation

`[M]` inside a `{#}` block invokes a `{M}` macro. Arguments use `[#]` — the same marker as the macro's parameter declarations:

```polyglot
{#} ##Int
   [M] #String.Subtype
      [#] <Name << "Int"
      [#] <Alias << "int"
         [<] !Alias.Clash << "integer"
         [<] !Alias.Clash << "Integer"
      [#] <Regex << "^-?[0-9]+$"
```

- `[M] #String.Subtype` invokes the `{M} #String.Subtype` macro
- `[#] <Name << "Int"` passes an argument matching the macro's `[#] <Name#RawString` parameter
- `[<] !Alias.Clash` provides error-driven fallback: if alias "int" clashes, try "integer", then "Integer"
- The outer `{#} ##Int` names the result; the macro fills the body. Any `[#]` lines after `[M]` extend or override the macro's output

### Macro Dispatch

Macros overload by **signature** — the ordered list of parameter count and kind:

| Kind | Syntax | Meaning |
|------|--------|---------|
| Type input | `[#] <#ParamName` | A `#` type definition as input (datatypes are data trees) |
| Value input | `[#] <ParamName##Type` | A typed value as input |

Dispatch matches by parameter count AND kind (`<#` type vs `<` value). For example, `{M} #Dataframe` has two overloads: signature `(<#, <#)` for compile-time safe (type inputs) and `(<, <)` for runtime flexible (value inputs).

### `[#]` Roles Inside `{#}` and `{M}` Blocks

`[#]` is overloaded but distinguished by what follows:

| Pattern | Role | Context |
|---------|------|---------|
| `[#] %Property` | Schema property declaration | `{#}` |
| `[#] << ##Schema` | Schema composition (property substitution) | `{#}` |
| `[#] <~ #Parent` | Inheritance (copy parent's `%` properties as defaults) | `{#}` |
| `[#] <Param` | Macro input parameter declaration | `{M}` only |
| `[#] <Param << "value"` | Macro argument (pass value to `[#] <Param`) | `[M]` invocation |

`<~` in `{#}` means **only** inheritance — copying the parent type's `%` metadata properties as defaults. It never means macro invocation. Macros are invoked exclusively via `[M]`.

### Bootstrap Layers

Type macros are compiled in a staged sequence:

| Layer | What | Capabilities | Cannot Use |
|-------|------|-------------|------------|
| 0 -- Hardcoded | `#RawString`, `#String`, `{M}` engine | Compiler intrinsics -- not defined in Polyglot code | N/A |
| 1 -- Self-hosted | `#String.Subtype` macro, all `##` scalar types, `##CommaSeparatedList` | `{$var}` interpolation, `{%This}` metadata access | `[r] =Pipeline` calls |
| 2 -- Full macros | `#Array`, `#Map`, `#Dataframe` | `=String.Lower`, `=UID`, `=#list.into.Enum` -- full pipeline execution | N/A |

Layer 1 macros bootstrap without a pipeline engine (string substitution only). Layer 2 macros run after scalar types exist. `##CommaSeparatedList` (Layer 1) breaks the circular dependency that `#Array1D:String` (Layer 2) would create.

### `%This` and `%Parent` Scoping

`%This` refers to the **innermost enclosing `{x}` definition block**:

| Context | `%This` refers to |
|---------|-------------------|
| Inside `{M} #String.Subtype` body (outside nested `{#}`) | The macro |
| Inside `{#} ##{$Name}` nested within the macro | The `{#}` definition being generated |
| Outside any `{x}` block | Compile error |

To reference the enclosing macro from inside a nested `{#}`, use `%Parent` (one level up).

### `%name` and `%name.Last` Metadata Accessors

`%name` returns the definition name string from the `{x}` block header:

| Context | `%name` returns |
|---------|----------------|
| `{#} #ThisName` | `"ThisName"` |
| `{M} #String.Subtype` | `"String.Subtype"` |
| `{=} =Pipeline.Name` | `"Pipeline.Name"` |

`%name.Last` splits by `.` and returns the last segment: `{M} #String.Subtype` yields `%name.Last` = `"Subtype"`.

### `#*` Wildcard Type

`#*` is the "any type" wildcard. In macro parameter defaults, `[#] <#ValueType` with no constraint means "accepts any type." In field declarations, `:*#*` means "any key, any value type."

### `[<]` Parameter Constraints

`[<]` blocks nested under `[#] <Param` declarations in `{M}` macros constrain parameters via `%` schema properties:

```polyglot
{M} #Array
   [#] <#ValueType
      [<] << ##Scalar
   [#] <Dim##Dimension <~ "1D"
      [<] << ##Scalar
```

The `[<]` constraint declares that any type passed as `ValueType` must satisfy `##Scalar` (`%##Depth.Max = 1`) — preventing nested collections like `#array:#array:#int`.

### `<#type` in Pipeline IO

The `<#` syntax extends from `{M}` macro type inputs to `{=}` pipeline IO declarations. A pipeline can receive a type definition's `%` metadata tree as input using `[=] <#type`:

```polyglot
{=} =ValidateConfig
   [=] <data#serial
   [=] <#type
   [=] >valid#bool
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =#.Validate
      [=] <data << $data
      [=] <#type << <#type
      [=] >valid >> >valid
```

The `<#type` input works with any tier of the prefix system:

| Tier | Example | What the pipeline receives |
|------|---------|--------------------------|
| `#` struct | `<#Config` | The `#Config` type definition's full `%` metadata tree |
| `##` schema | `<#Scalar` | The `##Scalar` schema's property declarations |
| `###` property | `<#Enum` | The `###Enum` field property definition |

This is the same mechanism as `[#] <#ParamName` in `{M}` macros (GT-1: all definitions are data trees), now available at runtime via `{=}` pipelines. See [[#\|pglib/pipelines/#]] for `=#.Match`, `=#.Validate`, `=#.Describe`, `=#.Coerce` — the validation pipelines that use this pattern.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths including GT-9 (macros replace generics)
- [[syntax/types/basic-types|Basic Types]] — scalar subtypes generated by `{M} #String.Subtype`
- [[syntax/types/schema-properties|Schema Properties]] — `%##` properties that macros set on generated types
