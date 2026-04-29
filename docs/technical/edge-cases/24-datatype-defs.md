---
audience: design
type: reference
updated: 2026-04-09
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
| EC-24.8 | #Boolean — ##Enum + ##Scalar + ###ScalarEnum | Dual schema composition |
| EC-24.9 | Enum inheritance via `<~` | Extending enum variants |
| EC-24.10 | #None — minimal type | No fields, no schema |
| EC-24.11 | #Array as generic `{#}` with `(#) <#param` | Generic type definition with schema accumulation |
| EC-24.12 | %##Gap override on ##Array | Contradicting property override (PGW11002) |
| EC-24.13 | 0D array | Dimension collapse to scalar |
| EC-24.14 | Empty collections | Zero-element #Array and ##Record |
| EC-24.15 | Invalid key type | Non-#KeyString key (PGE11004) |
| EC-24.16 | #Serial — maximally permissive schemas | Unlimited depth escape hatch (PGW11003 exemption) |
| EC-24.17 | #Dataframe status | Row-oriented access via `$df<row<column` |
| EC-24.18 | Stale %Property notation | pglib types.md missing `##` prefix |
| EC-24.19 | *(Retired)* Macro merge behavior | Macros removed — see #272 |
| EC-24.20 | *(Retired)* Macro dispatch ambiguity | Macros removed — see #272 |

---

### EC-24.1: #String `.regex` default

**EBNF:** `type_definition ::= "{#}" type_header { schema_line | field_line }` — field defaults via `<~`.
**What it tests:** Empty string `""` matches `.regex = ".*"` — the default regex accepts all strings, including empty. Verifies `##Scalar` is inherited via `[#] ##Scalar`. See [[syntax/types/basic-types#Layer 1: #String — The Foundation Type]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[variable-lifecycle]]

```aljam3
{#} #String
   [#] ##Scalar
   [#] %##Alias << "string"
   [.] .string#RawString
   [.] .regex#RawString <~ ".*"

[ ] empty string matches ".*"
[-] $empty#string << ""

[ ] any content matches ".*"
[-] $anything#string << "hello world 123 !@#"
```

### EC-24.2: #Int leading zeros and negative zero

**EBNF:** `type_definition` — `.regex` regex `"^-?[0-9]+$"` accepts leading zeros and `-0`.
**What it tests:** `"007"` and `"-0"` both match `#Int`'s regex. Leading zeros are valid serialized strings — Aljam3 does not normalize numeric representations. See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
{#} #Int
   (#) <~ #String
   [#] %##Alias << "int"
   [.] .regex#RawString << "^-?[0-9]+$"

[ ] leading zeros match regex
[-] $padded#int << 007

[ ] negative zero matches regex
[-] $negZero#int << -0

[ ] PGE04010 — decimal point not in regex
[-] $bad#int << 3.14
```

### EC-24.3: #Dimension 0D

**EBNF:** `type_definition` — `.regex` for #Dimension and the `:ND` syntax sugar.
**What it tests:** 0D is valid for scalars. The stored value includes the `D` suffix — `"2D"`, not `"2"`. Regex is `"^[0-9]+D$"`. See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[pglib/INDEX|Standard Library]]
**Status:** RESOLVED — regex corrected to `"^[0-9]+D$"` in both syntax/types.md and pglib/types/scalars.md.

```aljam3
[ ] Authoritative definition — corrected regex
{#} #Dimension
   (#) <~ #String
   [#] %##Alias << "dim"
   [ ] Stored value includes D suffix
   [.] .regex#RawString << "^[0-9]+D$"

[ ] 0D means scalar dimension
[-] $scalar#array:int:0D

[ ] standard dimensions
[-] $vector#array:float:1D
[-] $matrix#array:float:2D
```

### EC-24.4: #Eng exponent

**EBNF:** `type_definition` — `.regex` enforces multiples-of-3 exponent.
**What it tests:** `"1.5e4"` fails (4 is not a multiple of 3), `"1.5e3"` passes. The regex `(0|[369]|[1-9][0-9]*[0369])` constrains the exponent to 0, 3, 6, 9, ... See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
{#} #Eng
   (#) <~ #String
   [#] %##Alias << "eng"
   [.] .regex#RawString << "^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0|[369]|[1-9][0-9]*[0369])$"

[ ] exponent is multiple of 3
[-] $valid#eng << "1.5e3"
[-] $micro#eng << "2.47e-6"
[-] $tera#eng << "9.99e12"

[ ] PGE04010 — exponent 4 is not a multiple of 3
[-] $bad#eng << "1.5e4"

[ ] PGE04010 — exponent 1 is not a multiple of 3
[-] $bad2#eng << "3.0e1"
```

### EC-24.5: #KeyString excluded chars

**EBNF:** `type_definition` — `.regex` excludes syntax-reserved characters.
**What it tests:** `"my.key"` fails (dot is reserved for fixed-field navigation), `"my-key"` passes. Ensures tree path safety for `<` child access. See [[syntax/types/basic-types#Layer 2c: #KeyString — Key Type for Tree Access]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
{#} #KeyString
   (#) <~ #String
   [#] %##Alias << "key"
   [.] .regex#RawString << "^[^\s.<>:]+$"

[ ] hyphen allowed
[-] $valid#key << "my-key"
[-] $underscored#key << "my_key_123"

[ ] PGE04010 — dot is reserved (fixed-field separator)
[-] $dotted#key << "my.key"

[ ] PGE04010 — colon is reserved (flexible-field separator)
[-] $coloned#key << "my:key"

[ ] PGE04010 — angle bracket is reserved
[-] $angled#key << "my<key"
```

### EC-24.6: #NestedKeyString allows dot/colon

**EBNF:** `type_definition` — `.regex` allows `.` and `:` but excludes whitespace and angle brackets.
**What it tests:** `"error.file.read"` passes (dots allowed for alias paths), `"my<key"` fails. Used as element type for `%##Alias` paths. See [[syntax/types/basic-types#Layer 2d: #NestedKeyString — Key Type for Alias Paths]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
{#} #NestedKeyString
   (#) <~ #String
   [#] %##Alias << "nestedkey"
   [.] .regex#RawString << "^[^\s<>]+$"

[ ] dots and colons allowed for nested paths
[-] $alias#nestedkey << "error.file.read"
[-] $nested#nestedkey << "String:int"

[ ] PGE04010 — angle bracket excluded
[-] $bad#nestedkey << "my<key"

[ ] PGE04010 — whitespace excluded
[-] $bad2#nestedkey << "my key"
```

### EC-24.7: `<~` inheritance chain finality

**EBNF:** `inheritance ::= "[#]" "<~" type_ref` — schema inheritance with `<~` (overridable) vs `<<` (final).
**What it tests:** #Int inherits `.string` and `.regex` from #String via `<~`, then sets `.regex` with `<<` (final). A user-defined `#PositiveInt <~ #Int` cannot override `.regex` — it is already final. See [[syntax/types/basic-types#Layer 2: Scalar Subtypes — Specialize .regex]], [[variable-lifecycle]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[variable-lifecycle]]
**Status:** RESOLVED — PGE11005 (Final Field Override via Inheritance) added to COMPILE-RULES.md.

```aljam3
[ ] #String sets .regex with <~ (overridable default)
{#} #String
   [.] .regex#RawString <~ ".*"

[ ] #Int inherits and sets .regex with << (final)
{#} #Int
   (#) <~ #String
   [.] .regex#RawString << "^-?[0-9]+$"

[ ] PGE11005 — .regex is already << final in #Int
{#} #PositiveInt
   (#) <~ #Int
   [.] .regex#RawString << "^[1-9][0-9]*$"
```

### EC-24.8: #Boolean — ##Enum + ##Scalar + ###ScalarEnum

**EBNF:** `schema_line ::= "[#]" schema_ref` — multiple schema compositions accumulate.
**What it tests:** #Boolean composes `##Enum` (enum classification), `##Scalar` (depth 1), and `###ScalarEnum` (leaf content is variant selector). A type can be both scalar AND enum. Can a user define a custom enum with the same pattern? Yes — `###ScalarEnum` and `##Scalar` are orthogonal. See [[syntax/types/basic-types#Layer 2b: #Boolean — Independent ##Enum Type]], [[syntax/types/schema-properties#`###` Field Types — Leaf Content]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
{#} #Boolean
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "bool"
   [.] .True
   [.] .False

[ ] user-defined enum with same pattern
{#} #TrafficLight
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [.] .Red
   [.] .Yellow
   [.] .Green

[ ] usage
[-] $light << #TrafficLight.Red
```

### EC-24.9: Enum inheritance via `<~`

**EBNF:** `inheritance ::= "[#]" "<~" type_ref` — `<~` on an enum type.
**What it tests:** User defines `#MyStatus <~ #PipelineStatus` to add new variants. Enums use `[.]` fixed fields — `<~` on an enum should extend the field set (inheriting all parent variants). See [[syntax/types/structs#Enum Fields vs Value Fields]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
{#} #PipelineStatus
   [#] ###Enum
   [.] .AwaitTrigger
   [.] .Disabled
   [.] .Running
   [.] .Failed

[ ] Extends #PipelineStatus with additional variants
{#} #MyStatus
   (#) <~ #PipelineStatus
   [.] .Degraded
   [.] .Maintenance

[ ] inherited variants still accessible
[-] $status << #MyStatus.Running

[ ] new variant accessible
[-] $status << #MyStatus.Degraded

[ ] parent type does not gain child's variants
[-] $bad << #PipelineStatus.Degraded
```

### EC-24.10: #None — minimal type definition

**EBNF:** `type_definition ::= "{#}" type_header { schema_line | field_line }` — zero fields, `###None` field type.
**What it tests:** #None has no fields and uses `###None` — a third field type meaning "nullable." Empty string `""` is the only valid value. Only `###None` types accept empty string; all others reject it with PGE04021. See [[syntax/types/INDEX|types]], [[syntax/types/schema-properties#`###` Field Types — Leaf Content]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[pglib/INDEX|Standard Library]]
**Status:** RESOLVED — `###None` added as third field type, PGE04021 added, #None definition updated with `[#] ###None`.

```aljam3
{#} #None
   [ ] Represents the absence of a value
   [ ] Empty string "" is the only valid value
   [#] ##Scalar
   [#] ###None

[ ] usage — signals absence
[-] $result << #None

[ ] PGE04021 — empty string on non-###None type
[-] $bad#string << ""
```

### EC-24.11: #Array as generic `{#}` with `(#) <#param`

**EBNF:** `generic_param ::= "(#)" "<#" name`, `value_param ::= "(#)" "<" name schema_id`, `schema_param_bind` — generic type definition with parameterized schema composition.
**What it tests:** `{#} #Array` is a generic type with `<#ValueType` (type input) and `<Dim` (value input with default "1D"). Schema properties accumulate from `##Array`. The `:` separator in type annotations binds positionally: `#array:float:2D` → ValueType-Float, Dim=2D. See [[syntax/types/schema-properties#Approved ## Schema Types]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[pglib/INDEX|Standard Library]], [[technical/ebnf/04-type-system#4.3]]

```aljam3
[ ] {#} #Array — generic type with two parameters
{#} #Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] ##Array
      (#) <#ValueType << <#ValueType
      (#) <Dim << <Dim
   [#] %##Alias << "array"
   [#] %##Fields << #Range
   [ ] ##Array provides: %##Gap << #False, %##Ordered << #True, %##Propagate << #True
   [:] :*#<#ValueType
```

### EC-24.12: %##Gap override on ##Array

**EBNF:** `schema_line ::= "[#]" schema_ref` — contradicting property overrides.
**What it tests:** ##Array sets `%##Gap << #False`. A user type composing ##Array and then setting `%##Gap << #True` triggers PGW11002 (contradicting override). This tests that property contradictions are detected across composition. See [[syntax/types/schema-properties#Schema Properties]].
**Cross-refs:** [[syntax/types/INDEX|types]]
**Status:** Updated for #275 — ##Contiguous/##Sparse retired; properties stated directly on schemas.

```aljam3
[ ] ##Array sets %##Gap << #False, %##Ordered << #True
[ ] Overriding %##Gap on a type composing ##Array triggers PGW11002

{#} #SparseCustom
   [#] ##Array
      (#) <#ValueType << #int
   [#] %##Gap << #True                 [ ] ⚠ PGW11002 — overrides #False from ##Array

[-] $arr#array:int <~ {1, 2, 3}
[ ] no gaps — contiguous enforced by ##Array
[ ] ordered — indices 0, 1, 2
```

### EC-24.13: 0D array

**EBNF:** `array_type ::= "array" ":" element_type [ ":" dimension ]` — dimension 0.
**What it tests:** `$scalar#array:int:0D` — a 0D array is a typed scalar container holding exactly one element. Access is direct (no index). PGE04017 on any index attempt. See [[syntax/types/arrays#Multidimensional Arrays]].
**Cross-refs:** [[syntax/types/INDEX|types]]
**Status:** RESOLVED — 0D array semantics documented in types.md: direct access, no indexing, PGE04017 on index.

```aljam3
[ ] 0D array — scalar container
[-] $scalar#array:int:0D <~ {42}

[ ] %##Depth.Max = 0 — no flexible nesting
[ ] Holds exactly one element — access without index
[-] $val#int << $scalar

[ ] PGE04017 — no indices allowed on 0D
[-] $bad << $scalar:0
```

### EC-24.14: Empty collections

**EBNF:** `inline_data ::= "{" "}"` — zero-element initialization.
**What it tests:** Zero-element #Array and ##Record-based types. `%##Count.Min` is not explicitly set for #Array — the minimum child count defaults to 0. Empty collections are valid. See [[syntax/types/structs#Inline Data Shorthand]].
**Cross-refs:** [[syntax/types/INDEX|types]]
**Status:** Updated for #275 — #Map retired; ##Record replaces it.

```aljam3
[ ] empty array — zero elements, valid
[-] $empty#array:int <~ {}

[ ] %##Count.Min is not set — defaults to 0
[ ] Valid typed container with no elements
```

### EC-24.15: Invalid key type (PGE11004)

**EBNF:** `schema_property ::= "%##Fields" "<<" value` — when using `#Range`, key type must be valid.
**What it tests:** When `%##Fields << #Range`, the implicit key type inherits `#KeyString` to exclude syntax-reserved characters. Enum-keyed records (`%##Fields << SomeEnum`) have guaranteed-safe keys from enum variant names. Compiler raises PGE11004 if a range key type resolves to one allowing reserved chars. See [[syntax/types/basic-types#Layer 2c: #KeyString — Key Type for Tree Access]].
**Cross-refs:** [[syntax/types/INDEX|types]]
**Status:** Updated for #275 — %##Key retired; %##Fields replaces it. #Map retired; ##Record replaces it.

```aljam3
[ ] ##Record with enum keys — safe by definition
{#} #UserRecord
   [#] ##Record
      (#) <#Fields << #UserFields
      (#) <#ValueType << #string
[ ] enum variant names cannot contain reserved chars

[ ] correct — custom key type inheriting #KeyString
{#} #SafeKey
   (#) <~ #KeyString
   [.] .regex#RawString << "^[a-z][a-z0-9-]*$"
```

### EC-24.16: #Serial — maximally permissive schemas

**EBNF:** `type_definition` — `%##Depth.Max << .Inf` on a pglib type.
**What it tests:** #Serial uses maximally permissive schema properties to remove every structural constraint. PGW11003 warns about unlimited depth on USER types, but #Serial is pglib — it is the intentional escape hatch for unconstrained data. Show that user types with `.Inf` get the warning but #Serial does not. See [[syntax/types/schema-properties#Schema Properties]].
**Cross-refs:** [[syntax/types/INDEX|types]]

```aljam3
[ ] #Serial — pglib, no PGW11003
{#} #Serial
   [#] %##Alias << "serial"
   [#] %##Depth.Max << #Inf
   [#] %##Gap << #True
   [#] %##Ordered << #False
   [#] %##Count << #Inf
   [#] %##Fields << #Range

[ ] PGW11003 — user type with unlimited depth
{#} #MyFreeform
   [#] %##Depth.Max << #Inf

[ ] no warning — user type with bounded depth
{#} #MyNested
   [#] %##Depth.Max << 3
   [:] :*#string
```

### EC-24.17: #Dataframe row-oriented access — RESOLVED

**EBNF:** `generic_param`, `child_access` — Dataframe is a generic `{#}` type, row-oriented (Array of ##Record).
**What it tests:** #Dataframe is a generic type with `<#Columns` and `<#CellType` parameters, composing `##Dataframe`. Access is row-oriented: `$df<row<column` (not column-oriented). Each row is a ##Record keyed by the column enum; the outer structure is an `#Array` of rows. See [[syntax/types/INDEX|types]], [[pglib/INDEX|Standard Library]], [[concepts/collections/INDEX|collections]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[pglib/INDEX|Standard Library]], [[concepts/collections/INDEX|collections]], [[technical/ebnf/04-type-system#4.3]]

```aljam3
[ ] RESOLVED — #Dataframe is row-oriented (Array of ##Record)
{#} #SalesColumns
   [#] ##Scalar
   [#] ###ScalarEnum
   [.] .product
   [.] .price
   [.] .quantity

[-] $sales#dataframe:SalesColumns:string <~ {}

[ ] Row-oriented access: $df<row<column
[-] $name#string << $sales<0<product       [ ] row 0, column "product"
[-] $price#string << $sales<2<price        [ ] row 2, column "price"
[-] $row << $sales<0                            [ ] entire row as ##Record
```

### EC-24.18: Stale %Property notation

**EBNF:** `schema_property ::= "[#]" "%##" property_path "<<" value` — the `##` prefix is mandatory.
**What it tests:** pglib `types.md` uses stale notation without the `##` prefix. The correct notation per `syntax/types.md` requires `%##`. Document the complete mapping. See [[syntax/types/schema-properties#Schema Properties]].
**Cross-refs:** [[syntax/types/INDEX|types]], [[pglib/INDEX|Standard Library]]

```aljam3
[ ] STALE notation (pglib types.md)    -> CORRECT notation (syntax/types.md)
[ ] %Alias                                -> %##Alias
[ ] %Key.Type                             -> %##Key
[ ] %Key.Gap                              -> %##Gap
[ ] %Ordered                              -> %##Ordered
[ ] %Depth.Max                            -> %##Depth.Max

[ ] Stale: pglib types.md #Array definition
[#] %Alias << "array"
[#] %Key.Type << #UnsignedInt
[#] %Key.Gap << #False
[#] %Ordered << #True
[#] %Depth.Max << Dim

[ ] Correct: authoritative notation (Issue #272 redesign)
[#] %##Alias << "array"
[#] %##Key << #uint
[#] %##Gap << #False
[#] %##Ordered << #True
[#] %##Depth.Max << Dim
```

### EC-24.19: *(Retired)* Macro merge behavior

**Status:** Retired — macro block type removed in Issue #272. Parameterized `##` schemas with `[#]` inputs now handle type generation. See [[technical/ebnf/04-type-system#4.3]].

### EC-24.20: *(Retired)* Macro dispatch ambiguity (PGE01019)

**Status:** Retired — macro block type removed in Issue #272. PGE01019 retired. Parameterized `##` schemas with `[#]` inputs now handle type generation. See [[technical/ebnf/04-type-system#4.3]].

### Potential Follow-up Issues

Issues discovered during this audit that may warrant separate GitHub issues:

1. ~~**#Dimension regex correction**~~ — RESOLVED: regex corrected to `"^[0-9]+D$"` (EC-24.3).
2. ~~**`<~` finality semantics**~~ — RESOLVED: PGE11005 added (EC-24.7).
3. ~~**#None ###-classification**~~ — RESOLVED: `###None` added as third field type, PGE04021 added (EC-24.10).
4. ~~**#Dataframe resolution**~~ — RESOLVED: promoted to authoritative spec as row-oriented `#Dataframe:ColumnEnum:CellType` (Array of Map) with `##Dataframe` schema (EC-24.17).
5. ~~**0D array semantics**~~ — RESOLVED: 0D = scalar container, direct access, PGE04017 on index (EC-24.13).
