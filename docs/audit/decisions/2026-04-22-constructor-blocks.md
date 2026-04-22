---
audience: ai
type: audit-reference
scope: decision-record
category: syntax
issue: "#341"
date: 2026-04-22
updated: 2026-04-22
---

# {$} Constructor Blocks

## Summary

A new `{$}` block type was introduced for compile-time type construction. Constructors guarantee a Final value with no Failed state — the compiler proves all possible values are valid. This resolves the tension between Murphy's Law error philosophy and developer ergonomics for known-valid literals.

## Before

Type construction used inline pipeline calls:

- `$path -Path"/usr/local/bin"` called a pipeline to construct a `#path` value
- Pipeline calls are failable — every call required `[!]` error handling
- No distinction between "construct a value from a known literal" and "transform dynamic data"
- Developers wrote error handling for cases that could never fail (e.g., hardcoded regex patterns)

## After

Three-context rule governs how values are constructed:

1. **Infrastructure (`[T]`/`[Q]`/`[W]`)** — inline pipeline syntax kept (`-T.Daily"3AM"`)
2. **Known values in pipeline body** — `$Constructor"literal"` with no error handling
3. **Dynamic values in pipeline body** — `-Pipeline` call with error handling required

`{$}` block syntax:

- `($)` for IO lines with regex capture params: `($) <name.re << "pattern"`
- `[$]` for action (type binding): `[$] #TargetType`
- `[.]` for field mapping: `[.] .field << <capture`
- `.re` mandatory on all capture parameters
- Overloads resolved via compiled regex at compile time

Key constraints:

- `{$}` is its own block type (not a pipeline subtype)
- User-definable; only pglib uses `[-]` pipeline calls inside `{$}`
- No auto-derivation — explicit definitions only
- Constructor-sourced interpolation only (SQL injection analogy for safety)

## Impact

- New block type `{$}` added to the block system
- 9 pglib constructors created (issue #342): `$DT`, `$Path`, `$Re`, `$MIME`, `$Dur`, `$Ver`, `$URL`, `$IP`, `$Color`
- 4 type definitions updated, 9 parse pipelines created
- PGE14xxx compile rules added for constructor errors (issue #343)
- `docs/user/syntax/constructors.md` created
- `blocks.md`, `inline-calls.md`, `glossary.md`, `SPEC-INDEX.md` updated
- `%InlineString` retired from `{-}` pipelines (kept for `{T}/{Q}/{W}` only)

## Rationale

Issue #339 identified the tension: Polyglot's error philosophy demands all errors be handled, but requiring error handling for known-valid literals (like `$path"/usr/local/bin"`) was pure ceremony.

Constructors are not pipelines — they have different semantics:

- **Pipelines** process data at runtime, may fail, require error handling
- **Constructors** prove validity at compile time, cannot fail, need no error handling

Alternatives considered:

- **Compiler special-casing** of literal pipeline calls — rejected because it would make the type system inconsistent
- **Optional error handling annotation** — rejected because it weakens Murphy's Law guarantees
- **Separate literal syntax without blocks** — rejected because it wouldn't support regex-based overloads or user extensibility

## Related

- [[decisions/2026-04-22-retire-chain-operator|Chain operator retirement]] — another syntax decision from the same period
- GitHub: #341 (design), #342 (pglib catalog), #343 (compile errors), #344 (docs transition)
