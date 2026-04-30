---
audience: automation-builder
type: specification
updated: 2026-04-09
---

# Generic Types and Parameterized Schemas

<!-- @syntax/types/INDEX -->

## Generic Types

Types can be generic — they accept `(#) <param` inputs that the compiler resolves at use time through `:` positional binding (GT-10). A generic `{#}` definition declares parameters with `(#) <param` lines and composes `##` schemas that use those parameters:

```aljam3
{#} #Record
   (#) <#Fields << ##Enum
   (#) <#ValueType <~ #
   [#] ##Record
      (#) <#Fields << <#Fields
      (#) <#ValueType << <#ValueType
```

- `(#) <#Fields << ##Enum` — declares a type input parameter constrained to `##Enum` types
- `(#) <#ValueType <~ #` — declares a type input with default `#` (any type)
- `[#] ##Record` — composes the `##Record` parameterized schema, passing parameters through

### `:` Positional Binding

When using generic types, `:` separates positional arguments that bind to `(#) <param` declarations in order:

| User writes | Resolved | Parameters |
|-------------|----------|------------|
| `#record:DayOfWeek:int` | `#Record:DayOfWeek:Int` | Fields=DayOfWeek, ValueType=Int |
| `#record:DayOfWeek` | `#Record:DayOfWeek` | Fields=DayOfWeek, ValueType=# (default) |
| `#array:float:2D` | `#Array:Float:2D` | ValueType=Float, Dim=2D |
| `#pair:int:string` | `#Pair:Int:String` | First=Int, Second=String |

Default values (`<~`) fill unfilled positions. Missing required params = compile error.

### More Examples

```aljam3
{#} #Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] ##Array
      (#) <#ValueType << <#ValueType
      (#) <Dim << <Dim

{#} #Int
   [%] %alias << "int,integer,Integer"
   [#] ##Scalar
   [#] ##String
      (#) <regex << "^-?[0-9]+$"

{#} #Boolean
   [#] ##Enum
   [#] ##Scalar
   [.] .True
   [.] .False
```

Non-generic types like `#Int` and `#Boolean` compose schemas without declaring `(#) <param` inputs — they are fully resolved at definition time.

## Parameterized Schemas

`##` schemas can also accept `(#) <param` inputs (GT-9). A parameterized schema generates structural constraints at compose time:

```aljam3
{#} ##Record
   (#) <#Fields << ##Enum
   (#) <#ValueType <~ #
   [#] ##Flat
   [#] %##Fields << <#Fields
   [#] %##Active << #ActiveKind.All
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value

{#} ##String
   (#) <regex#RawString
   [#] %##Depth.Max << 1
   [#] %###Kind << #FieldKind.Value
```

When a `{#}` type composes a parameterized `##` schema via `[#] ##Schema`, it passes arguments to the schema's parameters. Non-parameterized schemas like `##Flat` or `##Sorted` set properties directly with no inputs.

## `[#]` Roles Inside `{#}` Blocks

`[#]` is overloaded but distinguished by what follows:

| Pattern | Role | Context |
|---------|------|---------|
| `[#] %##Property << value` | Schema property assignment | `{#}` |
| `[#] ##Schema` | Schema composition (property substitution) | `{#}` |
| `(#) <#ParamName` | Type input parameter declaration | `{#}` generic type |
| `(#) <ParamName##Type` | Value input parameter declaration | `{#}` generic type |
| `(#) <ParamName <~ "default"` | Parameter with default value | `{#}` generic type |

## `(<)` Parameter Constraints

`(<)` blocks nested under `(#) <Param` declarations constrain parameters via `%` schema properties:

```aljam3
{#} #Array
   (#) <#ValueType
      (<) ##Scalar
   (#) <Dim##Dimension <~ "1D"
```

The `(<)` constraint declares that any type passed as `ValueType` must satisfy `##Scalar` (`%##Depth.Max = 1`) — preventing nested collections like `#array:#array:#int`.

## Bootstrap Layers

Generic types are compiled in a staged sequence:

| Layer | What | Capabilities | Cannot Use |
|-------|------|-------------|------------|
| 0 -- Hardcoded | `#RawString`, `#String`, generic engine | Compiler intrinsics -- not defined in Aljam3 code | N/A |
| 1 -- Self-hosted | `##String` schema, all `##Scalar` types | `{$var}` interpolation, `{%This}` metadata access | `[-] -Pipeline` calls |
| 2 -- Full generics | `#Array`, `#Record`, `#Dataframe` | `-String.Lower`, `-UID`, `-#list.into.Enum` -- full pipeline execution | N/A |

Layer 1 types bootstrap without a pipeline engine (string substitution only). Layer 2 types run after scalar types exist. `##CommaSeparatedList` (Layer 1) breaks the circular dependency that `#Array1D:String` (Layer 2) would create.

## `%This` and `%name` Scoping

`%This` refers to the **innermost enclosing `{x}` definition block**:

| Context | `%This` refers to |
|---------|-------------------|
| Inside `{#} #MyType` | The type definition |
| Inside `{-} -MyPipeline` | The pipeline definition |
| Outside any `{x}` block | Compile error |

`%name` returns the definition name string from the `{x}` block header:

| Context | `%name` returns |
|---------|----------------|
| `{#} #ThisName` | `"ThisName"` |
| `{-} -Pipeline.Name` | `"Pipeline.Name"` |
| `{W} -W.Aljam3` | `"W.Aljam3"` |

`%name.Last` splits by `.` and returns the last segment.

`%Parent` refers to one level up from `%This` — useful inside nested definition contexts.

## Multi-Type Constraints

For multi-type constraints, use `##` schemas (e.g., `##Scalar`, `##Leaf`). An unconstrained generic parameter `(#) <#ValueType` with no `[<]` constraint accepts any type. There is no wildcard type in the grammar — `##` schemas are the mechanism for expressing type flexibility.

## `<#type` in Pipeline IO

The `<#` syntax extends from `{#}` generic type inputs to `{-}` pipeline IO declarations. A pipeline can receive a type definition's `%` metadata tree as input using `(-) <#type`:

```aljam3
{-} -ValidateConfig
   (-) <data#serial
   (-) <#type
   (-) >valid#bool
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] -#.Validate
      (-) <data << $data
      (-) <#type << <#type
      (-) >valid >> >valid
```

The `<#type` input works with any tier of the prefix system:

| Tier | Example | What the pipeline receives |
|------|---------|--------------------------|
| `#` struct | `<#Config` | The `#Config` type definition's full `%` metadata tree |
| `##` schema | `<#Scalar` | The `##Scalar` schema's property declarations |
| `###` property | `<#Enum` | The `###Enum` field property definition |

This is the same mechanism as `(#) <#ParamName` in `{#}` generic types (GT-1: all definitions are data trees), now available at runtime via `{-}` pipelines. See [[aj3lib/pipelines/Schema/INDEX|-#.* Schema Pipelines]] for `-#.Match`, `-#.Validate`, `-#.Describe`, `-#.Coerce` — the validation pipelines that use this pattern.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths including GT-9 and GT-10
- [[syntax/types/basic-types|Basic Types]] — scalar subtypes composed from `##String` schema
- [[syntax/types/schema-properties|Schema Properties]] — `%##` and `%###` property definitions
