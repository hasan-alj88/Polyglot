---
audience: ai-finder
type: reference
updated: 2026-04-23
---

# AI Retrieval Index

<!-- @c:audit/README -->
<!-- @c:audit/reference/glossary -->
<!-- @c:audit/audiences/ai-finder -->
<!-- @u:INDEX -->
This index is the query-shaped entry point for AI tools. Every row answers "given this concept, which file(s) do I read?" Sections partition by query shape (construct, compile-rule range, aj3lib namespace, type, error namespace, philosophy, audit area) rather than by human navigation. For human navigation use [[INDEX|u:INDEX]]; for repo shape use [[source-tree-analysis|u:source-tree-analysis]]; for flat inventory use [[component-inventory|u:component-inventory]]. All terminology matches [[audit/reference/glossary|c:glossary]] exactly.

## How to Read This Index

| Column | Meaning |
|--------|---------|
| Query term | The concept, construct, namespace, or error an agent is looking for |
| Primary doc | Single canonical doc for this query (read first) |
| Related docs | Adjacent docs with complementary content |
| Reference type | `c:` for mandatory (concept-of) reads, `u:` for usage reads |

All links below use typed cross-references: `[[path|c:term]]` for mandatory imports; `[[path|u:term]]` for usage references; `[[path|d:term]]` for deprecated pointers.

## By Language Construct

Construct → user-facing spec and EBNF grammar for that construct.

| Construct | Prefix | User spec | EBNF grammar |
|-----------|--------|-----------|--------------|
| Package definition | `{@}` | [[user/syntax/packages\|u:packages]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Type definition | `{#}` | [[user/syntax/types/INDEX\|u:types-index]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Pipeline definition | `{-}` | [[user/concepts/pipelines/INDEX\|u:pipelines-index]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Expander definition | `{=}` | [[user/concepts/collections/expand\|u:expand]] | [[technical/ebnf/12-collections\|u:ebnf-12]] |
| Collector definition | `{*}` | [[user/concepts/collections/collect\|u:collect]] | [[technical/ebnf/16-collector-definitions\|u:ebnf-16]] |
| Wrapper definition | `{W}` | [[user/concepts/pipelines/wrappers\|u:wrappers]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Trigger definition | `{T}` | [[user/concepts/pipelines/io-triggers\|u:io-triggers]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Queue definition | `{Q}` | [[user/concepts/pipelines/queue\|u:queue]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Permission definition | `{_}` | [[user/concepts/permissions\|u:permissions]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Error definition | `{!}` | [[user/concepts/errors\|u:errors]] | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| Constructor definition | `{$}` | [[user/syntax/constructors\|u:constructors]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Environment definition | `{;}` | [[user/syntax/environments\|u:environments]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Native block | `{N}` | [[user/concepts/pipelines/INDEX\|u:pipelines-index]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Foreign-code block | `[C]` | [[user/concepts/pipelines/INDEX\|u:pipelines-index]] | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| Conditional block | `[?]` | [[user/concepts/conditionals\|u:conditionals]] | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| Error block | `[!]` | [[user/concepts/errors\|u:errors]] | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| IO marker | `(-)` | [[user/syntax/io\|u:io]] | [[technical/ebnf/07-io-parameters\|u:ebnf-07]] |
| Operation label | `(-) $Label` | [[user/syntax/operation-labels\|u:operation-labels]] | [[technical/ebnf/10-execution\|u:ebnf-10]] |
| Chain (retired) | `->` | [[user/concepts/pipelines/chains\|u:chains]] | [[technical/ebnf/10-execution\|u:ebnf-10]] |

## By Syntax Topic

| Topic | Primary doc | Related |
|-------|-------------|---------|
| Line structure, indentation | [[user/syntax/line-structure\|u:line-structure]] | [[technical/ebnf/01-file-structure\|u:ebnf-01]] |
| Comments | [[user/syntax/comments\|u:comments]] | [[technical/ebnf/13-comments\|u:ebnf-13]] |
| Identifier prefixes | [[user/syntax/identifiers\|u:identifiers]] | [[technical/ebnf/03-identifiers\|u:ebnf-03]] |
| Block syntax | [[user/syntax/blocks\|u:blocks]] | [[technical/ebnf/05-block-elements\|u:ebnf-05]] |
| Operators | [[user/syntax/operators\|u:operators]] | [[technical/ebnf/06-operators\|u:ebnf-06]] |
| IO parameters | [[user/syntax/io\|u:io]] | [[technical/ebnf/07-io-parameters\|u:ebnf-07]] |
| Auto-wire | [[user/syntax/io/auto-wire\|u:auto-wire]] | [[technical/ebnf/10-execution\|u:ebnf-10]] |
| Chain IO | [[user/syntax/io/chain-io\|u:chain-io]] | — |
| Collection operators | [[user/syntax/io/collection-operators\|u:collection-operators]] | [[technical/ebnf/12-collections\|u:ebnf-12]] |
| IO labels | [[user/syntax/io/io-labels\|u:io-labels]] | — |
| IO variables | [[user/syntax/io/io-variables\|u:io-variables]] | — |
| Packages and imports | [[user/syntax/packages\|u:packages]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Variable lifecycle | [[user/concepts/variable-lifecycle\|u:variable-lifecycle]] | [[technical/ebnf/14-lifecycle\|u:ebnf-14]] |
| Conditionals | [[user/concepts/conditionals\|u:conditionals]] | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| Errors | [[user/concepts/errors\|u:errors]] | [[technical/ebnf/11-control-flow\|u:ebnf-11]] |
| Metadata | [[user/concepts/metadata\|u:metadata]] | [[technical/spec/metadata-tree/INDEX\|u:metadata-tree-index]] |
| Permissions | [[user/concepts/permissions\|u:permissions]] | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Data-as-trees | [[user/concepts/data-is-trees\|u:data-is-trees]] | [[philosophy/data-trees\|c:data-trees]] |

## By Type-System Concept

| Concept | Primary doc | EBNF |
|---------|-------------|------|
| Prefix system (`#`, `##`, `###`) | [[user/syntax/types/prefix-system\|u:prefix-system]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Basic types (`RawString`, `#String`, `int`, `float`) | [[user/syntax/types/basic-types\|u:basic-types]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Schema properties (`##` schemas) | [[user/syntax/types/schema-properties\|u:schema-properties]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Generic types (schema inputs) | [[user/syntax/types/generic-types\|u:generic-types]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Arrays (typed and multidimensional) | [[user/syntax/types/arrays\|u:arrays]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Structs | [[user/syntax/types/structs\|u:structs]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Flexible fields | [[user/syntax/types/flexible-fields\|u:flexible-fields]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Strings and interpolation | [[user/syntax/types/strings\|u:strings]] | — |
| Type conversions | [[user/syntax/types/conversions\|u:conversions]] | — |
| Type hierarchy and namespaces | [[user/syntax/types/hierarchy\|u:hierarchy]] | — |
| Structural type identity | [[technical/spec/type-identity\|u:type-identity]] | [[technical/ebnf/04-type-system\|u:ebnf-04]] |

## By Compile-Rule Range

PGE (errors) and PGW (warnings) numbering. Read [[technical/COMPILE-RULES|u:COMPILE-RULES]] for the master lookup; per-rule files live under `docs/technical/compile-rules/PGE/` and `docs/technical/compile-rules/PGW/`.

| Range | Topic area | Per-rule folder |
|-------|------------|-----------------|
| PGE01xxx | Pipeline structure, block elements, triggers, queues, wrappers | `technical/compile-rules/PGE/PGE01*.md` |
| PGE02xxx | Variable lifecycle, Final/Failed transitions | `technical/compile-rules/PGE/PGE02*.md` |
| PGE03xxx | Collectors, reassembly, job termination | `technical/compile-rules/PGE/PGE03*.md` |
| PGE04xxx | Type system, schemas, arrays, coercion | `technical/compile-rules/PGE/PGE04*.md` |
| PGE05xxx | Metadata paths, separator rules | `technical/compile-rules/PGE/PGE05*.md` |
| PGE06xxx | Conditional coverage, exhaustiveness | `technical/compile-rules/PGE/PGE06*.md` |
| PGE07xxx | Error handling, fallbacks, suppression | `technical/compile-rules/PGE/PGE07*.md` |
| PGE08xxx | Chains, references, self-assignment | `technical/compile-rules/PGE/PGE08*.md` |
| PGE10xxx | Permissions, foreign-code compliance | `technical/compile-rules/PGE/PGE10*.md` |
| PGE11xxx | Schema rules, property validation | `technical/compile-rules/PGE/PGE11*.md` |
| PGE12xxx | Identifiers, aliases, multi-alias declarations | `technical/compile-rules/PGE/PGE12*.md` |
| PGE14xxx | Constructor-block errors | `technical/compile-rules/PGE/PGE14*.md` |
| PGW01xxx - PGW10xxx | Warnings (non-blocking) | `technical/compile-rules/PGW/*.md` |

Compiler algorithms that back the rules live under `docs/technical/compile-rules/algorithms/`:

| Algorithm | File |
|-----------|------|
| Pipeline call cycle detection | [[technical/compile-rules/algorithms/cycle-detection\|u:cycle-detection]] |
| Match condition overlap detection | [[technical/compile-rules/algorithms/overlap-detection\|u:overlap-detection]] |
| Compound exhaustiveness | [[technical/compile-rules/algorithms/compound-exhaustiveness\|u:compound-exhaustiveness]] |

## By aj3lib Namespace

Query: "what pipelines does namespace X provide?" Each namespace has a sub-index under `docs/user/aj3lib/pipelines/{Namespace}/INDEX.md` (or a flat file for single-file namespaces).

| Namespace | Prefix | Purpose | Primary dir |
|-----------|--------|---------|-------------|
| File | `-File.*` | File-system read/write | [[user/aj3lib/pipelines/File/INDEX\|u:aj3lib-File]] |
| T (triggers) | `-T.*` | HTTP, queue, git, time, call triggers | [[user/aj3lib/pipelines/T/INDEX\|u:aj3lib-T]] |
| Q (queue control) | `-Q.*` | Queue inspection and control | [[user/aj3lib/pipelines/Q/INDEX\|u:aj3lib-Q]] |
| W (wrappers) | `-W.*` | Wrappers: RT, Env, Retry, Aljam3 | [[user/aj3lib/pipelines/W/INDEX\|u:aj3lib-W]] |
| Math | `-Math.*` | Arithmetic operations | [[user/aj3lib/pipelines/Math/INDEX\|u:aj3lib-Math]] |
| DT (datetime) | `-DT.*` | #DateTime construction, conversion, arithmetic | [[user/aj3lib/pipelines/DT/INDEX\|u:aj3lib-DT]] |
| RT (runtime) | `-RT.*` | Python/Rust/Go/JS runtime execution | [[user/aj3lib/pipelines/RT/INDEX\|u:aj3lib-RT]] |
| Run | `-Run.*` | Script, binary, Shell, Bridge execution | [[user/aj3lib/pipelines/Run/INDEX\|u:aj3lib-Run]] |
| Schema | `-Schema.*` | Schema match / validate / describe / coerce | [[user/aj3lib/pipelines/Schema/INDEX\|u:aj3lib-Schema]] |
| Variable | `-Variable.*` | Variable introspection | [[user/aj3lib/pipelines/Variable/INDEX\|u:aj3lib-Variable]] |
| Text | `-Text.*` | Text operations | [[user/aj3lib/pipelines/Text\|u:aj3lib-Text]] |
| Path | `-Path.*` | Path operations | [[user/aj3lib/pipelines/Path\|u:aj3lib-Path]] |
| Sys | `-Sys.*` | System inspection | [[user/aj3lib/pipelines/Sys\|u:aj3lib-Sys]] |

Standalone parser pipelines (file per name):

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

## By Expander / Collector

| Category | Namespace | Primary dir |
|----------|-----------|-------------|
| Expander | `=ForEach.*` | [[user/aj3lib/expanders/ForEach/INDEX\|u:ForEach-index]] |
| Collector — aggregate | `*Agg.*` | [[user/aj3lib/collectors/Agg/INDEX\|u:Agg-index]] |
| Collector — into-collection | `*Into.*` | [[user/aj3lib/collectors/Into/INDEX\|u:Into-index]] |
| Collector — sync/race | `*Sync.*` (incl. `*All`, `*First`, `*Nth`) | [[user/aj3lib/collectors/Sync/INDEX\|u:Sync-index]] |

## By Type

Query: "where is type X defined?" Types live under `docs/user/aj3lib/types/` — top-level files plus `datetime/`, `field-types/`, `properties/`, `scalars/`, `schemas/` subdirs.

| Type category | Files | Location |
|---------------|-------|----------|
| Boolean, scalar primitives | `boolean.md`, `string.md`, `path.md` | `docs/user/aj3lib/types/*.md` |
| Scalars (subtype tree) | `scalars/` (11 files) | `docs/user/aj3lib/types/scalars/` |
| Schemas (`##Record`, `##Leaf`, `##Inf`, `##Nullable`, etc.) | `schemas/` (20 files) | `docs/user/aj3lib/types/schemas/` |
| Schema properties | `properties/` (16 files) | `docs/user/aj3lib/types/properties/` |
| Field kinds | `field-types/` (6 files) | `docs/user/aj3lib/types/field-types/` |
| Date / time | `datetime/` (10 files) | `docs/user/aj3lib/types/datetime/` |
| Collections | `Array.md`, `Map.md`, `Serial.md`, `Dataframe.md`, `Set.md` | `docs/user/aj3lib/types/*.md` |
| Runtime types | `PyEnv.md`, `RsEnv.md`, `rt.md`, `Code.md` | `docs/user/aj3lib/types/*.md` |
| Service types | `Job.md`, `JobStatus.md`, `Queue.md`, `QueueState.md`, `Variable.md`, `VarState.md` | `docs/user/aj3lib/types/*.md` |
| Git | `git.md` | `docs/user/aj3lib/types/git.md` |
| Parsed entities | `Color.md`, `IP.md`, `URL.md`, `MIME.md`, `Ver.md`, `Re.md` | `docs/user/aj3lib/types/*.md` |
| Capability enums | `CPUCapability.md`, `GPUCapability.md`, `RAMCapability.md`, `IOCapability.md`, `ProcessCapability.md`, `DurationCapability.md` | `docs/user/aj3lib/types/*.md` |

Master type aggregators:

| File | Purpose |
|------|---------|
| [[user/aj3lib/types/types\|u:types]] | Top-level type catalog |
| [[user/aj3lib/types/collections\|u:collections-types]] | Collection-type summary |
| [[user/aj3lib/types/scalars\|u:scalars]] | Scalar-subtype summary |
| [[user/aj3lib/types/enums\|u:enums]] | Enum-type summary |
| [[user/aj3lib/types/structs\|u:structs-types]] | Struct-type summary |
| [[user/aj3lib/types/schemas\|u:schemas-index]] | Schema summary |

## By Error Namespace

Query: "what errors can pipeline X raise?" Error namespaces live under `docs/user/aj3lib/errors/`.

| Error file | Topic |
|------------|-------|
| [[user/aj3lib/errors/errors\|u:errors-aj3lib]] | Error-namespace overview |
| [[user/aj3lib/errors/error-struct\|u:error-struct]] | `!Error` struct definition |
| [[user/aj3lib/errors/custom-errors\|u:custom-errors]] | User-defined error types via `{!}` |
| [[user/aj3lib/errors/alias-clash\|u:alias-clash]] | Alias-clash error handling |
| [[user/aj3lib/errors/pipeline-associations\|u:pipeline-associations]] | Pipeline-to-error-namespace associations |

Built-in namespaces (under `docs/user/aj3lib/errors/builtin/`) are partitioned by concern:

| Namespace | Raised by |
|-----------|-----------|
| `!Permission.*` | Permission checks (`_` prefix, `{_}` definitions, foreign-code compliance) |
| `!Validation.*` | Schema / type / regex validation (`-Schema.*` pipelines) |
| `!Field.*` | Field-level validation |
| `!File.*` | File IO (`-File.*` pipelines) |
| `!RT.*` | Runtime execution (`-RT.*` pipelines) |
| `!Env.*` | Environment wiring (`{;}` blocks, `-W.Env`) |
| `!Queue.*` | Queue control (`-Q.*`) |

## By Permission Category

Permission categories live under `docs/user/aj3lib/permissions/` as one subdirectory per category. The authoritative concept document is [[user/concepts/permissions|u:permissions]].

| Category | Scope | Dir |
|----------|-------|-----|
| Crypto | Cryptographic primitives | `docs/user/aj3lib/permissions/Crypto/` |
| Database | Database connections | `docs/user/aj3lib/permissions/Database/` |
| Device | Device access | `docs/user/aj3lib/permissions/Device/` |
| File | File-system access | `docs/user/aj3lib/permissions/File/` |
| IPC | Inter-process communication | `docs/user/aj3lib/permissions/IPC/` |
| Memory | Memory allocation and limits | `docs/user/aj3lib/permissions/Memory/` |
| System | System-level access (shell, env, OS) | `docs/user/aj3lib/permissions/System/` |
| Web | Network and HTTP access | `docs/user/aj3lib/permissions/Web/` |

Concept-level permission sub-pages:

| Topic | File |
|-------|------|
| Permission objects (`{_}`) | [[user/concepts/permissions/permission-objects\|u:permission-objects]] |
| Permission prefixes (`_`, `__`, `___`) | [[user/concepts/permissions/permission-prefixes\|u:permission-prefixes]] |
| Permission schema | [[user/concepts/permissions/permission-schema\|u:permission-schema]] |
| Implicit-deny model | [[user/concepts/permissions/implicit-deny\|u:implicit-deny]] |
| Hierarchical scoping | [[user/concepts/permissions/hierarchical-scoping\|u:hierarchical-scoping]] |
| Capability enums | [[user/concepts/permissions/capability-enums\|u:capability-enums]] |
| Foreign-code compliance | [[user/concepts/permissions/foreign-code\|u:foreign-code]] |
| Enforcement | [[user/concepts/permissions/enforcement\|u:enforcement]] |

## By Philosophy Page

| Topic | File |
|-------|------|
| Core philosophy (entry point) | [[philosophy/core-philosophy\|c:core-philosophy]] |
| Accountability — human inspection | [[philosophy/accountability\|c:accountability]] |
| Behavior Contract | [[philosophy/behavioral-contract\|c:behavioral-contract]] |
| Cybersecurity — zero trust | [[philosophy/cybersecurity\|c:cybersecurity]] |
| Data as trees | [[philosophy/data-trees\|c:data-trees]] |
| Developer experience | [[philosophy/developer-experience\|c:developer-experience]] |
| Error philosophy | [[philosophy/error-philosophy\|c:error-philosophy]] |
| Extensibility | [[philosophy/extensibility\|c:extensibility]] |
| How Aljam3 differs | [[philosophy/how-aljam3-differs\|c:how-aljam3-differs]] |
| Language design | [[philosophy/language-design\|c:language-design]] |
| Symbol / prefix rationale | [[philosophy/symbology\|c:symbology]] |

## By Audit Area

| Audit area | File | Purpose |
|------------|------|---------|
| Entry point | [[audit/README\|c:audit/README]] | Read before writing any doc |
| Writing conventions | [[audit/rules/conventions\|c:conventions]] | Frontmatter, headings, code fences, typed refs |
| Checklist | [[audit/rules/checklist\|c:checklist]] | Pre-publish quality checks |
| Workflows | [[audit/rules/workflows\|c:workflows]] | Fix / Sweep / Gate |
| Glossary | [[audit/reference/glossary\|c:glossary]] | Authoritative term definitions |
| Audience — ai-finder | [[audit/audiences/ai-finder\|c:ai-finder]] | Rules for this document and siblings |
| Audience — automation-builder | [[audit/audiences/automation-builder\|c:automation-builder]] | External pg-file authors |
| Audience — design | [[audit/audiences/design\|c:design]] | Language/spec designers |
| Audience — developer | [[audit/audiences/developer\|c:developer]] | Compiler/runtime implementers |
| Audience — integrator | [[audit/audiences/integrator\|c:integrator]] | SDK / bridge integrators |
| Audience — product | [[audit/audiences/product\|c:product]] | Product managers |
| Decision records (index) | [[audit/decisions/README\|c:decisions-index]] | Architectural/syntactic decisions |
| Audit tracking — progress | [[audit/tracking/progress\|u:progress]] | Sweep/gate progress |
| Audit tracking — gaps | [[audit/tracking/coverage-gaps\|u:coverage-gaps]] | Open coverage gaps |
| Audit tracking — inconsistencies | [[audit/tracking/inconsistencies\|u:inconsistencies]] | Open inconsistencies |

## By Service Component

Query: "how does the runtime work?" Service components have their own `docs/technical/spec/` files.

| Component | File |
|-----------|------|
| Compiler floor (compile target boundary) | [[technical/spec/compiler-floor\|u:compiler-floor]] |
| Behavior Contract (signal-graph IR) | [[technical/spec/behavior-contract\|u:behavior-contract]] |
| Trigger Monitor / Queue Handler / Dispatch Coordinator / Runner | [[architecture\|u:architecture]] |
| Native dispatch | [[technical/spec/native-dispatch\|u:native-dispatch]] |
| Job sandbox (OS-level) | [[technical/spec/job-sandbox\|u:job-sandbox]] |
| Type identity (structural matching) | [[technical/spec/type-identity\|u:type-identity]] |
| Aljam3 SDK | [[technical/spec/aljam3-sdk\|u:aljam3-sdk]] |
| Collector definitions | [[technical/spec/collector-definitions\|u:collector-definitions]] |
| Metadata-tree spec | [[technical/spec/metadata-tree/INDEX\|u:metadata-tree-index]] |
| Integrator internals | [[technical/integrator-internals\|u:integrator-internals]] |

## By Observability Topic

| Topic | File |
|-------|------|
| OTel foundation — tracing infrastructure | [[technical/spec/otel-foundation\|u:otel-foundation]] |
| OTel permission / sandbox events | [[technical/spec/otel-permission-events\|u:otel-permission-events]] |
| OTel exporter configuration | [[technical/spec/otel-config\|u:otel-config]] |

## By Cross-Language Topic

| Topic | File |
|-------|------|
| Aljam3 SDK (encode / decode / call / pull / push) | [[technical/spec/aljam3-sdk\|u:aljam3-sdk]] |
| -Run.Bridge conversion algorithm | [[technical/algorithms/bridge-conversion\|u:bridge-conversion]] |
| Foreign-code AST analysis (permission compliance) | [[technical/algorithms/foreign-code-analysis\|u:foreign-code-analysis]] |
| AST-invisible registry (banned functions) | [[technical/compiler/ast-invisible-registry\|u:ast-invisible-registry]] |
| IO-sink registry (AST analysis) | [[technical/compiler/io-registry\|u:io-registry]] |
| Foreign-code parsers (per-language) | [[technical/compiler/foreign-code-parsers\|u:foreign-code-parsers]] |
| Compliance-report format | [[technical/compiler/compliance-report\|u:compliance-report]] |

## Retrieval Hints

Heuristics for picking the right starting file given a query shape.

| If the query is... | Start here |
|---------------------|------------|
| "What is Aljam3?" | [[vision\|c:vision]] then [[project-overview\|u:project-overview]] |
| "How do I write a pipeline?" | [[user/SPEC-INDEX\|u:SPEC-INDEX]] |
| "What does `{X}` mean?" | "By Language Construct" table above |
| "What does compile error PGE01005 mean?" (or any PGE / PGW code) | [[technical/COMPILE-RULES\|u:COMPILE-RULES]] then the matching file under `technical/compile-rules/PGE/` or `technical/compile-rules/PGW/` |
| "What pipelines does `-X.*` provide?" | "By aj3lib Namespace" table above |
| "What error namespaces exist?" | "By Error Namespace" table above |
| "What does #TypeName mean?" | "By Type" table above; start with [[user/aj3lib/types/types\|u:types]] |
| "How does the runtime work?" | [[architecture\|u:architecture]] |
| "Why did Aljam3 make decision X?" | [[audit/decisions/README\|c:decisions-index]] then [[philosophy/core-philosophy\|c:core-philosophy]] |
| "Can I write this .aj3 code?" | [[technical/ebnf/INDEX\|u:ebnf-index]] then [[technical/edge-cases/INDEX\|u:edge-cases-index]] |
| "Where is the full repo layout?" | [[source-tree-analysis\|u:source-tree-analysis]] |
| "Give me an inventory by category" | [[component-inventory\|u:component-inventory]] |
| "Where are the writing rules?" | [[audit/README\|c:audit/README]] |

## Cross-Reference Types Used

| Prefix | Meaning | Directive to AI |
|--------|---------|-----------------|
| `c:` | Concept-of — mandatory import | Read the referenced file before reasoning about the current file |
| `u:` | Usage reference — used by | Optional read; provides usage context but not required for correctness |
| `d:` | Deprecated pointer — informational | Do not cite as current; follow to see retirement history only |

Cross-references appear in two forms in every audit-compliant doc:

| Form | Purpose |
|------|---------|
| `<!-- @c:path -->` / `<!-- @u:path -->` / `<!-- @d:path -->` HTML comment | Claude-mandatory import directive |
| `[[path\|c:term]]` / `[[path\|u:term]]` / `[[path\|d:term]]` wikilink | Obsidian navigation + display anchor |

Both must be present. See [[audit/README|c:audit/README]] for the full typed-referencing rule.

## Versioning and Stability

This index is re-generated whenever:

- New top-level `docs/` files are added or removed
- aj3lib namespaces are added, renamed, or partitioned into subfolders
- PGE / PGW rule ranges are added or retired
- Philosophy pages are added or removed
- Audit audience tiers change

`updated:` in the frontmatter is the canonical "last regenerated" marker. Changes between regenerations are caught by the [[audit/tracking/progress|u:progress]] sweep.

## Related Documents

| Related | Shape |
|---------|-------|
| [[INDEX\|u:INDEX]] | Human-oriented master navigation |
| [[source-tree-analysis\|u:source-tree-analysis]] | Directory-shape map |
| [[component-inventory\|u:component-inventory]] | Flat categorical inventory |
| [[project-overview\|u:project-overview]] | Product-shaped overview |
| [[architecture\|u:architecture]] | Service + compiler architecture |
| [[development-guide\|u:development-guide]] | Contributor onboarding |
