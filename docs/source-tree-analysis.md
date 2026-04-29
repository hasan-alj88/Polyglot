---
audience: ai-finder
type: reference
updated: 2026-04-23
---

# Source Tree Analysis

<!-- @c:audit/README -->
<!-- @c:audit/reference/glossary -->
<!-- @u:INDEX -->
This document is a repository-shape map for AI tools navigating Aljam3. It lists every directory under the project root that contains documentation or tooling, with file counts and authoritative sub-indexes. All paths are relative to the repository root. Directory contents can change; file counts in tables are accurate as of the `updated` date in the frontmatter. Authoritative navigation remains [[INDEX|u:INDEX]]; this file serves retrieval, not learning.

## Repository Root

```text
Aljam3/
├── .paul/                      PAUL project management — state, plans, handoffs
├── docs/                       Documentation corpus (Obsidian vault)
├── scripts/                    Shell and Python tooling for docs / PDF
├── CLAUDE.md                   Claude Code instruction file
├── pyproject.toml              Python project manifest (uv)
├── uv.lock                     uv lockfile
└── README.md                   Repository entry point
```

No compiled code is currently present. All Rust source was removed in the 2026-03-12 reset; compiler implementation is deferred until the specification stabilises (see [[philosophy/core-philosophy|c:core-philosophy]]).

## docs/ — Top Level

```text
docs/
├── vision.md                   Highest authority — product vision
├── INDEX.md                    Master human-oriented index
├── project-overview.md         Product-shaped overview
├── architecture.md             Consolidated service + compiler architecture
├── ai-retrieval-index.md       Query-shaped AI retrieval layer
├── source-tree-analysis.md     This file — repo shape map
├── component-inventory.md      Flat categorical inventory
├── development-guide.md        Contributor onboarding
├── draft.md                    Scratch file for active drafting
├── philosophy/                 Philosophy sub-pages (11 files)
├── audit/                      Documentation ground truth
├── user/                       External-audience spec
├── technical/                  Internal-audience spec
├── archive/                    Retired documents (kept with replaced_by)
└── pdf/                        Generated PDF output
```

| Top-level file | Audience | Role |
|----------------|----------|------|
| `vision.md` | all | Authoritative product vision |
| `INDEX.md` | automation-builder, integrator, design | Human master index |
| `project-overview.md` | product | Product-shaped entry point |
| `architecture.md` | design | Consolidated architecture read |
| `ai-retrieval-index.md` | ai-finder | Query-shaped AI retrieval layer |
| `source-tree-analysis.md` | ai-finder | Repo shape map (this file) |
| `component-inventory.md` | ai-finder | Flat inventory by category |
| `development-guide.md` | developer | Contributor onboarding |
| `draft.md` | — | Scratch file (not authoritative) |

## docs/philosophy/

```text
docs/philosophy/
├── core-philosophy.md          Mind-shift, values, evolution
├── accountability.md           Human inspection principle
├── behavioral-contract.md      Compile-to-contract principle
├── cybersecurity.md            Zero-trust posture
├── data-trees.md               "Everything is a tree" principle
├── developer-experience.md     Developer UX goals
├── error-philosophy.md         Error-as-data, compile-time enforcement
├── extensibility.md            Open/closed boundaries
├── how-aljam3-differs.md     Comparative positioning
├── language-design.md          Language design principles
└── symbology.md                Symbol and prefix rationale
```

| File | Topic |
|------|-------|
| `core-philosophy.md` | Entry point for philosophy — mind-shift summary |
| `accountability.md` | Human inspection, auditable decisions |
| `behavioral-contract.md` | Compile to Behavior Contract; service reads contract |
| `cybersecurity.md` | Zero-trust, black-box monitoring, permission ceiling |
| `data-trees.md` | All data is tree-shaped; `%` metadata tree |
| `developer-experience.md` | Authoring ergonomics and guard rails |
| `error-philosophy.md` | Errors are data; compile-time exhaustiveness |
| `extensibility.md` | What is extensible (user pipelines) vs locked (grammar) |
| `how-aljam3-differs.md` | Contrast with general-purpose languages |
| `language-design.md` | Language design principles |
| `symbology.md` | Why the `@`, `#`, `=`, `$`, `!`, `%`, `{`, `[`, `(` symbols |

## docs/audit/ — Documentation Ground Truth

```text
docs/audit/
├── README.md                   Audit entry point — read before writing
├── audiences/                  Per-audience tone and rules (6 files)
├── rules/                      Writing conventions, checklist, workflows
├── reference/                  Glossary
├── decisions/                  Design decision records
└── tracking/                   Progress, gaps, issue resolution
```

```text
docs/audit/audiences/
├── ai-finder.md                Internal — AI retrieval layer
├── automation-builder.md       External — writes .aj3 files
├── design.md                   Internal — grammar, compile rules, philosophy
├── developer.md                Internal — compiler and runtime implementers
├── integrator.md               External — connects codebases via SDK
└── product.md                  Internal — product managers, requirements
```

```text
docs/audit/rules/
├── conventions.md              Frontmatter, headings, code fences, typed refs
├── checklist.md                Pre-publish quality checks
└── workflows.md                Fix / Sweep / Gate workflows
```

```text
docs/audit/reference/
└── glossary.md                 Authoritative term definitions
```

```text
docs/audit/decisions/
├── README.md                   Decision-record index
├── 2026-04-22-audience-tier-restructure.md
├── 2026-04-22-constructor-blocks.md
└── 2026-04-22-retire-chain-operator.md
```

```text
docs/audit/tracking/
├── progress.md                 Sweep and gate progress tracker
├── coverage-gaps.md            Open coverage gaps
├── inconsistencies.md          Open inconsistencies
├── audience-migration.md       Audience tier migration log
├── decisions.md                Decision tracker
├── issue-resolution-order.md   Issue resolution order log
└── ref-classification.md       Reference classification log
```

## docs/user/ — External-Audience Spec

```text
docs/user/
├── SPEC-INDEX.md               5-phase learning progression
├── PGLIB.md                    Standard library overview (top-level pointer)
├── syntax/                     Syntax foundations (10 files + subdirs)
├── concepts/                   Core concepts (7 files + 4 subdirs)
├── pglib/                      Standard library reference (large)
├── scenarios/                  500 real-world automation scenarios
└── integrator/                 Integrator-specific guidance (1 file)
```

### docs/user/syntax/

```text
docs/user/syntax/
├── line-structure.md           3-space indentation, one expression per line
├── comments.md                 {}, [], () comment styles
├── identifiers.md              @ # = $ ! % _ prefixes, separators
├── blocks.md                   {X} definition blocks, [X] block elements
├── operators.md                Assignment, comparison, negation, range
├── operation-labels.md         (-) $Label; operation-scoped addressing
├── constructors.md             {$} constructor blocks
├── environments.md             {;} environment blocks
├── packages.md                 Package declaration, address format, imports
├── io.md                       Input/output parameters, IO line patterns
├── io/                         IO subtopics (10 files)
└── types/                      Type-system syntax (11 files)
```

| syntax/ subdir | Files |
|----------------|-------|
| `io/` | auto-wire, chain-io, collection-operators, environment-declaration, error-declaration, io-labels, io-parameter-handling, io-variables, operation-labels, pipeline-call |
| `types/` | INDEX, prefix-system, basic-types, schema-properties, generic-types, arrays, structs, flexible-fields, strings, conversions, hierarchy |

### docs/user/concepts/

```text
docs/user/concepts/
├── variable-lifecycle.md       Declared → Default → Final → Failed → Released
├── conditionals.md             [?] chains, exhaustiveness, logical operators
├── errors.md                   Error model, scoping, chain addressing
├── metadata.md                 Full % tree field listings, access patterns
├── data-is-trees.md            Unified tree model
├── permissions.md              Permission model top level
├── permissions/                Permission sub-pages (8 files)
├── pipelines/                  Pipeline sub-pages (10 files)
└── collections/                Collection sub-pages (10 files)
```

| concepts/ subdir | Files |
|------------------|-------|
| `permissions/` | capability-enums, enforcement, foreign-code, hierarchical-scoping, implicit-deny, permission-objects, permission-prefixes, permission-schema |
| `pipelines/` | INDEX, chains, error-handling, execution, inline-calls, io-triggers, metadata, permissions, queue, wrappers |
| `collections/` | INDEX, map, array, serial, dataframe, user-struct, expand, collect, reassemble, examples |

### docs/user/pglib/ — Standard Library Reference

```text
docs/user/pglib/
├── INDEX.md                    Namespace registry
├── pipelines/                  {-} pipeline operators
├── expanders/                  {=} expander operators
├── collectors/                 {*} collector operators
├── reassemblers/               Reassembler operators
├── types/                      pglib type catalog (>70 top-level files)
├── constructors/               {$} constructor catalog (10 files)
├── errors/                     Error-namespace catalog (5 files + subdir)
└── permissions/                Permission category catalog (9 subdirs)
```

```text
docs/user/pglib/pipelines/
├── DT/                         42 files — #DateTime construction, conversion, arithmetic
├── File/                       12 files — file access (Text, Serial, Binary)
├── Math/                       9 files — arithmetic operations
├── Q/                          27 files — queue control pipelines
├── RT/                         8 files — runtime environment (Python, Rust, Go, JS)
├── Run/                        8 files — script and binary execution
├── Schema/                     10 files — schema match/validate/describe/coerce
├── T/                          11 files — triggers (HTTP, Queue, Git, Time)
├── Text/                       1 file  — text operations
├── Variable/                   2 files — variable introspection
├── W/                          14 files — wrappers (RT, Env, Retry, Aljam3)
├── Path.md, Sys.md             Standalone pglib pipelines
├── Color.Parse.md, Dur.Parse.md, IP.Parse.md, MIME.Parse.md,
├── Path.Parse.md, Re.Parse.md, URL.Parse.md, Ver.Parse.md
```

```text
docs/user/pglib/expanders/
├── INDEX.md
└── ForEach/                    10 files — Array, Map, Serial, Level, Dataframe variants
```

```text
docs/user/pglib/collectors/
├── INDEX.md
├── Agg/                        7 files — aggregate collectors (Sum, Count, Max, Min, etc.)
├── Into/                       10 files — into-collection collectors
└── Sync/                       5 files — sync/race collectors (All, First, Nth)
```

```text
docs/user/pglib/types/
├── INDEX.md (via types.md)
├── 72 top-level .md files       Catalog of pglib types
├── datetime/                    10 files — #DateTime subtype tree
├── field-types/                 6 files  — field type definitions
├── properties/                 16 files  — ## schema properties
├── scalars/                    11 files  — scalar subtype tree
└── schemas/                    20 files  — ## schemas (Record, Leaf, Inf, Nullable, etc.)
```

```text
docs/user/pglib/errors/
├── errors.md                   Error-namespace overview
├── error-struct.md             !Error struct definition
├── custom-errors.md            User-defined error types
├── alias-clash.md              Alias clash handling
├── pipeline-associations.md    Pipeline → error-namespace associations
└── builtin/                    Built-in error-namespace pages
```

```text
docs/user/pglib/permissions/
├── INDEX.md
├── Crypto/                     Cryptographic permissions
├── Database/                   Database permissions
├── Device/                     Device permissions
├── File/                       File-system permissions
├── IPC/                        Inter-process communication permissions
├── Memory/                     Memory permissions
├── System/                     System permissions
└── Web/                        Web permissions
```

```text
docs/user/pglib/constructors/
├── INDEX.md
├── Color.md, DT.md, Dur.md, IP.md, MIME.md
├── Path.md, Re.md, URL.md, Ver.md
```

```text
docs/user/pglib/reassemblers/
├── INDEX.md
├── Agg/                        Aggregate-reassembler pages
└── Into/                       Into-reassembler pages
```

### docs/user/scenarios/

```text
docs/user/scenarios/
├── INDEX.md                    Scenario registry
├── business-ops.md
├── commerce-finance.md
├── communication.md
├── data-processing.md
├── specialized.md
└── technical-ops.md
```

### docs/user/integrator/

```text
docs/user/integrator/
└── aljam3-interface.md       Integrator-facing interface reference
```

## docs/technical/ — Internal-Audience Spec

```text
docs/technical/
├── INDEX.md                    Technical doc index
├── COMPILE-RULES.md            PGE/PGW lookup root
├── integrator-internals.md     Integrator internals spec
├── ebnf/                       Formal grammar (16 sections + INDEX)
├── edge-cases/                 Edge-case catalog
├── compile-rules/              Per-rule files (PGE, PGW, algorithms)
├── spec/                       Runtime and service specs
├── algorithms/                 Standalone algorithm specs
├── compiler/                   Compiler-module specs
├── brainstorming/              Work-in-progress (not authoritative)
└── plan/                       Work-in-progress (not authoritative)
```

### docs/technical/ebnf/

```text
docs/technical/ebnf/
├── INDEX.md                    Section registry
├── 01-file-structure.md
├── 02-lexical.md
├── 03-identifiers.md
├── 04-type-system.md
├── 05-block-elements.md
├── 06-operators.md
├── 07-io-parameters.md
├── 08-expressions.md
├── 09-definition-blocks.md
├── 10-execution.md
├── 11-control-flow.md
├── 12-collections.md
├── 13-comments.md
├── 14-lifecycle.md
├── 15-example.md
├── 16-collector-definitions.md
└── definition-blocks/          Per-block-type EBNF details
```

### docs/technical/compile-rules/

```text
docs/technical/compile-rules/
├── INDEX.md                    Rule catalog entry
├── PGE/                        188 error-rule files (PGE01001 … PGE14xxx)
├── PGW/                         31 warning-rule files
└── algorithms/                  3 algorithms + INDEX
    ├── cycle-detection.md       Pipeline call cycle detection
    ├── overlap-detection.md     Match condition overlap detection
    └── compound-exhaustiveness.md Exhaustiveness on compound claims
```

PGE rule numbering is range-partitioned:

| Range | Topic |
|-------|-------|
| PGE01xxx | Pipeline structure and block elements |
| PGE02xxx | Variable lifecycle |
| PGE03xxx | Collectors and reassembly |
| PGE04xxx | Type system |
| PGE05xxx | Metadata paths |
| PGE06xxx | Conditional coverage |
| PGE07xxx | Error handling |
| PGE08xxx | Chains and references |
| PGE09xxx | (reserved) |
| PGE10xxx | Permissions and foreign-code compliance |
| PGE11xxx | Schemas and properties |
| PGE12xxx | Identifiers and aliases |
| PGE13xxx | (reserved) |
| PGE14xxx | Constructors |

### docs/technical/spec/

```text
docs/technical/spec/
├── behavior-contract.md        Signal-graph IR spec (compile target)
├── compiler-floor.md           Compiler/runtime boundary
├── type-identity.md            Structural type matching rules
├── native-dispatch.md          Native-code dispatch spec
├── native-config-example.yaml  Example native-dispatch config
├── job-sandbox.md              OS-level job sandbox spec
├── otel-foundation.md          OTel tracing foundation
├── otel-permission-events.md   OTel permission/sandbox events
├── otel-config.md              OTel exporter configuration
├── aljam3-sdk.md             Aljam3 SDK (encode/decode, call, pull, push)
├── collector-definitions.md    Collector-definition spec
└── metadata-tree/              Full % metadata-tree spec (11 files)
```

```text
docs/technical/spec/metadata-tree/
├── INDEX.md                    Metadata-tree entry
├── FULL-TREE.md                Canonical full-tree listing
├── branches.md                 Branch definitions
├── definition-templates.md     Templates
├── enum-rules.md               Enum-branch rules
├── field-expansion.md          Field expansion rules
├── instance-lifecycle.md       Instance lifecycle fields
├── io-ports.md                 IO-port branches
├── object-types.md             Object-type branches
├── path-grammar.md             Path grammar
└── string-subtypes.md          String-subtype tree
```

### docs/technical/algorithms/

```text
docs/technical/algorithms/
├── bridge-conversion.md        -Run.Bridge conversion algorithm
└── foreign-code-analysis.md    Foreign-code AST analysis algorithm
```

### docs/technical/compiler/

```text
docs/technical/compiler/
├── INDEX.md                    Compiler-module index
├── ast-invisible-registry.md   Banned AST-invisible function registry
├── compliance-report.md        Permission-compliance report format
├── foreign-code-parsers.md     Per-language foreign-code parser spec
└── io-registry.md              IO-sink registry for AST analysis
```

### docs/technical/edge-cases/

Edge-case files are split by EBNF section (matching `ebnf/01…16`). Each `NN-*.md` file enumerates accepted and rejected edge cases for that section. The `INDEX.md` lists all sections with counts.

### docs/technical/brainstorming/ and docs/technical/plan/

Work-in-progress content. Not authoritative. AI tools should not cite these paths as specification sources.

## .paul/ — Project Management

```text
.paul/
├── STATE.md                    Live project state — active issue, branch, handoffs
├── PROJECT.md                  Project constraints and goals
├── ROADMAP.md                  Milestone sequencing
├── SPECIAL-FLOWS.md            Specialised-flow routing
├── HANDOFF-*.md                Session handoffs (name varies by context)
├── handoffs/                   Archived handoffs
└── phases/                     Per-issue phase directories
```

Each phase directory under `.paul/phases/` has the name `issue-NNN-short-slug/` and contains:

```text
.paul/phases/issue-NNN-slug/
├── NN-PP-PLAN.md               Plan file (PLAN phase)
├── NN-PP-SUMMARY.md             Reconciliation summary (UNIFY phase)
└── …                           Additional plans as needed
```

## scripts/

```text
scripts/
├── docs-pdf-hook.sh            PDF generation hook
├── doc-template.typ            Typst document template
└── generate-docs-pdf.sh        PDF generation driver
```

## docs/archive/

Retired documents. Every archived file carries `replaced_by:` frontmatter pointing to current-spec replacements (`replaced_by: none` for files with no direct replacement). AI tools should not cite archived paths as authoritative; follow `replaced_by:` first.

## ASCII Character Conventions

Tree diagrams in this file use only the stable Unicode box-drawing characters:

| Char | Unicode | Purpose |
|------|---------|---------|
| `├──` | U+251C + U+2500 | Branch node |
| `└──` | U+2514 + U+2500 | Last branch at this level |
| `│`   | U+2502 | Vertical continuation |

These characters render consistently in terminals, Markdown viewers, PDF export, and most AI-tool surfaces. Do not substitute ASCII hyphens or other approximations.

## Related Documents

| Related | Purpose |
|---------|---------|
| [[INDEX\|u:INDEX]] | Human-oriented master index |
| [[ai-retrieval-index\|u:ai-retrieval-index]] | Query-shaped retrieval layer |
| [[component-inventory\|u:component-inventory]] | Flat categorical inventory |
| [[project-overview\|u:project-overview]] | Product-shaped overview |
| [[architecture\|u:architecture]] | Consolidated architecture read |
| [[development-guide\|u:development-guide]] | Contributor onboarding |
