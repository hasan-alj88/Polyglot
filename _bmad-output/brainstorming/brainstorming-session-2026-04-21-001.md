---
stepsCompleted: [1, 2, 3]
inputDocuments: []
session_topic: 'Inline pipeline infallibility — compiler-verified safety vs removal'
session_goals: 'Define infallible vs failable inline pipeline calls; balance ergonomics with error philosophy; decide keep/redesign/remove'
selected_approach: 'ai-recommended'
techniques_used: ['First Principles Thinking', 'Assumption Reversal', 'Morphological Analysis']
ideas_generated: [12]
context_file: 'GitHub Issue #339'
technique_execution_complete: true
---

# Brainstorming Session Results

**Facilitator:** Hasan AlJamea
**Date:** 2026-04-21

## Session Overview

**Topic:** Inline pipeline infallibility — compiler-verified safety vs removal
**Goals:** Define which inline pipeline calls can skip error handling, whether this is a compiler optimization or language-level distinction, and how it fits within Polyglot's error philosophy

### Context Guidance

_GitHub Issue #339: Inline pipeline calls like `-DT"today"` are failable operations requiring error handling, even when failure is unrealistic for compile-time-resolvable values. The user's leading solution is compiler-verified regex matching against pipeline metadata to grant infallibility._

### Session Setup

_Design tension between Polyglot's Murphy's Law error philosophy (every failable op must be handled) and developer ergonomics for common operations like datetime and file path construction._

## Technique Selection

**Approach:** AI-Recommended Techniques
**Analysis Context:** Deep language design decision with philosophical constraints

**Recommended Techniques:**

- **First Principles Thinking:** Strip away assumptions to identify non-negotiable truths about compile-time vs runtime knowledge
- **Assumption Reversal:** Challenge core assumptions about inline pipelines, fallibility, and convenience syntax
- **Morphological Analysis:** Systematically map the full design space of string source x metadata x compiler knowledge x error requirements

## Technique Execution Results

### Phase 1: First Principles Thinking

**Key Ideas Generated:**

**[First Principles #1]**: Inline Pipelines Scrapped
_Concept:_ Inline pipelines in execution body are removed entirely. Pipelines require error handling — period. No exceptions, no "infallible pipeline" concept.
_Novelty:_ Instead of fixing inline pipelines, eliminate the category entirely.

**[First Principles #2]**: `{$}` Constructor Block
_Concept:_ New definition block type `{$}` that defines compile-time string-to-tree parsers. Constructors produce exactly one Final value with no Failed state possible. The compiler validates the argument string against the pattern at compile time.
_Novelty:_ Constructors are NOT pipelines — no trigger, no queue, no wrapper, no error surface. A fundamentally different category.

**[First Principles #3]**: Constructor Contract — Guaranteed Existence
_Concept:_ The compiler guarantees a valid Final value WILL be produced. Not what the value is — just that it will exist and will not fail. `$DT"Today"` succeeds because the compiler proves all possible runtime values are valid `#DT` trees.
_Novelty:_ Separates "will this succeed?" (compiler answers YES) from "what will the result be?" (may depend on runtime).

**[First Principles #4]**: User-Definable with Native Restriction
_Concept:_ Users can define `{$}` constructors. Only jm3lib constructors can use pipeline calls to infallible native pipelines inside `{$}`. User constructors are pure regex-to-tree mapping only.
_Novelty:_ Extensible but sandboxed.

**[First Principles #5]**: Constructor Overloading via Regex
_Concept:_ A constructor can have multiple `{$}` definitions. The compiler compiles each pattern into a full regex (substituting `.re` for each capture), then matches the input string. One mechanism handles keywords, patterns, and validation.
_Novelty:_ No separate priority rules or structural matching — regex all the way down.

**[First Principles #6]**: Inline Strings Survive on Infrastructure Lines
_Concept:_ `[T]`, `[Q]`, `[W]` lines keep inline string syntax (`-T.Daily"3AM"`, etc.) because they are pipeline infrastructure configuration, not execution body. Triggered-or-not semantics — no error handling.
_Novelty:_ Clean rule: inline strings on infrastructure, constructors in body, pipeline calls for failable ops.

**[First Principles #7]**: `.re` / `.regex` Aliases
_Concept:_ Both `.re` and `.regex` are valid for capture validation — user's choice, like `-DT` / `-DateTime`.

### Phase 2: Assumption Reversal

**[Reversal #8]**: No Runtime Strings in Constructors (SQL Injection Parallel)
_Concept:_ Constructor argument strings must be compile-time resolvable. Interpolation allowed ONLY if the source variable was produced by another constructor. Runtime/IO-sourced variables are rejected — analogous to SQL injection prevention where dynamic strings never become structure.
_Novelty:_ The safety guarantee is absolute. If you need to parse a dynamic string, use a pipeline with error handling.

**[Reversal #9]**: Structural Integrity Check on Definitions
_Concept:_ When compiling a `{$}` definition, the compiler verifies that no slot's `.re` can match the pattern's literal separators. This makes structure/data separation provable at definition time. If separator is `:` and a slot's regex could match `:`, compile error on the definition.
_Novelty:_ Safety guaranteed by construction at definition time, not checked at every call site.

**[Reversal #10]**: Compile-Time Means Exhaustive Outcomes
_Concept:_ The compiler doesn't need to know values at compile time — it needs to know all POSSIBLE values and have exhaustive logic for each. `$DT"Today"` works because the compiler proves all possible runtime dates are valid trees. No runtime "decisions" — only pre-determined paths in the Behavioral Contract.
_Novelty:_ Aligns constructors with Polyglot's core philosophy of compile-time exhaustive resolution.

**[Reversal #11]**: Constructors Stay as Own `{$}` Block (Not Embedded in `{#}`)
_Concept:_ Rejected embedding constructors inside `{#}` type definitions. `{$}` stays separate because: separation of concerns (topology vs parsing), cross-package constructor definitions, and prefix symmetry (`{#}`→`#Type`, `{-}`→`-Pipeline`, `{$}`→`$Constructor`).

**[Reversal #12]**: No Auto-Derived Constructors
_Concept:_ Every constructor requires an explicit `{$}` definition. No compiler magic inferring constructors from `{#}` definitions. Explicit is better.

### Phase 3: Morphological Analysis

**Block Syntax (finalized):**

String-parsing overload:
```
{$} $DT"{hours}:{min}:{seconds}"
   [$] <hours.re << "[0-9][0-9]"
   [$] <min.re << "[0-9][0-9]"
   [$] <seconds.re << "[0-9][0-9]"
   [$] #DT.Time
   [.] .hours << <hours
   [.] .minutes << <min
   [.] .seconds << <seconds
```

Native pipeline overload:
```
{$} $DT"Today"
   [-] -DT.Current
      (-) >hours >> $hrs
      (-) >minutes >> $min
      (-) >seconds >> $sec
   [$] #DT.Time
   [.] .hours << $hrs
   [.] .minutes << $min
   [.] .seconds << $sec
```

**Overload Resolution:** Compiler compiles each overload pattern into full regex (substituting `.re` per capture), matches input string against each. Ambiguous overlapping regexes = definition compile error.

**Metadata Tree:**
- Definitions: `%definitions.$.*` with overloads enumerated
- Instances: `%{pipeline instance}.$.*`

**jm3lib Constructor Catalog:**

| Constructor | Target Type | Key Overloads |
|---|---|---|
| `$DT` / `$DateTime` | `#DT.*` | ISO-8601, time, date, keywords (Today, Now) |
| `$Path` | `#Path` | OS-agnostic path strings |
| `$Re` | `#Re` | Regex patterns |
| `$Ver` | `#Ver` | Semantic versioning |
| `$URL` | `#URL` | URL structure |
| `$IP` | `#IP` | IP addresses |
| `$MIME` | `#MIME` | MIME types |
| `$Dur` | `#Dur` | Duration (companion to $DT) |
| `$Color` | `#Color` | Color values (hex/named) |

**Compile Errors Needed:**

| Error | Description |
|---|---|
| No overload match | String doesn't match any overload regex |
| Ambiguous overloads | Two definitions with overlapping regex match sets |
| Non-literal interpolation | Interpolated var not constructor-sourced |
| Structural integrity violation | Slot `.re` can match separator characters |
| Missing `.re` | Capture slot without regex validation |
| Failable pipeline in `{$}` | User puts failable pipeline call inside constructor |
| Target type mismatch | Field mapping doesn't match `#Type` topology |
| Duplicate keyword | Two keyword overloads with same literal |

**Package Interaction:**
- Users define `{$}` for own types and cross-package types
- Only jm3lib can use `[-]` pipeline calls inside `{$}`
- `{@}` ceiling: no interaction (constructors don't use permissions/resources)
- `[@]` import brings in `{$}` definitions alongside `{#}` and `{-}`

## Summary: Three-Context Rule

| Context | Mechanism | Syntax | Error Handling |
|---|---|---|---|
| `[T]`/`[Q]`/`[W]` Infrastructure | Inline pipeline config | `-T.Name"config"` | Not applicable |
| Pipeline body — known values | Constructor | `$Constructor"literal"` | Guaranteed no error |
| Pipeline body — dynamic values | Pipeline call | `-Pipeline` | Error handling required |
