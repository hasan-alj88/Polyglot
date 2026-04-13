---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# {_} Permission Objects

<!-- @c:identifiers -->
<!-- @c:blocks -->

Permissions are declared as named `{_}` blocks — first-class, reusable permission objects. Each `{_}` block defines a permission policy with a name, intent, and one or more capability grants.

```polyglot
{_} _DataCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "data/*.csv"
   [.] .Database.Read "*.postgres"

{_} _ReportReader
   [.] .intent << #Grant
   [.] .File.Read "data/reports/q1.csv"
   [.] .File.Read "data/reports/q2.csv"
```

## Intent: Ceiling vs Grant

Every `{_}` object declares an `.intent` field — either `#Ceiling` or `#Grant`:

| Intent | Purpose | Scope values |
|--------|---------|--------------|
| `#Ceiling` | Maximum permissions a package **allows** | Glob patterns permitted (`"data/*.csv"`, `"/var/log/*"`) |
| `#Grant` | Specific permissions a pipeline **requests** | Narrow, specific values only (`"data/reports/q1.csv"`) |

- **Ceiling** — referenced by `{@}` packages via `[_]`. Sets the upper bound. Allows but does not grant.
- **Grant** — referenced by `{-}` pipelines via `[_]`. Requests specific capabilities within the ceiling.
- **Compiler validates: Grant must be a subset of Ceiling** — every grant value must fall within a ceiling pattern. A grant requesting `"data/secret.csv"` when the ceiling only allows `"data/reports/*"` is a compile error (PGE10001).
- **Narrowing allowed, expanding NOT** — a grant can request less than the ceiling allows, but never more.

## Fully Filled Requirement

Every `{_}` object must be **fully filled** — every leaf field must have a value assigned. Empty leaves are a compile error. This prevents accidental "allow everything" policies from incomplete declarations.

## No Inline Declarations

`[_]` in `{@}` and `{-}` always references a `{_}` object **by name** — no inline permission declarations. All permission policies are defined as standalone `{_}` blocks.

```polyglot
{ } VALID — reference by name
[_] _DataCeiling

{ } INVALID — no inline declarations
[_] _File.read"/var/log/*"
```
