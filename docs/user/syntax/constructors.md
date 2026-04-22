---
audience: automation-builder
type: specification
updated: 2026-04-22
---

<!-- @u:syntax/blocks -->
<!-- @u:concepts/pipelines/inline-calls -->
<!-- @c:philosophy/error-philosophy -->
<!-- @c:syntax/types/strings -->

## Constructors

Polyglot separates two fundamentally different operations that look similar on the surface: constructing a typed value from a known string, and calling a pipeline that performs async work. Both used to share inline pipeline syntax (`-Pipeline"string"`), but they have entirely different error profiles — and Polyglot's [[philosophy/error-philosophy|error philosophy]] demands that distinction be explicit.

A **constructor** is a `{$}` definition that produces a compile-time-guaranteed Final value with no error surface. When you write `$DT"2026-04-22"`, the compiler proves at compile time that every possible runtime outcome is a valid `#DT` tree. No Failed state, no `[!]` error handling, no ceremony — because the compiler has already done the work.

Inline pipeline calls (`-Pipeline"string"`) remain valid on infrastructure lines (`[T]`, `[Q]`, `[W]`) where they configure pipeline infrastructure. In the execution body, value construction uses constructors (`$Constructor"literal"`) and dynamic operations use pipeline calls (`[-] -Pipeline`) with full error handling.

### The Three-Context Rule

| Context | Mechanism | Syntax | Error Handling |
|---|---|---|---|
| `[T]`/`[Q]`/`[W]` Infrastructure | Inline pipeline config | `-T.Name"config"` | Not applicable |
| Pipeline body — known values | Constructor | `$Constructor"literal"` | Guaranteed no error |
| Pipeline body — dynamic values | Pipeline call | `[-] -Pipeline` | Error handling required |

This rule is absolute. If you know the value at compile time, use a constructor. If the value depends on runtime input, use a pipeline call with error handling. The compiler enforces this boundary — you cannot accidentally skip error handling on a genuinely failable operation, and you are not forced into error handling on a provably safe one.

## `{$}` Block Syntax

A constructor definition uses the `{$}` block type. The definition header specifies the constructor name and its string pattern:

```polyglot
{$} $Name"pattern"
```

The `$` prefix follows Polyglot's prefix symmetry: `{#}` defines `#Type`, `{-}` defines `-Pipeline`, `{$}` defines `$Constructor`.

### String-Parsing Overload

The string-parsing form defines capture slots with regex validation and maps them to fields of the target type:

```polyglot
{$} $DT"{hours}:{min}:{seconds}"
   ($) <hours.re << "[0-9][0-9]"
   ($) <min.re << "[0-9][0-9]"
   ($) <seconds.re << "[0-9][0-9]"
   [$] #DT.Time
   [.] .hours << <hours
   [.] .minutes << <min
   [.] .seconds << <seconds
```

**Element roles:**
- `($)` — IO line declaring a capture parameter. Each `($) <name.re << "regex"` declares one named capture with its regex validation pattern. The `.re` field is **mandatory** — every capture must have a regex constraint.
- `[$]` — Action line binding the target type. Exactly one `[$] #TargetType` per overload. Must appear after all `($)` lines and before `[.]` field assignments.
- `[.]` — Fixed field assignment, mapping captured values to type fields. Same syntax as in `{#}` definitions.

**Ordering:** `($)` IO lines first, then `[$]` type binding, then `[.]` field mapping. This follows the same top-down flow as pipeline definitions: IO declarations before body.

### Keyword Overload

Keywords are patterns with no capture slots — they compile to exact-match regex:

```polyglot
{$} $DT"Today"
   [$] #DT.Date
   [.] .year << %Runtime.Date.Year
   [.] .month << %Runtime.Date.Month
   [.] .day << %Runtime.Date.Day
```

A keyword overload has no `($)` lines — the entire argument string must match literally.

### Native Pipeline Overload (pglib only)

Only pglib constructors may use `[-]` pipeline calls inside `{$}`. The called pipeline must be an infallible native operation — the compiler grants this trust based on the `{N}` definition:

```polyglot
{$} $DT"Now"
   [-] -DT.Current
      (-) >hours >> $hrs
      (-) >minutes >> $min
      (-) >seconds >> $sec
   [$] #DT.Time
   [.] .hours << $hrs
   [.] .minutes << $min
   [.] .seconds << $sec
```

User-defined constructors cannot use `[-]` calls — they are limited to string-parsing with regex captures. This restriction ensures the no-error-surface guarantee holds: regex-to-tree mapping is provably safe, while pipeline calls introduce async complexity that only pglib's native operations can guarantee infallible.

## Constructor Contract

A constructor guarantees exactly one thing: **a valid Final value WILL be produced**. Specifically:

1. **Guaranteed existence** — the compiler proves that for all possible runtime values that pass the regex constraint, a valid typed tree will be produced. `$DT"Today"` succeeds because all possible runtime dates are valid `#DT` trees.

2. **No Failed state** — constructors cannot fail. There is no `[!]` error handling, no `!*-` suppression, no fallback chains. If the compiler cannot prove infallibility, the definition is rejected at compile time.

3. **Compile-time exhaustive outcomes** — the compiler does not need to know the *value* at compile time. It needs to know all *possible* values and that every one produces a valid tree. The Behavioral Contract encodes pre-determined paths for each outcome — no runtime "decisions."

4. **Final immediately** — the produced value enters Final state upon construction. No Default → Final transition, no waiting for triggers. The value is usable on the very next line.

## Overload Resolution

A constructor name (e.g., `$DT`) can have multiple `{$}` definitions — each is an **overload**. The compiler resolves which overload to use via regex matching:

1. **Pattern compilation** — for each `{$}` overload, the compiler builds a full regex by substituting each `{capture}` placeholder with the capture's `.re` pattern. Keyword overloads (no captures) compile to exact-match regex (`^Today$`).

2. **Input matching** — the caller's string argument is tested against each overload's compiled regex.

3. **Resolution rules:**
   - Exactly one match → that overload is selected
   - Zero matches → compile error (no matching constructor overload)
   - Multiple matches → compile error on the *definitions* (ambiguous overloads — detected at definition compile time, not call site)

### Structural Integrity Check

At definition compile time, the compiler verifies that no capture slot's `.re` can match the pattern's literal separators. This prevents values from breaking pattern structure — analogous to SQL injection prevention.

Example: if the pattern is `"{hours}:{minutes}"` and the separator is `:`, then `<hours.re` must not accept strings containing `:`. If it could, a single input value might span multiple capture slots, breaking the pattern's structural integrity.

This safety guarantee is **verified at definition time** — every call site inherits the guarantee automatically. One proof, used everywhere.

## Interpolation Rule

Constructor arguments may contain `{$var}` interpolation, but only under strict conditions:

- `{$var}` is allowed **if and only if** `$var` was produced by another constructor
- Runtime/IO-sourced variables → **compile error**
- Pure substitution only — no expressions, no pipeline call results

This rule is analogous to SQL prepared statements: structure is fixed at compile time, only typed parameter slots accept data. A runtime string can never become constructor input — if you need to parse a dynamic string, use a pipeline call with error handling.

```polyglot
[ ] Valid — $basePath is constructor-sourced
[-] $basePath << $Path"/reports"
[-] $fullPath << $Path"{$basePath}/daily"

[ ] Invalid — $userInput is IO-sourced (runtime)
(-) <userInput#string
[-] $parsed << $Path"{$userInput}"     ( ) compile error: non-constructor source
```

## Cross-Package Constructors

Constructors follow a **scoped extension model**: any package can define `{$}` constructors for types from other packages. Visibility is scoped by `[@]` imports — callers only see constructors from imported packages.

```polyglot
{@ } @MyUtils
   [@] @datetime

[ ] Define a constructor for #DT (from @datetime package)
{$} $DT"fiscal-{quarter}"
   ($) <quarter.re << "[1-4]"
   [$] #DT.Quarter
   [.] .quarter << <quarter
   [.] .fiscal << #True
```

This model means:
- Package owners define the canonical constructors for their types
- Users extend with domain-specific constructors for types they import
- No cross-package ambiguity because the compiler only sees constructors from `[@]`-imported packages
- If two imported packages define overlapping overloads, the compiler raises an ambiguity error at the import site

## Usage in Pipeline Body

Constructors are invoked with `[-]` assignment, using `$Constructor"literal"` as the value expression:

```polyglot
{-} -DailyReport
   [T] -T.Daily"9AM"
   (-) <input#string
   [Q] -Q.Default
   [W] -W.Polyglot

   [ ] constructor — no error handling needed
   [-] $reportDate << $DT"2026-04-21"

   [ ] constructor chain — interpolation from constructor source
   [-] $basePath << $Path"/reports"
   [-] $fullPath << $Path"{$basePath}/daily"

   [ ] pipeline call — dynamic input, error handling required
   [-] $parsed#DT << -DT.Parse
      (<) <raw#string << $input
      [!] !InvalidFormat
         [-] $parsed << $DT"Today"
```

The contrast is deliberate: `$DT"2026-04-21"` needs no error handling because the compiler proved it safe. `-DT.Parse` requires error handling because the input comes from IO — Murphy's Law applies. The `[!]` handler even uses a constructor as the recovery value.

## Metadata Tree

Constructor definitions create branches on the `%` metadata tree:

- **Definitions:** `%definition.$.*` — each constructor name gets a branch, overloads enumerated by index
- **Instances:** `%{pipeline-instance}.$.*` — constructor invocations within pipeline instances

See [[metadata]] for the full `%` tree and access patterns.

## Compile Rules

Constructor validation errors enforced at compile time. See [[COMPILE-RULES#14 — Constructors]] for full definitions.

### Definition-Time Errors

| Code | Name | What It Catches |
|------|------|-----------------|
| PGE14001 | Ambiguous Constructor Overload | Two `{$}` overloads with overlapping regex match sets |
| PGE14002 | Duplicate Constructor Keyword | Two keyword overloads with the same literal string |
| PGE14003 | Missing Capture Regex | Capture slot declared without `.re` validation |
| PGE14004 | Structural Integrity Violation | Slot `.re` can match pattern separator characters |
| PGE14005 | Target Type Mismatch | `[.]` field mapping to nonexistent field on target type |
| PGE14006 | Failable Pipeline In Constructor | User `{$}` contains `[-]` pipeline call (pglib only) |
| PGE14007 | Incomplete Field Mapping | Not all required fields of target type are mapped |

### Call-Site Errors

| Code | Name | What It Catches |
|------|------|-----------------|
| PGE14010 | No Constructor Overload Match | Input string matches no overload's compiled regex |
| PGE14011 | Non-Literal Interpolation | Interpolated variable not produced by another constructor |
| PGE14012 | Undefined Constructor | No `{$}` definition exists for the referenced name |
| PGE14013 | Interpolation Source Not Final | Interpolated constructor-sourced variable not in Final state |

## What Constructors Are NOT

- **Not pipelines** — no `[T]`, `[Q]`, `[W]`. No trigger, no queue, no wrapper, no async execution.
- **Not type definitions** — `{$}` does not define a type. It defines a *construction path* for an existing type.
- **Not auto-derived** — every constructor requires an explicit `{$}` definition. The compiler does not infer constructors from `{#}` definitions.
- **Not failable** — if the compiler cannot prove infallibility, the definition is rejected. There is no "maybe-infallible" constructor.

## See Also

- [[blocks]] — `{$}` in the block type registry
- [[concepts/pipelines/inline-calls]] — inline pipeline calls on infrastructure lines
- [[philosophy/error-philosophy]] — why error handling is mandatory for pipeline calls
- [[data-is-trees]] — how constructors produce typed trees
