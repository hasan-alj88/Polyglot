---
audience: pg-coder
type: specification
updated: 2026-04-05
status: complete
---

# Permissions

<!-- @identifiers -->
<!-- @blocks -->
Polyglot uses an implicit-deny permission system. Every pipeline starts with zero IO capabilities. To perform any IO — read a file, make a web request, access a database — the package or pipeline must reference a named `{_}` permission object. The `{_}` definition block and `[_]` block element are registered in [[blocks#Permissions]].

This follows the Cisco ACL model: if you don't explicitly allow it, it's denied.

## Implicit Deny

A pipeline with no `[_]` references is **pure computation** — it can transform data, run conditionals, and call other pipelines, but cannot touch the outside world. Any IO call without a matching permission is a compile error.

```polyglot
{-} PureComputation
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] no [_] lines — this pipeline cannot do IO
   [-] $result#int << -Math.Add $a $b
```

## {_} Permission Objects

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

### Intent: Ceiling vs Grant

Every `{_}` object declares an `.intent` field — either `#Ceiling` or `#Grant`:

| Intent | Purpose | Scope values |
|--------|---------|--------------|
| `#Ceiling` | Maximum permissions a package **allows** | Glob patterns permitted (`"data/*.csv"`, `"/var/log/*"`) |
| `#Grant` | Specific permissions a pipeline **requests** | Narrow, specific values only (`"data/reports/q1.csv"`) |

- **Ceiling** — referenced by `{@}` packages via `[_]`. Sets the upper bound. Allows but does not grant.
- **Grant** — referenced by `{-}` pipelines via `[_]`. Requests specific capabilities within the ceiling.
- **Compiler validates: Grant must be a subset of Ceiling** — every grant value must fall within a ceiling pattern. A grant requesting `"data/secret.csv"` when the ceiling only allows `"data/reports/*"` is a compile error (PGE10001).
- **Narrowing allowed, expanding NOT** — a grant can request less than the ceiling allows, but never more.

### Fully Filled Requirement

Every `{_}` object must be **fully filled** — every leaf field must have a value assigned. Empty leaves are a compile error. This prevents accidental "allow everything" policies from incomplete declarations.

### No Inline Declarations

`[_]` in `{@}` and `{-}` always references a `{_}` object **by name** — no inline permission declarations. All permission policies are defined as standalone `{_}` blocks.

```polyglot
{ } VALID — reference by name
[_] _DataCeiling

{ } INVALID — no inline declarations
[_] _File.read"/var/log/*"
```

## Tier System: _ / __ / ___

Permission identifiers use a three-tier prefix system that mirrors the `#` / `##` / `###` data tier:

| Prefix | Tier | Purpose | Parallel |
|--------|------|---------|----------|
| `_` | Object | Named permission policy instance | `#` (struct instance) |
| `__` | Descriptor | Permission schema definition | `##` (schema descriptor) |
| `___` | Constraint | Leaf-level constraint descriptors | `###` (field constraint) |

### _ Permission Objects

Named, reusable permission policies. Each `_` object holds concrete capability declarations with an intent (Ceiling or Grant).

```polyglot
{_} _WebAccess
   [.] .intent << #Ceiling
   [.] .Web.Request "https://api.example.com/*"
   [.] .Web.Socket "wss://stream.example.com/*"
```

### __ Permission Descriptors (Generic Permissions)

`__` descriptors are **generic permission templates** — they mirror `##` schemas with `[#]` inputs. A `__` descriptor takes parameters and produces a concrete `_` permission object at compile time, serving as syntax sugar so you don't have to write the full permission schema for common patterns.

```polyglot
{_} __FileReader
   [#] <path;path

   [.] .intent << #Grant
   [.] .File.Read "{$path}"

[ ] Usage — produces a concrete _ object at compile time
[_] __FileReader "data/reports/q1.csv"
```

Without `__FileReader`, you would write the full `{_}` block each time you need file read permission for a different path. The generic descriptor eliminates this repetition.

**Compile-time resolution:** All generic permissions are fully resolved at compile time. The resulting `_` object has all leaves in Final or Default state. When a Default leaf is pulled, it transitions to Final (see [[variable-lifecycle#Default]]). No runtime permission evaluation occurs — the compiler validates all grants against ceilings statically.

### ___ Constraint Descriptors

`___` descriptors are **leaf-level constraint templates** — they mirror `###` field types. They restrict permission behavior based on environment or policy, and like `__` descriptors, resolve entirely at compile time.

Examples:

- `___Unix` — Unix-specific permission constraints (file modes, signals)
- `___Sandboxed` — sandboxed environment restrictions
- `___ReadOnly` — read-only access constraint

## Per-Category Capability Enums

Permission categories use typed enums to prevent nonsensical combinations. Each category has its own capability enum:

| Category | Enum | Capabilities |
|----------|------|-------------|
| `File` | `#FileCapability` | `Read`, `Write`, `Execute`, `Delete`, `Create` |
| `Web` | `#WebCapability` | `Request`, `Socket`, `Listen` |
| `Database` | `#DatabaseCapability` | `Connect`, `Read`, `Write` |
| `System` | `#SystemCapability` | `Env`, `Process`, `Signal` |
| `Crypto` | `#CryptoCapability` | `Key`, `Sign`, `Encrypt` |
| `IPC` | `#IPCCapability` | `Send`, `Receive`, `Subscribe` |
| `Device` | `#DeviceCapability` | `Camera`, `Microphone`, `Location`, `Bluetooth` |
| `Memory` | `#MemoryCapability` | `Allocate`, `Shared` |

Eight predefined categories cover all IO capabilities. Categories and capabilities are Polyglot-defined — not user-extensible. The enum type prevents invalid combinations like `.Device.Read` (devices don't have a `Read` capability).

## Hierarchical Scoping

Permissions operate at two levels: **package ceiling** and **pipeline grant**.

### Package Ceiling

`[_]` lines in `{@}` reference `{_}` ceiling objects, setting the maximum permissions any definition in the package may request. The package ceiling **allows but does not grant** — no definition inherits permissions automatically. See [[packages#Permissions]] for the full ceiling syntax and compile rules (PGE10001, PGE10002).

```polyglot
{_} _LogAnalyzerCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "/var/log/*"
   [.] .File.Write "/tmp/reports/*"
   [.] .Web.Request "https://alerts.internal/*"
   [.] .System.Env "LOG_LEVEL"

{@} LogAnalyzer
   [_] _LogAnalyzerCeiling
```

### Pipeline Grant

Each `{-}` pipeline must explicitly reference `{_}` grant objects for the permissions it needs. Grants can only **narrow** what the package ceiling allows — never widen. See [[concepts/pipelines/permissions#Permissions]] for placement within pipeline definitions.

```polyglot
{_} _LogFileGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*.log"

{_} _AlertGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*.log"
   [.] .Web.Request "https://alerts.internal/notify"

{-} -ProcessLogs
   [_] _LogFileGrant
   [ ] narrower than ceiling — granted
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   ...

{-} -ComputeStats
   [ ] no [_] lines — pure computation, zero IO
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   ...

{-} -SneakyPipeline
   [_] _AlertGrant
   [ ] grant references Web.Request — valid only if ceiling includes it
   ...
```

### No Inheritance

Permissions are never inherited. Every definition must reference the `{_}` grant objects it requires, even if the package ceiling allows them. This makes each definition's IO footprint explicit and auditable.

## No Instances

Permissions are **compile-time declarations** — they apply across all instances of a pipeline. There are no per-instance permissions. If `-ProcessLogs` has `[_] _LogFileGrant`, every instance of `-ProcessLogs` shares that grant. The `%_` metadata tree branch has no `:{instance}` level (see [[data-is-trees]]).

## __Permission Schema

The `__Permission` descriptor defines the full structure of a permission object:

```polyglot
__Permission
+-- .target                    __PermissionTarget
|   +-- .category              #PermissionCategory     <- enum: File, Web, Database, System, Crypto, IPC, Device, Memory
|   +-- .capability            #Capability             <- per-category enum
|   +-- .scope                 __PermissionScope
|       +-- .pattern           #GlobPattern (ceiling) or specific value (grant)
|       +-- .direction         #IODirection            <- enum: Inbound, Outbound, Both
+-- .grant                     __PermissionGrant
|   +-- .level                 #AccessLevel            <- enum: Allow, Deny
|   +-- .authority             #GrantAuthority         <- enum: Package, Pipeline
|   +-- .intent                #PermissionIntent       <- enum: Ceiling, Grant
+-- .resource                  __ResourceDescriptor
|   +-- .os                    #OSTarget               <- enum: Any, Linux, Windows, MacOS
|   +-- .protocol              #Protocol               <- enum: File, TCP, UDP, HTTPS, IPC, SharedMemory, USB, Bluetooth
|   +-- .handle                #HandleKind             <- enum: Path, ConnectionString, Descriptor, Address, DeviceID
+-- .audit                     __PermissionAudit
    +-- .log                   #AuditLevel             <- enum: None, OnUse, OnDeny, All
    +-- .alert                 #AlertLevel             <- enum: None, OnDeny, OnEscalation
```

All fields use `.` fixed-field navigation — permission schemas are Polyglot-defined, not user-extensible.

## Compile-Time Enforcement

All permission checks are **static analysis** — resolved at compile time, not runtime. The compiler verifies:

1. **Grant within ceiling** — every `[_]` grant in a `{-}` must reference a `{_}` object whose capabilities fall within the `{@}` package ceiling (PGE10001)
2. **Import ceiling compatibility** — imported package ceilings must fall within the importer's ceiling (PGE10002)
3. **Pure computation enforced** — any IO call in a pipeline with no `[_]` lines is a compile error
4. **Fully filled** — every `{_}` object must have all leaf fields assigned (no empty leaves)
5. **Intent validation** — `#Ceiling` objects may use glob patterns; `#Grant` objects must use specific narrow values

No runtime permission checks exist. If it compiles, the permissions are satisfied.

## File Ordering

`{@}` must appear first in every `.pg` file (compiler-enforced). The recommended stylistic ordering for the remaining blocks is:

```polyglot
{@}   <- mandatory first (compiler-enforced)
{_}   <- permission objects (recommended second)
{#}   <- data definitions
{-}   <- pipelines
```

## Complete Example

A full package showing the ceiling-to-grant flow:

```polyglot
{ } Package declaration
{@} @Local:999::DataProcessor:v1.0.0
   [_] _DataCeiling

{ } Permission objects
{_} _DataCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "data/*.csv"
   [.] .File.Write "output/*.json"
   [.] .Database.Read "*.postgres"

{_} _ReportReader
   [.] .intent << #Grant
   [.] .File.Read "data/reports/q1.csv"
   [.] .File.Read "data/reports/q2.csv"

{_} _OutputWriter
   [.] .intent << #Grant
   [.] .File.Read "data/reports/q1.csv"
   [.] .File.Write "output/summary.json"

{ } Data definitions
{#} #Report
   [.] .name#string
   [.] .rows#int

{ } Pipelines
{-} -ReadReports
   [_] _ReportReader
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >reports#array.Report
   [-] >reports << -File.Serial.CSV.Parse "data/reports/q1.csv"

{-} -WriteOutput
   [_] _OutputWriter
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#Report
   (-) >result#string
   [-] >result << -File.Serial.JSON.Serialize $data >> "output/summary.json"
```

## Foreign Code

<!-- @blocks#Foreign Code -->
Pipelines using `[C]` foreign code blocks ([[blocks#Foreign Code]]) interact with permissions as follows:

- The pipeline must reference `[_]` permission objects for the IO the foreign code will perform
- The **compiler issues a warning** (not an error) that foreign code cannot be statically verified against declared permissions
- The **programmer takes responsibility** for ensuring the foreign code stays within declared permissions
- The **foreign runtime** (Python, Node, etc.) handles its own enforcement mechanisms if any

```polyglot
{_} _AnalyzeGrant
   [.] .intent << #Grant
   [.] .File.Read "data/report.csv"

{-} -AnalyzeData
   [_] _AnalyzeGrant
   [ ] compiler warning: [C] block cannot be statically verified
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -RT.Python.Script
      (-) <env << $env
      (-) <script <<
         [C] import pandas as pd
         [C] df = pd.read_csv("data/report.csv")
         [C] result = df.describe()
      (-) >stdout >> $output
```
