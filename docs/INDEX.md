---
audience: [pg-coder, integrator, architect, designer]
type: master-index
scope: all-documentation
updated: 2026-04-03
---

# Polyglot Documentation — Master Index

<!-- @vision -->
<!-- @audit/README -->
<!-- @glossary -->
This is the single entry point for all Polyglot documentation. Authority chain: [[vision]] > [[audit/README]] > `.paul/`. Terms follow [[reference/glossary]].

## Quick Navigation

| I need to... | Go to |
|-------------|-------|
| Learn the language from scratch | [[user/SPEC-INDEX]] |
| Look up a pglib pipeline/expander/collector | [[user/pglib/INDEX]] |
| Find a compile error code (PGE/PGW) | [[technical/COMPILE-RULES]] → [[technical/compile-rules/PGE/]] |
| Check formal grammar for a construct | [[technical/ebnf/INDEX]] |
| Validate an edge case | [[technical/edge-cases/INDEX]] |
| Understand the type system | [[user/syntax/types/INDEX]] |
| Find a real-world automation scenario | [[user/scenarios/INDEX]] |
| Check documentation writing rules | [[audit/README]] |
| See the product vision | [[vision]] |
| Track documentation gaps | [[audit/tracking/coverage-gaps]] |

## By Polyglot Object

| Object | Prefix | User Concept | pglib Reference | EBNF Grammar | Edge Cases |
|--------|--------|-------------|-----------------|-------------|------------|
| Pipeline | `=` | [[user/concepts/pipelines/INDEX]] | [[user/pglib/pipelines/]] | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |
| Queue | `{Q}` | [[user/concepts/pipelines/queue]] | [[user/pglib/pipelines/Q]] | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |
| Trigger | `{T}` | [[user/concepts/pipelines/io-triggers]] | [[user/pglib/pipelines/T]] | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |
| Wrapper | `{W}` | [[user/concepts/pipelines/wrappers]] | [[user/pglib/pipelines/W]] | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |
| Expander | `=` | [[user/concepts/collections/expand]] | [[user/pglib/expanders/]] | [[technical/ebnf/12-collections]] | [[technical/edge-cases/12-collections]] |
| Collector | `*` | [[user/concepts/collections/collect]] | [[user/pglib/collectors/]] | [[technical/ebnf/12-collections]] | [[technical/edge-cases/12-collections]] |
| Data / Type | `#` | [[user/syntax/types/INDEX]] | [[user/pglib/types/]] | [[technical/ebnf/04-type-system]] | [[technical/edge-cases/04-type-system]] |
| Metadata | `%` | [[user/concepts/metadata]] | — | [[technical/ebnf/05-block-elements]] | [[technical/edge-cases/15-metadata-blocks]] |
| Macro | `{M}` | [[user/syntax/types/macro-types]] | — | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/18-macro-structure]] |
| Error | `!` | [[user/concepts/errors]] | [[user/pglib/errors/]] | [[technical/ebnf/11-control-flow]] | [[technical/edge-cases/11-control-flow]] |
| Permission | `{_}` | [[user/concepts/permissions]] | — | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |
| Native | `{N}` | [[user/concepts/pipelines/INDEX]] | — | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |
| Package | `@` | [[user/syntax/packages]] | — | [[technical/ebnf/09-definition-blocks]] | [[technical/edge-cases/09-definition-blocks]] |

## By Audience

### For Users (learning Polyglot Code)

| Path | Content |
|------|---------|
| [[user/SPEC-INDEX]] | 5-phase learning progression (16 files → practice) |
| [[user/syntax/]] | Syntax foundations: line structure, comments, identifiers, blocks, types, operators, IO, packages |
| [[user/concepts/]] | Core concepts: variable lifecycle, collections, conditionals, pipelines, errors, permissions, metadata |
| [[user/pglib/]] | Standard library: pipelines, expanders, collectors, types, errors |
| [[user/scenarios/INDEX]] | 500 real-world automation scenarios (split into 6 thematic files) |

### For Designers (language syntax & semantics)

| Path | Content |
|------|---------|
| [[technical/ebnf/INDEX]] | Formal EBNF grammar (15 sections) |
| [[technical/edge-cases/INDEX]] | 42+ edge cases organized by grammar section |
| [[technical/COMPILE-RULES]] | Error/warning code lookup (PGE-xxx, PGW-xxx) |
| [[technical/compile-rules/PGE/]] | Individual error rule files (108 rules) |
| [[technical/compile-rules/PGW/]] | Individual warning rule files (21 rules) |
| [[technical/spec/type-identity]] | Structural type matching rules |
| [[technical/compile-rules/algorithms/]] | Algorithms: cycle detection, overlap detection, compound exhaustiveness |

### For Architects (runtime & service architecture)

| Path | Content |
|------|---------|
| [[technical/spec/metadata-tree/INDEX]] | Formal `%` metadata tree specification |

### For all Contributors

| Path | Content |
|------|---------|
| [[technical/INDEX]] | Complete technical documentation index (all areas) |

### For Documentation Authors

| Path | Content |
|------|---------|
| [[audit/README]] | Documentation ground truth — read BEFORE writing any doc |
| [[audit/rules/conventions]] | Writing style, structure, formatting rules |
| [[audit/rules/checklist]] | Pre-publish quality checks |
| [[audit/audiences/user]] | Tone for user-facing docs |
| [[audit/audiences/developer]] | Tone for developer docs |
| [[audit/audiences/ai]] | Tone for AI-facing docs |
| [[audit/reference/glossary]] | Authoritative term definitions |
| [[audit/tracking/coverage-gaps]] | Active inconsistencies and coverage gaps |

## File Registry

### docs/

| File | Audience | Type | Description |
|------|----------|------|-------------|
| vision.md | all | vision | Product vision and philosophy (highest authority) |
| INDEX.md | all | master-index | This file — documentation entry point |

### docs/audit/

| File | Audience | Type | Description |
|------|----------|------|-------------|
| README.md | ai | audit-index | Ground truth index for documentation authors |
| rules/conventions.md | ai | audit-rules | Writing style and structure rules |
| rules/checklist.md | ai | audit-rules | Pre-publish quality verification |
| audiences/user.md | ai | audit-rules | User audience tone guide |
| audiences/developer.md | ai | audit-rules | Developer audience tone guide |
| audiences/ai.md | ai | audit-rules | AI audience tone guide |
| reference/glossary.md | all | reference | Authoritative term definitions |
| tracking/coverage-gaps.md | ai | audit-tracking | Inconsistency and gap tracker |

### docs/user/syntax/

| File | Audience | Type | Description |
|------|----------|------|-------------|
| line-structure.md | user | spec | 3-space indentation, one expression per line |
| comments.md | user | spec | Single-line and multi-line comment syntax |
| identifiers.md | user | spec | Prefixes (@#=$!%), fixed/flexible separators |
| blocks.md | user | spec | {X} definitions, [X] block elements |
| types/INDEX.md | user | spec | Type system overview and ground truths |
| types/prefix-system.md | user | spec | Three-tier prefix system, < operator |
| types/basic-types.md | user | spec | RawString, #String, int, float |
| types/schema-properties.md | user | spec | ## schema properties |
| types/macro-types.md | user | spec | {M} macro-generated types |
| types/arrays.md | user | spec | Element-typed and multidimensional arrays |
| types/structs.md | user | spec | Struct types, inline data, enum vs value fields |
| types/flexible-fields.md | user | spec | Typed flexible fields |
| types/strings.md | user | spec | String interpolation, path type |
| types/conversions.md | user | spec | Type conversions |
| types/hierarchy.md | user | spec | Namespaced types, hierarchy summary, live modifier |
| operators.md | user | spec | Assignment, comparison, negation, range, arithmetic |
| io.md | user | spec | Input/output parameters, IO line patterns |
| packages.md | user | spec | Package declaration, address format, imports |

### docs/user/concepts/

| File | Audience | Type | Description |
|------|----------|------|-------------|
| variable-lifecycle.md | user | spec | Declared → Default → Final → Failed → Released |
| conditionals.md | user | spec | [?] chains, exhaustiveness, logical operators |
| errors.md | user | spec | Error model, scoping, chain addressing, recovery |
| permissions.md | user | spec | Permission model |
| data-is-trees.md | user | spec | Everything is a tree — unified % connection |
| metadata.md | user | spec | Full % tree field listings, access patterns |
| pipelines/INDEX.md | user | spec | Pipeline mandatory structure overview |
| pipelines/metadata.md | user | spec | Pipeline metadata and error trees |
| pipelines/error-handling.md | user | spec | Pipeline error handling |
| pipelines/io-triggers.md | user | spec | IO as implicit triggers, trigger section |
| pipelines/permissions.md | user | spec | Pipeline permissions |
| pipelines/queue.md | user | spec | Queue configuration |
| pipelines/wrappers.md | user | spec | Wrapper structure |
| pipelines/execution.md | user | spec | Execution body |
| pipelines/chains.md | user | spec | Chain execution |
| pipelines/inline-calls.md | user | spec | Inline calls, call site rules, compile rules |
| collections/INDEX.md | user | spec | Collection hierarchy and types summary |
| collections/map.md | user | spec | #Map base collection |
| collections/array.md | user | spec | #Array map variant |
| collections/serial.md | user | spec | #Serial unconstrained tree |
| collections/dataframe.md | user | spec | #Dataframe and nested safety |
| collections/user-struct.md | user | spec | User-defined struct as collection |
| collections/expand.md | user | spec | = expand operators |
| collections/collect.md | user | spec | * collect operators and collect-all/race |
| collections/examples.md | user | spec | Expand/transform/collect examples |

### docs/user/pglib/

| File | Audience | Type | Description |
|------|----------|------|-------------|
| INDEX.md | user | reference | Namespace registry for all pglib |
| pipelines/*.md | user | reference | -File, -Path, -Sys, -T, -Q, -Math, -W, -# |
| expanders/ForEach/*.md | user | reference | =ForEach variants (Array, Map, Serial, Level, Dataframe) |
| collectors/*.md | user | reference | *Into variants, *Agg, *Sync |
| types/*.md | user | reference | #String, scalars, boolean, collections, enums, structs |
| errors/errors.md | user | reference | Error namespaces |

### docs/technical/

| File | Audience | Type | Description |
|------|----------|------|-------------|
| INDEX.md | developer | spec-index | Technical documentation index |
| COMPILE-RULES.md | developer | reference | Error/warning code lookup tables |
| ebnf/INDEX.md | developer | spec | EBNF notation conventions and section links |
| ebnf/01–15-*.md | developer | spec | Formal grammar by section |
| edge-cases/INDEX.md | developer | reference | Edge case usage guide and coverage matrix |
| edge-cases/01–24-*.md | developer | reference | Edge cases by grammar section |
| spec/metadata-tree/ | [architect, designer] | spec | Complete % metadata tree specification (split) |
| spec/type-identity.md | developer | spec | Structural type matching rules |
| compile-rules/PGE/*.md | developer | spec | Individual error rules (108 files) |
| compile-rules/PGW/*.md | developer | spec | Individual warning rules (21 files) |
| compile-rules/algorithms/*.md | developer | spec | Compiler algorithms |
