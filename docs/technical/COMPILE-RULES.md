---
audience: developer
type: specification
updated: 2026-03-18
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
| PGE-1001 | 10.1 | Undefined Metadata Field Access |

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

## Retired Codes

Codes that have been merged into other rules or removed. Listed here so searches for the old code find the redirect.

| Code | Former Rule | Name | Redirect |
|------|-------------|------|----------|
| PGE-204 | 2.4 | Default Allows Exactly One More Push | Merged into PGE-203 (Final Is Push-Once) |
| PGE-908 | 9.8 | Duplicate Data Definition | Merged into PGE-907 (Duplicate Definition) |
