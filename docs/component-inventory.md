---
audience: ai-finder
type: reference
updated: 2026-04-23
---

# Component Inventory

<!-- @c:audit/README -->
<!-- @c:audit/reference/glossary -->
<!-- @u:INDEX -->
<!-- @u:ai-retrieval-index -->
This document is the flat, categorical inventory of every Aljam3 component. It answers "what exists, by category" — block types, aj3lib namespaces, types, error namespaces, compile-rule groups, EBNF sections, philosophy pages, audit rules. Every row points at the authoritative file or sub-index; this inventory does not duplicate enumeration. For query-shaped lookup use [[ai-retrieval-index|u:ai-retrieval-index]]; for repo layout use [[source-tree-analysis|u:source-tree-analysis]]. All terminology matches [[audit/reference/glossary|c:glossary]] exactly.

## How to Use This Inventory

| Goal | Section |
|------|---------|
| List every definition-block type | Block Types |
| Find a aj3lib pipeline namespace | aj3lib Pipelines |
| Find an expander / collector | aj3lib Expanders / Collectors |
| List types and schemas | aj3lib Types |
| List error namespaces | Error Namespaces |
| Find a compile rule by range | Compile Rule Groups |
| List EBNF grammar sections | EBNF Sections |
| List philosophy pages | Philosophy Pages |
| List audit rules | Audit Rules |
| Find retired or renamed components | Retired Components |

## Block Types

Definition blocks use the `{X}` family. Block elements (inside definition bodies) use `[X]`. IO markers use `(X)`. See [[user/syntax/blocks|u:blocks]] for the three-bracket system and [[technical/ebnf/09-definition-blocks|u:ebnf-09]] for the formal grammar.

| Block | Purpose | User doc | EBNF |
|-------|---------|----------|------|
| `{@}` | Package declaration (first block in file) | [[user/syntax/packages\|u:packages]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{#}` | Type / schema / alias definition | [[user/syntax/types/INDEX\|u:types-index]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| `{-}` | Pipeline definition | [[user/concepts/pipelines/INDEX\|u:pipelines-index]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{=}` | Expander definition | [[user/concepts/collections/expand\|u:expand]] | [[technical/ebnf/12-collections\|u:ebnf-12]] |
| `{*}` | Collector definition | [[user/concepts/collections/collect\|u:collect]] | [[technical/ebnf/16-collector-definitions\|u:ebnf-16]] |
| `{W}` | Wrapper definition | [[user/concepts/pipelines/wrappers\|u:wrappers]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{T}` | Trigger definition | [[user/concepts/pipelines/io-triggers\|u:io-triggers]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{Q}` | Queue definition | [[user/concepts/pipelines/queue\|u:queue]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{!}` | Error namespace definition | [[user/concepts/errors\|u:errors]] | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| `{_}` | Permission object definition | [[user/concepts/permissions\|u:permissions]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{$}` | Constructor definition | [[user/syntax/constructors\|u:constructors]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{;}` | Environment definition | [[user/syntax/environments\|u:environments]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| `{N}` | Native-code block | [[user/aj3lib/types/NativeKind\|u:NativeKind]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |

### Block Elements (inside definitions)

| Element | Purpose | EBNF |
|---------|---------|------|
| `[T]` | Trigger section | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[Q]` | Queue section | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[W]` | Wrapper section | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[\]` | Setup section | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[/]` | Cleanup section | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[-]` | Pipeline call / execution line | [[technical/ebnf/10-execution\|u:ebnf-10]] |
| `[?]` | Conditional | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| `[!]` | Error handler | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| `[C]` | Foreign-code block | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[_]` | Permission IO marker (retired — now via IO markers) | [[audit/decisions/README\|c:decisions-index]] |
| `[|]` | OR marker (conditional arms) | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| `[+]` | Line continuation | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[#]` | Data-load section | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| `[b]` | Parallel work without output | [[technical/ebnf/10-execution\|u:ebnf-10]] |
| `[p]` | Parallel fork | [[technical/ebnf/10-execution\|u:ebnf-10]] |
| `[=]` | Collector invocation | [[technical/ebnf/12-collections\|u:ebnf-12]] |
| `[~]` | IO expansion marker | [[technical/ebnf/12-collections\|u:ebnf-12]] |
| `[*]` | Collect marker | [[technical/ebnf/12-collections\|u:ebnf-12]] |

### IO Markers

| Marker | Purpose | EBNF |
|--------|---------|------|
| `(-)` | IO declaration / operation label | [[technical/ebnf/07-io-parameters\|u:ebnf-07]] |
| `(<)` | Input fallback / named error fallback | [[technical/ebnf/07-io-parameters\|u:ebnf-07]] |
| `(>)` | Output fallback | [[technical/ebnf/07-io-parameters\|u:ebnf-07]] |
| `(!)` | Error-raise info line | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| `(*)` | Collector IO line | [[technical/ebnf/12-collections\|u:ebnf-12]] |
| `(#)` | Generic-input marker | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| `($)` | Operation-label output addressing | [[technical/ebnf/10-execution\|u:ebnf-10]] |

## aj3lib Pipelines

Pipeline namespace → sub-index. Each namespace sub-index enumerates its pipelines and errors.

| Namespace | Count | Sub-index |
|-----------|-------|-----------|
| `-File.*` | 12 | [[user/aj3lib/pipelines/File/INDEX\|u:aj3lib-File-index]] |
| `-T.*` | 11 | [[user/aj3lib/pipelines/T/INDEX\|u:aj3lib-T-index]] |
| `-Q.*` | 27 | [[user/aj3lib/pipelines/Q/INDEX\|u:aj3lib-Q-index]] |
| `-W.*` | 14 | [[user/aj3lib/pipelines/W/INDEX\|u:aj3lib-W-index]] |
| `-Math.*` | 9 | [[user/aj3lib/pipelines/Math/INDEX\|u:aj3lib-Math-index]] |
| `-DT.*` | 42 | [[user/aj3lib/pipelines/DT/INDEX\|u:aj3lib-DT-index]] |
| `-RT.*` | 8 | [[user/aj3lib/pipelines/RT/INDEX\|u:aj3lib-RT-index]] |
| `-Run.*` | 8 | [[user/aj3lib/pipelines/Run/INDEX\|u:aj3lib-Run-index]] |
| `-Schema.*` | 10 | [[user/aj3lib/pipelines/Schema/INDEX\|u:aj3lib-Schema-index]] |
| `-Variable.*` | 2 | [[user/aj3lib/pipelines/Variable/INDEX\|u:aj3lib-Variable-index]] |
| `-Text.*` | 1 | [[user/aj3lib/pipelines/Text\|u:aj3lib-Text]] |
| `-Path` | 1 (single file) | [[user/aj3lib/pipelines/Path\|u:aj3lib-Path]] |
| `-Sys` | 1 (single file) | [[user/aj3lib/pipelines/Sys\|u:aj3lib-Sys]] |

### Standalone Parser Pipelines

| Pipeline | File |
|----------|------|
| `-Color.Parse` | [[user/aj3lib/pipelines/Color.Parse\|u:Color-Parse]] |
| `-Dur.Parse` | [[user/aj3lib/pipelines/Dur.Parse\|u:Dur-Parse]] |
| `-IP.Parse` | [[user/aj3lib/pipelines/IP.Parse\|u:IP-Parse]] |
| `-MIME.Parse` | [[user/aj3lib/pipelines/MIME.Parse\|u:MIME-Parse]] |
| `-Path.Parse` | [[user/aj3lib/pipelines/Path.Parse\|u:Path-Parse]] |
| `-Re.Parse` | [[user/aj3lib/pipelines/Re.Parse\|u:Re-Parse]] |
| `-URL.Parse` | [[user/aj3lib/pipelines/URL.Parse\|u:URL-Parse]] |
| `-Ver.Parse` | [[user/aj3lib/pipelines/Ver.Parse\|u:Ver-Parse]] |

## aj3lib Expanders

Expanders use the `=ForEach.*` namespace. Every expander fans out a collection into per-element jobs; termination is collector-driven.

| Namespace | Count | Sub-index |
|-----------|-------|-----------|
| `=ForEach.*` | 10 | [[user/aj3lib/expanders/ForEach/INDEX\|u:ForEach-index]] |

| Variant | Expands | File |
|---------|---------|------|
| `=ForEach.Array` | `#Array` | inside `ForEach/` |
| `=ForEach.Map` | `#Map` | inside `ForEach/` |
| `=ForEach.Serial` | `#Serial` | inside `ForEach/` |
| `=ForEach.Level` | Level iteration | inside `ForEach/` |
| `=ForEach.Dataframe` | `#Dataframe` columns | inside `ForEach/` |

The top-level expanders index also includes `=#.Column` (column extraction) and related column-oriented expanders documented in the Dataframe-collection area.

## aj3lib Collectors

Collectors use three families: aggregate (`*Agg.*`), into-collection (`*Into.*`), and sync/race (`*Sync.*`).

| Family | Namespace | Count | Sub-index |
|--------|-----------|-------|-----------|
| Aggregate | `*Agg.*` | 7 | [[user/aj3lib/collectors/Agg/INDEX\|u:Agg-index]] |
| Into-collection | `*Into.*` | 10 | [[user/aj3lib/collectors/Into/INDEX\|u:Into-index]] |
| Sync / race | `*Sync.*` (with `*All`, `*First`, `*Nth`) | 5 | [[user/aj3lib/collectors/Sync/INDEX\|u:Sync-index]] |

### Reassemblers

Reassemblers are a separate operator family that rebuilds a collection shape from expanded elements. Listed under `docs/user/aj3lib/reassemblers/`.

| Category | Dir |
|----------|-----|
| Aggregate reassemblers | [[user/aj3lib/reassemblers/Agg/INDEX\|u:reassemble-Agg]] (if index present) |
| Into reassemblers | [[user/aj3lib/reassemblers/Into/INDEX\|u:reassemble-Into]] (if index present) |

## aj3lib Types

Types are partitioned into five subdirectories plus 72 top-level files. Each subdirectory has its own `INDEX.md` (or umbrella file).

### Top-Level aj3lib Type Categories

| Category | Key files |
|----------|-----------|
| Basic | `string.md`, `path.md`, `boolean.md` |
| Collections | `Array.md`, `Map.md`, `Serial.md`, `Dataframe.md`, `Set.md` |
| Runtime | `PyEnv.md`, `RsEnv.md`, `rt.md`, `Code.md` |
| Service | `Job.md`, `JobStatus.md`, `Queue.md`, `QueueState.md`, `Variable.md`, `VarState.md` |
| Git | `git.md` |
| Parsed entities | `Color.md`, `IP.md`, `URL.md`, `MIME.md`, `Ver.md`, `Re.md` |
| Capability enums | `CPUCapability.md`, `GPUCapability.md`, `RAMCapability.md`, `IOCapability.md`, `ProcessCapability.md`, `DurationCapability.md` |
| Error-domain enums | `AccessLevel.md`, `AuditLevel.md`, `AlertLevel.md` |
| Field / Collect shape | `FieldKind.md`, `IncomingDataFrame.md`, `ActiveKind.md`, `HandleKind.md`, `NativeKind.md`, `NativeType.md`, `CollectorScope.md`, `CollectorCategory.md`, `CollectOrder.md` |
| Permissions / Grants | `PermissionCategory.md`, `PermissionIntent.md`, `GrantAuthority.md`, `LimitAction.md`, `LimitConfig.md`, `KillPropagation.md`, `FileAccess.md`, `IODirection.md`, `OverflowStrategy.md`, `RetriggerStrategy.md`, `Protocol.md`, `RotationKind.md`, `ResourceTag.md`, `QueueStrategy.md`, `PipelineStatus.md`, `Bound.md` |
| Merge / diff | `TextDiff.md`, `DiffOp.md`, `DiffStats.md`, `MergeConflict.md`, `MergeResult.md`, `MergeStrategy.md` |
| OS | `OS.md`, `OSTarget.md` |
| Umbrella aggregators | `types.md`, `scalars.md`, `structs.md`, `enums.md`, `collections.md` |

### aj3lib Type Subfolders

| Subfolder | Count | Purpose | INDEX |
|-----------|-------|---------|-------|
| `scalars/` | 11 | Scalar subtype tree | — (umbrella: `scalars.md`) |
| `schemas/` | 20 | `##` schemas (Record, Leaf, Inf, Nullable, etc.) | — (umbrella: `schemas.md` if present) |
| `properties/` | 16 | `##` schema properties | — |
| `field-types/` | 6 | Field type definitions | — |
| `datetime/` | 10 | `#DateTime` subtype tree | — |

## Error Namespaces

User-facing error-namespace catalog lives under `docs/user/aj3lib/errors/`.

| File | Purpose |
|------|---------|
| [[user/aj3lib/errors/errors\|u:errors-aj3lib]] | Error-namespace overview |
| [[user/aj3lib/errors/error-struct\|u:error-struct]] | `!Error` struct |
| [[user/aj3lib/errors/custom-errors\|u:custom-errors]] | User-defined `{!}` errors |
| [[user/aj3lib/errors/alias-clash\|u:alias-clash]] | Alias-clash handling |
| [[user/aj3lib/errors/pipeline-associations\|u:pipeline-associations]] | Pipeline → namespace binding |

Built-in namespaces (`docs/user/aj3lib/errors/builtin/`):

| Namespace | Raised by |
|-----------|-----------|
| `!Permission.*` | `{_}` enforcement, foreign-code compliance |
| `!Validation.*` | `-Schema.*` validation pipelines |
| `!Field.*` | Field-level validation |
| `!File.*` | `-File.*` pipelines |
| `!RT.*` | `-RT.*` runtime pipelines |
| `!Env.*` | `{;}` environments, `-W.Env` |
| `!Queue.*` | `-Q.*` queue control |

## Permission Catalog

Permission categories live under `docs/user/aj3lib/permissions/`.

| Category | Dir | Purpose |
|----------|-----|---------|
| Crypto | `permissions/Crypto/` | Cryptographic primitives |
| Database | `permissions/Database/` | Database access |
| Device | `permissions/Device/` | Device access |
| File | `permissions/File/` | File-system access |
| IPC | `permissions/IPC/` | Inter-process communication |
| Memory | `permissions/Memory/` | Memory |
| System | `permissions/System/` | System / shell |
| Web | `permissions/Web/` | Network / HTTP |

Concept-level permission sub-pages under `docs/user/concepts/permissions/`:

| Topic | File |
|-------|------|
| Permission objects | [[user/concepts/permissions/permission-objects\|u:permission-objects]] |
| Permission prefixes | [[user/concepts/permissions/permission-prefixes\|u:permission-prefixes]] |
| Permission schema | [[user/concepts/permissions/permission-schema\|u:permission-schema]] |
| Implicit-deny model | [[user/concepts/permissions/implicit-deny\|u:implicit-deny]] |
| Hierarchical scoping | [[user/concepts/permissions/hierarchical-scoping\|u:hierarchical-scoping]] |
| Capability enums | [[user/concepts/permissions/capability-enums\|u:capability-enums]] |
| Foreign-code compliance | [[user/concepts/permissions/foreign-code\|u:foreign-code]] |
| Enforcement | [[user/concepts/permissions/enforcement\|u:enforcement]] |

## Constructor Catalog

`{$}` constructors live under `docs/user/aj3lib/constructors/` and are paired with the corresponding type. Parser pipelines live under `docs/user/aj3lib/pipelines/*.Parse.md`.

| Constructor | Type file | Parser pipeline |
|-------------|-----------|------------------|
| `$Color` | [[user/aj3lib/types/Color\|u:Color]] | [[user/aj3lib/pipelines/Color.Parse\|u:Color-Parse]] |
| `$DT` | [[user/aj3lib/types/datetime/INDEX\|u:datetime-index]] | `-DT.Parse` (within DT/) |
| `$Dur` | — | [[user/aj3lib/pipelines/Dur.Parse\|u:Dur-Parse]] |
| `$IP` | [[user/aj3lib/types/IP\|u:IP]] | [[user/aj3lib/pipelines/IP.Parse\|u:IP-Parse]] |
| `$MIME` | [[user/aj3lib/types/MIME\|u:MIME]] | [[user/aj3lib/pipelines/MIME.Parse\|u:MIME-Parse]] |
| `$Path` | [[user/aj3lib/types/path\|u:path]] | [[user/aj3lib/pipelines/Path.Parse\|u:Path-Parse]] |
| `$Re` | [[user/aj3lib/types/Re\|u:Re]] | [[user/aj3lib/pipelines/Re.Parse\|u:Re-Parse]] |
| `$URL` | [[user/aj3lib/types/URL\|u:URL]] | [[user/aj3lib/pipelines/URL.Parse\|u:URL-Parse]] |
| `$Ver` | [[user/aj3lib/types/Ver\|u:Ver]] | [[user/aj3lib/pipelines/Ver.Parse\|u:Ver-Parse]] |

Constructor block design is recorded in [[audit/decisions/2026-04-22-constructor-blocks|c:decision-constructor-blocks]].

## Compile Rule Groups

Master catalog: [[technical/COMPILE-RULES|u:COMPILE-RULES]]. Per-rule files: `docs/technical/compile-rules/PGE/` (188 files) and `docs/technical/compile-rules/PGW/` (31 files).

| Range | Topic | Example rules |
|-------|-------|---------------|
| PGE01xxx | Pipeline structure, block elements, triggers, queues, wrappers | PGE01001 (execution order), PGE01002 (IO before trigger), PGE01005 (missing trigger), PGE01006 (missing queue), PGE01007 (missing setup/cleanup), PGE01028 (native baseCode), PGE01040 (orphan parallel) |
| PGE02xxx | Variable lifecycle, Final/Failed transitions | PGE02003 (write after Final), PGE02010 (discard default), PGE02011 (lifecycle misuse) |
| PGE03xxx | Collectors, reassembly, job termination | PGE03011 (collector misuse), PGE03025 (all-jobs-released paired scope) |
| PGE04xxx | Type system, schemas, arrays, coercion | PGE04010 (raw arithmetic), PGE04022/04023 (dataframe coercion), PGE04025 (untyped array), PGE04026-28 (DateTime) |
| PGE05xxx | Metadata paths, separator rules | PGE05001 (separator homogeneity), PGE05005 (enum rules) |
| PGE06xxx | Conditional coverage | PGE06014 (conditional coverage) |
| PGE07xxx | Error handling | PGE07008 (non-failable fallback), PGE07009 (unterminated chain) |
| PGE08xxx | Chains, references | PGE08001/02/03 (wildcard auto-wire), PGE08011 (self-assignment), PGE08012 (self-chain indexing), PGE08013 (inline-value recursion) |
| PGE10xxx | Permissions, foreign-code compliance | PGE10001/03/05/06, PGE10008 (parallel-write exclusion), PGE10009/10 (permission-as-resource), PGE10011-14 (foreign-code), PGE10015/16 (job sandbox) |
| PGE11xxx | Schema rules | PGE11005 (schema misuse) |
| PGE12xxx | Identifiers, aliases | PGE12002 (multi-alias), PGE12004 (identifier misuse) |
| PGE14xxx | Constructors | PGE14xxx (constructor errors) |

Warning range PGW is partitioned similarly (PGW01xxx through PGW10xxx). Current total: 31 warnings.

### Compiler Algorithms

| Algorithm | File |
|-----------|------|
| Pipeline call cycle detection | [[technical/compile-rules/algorithms/cycle-detection\|u:cycle-detection]] |
| Match condition overlap detection | [[technical/compile-rules/algorithms/overlap-detection\|u:overlap-detection]] |
| Compound exhaustiveness | [[technical/compile-rules/algorithms/compound-exhaustiveness\|u:compound-exhaustiveness]] |

## EBNF Sections

Formal grammar is partitioned into 16 sections plus an INDEX and a `definition-blocks/` subfolder. Authority: [[technical/ebnf/INDEX|u:ebnf-index]].

| Section | Topic |
|---------|-------|
| 01 — file-structure | Top-level file layout |
| 02 — lexical | Lexical productions |
| 03 — identifiers | Prefix system, identifier shapes |
| 04 — type-system | Types, schemas, generics |
| 05 — block-elements | `[X]` block elements |
| 06 — operators | Assignment, comparison, negation, range |
| 07 — io-parameters | IO lines and patterns |
| 08 — expressions | Expression productions |
| 09 — definition-blocks | `{X}` definition blocks |
| 10 — execution | Execution body, chains, inline calls |
| 11 — control-flow | Conditionals, errors |
| 12 — collections | Expand, collect, reassemble |
| 13 — comments | Comment forms |
| 14 — lifecycle | Variable lifecycle |
| 15 — example | Worked example |
| 16 — collector-definitions | `{*}` collector grammar |

## Edge-Case Catalog

Edge cases live under `docs/technical/edge-cases/` and mirror EBNF section numbers. Authority: [[technical/edge-cases/INDEX|u:edge-cases-index]].

## Philosophy Pages

Authority: [[philosophy/core-philosophy|c:core-philosophy]] (entry point). Eleven pages total.

| Page | Topic |
|------|-------|
| [[philosophy/core-philosophy\|c:core-philosophy]] | Core mind-shift and values |
| [[philosophy/accountability\|c:accountability]] | Human inspection |
| [[philosophy/behavioral-contract\|c:behavioral-contract]] | Compile-to-contract |
| [[philosophy/cybersecurity\|c:cybersecurity]] | Zero trust |
| [[philosophy/data-trees\|c:data-trees]] | Everything is a tree |
| [[philosophy/developer-experience\|c:developer-experience]] | Developer UX goals |
| [[philosophy/error-philosophy\|c:error-philosophy]] | Errors as data |
| [[philosophy/extensibility\|c:extensibility]] | Open/closed boundaries |
| [[philosophy/how-aljam3-differs\|c:how-aljam3-differs]] | Positioning |
| [[philosophy/language-design\|c:language-design]] | Language-design principles |
| [[philosophy/symbology\|c:symbology]] | Symbol/prefix rationale |

## Audit Rules

Authority: [[audit/README|c:audit/README]].

### Rule Files

| File | Purpose |
|------|---------|
| [[audit/README\|c:audit/README]] | Audit entry point |
| [[audit/rules/conventions\|c:conventions]] | Writing conventions — frontmatter, headings, code fences, typed refs |
| [[audit/rules/checklist\|c:checklist]] | Pre-publish checklist |
| [[audit/rules/workflows\|c:workflows]] | Fix / Sweep / Gate workflows |

### Audience Files

| Audience | Classification | File |
|----------|----------------|------|
| ai-finder | Internal | [[audit/audiences/ai-finder\|c:ai-finder]] |
| automation-builder | External | [[audit/audiences/automation-builder\|c:automation-builder]] |
| design | Internal | [[audit/audiences/design\|c:design]] |
| developer | Internal | [[audit/audiences/developer\|c:developer]] |
| integrator | External | [[audit/audiences/integrator\|c:integrator]] |
| product | Internal | [[audit/audiences/product\|c:product]] |

### Reference

| File | Purpose |
|------|---------|
| [[audit/reference/glossary\|c:glossary]] | Authoritative term definitions |

### Decision Records

| File | Purpose |
|------|---------|
| [[audit/decisions/README\|c:decisions-index]] | Decision-record index |
| `audit/decisions/YYYY-MM-DD-short-title.md` | Per-decision records |

### Tracking

| File | Purpose |
|------|---------|
| [[audit/tracking/progress\|u:progress]] | Sweep/gate progress |
| [[audit/tracking/coverage-gaps\|u:coverage-gaps]] | Open coverage gaps |
| [[audit/tracking/inconsistencies\|u:inconsistencies]] | Open inconsistencies |
| [[audit/tracking/audience-migration\|u:audience-migration]] | Audience-tier migration log |
| [[audit/tracking/decisions\|u:decision-tracking]] | Decision-record tracker |
| [[audit/tracking/issue-resolution-order\|u:issue-resolution-order]] | Issue resolution order |
| [[audit/tracking/ref-classification\|u:ref-classification]] | Reference classification log |

## Service / Runtime Specs

Authority: [[technical/INDEX|u:technical-index]].

| Component | File |
|-----------|------|
| Compiler floor | [[technical/spec/compiler-floor\|u:compiler-floor]] |
| Behavior Contract | [[technical/spec/behavior-contract\|u:behavior-contract]] |
| Type identity | [[technical/spec/type-identity\|u:type-identity]] |
| Native dispatch | [[technical/spec/native-dispatch\|u:native-dispatch]] |
| Job sandbox | [[technical/spec/job-sandbox\|u:job-sandbox]] |
| OTel foundation | [[technical/spec/otel-foundation\|u:otel-foundation]] |
| OTel permission events | [[technical/spec/otel-permission-events\|u:otel-permission-events]] |
| OTel config | [[technical/spec/otel-config\|u:otel-config]] |
| Aljam3 SDK | [[technical/spec/aljam3-sdk\|u:aljam3-sdk]] |
| Collector definitions | [[technical/spec/collector-definitions\|u:collector-definitions]] |
| Metadata-tree spec | [[technical/spec/metadata-tree/INDEX\|u:metadata-tree-index]] |
| Integrator internals | [[technical/integrator-internals\|u:integrator-internals]] |

Standalone algorithms (outside compile-rules):

| Algorithm | File |
|-----------|------|
| `-Run.Bridge` conversion | [[technical/algorithms/bridge-conversion\|u:bridge-conversion]] |
| Foreign-code AST analysis | [[technical/algorithms/foreign-code-analysis\|u:foreign-code-analysis]] |

Compiler modules:

| Module | File |
|--------|------|
| AST-invisible registry | [[technical/compiler/ast-invisible-registry\|u:ast-invisible-registry]] |
| IO-sink registry | [[technical/compiler/io-registry\|u:io-registry]] |
| Foreign-code parsers | [[technical/compiler/foreign-code-parsers\|u:foreign-code-parsers]] |
| Compliance-report format | [[technical/compiler/compliance-report\|u:compliance-report]] |

## Metadata Tree

Authority: [[technical/spec/metadata-tree/INDEX|u:metadata-tree-index]]. Specifies the full `%` metadata tree.

| File | Purpose |
|------|---------|
| [[technical/spec/metadata-tree/FULL-TREE\|u:metadata-FULL-TREE]] | Canonical full-tree listing |
| [[technical/spec/metadata-tree/branches\|u:metadata-branches]] | Branch definitions |
| [[technical/spec/metadata-tree/definition-templates\|u:metadata-templates]] | Definition templates |
| [[technical/spec/metadata-tree/enum-rules\|u:metadata-enum-rules]] | Enum-branch rules |
| [[technical/spec/metadata-tree/field-expansion\|u:metadata-field-expansion]] | Field-expansion rules |
| [[technical/spec/metadata-tree/instance-lifecycle\|u:metadata-instance-lifecycle]] | Instance-lifecycle fields |
| [[technical/spec/metadata-tree/io-ports\|u:metadata-io-ports]] | IO-port branches |
| [[technical/spec/metadata-tree/object-types\|u:metadata-object-types]] | Object-type branches |
| [[technical/spec/metadata-tree/path-grammar\|u:metadata-path-grammar]] | Path grammar |
| [[technical/spec/metadata-tree/string-subtypes\|u:metadata-string-subtypes]] | String-subtype tree |

## Scenarios

Authority: [[user/scenarios/INDEX|u:scenarios-index]]. 500 real-world automation scenarios partitioned into six thematic files.

| File | Theme |
|------|-------|
| [[user/scenarios/business-ops\|u:scenarios-business-ops]] | Business operations |
| [[user/scenarios/commerce-finance\|u:scenarios-commerce-finance]] | Commerce and finance |
| [[user/scenarios/communication\|u:scenarios-communication]] | Communication |
| [[user/scenarios/data-processing\|u:scenarios-data-processing]] | Data processing |
| [[user/scenarios/specialized\|u:scenarios-specialized]] | Specialised |
| [[user/scenarios/technical-ops\|u:scenarios-technical-ops]] | Technical operations |

## Retired Components

Components that existed in earlier spec versions and are no longer current. AI tools should not cite these as live; they appear here for retrieval continuity and decision-history context.

| Retired | Replaced by | Authority |
|---------|-------------|-----------|
| `{M}` macro block | Parameterised `##` schemas (`[#]` inputs as generic type templates) | Issue #272 decision |
| `[_]` inline permission block | Permission-as-resource model (`{_}` + IO markers) | Issue #310 decision, [[audit/decisions/README\|c:decisions-index]] |
| `->` chain operator | Labeled `[-]` calls with operation-label addressing | [[audit/decisions/2026-04-22-retire-chain-operator\|c:decision-retire-chain]] |
| `*Continue` | Compiler-enforced error handling; `<!` / `>!` fallbacks | Issue #152 decision |
| `*Aggregate` (name) | `*Agg` | Issue #154 decision |
| `[c]` lowercase foreign-code element | `[C]` uppercase | Issue #112 decision |
| `[t]` lowercase trigger element | `[T]` uppercase | Issue #109 decision |
| `;type` semicolon type annotation | `#type` hash annotation | Issue #146 decision |
| `:ND` array-dimension annotation via `:` | `<N>D` via `<` | Issue #156 decision |
| `#Dict` | `#Map` | Issue #89 decision |
| `=W.Rust` / `=W.Node` | Removed (unused) | Issue #28 decision |
| `=T.Schedule` / `=T.HTTP` / `=T.File` | Removed (unused) | Issue #26 decision |

Archived spec files live under `docs/archive/` with `replaced_by:` frontmatter pointers. Three files carry `replaced_by: none` (database-schema, ir-representation, contributing).

## Related Documents

| Related | Shape |
|---------|-------|
| [[INDEX\|u:INDEX]] | Human-oriented master index |
| [[ai-retrieval-index\|u:ai-retrieval-index]] | Query-shaped retrieval layer |
| [[source-tree-analysis\|u:source-tree-analysis]] | Directory-shape map |
| [[project-overview\|u:project-overview]] | Product-shaped overview |
| [[architecture\|u:architecture]] | Service + compiler architecture |
| [[development-guide\|u:development-guide]] | Contributor onboarding |
