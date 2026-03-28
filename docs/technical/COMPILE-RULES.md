---
audience: developer
type: specification
updated: 2026-03-28
status: draft
---

# Polyglot Code — Compiler Rules

Semantic and behavioral constraints enforced at compile time. These rules go beyond EBNF grammar (which captures syntax only) and represent what the compiler must verify to accept a valid program.

**Legend:**
- `VALID` — accepted by compiler
- `INVALID` — rejected by compiler (error code shown)
- `WARNING` — accepted with a diagnostic (warning code shown)
- `[ ] ✓` — comment explaining why code is valid
- `[ ] ✗ PGE-NNN` — comment marking where the error is triggered
- `[ ] ⚠ PGW-NNN` — comment marking where the warning is emitted

---

## Error Code Reference (PGE)

Error codes use the `PGE-NNN` format. Ranges are grouped by semantic category — not by compiler phase — so codes stay stable as the compiler evolves.

| Code | Rule | Name |
|------|------|------|
| PGE-101 | 1.1 | Pipeline Section Misordering |
| PGE-102 | 1.2 | IO Before Trigger |
| PGE-103 | 1.3 | One Package Declaration Per File |
| PGE-104 | 1.4 | Macro Structural Constraints |
| PGE-105 | 1.5 | Missing Pipeline Trigger |
| PGE-106 | 1.6 | Missing Pipeline Queue |
| PGE-107 | 1.7 | Missing Pipeline Setup/Cleanup |
| PGE-108 | 1.8 | Wrapper Must Reference Macro |
| PGE-109 | 1.9 | Wrapper IO Mismatch |
| PGE-110 | 1.10 | Pipeline IO Name Mismatch |
| PGE-111 | 1.11 | Duplicate IO Parameter Name |
| PGE-112 | 1.12 | Queue Definition Must Use #Queue: Prefix |
| PGE-113 | 1.13 | Queue Control Contradicts Queue Default |
| PGE-114 | 1.14 | Unresolved Queue Reference |
| PGE-115 | 1.15 | Duplicate Metadata Field |
| PGE-116 | 1.16 | Unmarked Execution Line |
| PGE-117 | 1.17 | Wrong Block Element Marker |
| PGE-118 | 1.18 | Tautological or Contradictory Trigger Condition |
| PGE-201 | 2.1 | Lifecycle Stages |
| PGE-202 | 2.2 | Declared State Is Unreadable |
| PGE-203 | 2.3 | Final Is Push-Once |
| PGE-205 | 2.5 | Failed Is Terminal |
| PGE-206 | 2.6 | `live` Metadata Fields Are Pull-Only |
| PGE-207 | 2.7 | Continue After Error |
| PGE-208 | 2.8 | Access After Release |
| PGE-209 | 2.9 | Unreachable Code |
| PGE-301 | 3.1 | No Push Across Parallel Boundaries |
| PGE-302 | 3.2 | Parallel Output Must Be Collected |
| PGE-303 | 3.3 | Pull Isolation Until Collection |
| PGE-304 | 3.4 | Section-Boundary Pairing |
| PGE-305 | 3.5 | `[b]` Has No Collectible Output |
| PGE-306 | 3.6 | Race Collector Type Homogeneity |
| PGE-307 | 3.7 | Expand Operator Input Mismatch |
| PGE-308 | 3.8 | Collect Operator IO Mismatch |
| PGE-309 | 3.9 | Nested Expand Without Collect |
| PGE-311 | 3.11 | Collector Without Expand |
| PGE-401 | 4.1 | Type Mismatch |
| PGE-402 | 4.2 | Schema Mismatch |
| PGE-403 | 4.3 | Leaf-Only Assignment |
| PGE-404 | 4.4 | Fixed-Schema Keys Are Compile-Time Only |
| PGE-405 | 4.5 | Undefined Interpolation Variable |
| PGE-406 | 4.6 | Undefined Variable Reference |
| PGE-407 | 4.7 | Invalid Path String |
| PGE-408 | 4.8 | Missing Path Platform Subfield |
| PGE-409 | 4.9 | Unhandled Serial→Struct Conversion |
| PGE-410 | 4.10 | Invalid Arithmetic Operator |
| PGE-411 | 4.11 | Negative Array Index Literal |
| PGE-412 | 4.12 | Nested Array Type |
| PGE-413 | 4.13 | Duplicate Data Field Name |
| PGE-414 | 4.14 | Recursive Data Definition |
| PGE-415 | 4.15 | Conditional Type-Operator Mismatch |
| PGE-416 | 4.16 | Invalid Pipeline Input Literal |
| PGE-417 | 4.17 | Array Dimension Access Mismatch |
| PGE-418 | 4.18 | Type Parameter Constraint Violation |
| PGE-419 | 4.19 | Duplicate Dictionary Key |
| PGE-420 | 4.20 | Key Gap Violation |
| PGE-421 | 4.21 | Empty String on Non-None Type |
| PGE-501 | 5.1 | Sibling Separator Homogeneity |
| PGE-502 | 5.2 | Sibling Kind Homogeneity |
| PGE-601 | 6.1 | Conditional Must Be Exhaustive |
| PGE-602 | 6.2 | Enum Exhaustiveness |
| PGE-603 | 6.3 | Numeric Range Not Exhaustive |
| PGE-604 | 6.4 | Numeric Range Overlap |
| PGE-605 | 6.5 | Compound Condition Overlap |
| PGE-606 | 6.6 | String Exhaustiveness |
| PGE-607 | 6.7 | Flexible Field Exhaustiveness |
| PGE-608 | 6.8 | Compound Condition Exhaustiveness |
| PGE-609 | 6.9 | Conditional Missing Comparison Operator |
| PGE-610 | 6.10 | Empty Conditional Scope |
| PGE-611 | 6.11 | Duplicate Wildcard Catch-All |
| PGE-612 | 6.12 | Unreachable Branch After Wildcard |
| PGE-613 | 6.13 | Tautological or Contradictory Branch Condition |
| PGE-701 | 7.1 | `[!]` Error Block Scoping |
| PGE-702 | 7.2 | Chain Error Scoping |
| PGE-703 | 7.3 | Duplicate Fallback Assignment |
| PGE-704 | 7.4 | Duplicate Error Handler |
| PGE-705 | 7.5 | Undeclared Error Raise |
| PGE-706 | 7.6 | Unused Error Declaration |
| PGE-707 | 7.7 | Error Handling Must Be Exhaustive |
| PGE-801 | 8.1 | Auto-Wire Type Mismatch |
| PGE-802 | 8.2 | Auto-Wire Ambiguous Type |
| PGE-803 | 8.3 | Auto-Wire Unmatched Parameter |
| PGE-804 | 8.4 | Ambiguous Step Reference |
| PGE-805 | 8.5 | Unresolved Step Reference |
| PGE-806 | 8.6 | Non-Pipeline Step in Chain |
| PGE-807 | 8.7 | Invalid Assignment Target |
| PGE-808 | 8.8 | Missing Required Input at Call Site |
| PGE-809 | 8.9 | Uncaptured Required Output at Call Site |
| PGE-810 | 8.10 | IO Direction Mismatch |
| PGE-901 | 9.1 | Undefined Import Alias |
| PGE-902 | 9.2 | Circular Package Dependency |
| PGE-903 | 9.3 | Unresolved Pipeline Reference |
| PGE-904 | 9.4 | Unresolved Import Pipeline Reference |
| PGE-905 | 9.5 | Multi-File Version Mismatch |
| PGE-906 | 9.6 | Multi-File Package Name Mismatch |
| PGE-907 | 9.7 | Duplicate Definition |
| PGE-909 | 9.9 | Multi-File Reference Not Found |
| PGE-910 | 9.10 | Multi-File Self-Reference |
| PGE-911 | 9.11 | Asymmetric Multi-File Reference |
| PGE-914 | 9.14 | Circular Pipeline Call |
| PGE-915 | 9.15 | Pipeline Exceeds Package Permission Ceiling |
| PGE-916 | 9.16 | Imported Package Exceeds Importer Permission Ceiling |
| PGE-917 | 9.17 | Unknown Permission Category |
| PGE-918 | 9.18 | Undeclared Permission |
| PGE-919 | 9.19 | Permission Output |
| PGE-920 | 9.20 | Duplicate Permission |
| PGE-921 | 9.21 | Schema Property Scope Conflict |
| PGE-922 | 9.22 | Unbounded Collection Nesting |
| PGE-923 | 9.23 | Field Type Contradiction |
| PGE-924 | 9.24 | Invalid Key Type |
| PGE-925 | 9.25 | Mixed Field Kinds |
| PGE-926 | 9.26 | Schema Outside Type Definition |
| PGE-927 | 9.27 | Final Field Override via Inheritance |
| PGE-1001 | 10.1 | Undefined Metadata Field Access |
| PGE-1002 | 10.2 | Duplicate Alias |

## Warning Code Reference (PGW)

Warning codes use the `PGW-NNN` format. Category ranges mirror PGE so a developer can immediately see which domain a warning relates to.

| Code | Rule | Name |
|------|------|------|
| PGW-101 | 1.1 | Empty Execution Body |
| PGW-102 | 1.2w | Empty Data Definition |
| PGW-201 | 2.7 | Default Pull Across State Change |
| PGW-202 | 2.2 | Unused Variable |
| PGW-203 | 2.3 | Unpushed Output Port |
| PGW-205 | 2.5 | Pipeline Terminates on Error |
| PGW-301 | 3.5 | `[b]` Called Pipeline Has Discarded Outputs |
| PGW-302 | 3.5w | Error Handler on Fire-and-Forget |
| PGW-408 | 4.8 | Single-Platform Path |
| PGW-801 | 8.1 | Auto-Wire Succeeded |
| PGW-808 | 8.8w | Unaddressed Input With Default |
| PGW-809 | 8.9w | Uncaptured Output With Default/Fallback |
| PGW-701 | 7.1w | Error Handler on Non-Failable Call |
| PGW-702 | 7.2w | Caller Overrides Pipeline Fallback |
| PGW-703 | 7.3w | Missing Fallback Message |
| PGW-704 | 7.4w | Fallback on Non-Failable IO |
| PGW-901 | 9.1 | Deprecated Pipeline Reference |
| PGW-902 | 9.2 | Unused Import |
| PGW-903 | 9.3 | Unused Permission |
| PGW-904 | 9.21w | Redundant Schema Property |
| PGW-905 | 9.22w | Contradicting Schema Override |
| PGW-906 | 9.23w | Unlimited Depth on User Type |
| PGW-1001 | 2.9 | Unreachable Code |
| PGW-1002 | 10.2 | Missing Inline Format Metadata |

---

## Rule Format

Each rule follows this structure:

```
### Rule N.N — Name
`PGE-NNN` or `PGW-NNN`

**Statement:** What the compiler enforces.
**Rationale:** Why this constraint exists.

**VALID:**
​```polyglot
(minimal example — accepted by compiler)
​```

**INVALID:**
​```polyglot
(minimal example — triggers PGE-NNN)
​```

**WARNING:**
​```polyglot
(minimal example — triggers PGW-NNN)
​```

**Open point:** (kept until resolved; removed when confirmed)
```

---

## Schema Rules

### Rule 9.21 — Schema Property Scope Conflict
`PGE-921`

**Statement:** A `%##` schema property set universally via `[#]` cannot also be assigned branch-wise via `[.]` or `[:]` in the same type definition.

**Rationale:** Universal properties apply to every branch uniformly. A branch-wise override would create an ambiguous structural invariant — the compiler cannot enforce both a universal constraint and a per-branch exception simultaneously.

**VALID:**
```polyglot
{#} #MyCollection
   [#] %##Children.Gap << #False
   [:] :*#string
```

**INVALID:**
```polyglot
{#} #MyCollection
   [#] %##Children.Gap << #False     [ ] ✗ PGE-921 — universal scope
   [.] .items
      [.] %##Children.Gap << #True   [ ] ✗ PGE-921 — branch-wise conflicts with universal
```

### Rule 9.22 — Unbounded Collection Nesting
`PGE-922`

**Statement:** A collection type used as a value type within another collection must have an explicit `%##Depth.Max`. Depth must be bounded.

**Rationale:** Without a depth bound, nested collections can produce infinitely deep trees. The compiler requires an explicit limit so that tree traversal and memory allocation are predictable.

**VALID:**
```polyglot
{#} #Matrix
   [#] <~ #Array<#Array<#float
   [#] %##Depth.Max << 2
```

**INVALID:**
```polyglot
{#} #Nested
   [#] <~ #Array<#Array<#float     [ ] ✗ PGE-922 — no %##Depth.Max declared
```

**WARNING:**
```polyglot
{#} #FlexNested
   [#] <~ #Array<#Array<#float
   [#] %##Depth.Max << -1          [ ] ⚠ PGW-906 — unlimited depth on user type
```

### Rule 9.23 — Field Type Contradiction
`PGE-923`

**Statement:** An explicit `###Value` declaration on a type whose fields are untyped enum fields, or an explicit `###Enum` declaration on a type whose fields have `#type` annotations, is a contradiction.

**Rationale:** The `###` classification must match the actual field declarations. Mismatches indicate a design error — the type's fields and its declared leaf nature disagree.

**VALID:**
```polyglot
{#} #Boolean
   [#] << ###Enum
   [.] .True
   [.] .False
```

**INVALID:**
```polyglot
{#} #BadEnum
   [#] << ###Enum                 [ ] ✗ PGE-923 — declares ###Enum
   [.] .name#string               [ ] ✗ PGE-923 — but fields have #type (value fields)
```

### Rule 9.24 — Invalid Key Type
`PGE-924`

**Statement:** `%##Children.Type` must be set to a type that inherits from `#KeyString`. Keys must exclude syntax-reserved characters (whitespace, `.`, `:`, `<`, `>`).

**Rationale:** Tree child keys appear in accessor syntax (`$var<key`). Types that permit syntax-reserved characters in their values would create parse ambiguity.

**VALID:**
```polyglot
{#} #NamedMap
   [#] << ##Flat
   [#] %##Children.Type << #KeyString
```

**INVALID:**
```polyglot
{#} #BadMap
   [#] << ##Flat
   [#] %##Children.Type << #string   [ ] ✗ PGE-924 — #string allows '.', ':', '<', '>'
```

### Rule 9.25 — Mixed Field Kinds
`PGE-925`

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
   [.] .Active                    [ ] ✗ PGE-925 — untyped enum field
   [.] .count#int                 [ ] ✗ PGE-925 — typed value field at same level
```

### Rule 9.26 — Schema Outside Type Definition
`PGE-926`

**Statement:** `##` schema references are only valid inside `{#}` type definitions. Using `##` outside a `{#}` block is a compile error.

**Rationale:** Schemas describe tree structure of data types. They have no meaning outside a type definition context.

**VALID:**
```polyglot
{#} #MyType
   [#] << ##Flat
```

**INVALID:**
```polyglot
{=} =MyPipeline
   [r] $x << ##Flat               [ ] ✗ PGE-926 — ## used outside {#}
```

### Rule 9.27 — Final Field Override via Inheritance
`PGE-927`

**Statement:** If a parent type sets a field with `<<` (final), a child type inheriting via `<~` cannot redeclare that field. Any attempt to reassign a final-inherited field is a compile error.

**Rationale:** `<<` means the value is sealed — no further pushes, including through inheritance. Without this rule, a child type could silently override a field the parent declared immutable, breaking the finality guarantee.

**VALID:**
```polyglot
{#} #String
   [.] .re#RawString <~ ".*"

[ ] ✓ .re is <~ (default) in #String — child CAN override
{#} #Int
   [#] <~ #String
   [.] .re#RawString << "^-?[0-9]+$"
```

**INVALID:**
```polyglot
{#} #Int
   [#] <~ #String
   [.] .re#RawString << "^-?[0-9]+$"

[ ] ✗ PGE-927 — .re is already << final in #Int
{#} #PositiveInt
   [#] <~ #Int
   [.] .re#RawString << "^[1-9][0-9]*$"
```

### Rule 9.21w — Redundant Schema Property
`PGW-904`

**Statement:** A `%##` or `%###` property that is already inherited from a parent type or composed schema is redundant. The compiler emits a warning.

**Rationale:** Redundant declarations add noise. The inherited value already applies. If the intent is to override, the value must differ (see PGW-905).

**WARNING:**
```polyglot
{#} #MyArray
   [#] <~ #Array<#int
   [#] %##Children.Gap << #False   [ ] ⚠ PGW-904 — already inherited from ##Contiguous via #Array
```

### Rule 9.22w — Contradicting Schema Override
`PGW-905`

**Statement:** A `%##` property that overrides an inherited value from a composed `##` schema emits a warning. The override takes effect, but the compiler flags it for verification.

**Rationale:** Overriding inherited schema properties is allowed but unusual. The warning ensures the developer intended the override rather than accidentally contradicting the schema.

**WARNING:**
```polyglot
{#} #SparseArray
   [#] <~ #Array<#int
   [#] %##Children.Gap << #True    [ ] ⚠ PGW-905 — overrides #False from ##Contiguous
```

### Rule 9.23w — Unlimited Depth on User Type
`PGW-906`

**Statement:** Setting `%##Depth.Max << -1` (unlimited depth) on a user-defined type emits a warning. Only `#Serial` should use unlimited depth.

**Rationale:** Unlimited depth is a deliberate escape hatch for schema-free data. User-defined types should have bounded depth for predictable tree traversal and memory use.

**WARNING:**
```polyglot
{#} #DeepTree
   [#] %##Depth.Max << -1         [ ] ⚠ PGW-906 — unlimited depth on user type
```

---

## Retired Codes

Codes that have been merged into other rules or removed. Listed here so searches for the old code find the redirect.

| Code | Former Rule | Name | Redirect |
|------|-------------|------|----------|
| PGE-204 | 2.4 | Default Allows Exactly One More Push | Merged into PGE-203 (Final Is Push-Once) |
| PGE-908 | 9.8 | Duplicate Data Definition | Merged into PGE-907 (Duplicate Definition) |
