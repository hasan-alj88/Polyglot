---
issue: 153
group: 2
group_name: "Syntax Documentation Gaps"
priority: P2-high
status: brief-ready
---

# Issue #153: =#.Column creates undocumented prefix collision (= + #)

## Inconsistency
The `=#` namespace (used by `=#.Column`, `=#.Match`, `=#.Validate`, etc.) combines the `=` pipeline prefix with the `#` struct prefix into a compound `=#` that is not defined anywhere in the identifier or prefix system docs. The identifiers.md prefix table lists `=` for pipelines and `#` for structs as separate prefixes, with no mention of compound forms. The aj3lib file `docs/user/aj3lib/pipelines/#.md` documents all nine `=#.*` pipelines, and `docs/user/concepts/pipelines/INDEX.md` references `=#.JSON.Parse` and `=#.Validate` as NativeKind.Intrinsic pipelines. However, there is no formal definition of `=#` as a valid namespace prefix, no EBNF production for it, and no explanation of why schema validation pipelines use this compound form instead of a regular `=Schema.*` or `=S.*` namespace.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/user/syntax/identifiers.md` | Prefix table lists `=` and `#` separately; no mention of `=#` compound prefix |
| `docs/user/aj3lib/pipelines/#.md` | Defines 9 `=#.*` pipelines as aj3lib; compound prefix used throughout without formal justification |
| `docs/user/concepts/pipelines/INDEX.md` | References `=#.JSON.Parse`, `=#.Validate` as NativeKind.Intrinsic without noting the unusual prefix |
| `docs/technical/ebnf/09-definition-blocks.md` | `pipeline_id` production does not account for `=#` as a valid pipeline identifier prefix |

## Example
**Source A** (`docs/user/syntax/identifiers.md`, line ~19):
> | `=` | Pipelines | `=ProcessData`, `=Pipeline.Name` |

**Source B** (`docs/user/aj3lib/pipelines/#.md`, line ~1-9):
> # =# -- Schema Validation & Field Extraction
>
> aj3lib pipelines for schema validation, field extraction, format parsing, and dataframe column extraction. No `[@]` import needed.

**Source C** (`docs/user/concepts/pipelines/INDEX.md`, line ~128):
> | `.Intrinsic` | Compiler-internal operations | `=#.JSON.Parse`, `=DT.Now`, `=#.Validate` |

## Prior Related Work
- Issue #94 -- `=#.Column` replaces `~ForEach.Dataframe.Column` (closed 2026-03-30). STATE.md records: "=#.Column pipeline for column extraction -- Row-oriented Dataframe loses $df.column accessor; =#.Column replaces ~ForEach.Dataframe.Column". The compound prefix was introduced here without formal prefix system documentation.
- Issue #110 -- Document base pipelines and #BaseCode enum (closed 2026-03-31). Established NativeKind categories but did not address the `=#` prefix anomaly.

## Recommendation
Document `=#` as a valid compound namespace for pipelines that operate on type definitions (schema introspection). The `=#` form signals "pipeline that takes `#` type metadata as primary input" -- it is a semantic namespace, not a prefix collision. Add a "Compound Namespaces" subsection to identifiers.md (or to the aj3lib INDEX.md) that explains the naming convention: `=#.*` pipelines accept `<#type` inputs and perform schema-level operations. Alternatively, if the team prefers to avoid compound prefixes, rename to `=Schema.*` and update all references. Either way, the EBNF `pipeline_id` production needs to explicitly permit or exclude the `=#` form.

## Discussion Prompts
1. Is `=#` an intentional semantic convention (pipeline + type = schema operation) or an accidental naming choice that should be renamed to `=Schema.*`?
2. Should the EBNF grammar explicitly define `schema_pipeline_id ::= '=' '#' '.' dotted_name` as a production, or treat `=#` as just a regular dotted pipeline name where the first segment happens to be `#`?
3. Are there other compound prefix forms that might emerge (e.g., `=!` for error-handling pipelines), and should the identifier system preemptively define rules for compound prefixes?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 153*
