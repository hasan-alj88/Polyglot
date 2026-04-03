---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 24. Datatype Definitions (S24)

Type DEFINITIONS — `{#}` blocks, `%##` schema properties, `<~` inheritance, and `###` field types. Complements S4 (type USAGE/ANNOTATIONS). See [[syntax/types/INDEX|types]].

| Edge Case | Title | Tests |
|-----------|-------|-------|
| EC-24.1 | #String `.regex` default | Empty string matches `".*"`, ##Scalar inherited |
| EC-24.2 | #Int leading zeros and negative zero | `"007"` and `"-0"` both match regex |
| EC-24.3 | #Dimension 0D | 0D allowed for scalars, regex discrepancies across specs |
| EC-24.4 | #Eng exponent | Multiples-of-3 constraint on exponent |
| EC-24.5 | #KeyString excluded chars | Dot excluded, hyphen allowed |
| EC-24.6 | #NestedKeyString allows dot/colon | Alias paths with separators |
| EC-24.7 | `<~` inheritance chain finality | `<<` final prevents further override |
| EC-24.8 | #Boolean — ##Scalar + ###Enum | Dual schema composition |
| EC-24.9 | Enum inheritance via `<~` | Extending enum variants |
| EC-24.10 | #None — minimal type | No fields, no schema |
| EC-24.11 | #Array via `{M}` macro with `<~` inheritance | Macro-generated definition with schema accumulation |
| EC-24.12 | ##Contiguous vs ##Sparse override | Contradicting property override (PGW11002) |
| EC-24.13 | 0D array | Dimension collapse to scalar |
| EC-24.14 | Empty collections | Zero-element #Array and #Map |
| EC-24.15 | Invalid key type | #Int as map key (PGE11004) |
| EC-24.16 | #Serial — no ## schema constraints | Unlimited depth escape hatch (PGW11003 exemption) |
| EC-24.17 | #Dataframe status | Row-oriented access via `$df<row<column` |
| EC-24.18 | Stale %Property notation | stdlib types.md missing `##` prefix |
| EC-24.19 | [M] merge behavior (identity rule) | Outer {#} names result, [M] fills body |
| EC-24.20 | Macro dispatch ambiguity | Two overloads with identical signature = PGE01019 |

---

### EC-24.1: #String `.regex` default

**EBNF:** `type_definition ::= "{#}" type_header { schema_line | field_line }` — field defaults via `<~`.
**What it tests:** Empty string `""` matches `.regex = ".*"` — the default regex accepts all strings, including empty. Verifies `##Scalar` is inherited via `[#] << ##Scalar`. See [[syntax/types/basic-types#Layer 1: #String — The Foundation Type]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[variable-lifecycle]]

```polyglot
{#} #String
   [#] << ##Scalar
   [#] %##Alias << "string"
   [.] .string#RawString
   [.] .regex#RawString <~ ".*"

[ ] empty string matches ".*"
[r] $empty#string << ""

[ ] any content matches ".*"
[r] $anything#string << "hello world 123 !@#"
```

### EC-24.2: #Int leading zeros and negative zero

**EBNF:** `type_definition` — `.regex` regex `"^-?[0-9]+$"` accepts leading zeros and `-0`.
**What it tests:** `"007"` and `"-0"` both match `#Int`'s regex. Leading zeros are valid serialized strings — Polyglot does not normalize numeric representations. See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} #Int
   [#] <~ #String
   [#] %##Alias << "int"
   [.] .regex#RawString << "^-?[0-9]+$"

[ ] leading zeros match regex
[r] $padded#int << 007

[ ] negative zero matches regex
[r] $negZero#int << -0

[ ] PGE04010 — decimal point not in regex
[r] $bad#int << 3.14
```

### EC-24.3: #Dimension 0D

**EBNF:** `type_definition` — `.regex` for #Dimension and the `:ND` syntax sugar.
**What it tests:** 0D is valid for scalars. The stored value includes the `D` suffix — `"2D"`, not `"2"`. Regex is `"^[0-9]+D$"`. See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[stdlib/INDEX|Standard Library]]
**Status:** RESOLVED — regex corrected to `"^[0-9]+D$"` in both syntax/types.md and stdlib/types/scalars.md.

```polyglot
[ ] Authoritative definition — corrected regex
{#} #Dimension
   [#] <~ #String
   [#] %##Alias << "dim"
   [ ] Stored value includes D suffix
   [.] .regex#RawString << "^[0-9]+D$"

[ ] 0D means scalar dimension
[r] $scalar#array:int:0D

[ ] standard dimensions
[r] $vector#array:float:1D
[r] $matrix#array:float:2D
```

### EC-24.4: #Eng exponent

**EBNF:** `type_definition` — `.regex` enforces multiples-of-3 exponent.
**What it tests:** `"1.5e4"` fails (4 is not a multiple of 3), `"1.5e3"` passes. The regex `(0|[369]|[1-9][0-9]*[0369])` constrains the exponent to 0, 3, 6, 9, ... See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} #Eng
   [#] <~ #String
   [#] %##Alias << "eng"
   [.] .regex#RawString << "^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0|[369]|[1-9][0-9]*[0369])$"

[ ] exponent is multiple of 3
[r] $valid#eng << "1.5e3"
[r] $micro#eng << "2.47e-6"
[r] $tera#eng << "9.99e12"

[ ] PGE04010 — exponent 4 is not a multiple of 3
[r] $bad#eng << "1.5e4"

[ ] PGE04010 — exponent 1 is not a multiple of 3
[r] $bad2#eng << "3.0e1"
```

### EC-24.5: #KeyString excluded chars

**EBNF:** `type_definition` — `.regex` excludes syntax-reserved characters.
**What it tests:** `"my.key"` fails (dot is reserved for fixed-field navigation), `"my-key"` passes. Ensures tree path safety for `<` child access. See [[syntax/types/basic-types#Layer 2c: #KeyString — Key Type for Tree Access]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} #KeyString
   [#] <~ #String
   [#] %##Alias << "key"
   [.] .regex#RawString << "^[^\s.<>:]+$"

[ ] hyphen allowed
[r] $valid#key << "my-key"
[r] $underscored#key << "my_key_123"

[ ] PGE04010 — dot is reserved (fixed-field separator)
[r] $dotted#key << "my.key"

[ ] PGE04010 — colon is reserved (flexible-field separator)
[r] $coloned#key << "my:key"

[ ] PGE04010 — angle bracket is reserved
[r] $angled#key << "my<key"
```

### EC-24.6: #NestedKeyString allows dot/colon

**EBNF:** `type_definition` — `.regex` allows `.` and `:` but excludes whitespace and angle brackets.
**What it tests:** `"error.file.read"` passes (dots allowed for alias paths), `"my<key"` fails. Used as element type for `%##Alias` paths. See [[syntax/types/basic-types#Layer 2d: #NestedKeyString — Key Type for Alias Paths]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} #NestedKeyString
   [#] <~ #String
   [#] %##Alias << "nestedkey"
   [.] .regex#RawString << "^[^\s<>]+$"

[ ] dots and colons allowed for nested paths
[r] $alias#nestedkey << "error.file.read"
[r] $nested#nestedkey << "String:int"

[ ] PGE04010 — angle bracket excluded
[r] $bad#nestedkey << "my<key"

[ ] PGE04010 — whitespace excluded
[r] $bad2#nestedkey << "my key"
```

### EC-24.7: `<~` inheritance chain finality

**EBNF:** `inheritance ::= "[#]" "<~" type_ref` — schema inheritance with `<~` (overridable) vs `<<` (final).
**What it tests:** #Int inherits `.string` and `.regex` from #String via `<~`, then sets `.regex` with `<<` (final). A user-defined `#PositiveInt <~ #Int` cannot override `.regex` — it is already final. See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]], [[variable-lifecycle]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[variable-lifecycle]]
**Status:** RESOLVED — PGE11005 (Final Field Override via Inheritance) added to COMPILE-RULES.md.

```polyglot
[ ] #String sets .regex with <~ (overridable default)
{#} #String
   [.] .regex#RawString <~ ".*"

[ ] #Int inherits and sets .regex with << (final)
{#} #Int
   [#] <~ #String
   [.] .regex#RawString << "^-?[0-9]+$"

[ ] PGE11005 — .regex is already << final in #Int
{#} #PositiveInt
   [#] <~ #Int
   [.] .regex#RawString << "^[1-9][0-9]*$"
```

### EC-24.8: #Boolean — ##Scalar + ###Enum

**EBNF:** `schema_line ::= "[#]" "<<" schema_ref` — multiple schema compositions accumulate.
**What it tests:** #Boolean composes both `##Scalar` (depth 0) and `###Enum` (leaf content is variant selector). A type can be both scalar AND enum. Can a user define a custom enum with the same pattern? Yes — `###Enum` and `##Scalar` are orthogonal. See [[syntax/types/basic-types#Layer 2b: #Boolean — Independent Enum Struct]], [[syntax/types/schema-properties#`###` Field Types — Leaf Content]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} #Boolean
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "bool"
   [.] .True
   [.] .False

[ ] user-defined enum with same pattern
{#} #TrafficLight
   [#] << ##Scalar
   [#] << ###Enum
   [.] .Red
   [.] .Yellow
   [.] .Green

[ ] usage
[r] $light << #TrafficLight.Red
```

### EC-24.9: Enum inheritance via `<~`

**EBNF:** `inheritance ::= "[#]" "<~" type_ref` — `<~` on an enum type.
**What it tests:** User defines `#MyStatus <~ #PipelineStatus` to add new variants. Enums use `[.]` fixed fields — `<~` on an enum should extend the field set (inheriting all parent variants). See [[syntax/types/structs#Enum Fields vs Value Fields]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} #PipelineStatus
   [#] << ###Enum
   [.] .AwaitTrigger
   [.] .Disabled
   [.] .Running
   [.] .Failed

[ ] Extends #PipelineStatus with additional variants
{#} #MyStatus
   [#] <~ #PipelineStatus
   [.] .Degraded
   [.] .Maintenance

[ ] inherited variants still accessible
[r] $status << #MyStatus.Running

[ ] new variant accessible
[r] $status << #MyStatus.Degraded

[ ] parent type does not gain child's variants
[r] $bad << #PipelineStatus.Degraded
```

### EC-24.10: #None — minimal type definition

**EBNF:** `type_definition ::= "{#}" type_header { schema_line | field_line }` — zero fields, `###None` field type.
**What it tests:** #None has no fields and uses `###None` — a third field type meaning "nullable." Empty string `""` is the only valid value. Only `###None` types accept empty string; all others reject it with PGE04021. See [[syntax/types/INDEX|types]], [[syntax/types/schema-properties#`###` Field Types — Leaf Content]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[stdlib/INDEX|Standard Library]]
**Status:** RESOLVED — `###None` added as third field type, PGE04021 added, #None definition updated with `[#] << ###None`.

```polyglot
{#} #None
   [ ] Represents the absence of a value
   [ ] Empty string "" is the only valid value
   [#] << ##Scalar
   [#] << ###None

[ ] usage — signals absence
[r] $result << #None

[ ] PGE04021 — empty string on non-###None type
[r] $bad#string << ""
```

### EC-24.11: #Array via `{M}` macro with `<~` inheritance

**EBNF:** `macro_def ::= "{M}" "#" dotted_name`, `macro_type_param ::= "[#]" "<#" name`, `schema_inheritance ::= "[#]" "<~" data_id` — macro-generated definition with parameterized inheritance.
**What it tests:** `{M} #Array` macro takes `<#ValueType` (type input) and `<Dim` (value input). The macro body generates a `{#}` definition that inherits from `#Map` via `<~`, substituting `#UnsignedInt` for the key type. Schema properties accumulate — `##Contiguous` overrides inherited `##Sparse` properties. See [[syntax/types/schema-properties#Approved ## Schema Types]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[stdlib/INDEX|Standard Library]], [[technical/ebnf/04-type-system#4.3]]

```polyglot
[ ] {M} #Array — type macro with two parameters
{M} #Array
   [#] <#ValueType
      [<] << ##Scalar
   [#] <Dim##Dimension <~ "1D"
      [<] << ##Scalar

   [r] $ArrayName##DataTypeString << "Array{$Dim}:{$ValueType%name}"
   [r] $dim#RawString << =String.Lower"{$Dim}"

   {#} #{$ArrayName}
      [#] <~ #Map:#UnsignedInt:$ValueType
      [#] %##Alias
         [:] << "array:{$ValueType%name}:{$dim}"
         [:] << "array{$dim}:{$ValueType%name}"
         [:] << "Array{$Dim}:{$ValueType%name}"
      [#] %##Children.Type << #UnsignedInt
      [#] %##Children.Ordered << #True
      [#] %##Children.Uniform << #True
      [ ] ##Contiguous overrides ##Sparse properties:
      [ ]   %##Children.Gap: #True -> #False
      [ ]   %##Children.Ordered: (unset) -> #True
      [#] << ##Contiguous
      [#] << ##Rectangular
      [#] %##Depth.Max << $Dim
      [:] :*#$ValueType
```

### EC-24.12: ##Contiguous vs ##Sparse override

**EBNF:** `schema_line ::= "[#]" "<<" schema_ref` — contradicting schema compositions.
**What it tests:** #Map applies `##Sparse` (`%##Children.Gap << #True`). #Array inherits then applies `##Contiguous` (`%##Children.Gap << #False`, `%##Children.Ordered << #True`). This directly contradicts the inherited `%##Children.Gap` — the compiler raises PGW11002 (contradicting override). This is intentional: #Array IS a contiguous #Map variant. See [[syntax/types/schema-properties#Schema Properties]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
{#} ##Sparse
   [#] %##Children.Gap << #True

{#} ##Contiguous
   [#] %##Children.Gap << #False
   [#] %##Children.Ordered << #True

[ ] #Array inherits ##Sparse from #Map, then overrides with ##Contiguous
[ ] PGW11002 — %##Children.Gap contradicts inherited value
[ ] This is intentional — #Array is a contiguous specialization of #Map

[r] $arr#array:int <~ {1, 2, 3}
[ ] no gaps — contiguous enforced
[ ] ordered — indices 0, 1, 2
```

### EC-24.13: 0D array

**EBNF:** `array_type ::= "array" ":" element_type [ ":" dimension ]` — dimension 0.
**What it tests:** `$scalar#array:int:0D` — a 0D array is a typed scalar container holding exactly one element. Access is direct (no index). PGE04017 on any index attempt. See [[syntax/types/arrays#Multidimensional Arrays]].
**Cross-refs:** [[syntax/types/INDEX|types]]
**Status:** RESOLVED — 0D array semantics documented in types.md: direct access, no indexing, PGE04017 on index.

```polyglot
[ ] 0D array — scalar container
[r] $scalar#array:int:0D <~ {42}

[ ] %##Depth.Max = 0 — no flexible nesting
[ ] Holds exactly one element — access without index
[r] $val#int << $scalar

[ ] PGE04017 — no indices allowed on 0D
[r] $bad << $scalar:0
```

### EC-24.14: Empty collections

**EBNF:** `inline_data ::= "{" "}"` — zero-element initialization.
**What it tests:** Zero-element #Array and #Map. `%##Children.Min` is not explicitly set for #Array or #Map — the minimum child count defaults to 0. Empty collections are valid. See [[syntax/types/structs#Inline Data Shorthand]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
[ ] empty array — zero elements, valid
[r] $empty#array:int <~ {}

[ ] empty map — zero entries, valid
[r] $emptyMap#map:string:int <~ {}

[ ] %##Children.Min is not set — defaults to 0
[ ] Both are valid typed containers with no elements
```

### EC-24.15: Invalid key type (PGE11004)

**EBNF:** `schema_property ::= "%##Children.Type" "<<" type_ref` — key type must inherit #KeyString.
**What it tests:** `#map:int:string` — ##Int inherits from #String, NOT from #KeyString. The `%##Children.Type` must inherit `#KeyString` to exclude syntax-reserved characters. Compiler raises PGE11004. See [[syntax/types/basic-types#Layer 2c: #KeyString — Key Type for Tree Access]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
[ ] PGE11004 — #Int does not inherit #KeyString
[ ] #Int.regex allows "-7" — the hyphen is fine, but dots/colons are
[ ] not excluded by #Int's regex, only by #KeyString's
[r] $bad#map:int:string <~ {}

[ ] correct — #KeyString (default) excludes reserved chars
[r] $good#map:string:int <~ {}

[ ] correct — custom key type inheriting #KeyString
{#} #SafeKey
   [#] <~ #KeyString
   [.] .regex#RawString << "^[a-z][a-z0-9-]*$"

[r] $custom#map:SafeKey:int <~ {}
```

### EC-24.16: #Serial — no ## schema constraints

**EBNF:** `type_definition` — `%##Depth.Max << -1` on a stdlib type.
**What it tests:** #Serial uses `%##Depth.Max << -1` (unlimited depth). PGW11003 warns about unlimited depth on USER types, but #Serial is stdlib — it is the intentional escape hatch for schema-free data. Show that user types with `-1` get the warning but #Serial does not. See [[syntax/types/schema-properties#Schema Properties]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```polyglot
[ ] #Serial — stdlib, no PGW11003
{#} #Serial
   [#] %##Alias << "serial"
   [#] %##Children.Gap << #True
   [#] %##Children.Ordered << #False
   [#] %##Depth.Max << -1
   [:] :*#*

[ ] PGW11003 — user type with unlimited depth
{#} #MyFreeform
   [#] %##Depth.Max << -1
   [:] :*#*

[ ] no warning — user type with bounded depth
{#} #MyNested
   [#] %##Depth.Max << 3
   [:] :*#string
```

### EC-24.17: #Dataframe row-oriented access — RESOLVED

**EBNF:** `macro_def`, `child_access` — Dataframe is macro-generated, row-oriented (Array of Map).
**What it tests:** #Dataframe is generated by `{M} #Dataframe`. Access is row-oriented: `$df<row<column` (not column-oriented). Each row is a `#Map` keyed by the column enum; the outer structure is an `#Array` of rows. See [[syntax/types/INDEX|types]], [[stdlib/INDEX|Standard Library]], [[concepts/collections/INDEX|collections]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[stdlib/INDEX|Standard Library]], [[concepts/collections/INDEX|collections]], [[technical/ebnf/04-type-system#4.3]]

```polyglot
[ ] RESOLVED — #Dataframe is row-oriented (Array of Map)
{#} #SalesColumns
   [#] << ##Scalar
   [#] << ###Enum
   [.] .product
   [.] .price
   [.] .quantity

[r] $sales#dataframe:SalesColumns:string <~ {}

[ ] Row-oriented access: $df<row<column
[r] $name#string << $sales<0<product       [ ] row 0, column "product"
[r] $price#string << $sales<2<price        [ ] row 2, column "price"
[r] $row#map:SalesColumns:string << $sales<0   [ ] entire row as Map
```

### EC-24.18: Stale %Property notation

**EBNF:** `schema_property ::= "[#]" "%##" property_path "<<" value` — the `##` prefix is mandatory.
**What it tests:** stdlib `types.md` uses stale notation without the `##` prefix. The correct notation per `syntax/types.md` requires `%##`. Document the complete mapping. See [[syntax/types/schema-properties#Schema Properties]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[stdlib/INDEX|Standard Library]]

```polyglot
[ ] STALE notation (stdlib types.md)    -> CORRECT notation (syntax/types.md)
[ ] %Alias                                -> %##Alias
[ ] %Key.Type                             -> %##Children.Type
[ ] %Key.Gap                              -> %##Children.Gap
[ ] %Ordered                              -> %##Children.Ordered
[ ] %Depth.Max                            -> %##Depth.Max

[ ] Stale: stdlib types.md #Array definition
[#] %Alias << "array"
[#] %Key.Type << #UnsignedInt
[#] %Key.Gap << #False
[#] %Ordered << #True
[#] %Depth.Max << Dim

[ ] Correct: authoritative syntax/types.md notation
[#] %##Alias << "array"
[#] %##Children.Type << #UnsignedInt
[#] %##Children.Gap << #False
[#] %##Children.Ordered << #True
[#] %##Depth.Max << Dim
```

### EC-24.19: [M] merge behavior (identity rule)

**EBNF:** `macro_invoke ::= "[M]" "#" dotted_name` — macro invocation inside `{#}`.
**What it tests:** When `[M] #String.Subtype` is invoked inside `{#} ##Int`, the outer `{#}` names the result and the macro fills the body. The macro's internal `{#}` resolves to the same name (identity). Any `[#]` lines after `[M]` in the outer `{#}` extend or override the macro's output. See [[technical/ebnf/04-type-system#4.3]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[technical/ebnf/INDEX|EBNF]]

```polyglot
[ ] Outer {#} names the result — [M] fills the body
{#} ##Int
   [M] #String.Subtype
      [#] <Name << "Int"
      [#] <Alias << "int"
         [<] !Alias.Clash << "integer"
         [<] !Alias.Clash << "Integer"
      [#] <Regex << "^-?[0-9]+$"

[ ] Lines after [M] extend the macro output
{#} ##Custom
   [M] #String.Subtype
      [#] <Name << "Custom"
      [#] <Alias << "custom"
      [#] <Regex << "^[A-Z]{3}$"
   [ ] Additional schema property — extends macro output
   [#] %##MaxLength << 3
```

### EC-24.20: Macro dispatch ambiguity (PGE01019)

**EBNF:** `macro_def ::= "{M}" "#" dotted_name` — macro overloading by signature.
**What it tests:** Two `{M}` macros with the same name AND identical parameter signature (same count and kind) is a compile error PGE01019. Dispatch is unambiguous when signatures differ by count or kind (`<#` type vs `<` value). See [[technical/ebnf/04-type-system#4.3]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[technical/ebnf/INDEX|EBNF]]

```polyglot
[ ] Valid — different signatures
{M} #Map
   [#] <#KeyType
   [#] <#ValueType
   [ ] Signature: (<#, <#) — homogeneous

{M} #Map
   [#] <#KeyType
   [ ] Signature: (<#) — heterogeneous

[ ] Invalid — PGE01019: identical signatures
{M} #Foo
   [#] <#A
   [#] <#B
   [ ] Signature: (<#, <#)

{M} #Foo
   [#] <#X
   [#] <#Y
   [ ] Signature: (<#, <#) — CLASH with above -> PGE01019
```

### Potential Follow-up Issues

Issues discovered during this audit that may warrant separate GitHub issues:

1. ~~**#Dimension regex correction**~~ — RESOLVED: regex corrected to `"^[0-9]+D$"` (EC-24.3).
2. ~~**`<~` finality semantics**~~ — RESOLVED: PGE11005 added (EC-24.7).
3. ~~**#None ###-classification**~~ — RESOLVED: `###None` added as third field type, PGE04021 added (EC-24.10).
4. ~~**#Dataframe resolution**~~ — RESOLVED: promoted to authoritative spec as row-oriented `#Dataframe:ColumnEnum:CellType` (Array of Map) with `##EnumLeafs` (EC-24.17).
5. ~~**0D array semantics**~~ — RESOLVED: 0D = scalar container, direct access, PGE04017 on index (EC-24.13).
