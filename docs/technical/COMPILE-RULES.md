---
audience: designer
type: specification
updated: 2026-04-09
status: draft
---

# Polyglot Code — Compiler Rules

Semantic and behavioral constraints enforced at compile time. These rules go beyond EBNF grammar (which captures syntax only) and represent what the compiler must verify to accept a valid program.

**Legend:**
- `VALID` — accepted by compiler
- `INVALID` — rejected by compiler (error code shown)
- `WARNING` — accepted with a diagnostic (warning code shown)
- `[ ] ✓` — comment explaining why code is valid
- `[ ] ✗ PGExxNNN` — comment marking where the error is triggered
- `[ ] ⚠ PGWxxNNN` — comment marking where the warning is emitted

---

## Error Code Reference (PGE)

Error codes use the `PGExxNNN` format where `xx` is the category (01–99) and `NNN` is the sequential number (001–999). Categories are grouped by semantic domain — not by compiler phase — so codes stay stable as the compiler evolves.

### 01 — Pipeline Structure

| Code | Name |
|------|------|
| PGE01001 | Pipeline Section Misordering |
| PGE01002 | IO Before Trigger |
| PGE01003 | One Package Declaration Per File |
| PGE01004 | Definition Structural Constraints |
| PGE01005 | Missing Pipeline Trigger |
| PGE01006 | Missing Pipeline Queue |
| PGE01007 | Missing Pipeline Setup/Cleanup |
| PGE01008 | Wrapper Must Reference Wrapper Definition |
| PGE01009 | Wrapper IO Mismatch |
| PGE01010 | Pipeline IO Name Mismatch |
| PGE01011 | Duplicate IO Parameter Name |
| PGE01012 | Queue Definition Must Use #Queue: Prefix |
| PGE01013 | Queue Control Contradicts Queue Default |
| PGE01014 | Unresolved Queue Reference |
| PGE01015 | Duplicate Metadata Field |
| PGE01016 | Unmarked Execution Line |
| PGE01017 | Wrong Block Element Marker |
| PGE01018 | Tautological or Contradictory Trigger Condition |
| PGE01019 | *(Retired — macros removed; see #272)* |
| PGE01020 | Effectless Execution Expression |
| PGE01021 | Empty Data Definition |
| PGE01022 | Empty Error Namespace |
| PGE01023 | *(Retired — macros removed; see #272)* |
| PGE01024 | Incompatible Operation Marker |
| PGE01025 | Discard in Wrapper IO |
| PGE01026 | Orphan Continuation Line |
| PGE01027 | Empty Foreign Code Block |
| PGE01028 | Base-Derived Mutual Exclusion |
| PGE01029 | Invalid Marker for Definition Type |
| PGE01030 | Missing Pipeline Wrapper |
| PGE01031 | Forbidden Element in Definition |
| PGE01032 | Missing Trigger Boolean Output |
| PGE01033 | Unbound Script Variable |
| PGE01034 | Unbound Script Output |
| PGE01035 | Unbound Function Argument |
| PGE01036 | Unbound Function Kwarg |
| PGE01037 | Bind Schema Mismatch |
| PGE01038 | Code Source Conflict |
| PGE01039 | CLI Non-String Argument |
| PGE01040 | Orphan Parallel Marker |

### 02 — Lifecycle & State

| Code | Name |
|------|------|
| PGE02001 | Lifecycle Stages |
| PGE02002 | Declared State Is Unreadable |
| PGE02003 | Final Is Push-Once |
| PGE02004 | Default Allows One More Push *(retired → PGE02003)* |
| PGE02005 | Failed Is Terminal |
| PGE02006 | `live` Metadata Fields Are Pull-Only |
| PGE02007 | *(retired — `*Continue` removed; use `[!]` + `!<`/`!>` fallback)* |
| PGE02008 | Access After Release |
| PGE02009 | Unreachable Code |
| PGE02010 | Discard Default Assignment |
| PGE02011 | Data Load Schema Mismatch |
| PGE02012 | Duplicate Operation Label |
| PGE02013 | Write to Label Accessor |
| PGE02014 | Label Access Before Completion |
| PGE02015 | Unused Background Label |

### 03 — Parallelism

| Code | Name |
|------|------|
| PGE03001 | No Push Across Parallel Boundaries |
| PGE03002 | Parallel Output Must Be Collected |
| PGE03003 | Pull Isolation Until Collection |
| PGE03004 | Section-Boundary Pairing |
| PGE03005 | `[b]` Has No Collectible Output |
| PGE03006 | Race Collector Type Homogeneity |
| PGE03007 | Expand Operator Input Mismatch |
| PGE03008 | Collect Operator IO Mismatch |
| PGE03009 | Nested Expand Without Collect |
| PGE03010 | Collector Without Expand |
| PGE03011 | Orphaned Expand IO Marker |
| PGE03012 | Parallel Label Isolation |

### 04 — Types & Values

| Code | Name |
|------|------|
| PGE04001 | Type Mismatch |
| PGE04002 | Schema Mismatch |
| PGE04003 | Leaf-Only Assignment |
| PGE04004 | Fixed-Schema Keys Are Compile-Time Only |
| PGE04005 | Undefined Interpolation Variable |
| PGE04006 | Undefined Variable Reference |
| PGE04007 | Invalid Path String |
| PGE04008 | Missing Path Platform Subfield |
| PGE04009 | Unhandled Serial→Struct Conversion |
| PGE04010 | Invalid Arithmetic Operator |
| PGE04011 | Negative Array Index Literal |
| PGE04012 | Division by Literal Zero |
| PGE04013 | Nested Array Type |
| PGE04014 | Invalid Range Bounds |
| PGE04015 | Conditional Type-Operator Mismatch |
| PGE04016 | Invalid Pipeline Input Literal |
| PGE04017 | Array Dimension Access Mismatch |
| PGE04018 | Type Parameter Constraint Violation *(planned)* |
| PGE04019 | Duplicate Dictionary Key *(planned)* |
| PGE04020 | Key Gap Violation *(planned)* |
| PGE04021 | Empty String on Non-None Type *(planned)* |
| PGE04022 | Generic Type Constraint Violation *(planned)* |
| PGE04023 | Generic Field Constraint Violation *(planned)* |
| PGE04024 | Non-Value Comparison |
| PGE04025 | Untyped Array |
| PGE04026 | Invalid IANA Timezone |
| PGE04027 | Missing Required DateTime Subfield |
| PGE04028 | Invalid Epoch Value |

### 05 — Data Definitions

| Code | Name |
|------|------|
| PGE05001 | Sibling Separator Homogeneity |
| PGE05002 | Sibling Kind Homogeneity |
| PGE05003 | Duplicate Data Field Name |
| PGE05004 | Recursive Data Definition |
| PGE05005 | Mixed Field Kinds *(planned)* |
| PGE05006 | Schema Outside Type Definition *(planned)* |

### 06 — Conditionals

| Code | Name |
|------|------|
| PGE06001 | Conditional Must Be Exhaustive |
| PGE06002 | Enum Exhaustiveness |
| PGE06003 | Numeric Range Not Exhaustive |
| PGE06004 | Numeric Range Overlap |
| PGE06005 | Compound Condition Overlap |
| PGE06006 | String Exhaustiveness |
| PGE06007 | Flexible Field Exhaustiveness |
| PGE06008 | Compound Condition Exhaustiveness |
| PGE06009 | Conditional Missing Comparison Operator |
| PGE06010 | Empty Conditional Scope |
| PGE06011 | Duplicate Wildcard Catch-All |
| PGE06012 | Unreachable Branch After Wildcard |
| PGE06013 | Tautological or Contradictory Branch Condition |
| PGE06014 | Wildcard-Only Match |

### 07 — Error Handling

| Code | Name |
|------|------|
| PGE07001 | `[!]` Error Block Scoping |
| PGE07002 | Chain Error Scoping |
| PGE07003 | Duplicate Fallback Assignment |
| PGE07004 | Duplicate Error Handler |
| PGE07005 | Undeclared Error Raise |
| PGE07006 | Unused Error Declaration |
| PGE07007 | Error Handling Must Be Exhaustive |
| PGE07008 | Fallback on Non-Failable Source |
| PGE07009 | Unterminated Fallback Chain |

### 08 — Wiring & Calls

| Code | Name |
|------|------|
| PGE08001 | Auto-Wire Type Mismatch |
| PGE08002 | Auto-Wire Ambiguous Type |
| PGE08003 | Auto-Wire Unmatched Parameter |
| PGE08004 | Ambiguous Step Reference |
| PGE08005 | Unresolved Step Reference |
| PGE08006 | Non-Pipeline Step in Chain |
| PGE08007 | Invalid Assignment Target |
| PGE08008 | Missing Required Input at Call Site |
| PGE08009 | Uncaptured Required Output at Call Site |
| PGE08010 | IO Direction Mismatch |
| PGE08011 | Self-Assignment |
| PGE08012 | Self-Chain Requires Numeric Indexing |
| PGE08013 | Nested Inline Data |

### 09 — Imports & Dependencies

| Code | Name |
|------|------|
| PGE09001 | Undefined Import Alias |
| PGE09002 | Circular Package Dependency |
| PGE09003 | Unresolved Pipeline Reference |
| PGE09004 | Unresolved Import Pipeline Reference |
| PGE09005 | Multi-File Version Mismatch |
| PGE09006 | Multi-File Package Name Mismatch |
| PGE09007 | Duplicate Definition |
| PGE09008 | Multi-File Reference Not Found |
| PGE09009 | Multi-File Self-Reference |
| PGE09010 | Asymmetric Multi-File Reference |
| PGE09011 | Duplicate Import Alias |
| PGE09012 | Import Alias Shadows pglib |
| PGE09013 | Circular Pipeline Call |

### 10 — Permissions

| Code | Name |
|------|------|
| PGE10001 | Pipeline Exceeds Package Permission Ceiling *(planned)* |
| PGE10002 | Imported Package Exceeds Importer Permission Ceiling *(planned)* |
| PGE10003 | Unknown Permission Category |
| PGE10004 | Undeclared Permission |
| PGE10005 | Invalid Permission Block Marker |
| PGE10006 | Duplicate Permission |
| PGE10007 | Chain Step Label Overflow |
| PGE10008 | Parallel Write Permission Exclusion |

### 11 — Schema Properties

| Code | Name |
|------|------|
| PGE11001 | Schema Property Scope Conflict *(planned)* |
| PGE11002 | Unbounded Collection Nesting *(planned)* |
| PGE11003 | Field Type Contradiction *(planned)* |
| PGE11004 | Invalid Key Type *(planned)* |
| PGE11005 | Final Field Override via Inheritance *(planned)* |

### 12 — Metadata & Aliases

| Code | Name |
|------|------|
| PGE12001 | Undefined Metadata Field Access |
| PGE12002 | Duplicate Alias |
| PGE12003 | Undefined Inline Template |
| PGE12004 | Empty Metadata Alias |
| PGE12005 | Inline Format Mismatch |
| PGE12006 | Unresolved Template Placeholder |
| PGE12007 | Required Input Not In Template |
| PGE12008 | Duplicate Template Placeholder |
| PGE12009 | Template Type Coercion Failure |
| PGE12010 | Optional Placeholder Without Default |

## Warning Code Reference (PGW)

Warning codes use the `PGWxxNNN` format. Category numbers mirror PGE so a developer can immediately see which domain a warning relates to.

| Code | Name |
|------|------|
| PGW01001 | Empty Execution Body |
| PGW01002 | *(Retired — see PGE01021)* |
| PGW01003 | No Definitions in File |
| PGW02001 | Default Pull Across State Change |
| PGW02002 | Unused Variable |
| PGW02003 | Unpushed Output Port |
| PGW02004 | Pipeline Terminates on Error |
| PGW02005 | Unreachable Code |
| PGW03001 | `[b]` Called Pipeline Has Discarded Outputs |
| PGW03002 | Error Handler on Fire-and-Forget |
| PGW04001 | Single-Platform Path |
| PGW04002 | Leading Zeros in Literal |
| PGW07001 | Error Handler on Non-Failable Call |
| PGW07002 | Caller Overrides Pipeline Fallback |
| PGW07003 | Missing Fallback Message |
| PGW07004 | Fallback on Non-Failable IO |
| PGW07010 | Suppress on Consumed Output |
| PGW08001 | Auto-Wire Succeeded |
| PGW08002 | Unaddressed Input With Default |
| PGW08003 | Uncaptured Output With Default/Fallback |
| PGW09001 | Deprecated Pipeline Reference |
| PGW09002 | Unused Import |
| PGW10001 | Unused Permission |
| PGW11001 | Redundant Schema Property *(planned)* |
| PGW11002 | Contradicting Schema Override *(planned)* |
| PGW11003 | Unlimited Depth on User Type *(planned)* |
| PGW12001 | Template With No Placeholders |
| PGW12002 | Optional Placeholder Never Provided |

---

## Rule Format

Each rule follows this structure:

```markdown
### Rule N.N — Name
`PGExxNNN` or `PGWxxNNN`

**Statement:** What the compiler enforces.
**Rationale:** Why this constraint exists.

**VALID:**
​```polyglot
(minimal example — accepted by compiler)
​```

**INVALID:**
​```polyglot
(minimal example — triggers PGExxNNN)
​```

**WARNING:**
​```polyglot
(minimal example — triggers PGWxxNNN)
​```

**Open point:** (kept until resolved; removed when confirmed)
```

---

## Schema Rules

### Rule 9.21 — Schema Property Scope Conflict
`PGE11001`

**Statement:** A `%##` schema property set universally via `[#]` cannot also be assigned branch-wise via `[.]` or `[:]` in the same type definition.

**Rationale:** Universal properties apply to every branch uniformly. A branch-wise override would create an ambiguous structural invariant — the compiler cannot enforce both a universal constraint and a per-branch exception simultaneously.

**VALID:**
```polyglot
{#} #MyCollection
   [#] %##Gap << #False
   [:] :*#string
```

**INVALID:**
```polyglot
{#} #MyCollection
   [#] %##Gap << #False     [ ] ✗ PGE11001 — universal scope
   [.] .items
      [.] %##Gap << #True   [ ] ✗ PGE11001 — branch-wise conflicts with universal
```

### Rule 9.22 — Unbounded Collection Nesting
`PGE11002`

**Statement:** A collection type used as a value type within another collection must have an explicit `%##Depth.Max`. Depth must be bounded.

**Rationale:** Without a depth bound, nested collections can produce infinitely deep trees. The compiler requires an explicit limit so that tree traversal and memory allocation are predictable.

**VALID:**
```polyglot
{#} #Matrix
   [#] ##Array
      (#) <#ValueType << #float
      (#) <Dim << "2D"
   [#] %##Depth.Max << 2
```

**INVALID:**
```polyglot
{#} #Nested
   [#] ##Array
      (#) <#ValueType << #int        [ ] ✗ PGE11002 — no %##Depth.Max declared
```

**WARNING:**
```polyglot
{#} #FlexNested
   [#] ##Array
      (#) <#ValueType << #int
   [#] %##Depth.Max << #Inf          [ ] ⚠ PGW11003 — unlimited depth on user type
```

### Rule 9.23 — Field Type Contradiction
`PGE11003`

**Statement:** An explicit `###Value` declaration on a type whose fields are untyped enum fields, or an explicit `###Enum` declaration on a type whose fields have `#type` annotations, is a contradiction.

**Rationale:** The `###` classification must match the actual field declarations. Mismatches indicate a design error — the type's fields and its declared leaf nature disagree.

**VALID:**
```polyglot
{#} #Boolean
   [#] ###ScalarEnum
   [.] .True
   [.] .False
```

**INVALID:**
```polyglot
{#} #BadEnum
   [#] ###Enum                    [ ] ✗ PGE11003 — declares ###Enum
   [.] .name#string               [ ] ✗ PGE11003 — but fields have #type (value fields)
```

### Rule 9.24 — Invalid Key Type
`PGE11004`

**Statement:** When `%##Fields << #Range`, the implicit key type must inherit from `#KeyString`. Keys must exclude syntax-reserved characters (whitespace, `.`, `:`, `<`, `>`). For enum-keyed types (`%##Fields << SomeEnum`), key validity is guaranteed by the enum definition.

**Rationale:** Tree child keys appear in accessor syntax (`$var<key`). Types that permit syntax-reserved characters in their values would create parse ambiguity.

**VALID:**
```polyglot
{#} #NamedRecord
   [#] ##Flat
   [#] %##Fields << #Range
```

**INVALID:**
```polyglot
[ ] PGE11004 — explicit key type override that allows reserved chars
{#} #BadRecord
   [#] ##Flat
   [#] %##Fields << #Range
   [ ] ✗ PGE11004 if key type resolves to one allowing '.', ':', '<', '>'
```

### Rule 9.25 — Mixed Field Kinds
`PGE05005`

**Statement:** Sibling fields at the same level cannot mix typed (`#type` annotated) and untyped (enum) declarations. All siblings must be the same `###` kind.

**Rationale:** A type's fields are either all value fields or all enum fields. Mixing creates ambiguity in the `###` classification and violates the leaf-only values invariant for enum branches.

**VALID:**
```polyglot
{#} #Record
   [.] .name#string
   [.] .age#int
```

**INVALID:**
```polyglot
{#} #Mixed
   [.] .Active                    [ ] ✗ PGE05005 — untyped enum field
   [.] .count#int                 [ ] ✗ PGE05005 — typed value field at same level
```

### Rule 9.26 — Schema Outside Type Definition
`PGE05006`

**Statement:** `##` schema references are only valid inside `{#}` type definitions. Using `##` outside a `{#}` block is a compile error.

**Rationale:** Schemas describe tree structure of data types. They have no meaning outside a type definition context.

**VALID:**
```polyglot
{#} #MyType
   [#] ##Flat

{#} #UserData
   [#] ##Record
      (#) <#Fields << #UserFields
      (#) <#ValueType << #string
```

**INVALID:**
```polyglot
{-} -MyPipeline
   [-] $x << ##Flat               [ ] ✗ PGE05006 — ## used outside {#}
```

### Rule 9.27 — Final Field Override via Inheritance
`PGE11005`

**Statement:** If a parent type sets a field with `<<` (final), a child type inheriting via `<~` cannot redeclare that field. Any attempt to reassign a final-inherited field is a compile error.

**Rationale:** `<<` means the value is sealed — no further pushes, including through inheritance. Without this rule, a child type could silently override a field the parent declared immutable, breaking the finality guarantee.

**VALID:**
```polyglot
{#} #String
   [.] .regex#RawString <~ ".*"

[ ] ✓ .regex is <~ (default) in #String — child CAN override
{#} #Int
   (#) <~ #String
   [.] .regex#RawString << "^-?[0-9]+$"
```

**INVALID:**
```polyglot
{#} #Int
   (#) <~ #String
   [.] .regex#RawString << "^-?[0-9]+$"

[ ] ✗ PGE11005 — .regex is already << final in #Int
{#} #PositiveInt
   (#) <~ #Int
   [.] .regex#RawString << "^[1-9][0-9]*$"
```

### Rule 9.21w — Redundant Schema Property
`PGW11001`

**Statement:** A `%##` or `%###` property that is already inherited from a parent type or composed schema is redundant. The compiler emits a warning.

**Rationale:** Redundant declarations add noise. The inherited value already applies. If the intent is to override, the value must differ (see PGW11002).

**WARNING:**
```polyglot
{#} #MyArray
   [#] ##Array
      (#) <#ValueType << #int
   [#] %##Gap << #False                [ ] ⚠ PGW11001 — already set by ##Array
```

### Rule 9.22w — Contradicting Schema Override
`PGW11002`

**Statement:** A `%##` property that overrides an inherited value from a composed `##` schema emits a warning. The override takes effect, but the compiler flags it for verification.

**Rationale:** Overriding inherited schema properties is allowed but unusual. The warning ensures the developer intended the override rather than accidentally contradicting the schema.

**WARNING:**
```polyglot
{#} #SparseArray
   [#] ##Array
      (#) <#ValueType << #int
   [#] %##Gap << #True                 [ ] ⚠ PGW11002 — overrides #False from ##Array
```

### Rule 9.23w — Unlimited Depth on User Type
`PGW11003`

**Statement:** Setting `%##Depth.Max << .Inf` (unlimited depth via `##Inf`) on a user-defined type emits a warning. Only `#Serial` should use unlimited depth.

**Rationale:** Unlimited depth is a deliberate escape hatch for unconstrained data. User-defined types should have bounded depth for predictable tree traversal and memory use.

**WARNING:**
```polyglot
{#} #DeepTree
   [#] %##Depth.Max << .Inf         [ ] ⚠ PGW11003 — unlimited depth on user type
```

---

## Retired Codes

Codes that have been merged into other rules or removed. Listed here so searches for the old code find the redirect.

| Code | Name | Redirect |
|------|------|----------|
| PGE02004 | Default Allows Exactly One More Push | Merged into PGE02003 (Final Is Push-Once) |
| PGE09007† | Duplicate Data Definition (formerly separate code) | Merged into PGE09007 (Duplicate Definition) |
