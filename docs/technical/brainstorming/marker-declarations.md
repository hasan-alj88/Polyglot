# Marker Declaration Syntax — Examples for Review

**Status:** Draft for review
**Date:** 2026-03-31

### Issue Tracking

| Section | Issue |
|---|---|
| §0 Object Type Hierarchy | #107 |
| §0 Marker Groups / `[exe]` | #108 |
| Cross-cutting: `[T]`→`[T]` | #109 |
| §1-§2 Base Pipelines / #BaseCode | #110 |
| §4 Compiler Rules A-G | #111 |
| §5 `[C]` Inline Code | #112 |
| `{Q}` Dual-Purpose | #113 |
| Metadata Tree `%T` | #114 |
| §5 `=RT.*` Pipelines | #78 (existing) |
| §5 `=W.RT` Wrapper | #76 (existing) |
| §5 `!RT` Errors | #77 (existing) |

---

## 0. Object Type Hierarchy

Polyglot has two base object types. All others are subtypes:

```
{#} — Data definition (base)
 ├── {!} — Error definition (subtype of {#})
 ├── {M} — Macro definition (subtype of {#}, compile-time type generation)
 └── {Q} #Name — Queue data definition (subtype of {#}, uses #Queue schema)

{=} — Pipeline definition (base)
 ├── {T} =T.* — Trigger pipeline (subtype of {=}, equivalent to {=}[T])
 ├── {W} =W.* — Wrapper pipeline (subtype of {=}, equivalent to {=}[W])
 └── {Q} =Q.* — Queue pipeline operation (subtype of {=}, equivalent to {=}[Q])
```

**Key insight:** `{T}`, `{W}`, `{Q} =Q.*` are syntactic sugar for `{=}[T]`, `{=}[W]`, `{=}[Q]`.
`{Q}` is **dual-purpose** — the identifier prefix disambiguates:
- `{Q} #QueueName` → data definition (kind of `{#}`, uses `#Queue` schema)
- `{Q} =Q.*` → pipeline operation (kind of `{=}[Q]`)

Similarly, `{!}` and `{M}` are specialized `{#}` data types.

### Marker Groups

| Syntax | Equivalent | Markers |
|--------|-----------|---------|
| `{=}[exe]` | `{=}` (default) | Execution pipeline: invocable via `[r]`, `[p]`, `[b]` |
| `{T}` | `{=}[T]` | Trigger pipeline: invocable via `[T]` |
| `{Q} =Q.*` | `{=}[Q]` | Queue pipeline: invocable via `[Q]` |
| `{Q} #Name` | kind of `{#}` | Queue data: defines a queue instance schema |
| `{W}` | `{=}[W]` | Wrapper pipeline: invocable via `[W]` |

`[exe]` = `[rpb]` — the execution marker covering `[r]`, `[p]`, and `[b]`.

`{=}` without a marker defaults to `{=}[exe]`. A suppressible warning
(PGW-level) reminds authors to be explicit. Suppress file-wide with a
directive if desired.

---

## 1. Base Pipelines — Native Implementation

Base pipelines are implemented in the compiler's native language (Rust). They are
defined in stdlib `.pg` files with full interface (IO, errors, metadata) but
**no execution body**. A `[%] .baseCode` metadata line links to the native code.

### `#BaseCode` Enum

```polyglot
{#} #BaseCode
   [.] .Rust
      [.] .T
         [.] .Call
         [.] .Daily
         [.] .Webhook
         [.] .Folder
            [.] .NewFiles
      [.] .File
         [.] .Text
            [.] .Read
            [.] .Write
         [.] .Binary
            [.] .Read
      [.] .DB
         [.] .Connect
         [.] .Disconnect
         [.] .Query
         [.] .BeginTransaction
         [.] .Commit
         [.] .Rollback
      [.] .Math
         [.] .Add
         [.] .Subtract
         [.] .Multiply
         [.] .Divide
      [.] .DoNothing
      [.] .RT
         [.] .Python
            [.] .Script
            [.] .Function
            [.] .SetupEnv
            [.] .TeardownEnv
         [.] .JS
            [.] .Script
            [.] .Function
            [.] .SetupEnv
            [.] .TeardownEnv
         [.] .Shell
            [.] .Script
      [.] .Q
         [.] .Default
         [.] .Pause
            [.] .Hard
         [.] .Resume
         [.] .Kill
            [.] .Graceful
      [.] .W
         [.] .Polyglot
```

### Configuration

Polyglot config file selects the active base language:

```
base: Rust
```

The compiler validates that all `#BaseCode` references use the configured base
language. Future bases (e.g., `#BaseCode.Go.*`) can be added without changing
pipeline definitions — only the config and enum expand.

---

## 2. Stdlib Definitions

### Trigger Pipelines

Triggers are mostly base (compiler native). Users can compose derived triggers
from other triggers, or use `=RT.*` to bridge to codebase code.

All triggers must output `>IsTriggered#bool` (mandatory). They may also have
additional outputs that wire into the execution pipeline's inputs, supplying
data alongside the fire signal.

**Base triggers (bodyless):**

```polyglot
{T} =T.Call
   [%] .baseCode << #BaseCode.Rust.T.Call
   [=] >IsTriggered#bool

{T} =T.Daily
   [%] .baseCode << #BaseCode.Rust.T.Daily
   [=] <schedule#string
   [=] >IsTriggered#bool

{T} =T.Webhook
   [%] .baseCode << #BaseCode.Rust.T.Webhook
   [=] <endpoint#string
   [=] >IsTriggered#bool
   [=] >payload#serial

{T} =T.Folder.NewFiles
   [%] .baseCode << #BaseCode.Rust.T.Folder.NewFiles
   [=] <path#path
   [=] >IsTriggered#bool
   [=] >files#array:path
```

**Derived triggers (composed or =RT.* bridge):**

```polyglot
[ ] Composed from other triggers — AND semantics
{T} =T.DailyWebhookReady
   [T] =T.Daily"3AM"
   [T] =T.Webhook"/api/ready"
   [=] >IsTriggered#bool

[ ] =RT.* — bridge to codebase code for custom trigger logic
{T} =RT.Custom.PollExternalAPI
   [=] <endpoint#string
   [=] <interval#int
   [=] >IsTriggered#bool
   [=] >event#serial
```

### Execution Pipelines (base — bodyless)

```polyglot
{=}[exe] =File.Text.Read
   [%] .baseCode << #BaseCode.Rust.File.Text.Read
   [=] <path#path
   [=] >content#string
   [=] !File.NotFound
   [=] !File.PermissionDenied

{=}[exe] =DB.Query
   [%] .baseCode << #BaseCode.Rust.DB.Query
   [=] <conn#serial
   [=] <sql#string
   [=] >rows#array:serial
   [=] !DB.QueryFailed

{=}[exe] =Math.Add
   [%] .baseCode << #BaseCode.Rust.Math.Add
   [=] <a#int
   [=] <b#int
   [=] >sum#int

{=}[exe] =DoNothing
   [%] .baseCode << #BaseCode.Rust.DoNothing
```

### Queue Pipelines (base — bodyless)

```polyglot
{Q} =Q.Default
   [%] .baseCode << #BaseCode.Rust.Q.Default

{Q} =Q.Pause.Hard
   [%] .baseCode << #BaseCode.Rust.Q.Pause.Hard
   [=] <condition#string

{Q} =Q.Resume
   [%] .baseCode << #BaseCode.Rust.Q.Resume
   [=] <condition#string

{Q} =Q.Kill.Graceful
   [%] .baseCode << #BaseCode.Rust.Q.Kill.Graceful
   [=] <condition#string
```

### Wrapper Definitions

```polyglot
[ ] Base — bodyless, no setup/cleanup
{W} =W.Polyglot
   [%] .baseCode << #BaseCode.Rust.W.Polyglot

[ ] Derived — has body (setup/cleanup), no .baseCode
{W} =W.DB.Connection
   [{] $connectionString#string
   [}] $dbConn
   [\]
      [r] =DB.Connect
         [=] <connStr << $connectionString
         [=] >conn >> $dbConn
   [/]
      [r] =DB.Disconnect
         [=] <conn << $dbConn

[ ] Derived — composes =W.DB.Connection
{W} =W.DB.Transaction
   [{] $connectionString#string
   [}] $txHandle#string
   [\]
      [W] =W.DB.Connection
         [=] $connectionString << $connectionString
         [=] $dbConn >> $conn
      [r] =DB.BeginTransaction
         [=] <conn << $conn
         [=] >handle >> $txHandle
   [/]
      [r] =DB.Commit
         [=] <handle << $txHandle
```

---

## 3. User Pipeline (typical — derived)

```polyglot
{@} @Local:1000.InvoiceApp:v1.0.0

{=}[exe] =Invoice.Save
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connectionString << $dbConnStr
      [=] $txHandle >> $txHandle
   [=] <invoice#Invoice
   [=] >savedId#string
   [r] =DB.Insert
      [=] <conn << $txHandle
      [=] <data << $invoice
      [=] >id >> >savedId
```

---

## 4. Compiler Rules to Enforce

> **Resolved:** All rules below have PGE codes assigned. See compile-rules/PGE/ files.

### Rule A: Marker group must be valid for definition type
**PGE01029 — Invalid Marker for Definition Type**

| Definition | Valid markers | Notes |
|---|---|---|
| `{=}` | `[exe]` (default) | Execution pipeline |
| `{T}` | (implicit `[T]`) | `{T}` = `{=}[T]` |
| `{W}` | (implicit `[W]`) | `{W}` = `{=}[W]` |
| `{Q} =Q.*` | (implicit `[Q]`) | `{Q} =Q.*` = `{=}[Q]` |
| `{Q} #Name` | (none — data) | `{Q} #Name` = kind of `{#}` |
| `{#}` | (none — data, not callable) | `{!}`, `{M}` inherit this |

**VALID:**
```polyglot
{=}[exe] =MyPipeline
{T} =T.Custom
{Q} =Q.Custom
{W} =W.Custom
```

**INVALID:**
```polyglot
[ ] ✗ PGE01029 — {=} cannot declare [W] (use {W} instead)
{=}[W] =Bad.Pipeline

[ ] ✗ PGE01029 — {=} cannot declare [T] (use {T} instead)
{=}[T] =Bad.Trigger

[ ] ✗ PGE01029 — {W} cannot declare [exe] (it's already [W])
{W}[exe] =W.Bad

[ ] ✗ PGE01029 — {T} cannot declare [exe] (it's already [T])
{T}[exe] =T.Bad

[ ] ✗ PGE01029 — {#} cannot have markers
{#}[exe] #Bad

[ ] ✗ PGE01029 — {M} cannot have markers (subtype of {#})
{M}[T] #Bad

[ ] ✗ PGE01029 — {!} cannot have markers (subtype of {#})
{!}[exe] !Bad
```

### Rule B: Required elements per marker group
**PGE01005 (trigger), PGE01006 (queue), PGE01030 (wrapper)**

```polyglot
[ ] ✗ PGE01005 — {=}[exe] requires [T]
{=}[exe] =Bad.NoTrigger
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string

[ ] ✗ PGE01006 — {=}[exe] requires [Q]
{=}[exe] =Bad.NoQueue
   [T] =T.Call
   [W] =W.Polyglot
   [=] <input#string

[ ] ✗ PGE01030 — {=}[exe] requires [W]
{=}[exe] =Bad.NoWrapper
   [T] =T.Call
   [Q] =Q.Default
   [=] <input#string
```

```polyglot
[ ] ✓ — {T} with only IO (simple trigger)
{T} =T.Custom
   [=] <config#string
   [=] >IsTriggered#bool

[ ] ✓ — {T} with full body ([Q]/[W] optional)
{T} =T.Complex.SystemReady
   [Q] =Q.Default
   [W] =W.DB.Connection
      [=] $connectionString << "postgres://..."
      [=] $dbConn >> $dbConn
   [=] <config#string
   [=] >IsTriggered#bool
   [r] =DB.Query
      [=] <conn << $dbConn
      [=] <sql << "SELECT ready FROM system"
      [=] >rows >> $rows
```

```polyglot
[ ] ✓ — {Q} needs only IO, no [T]/[W]/body
{Q} =Q.Custom
   [=] <threshold#float
```

### Rule C: Forbidden elements per marker group
**PGE01031 — Forbidden Element in Definition**

> **Design decision:** `{T}` triggers may have execution body, `[Q]`, and `[W]`. These are optional.
> The only constraint unique to `{T}` is `>IsTriggered#bool` (PGE01032).

```polyglot
[ ] ✗ PGE01031 — {Q} cannot have [T]
{Q} =Q.Bad
   [T] =T.Call                     [ ] ✗ — queue ops don't have triggers

[ ] ✗ PGE01031 — {W} cannot have [T]
{W} =W.Bad
   [T] =T.Call                     [ ] ✗ — wrappers don't have triggers

[ ] ✗ PGE01031 — {W} cannot have [Q]
{W} =W.Bad
   [Q] =Q.Default                  [ ] ✗ — wrappers don't use queues

[ ] ✗ PGE01031 — {W} cannot have [=] pipeline IO
{W} =W.Bad
   [=] <input#string               [ ] ✗ — wrappers use [{]/[}], not [=] IO

[ ] ✗ PGE01031 — {Q} cannot have execution body
{Q} =Q.Bad
   [=] <threshold#float
   [r] =SomeWork                   [ ] ✗ — queue ops don't execute work

[ ] ✗ PGE01031 — {Q} cannot have [W]
{Q} =Q.Bad
   [W] =W.Polyglot                 [ ] ✗ — queue ops don't use wrappers
```

### Rule D: Invocation must match declaration
**PGE01024 — Incompatible Operation Marker** (existing)

```polyglot
{T} =T.Custom
   [=] <config#string
   [=] >IsTriggered#bool

{=}[exe] =Worker
   [=] <data#string
   [=] >result#string

[ ] ✓ — [T] invokes a {T}-declared pipeline
{=}[exe] =MyPipeline
   [T] =T.Custom
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Worker
      [=] <data << $input
      [=] >result >> >output
```

```polyglot
[ ] ✗ PGE01024 — [r] invokes a {T}-declared pipeline
[r] =T.Custom

[ ] ✗ PGE01024 — [T] invokes an {=}[exe]-declared pipeline
[T] =Worker

[ ] ✗ PGE01024 — [p] invokes a {T}-declared pipeline
[p] =T.Daily
```

### Rule E: Implicit marker defaults
**No PGE code — compiler behavior only** (decided in #108: no warning)

```polyglot
[ ] ✓ — {=} without marker defaults to {=}[exe], no diagnostic emitted
{=} =Implicit.Exe
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork

[ ] ✗ — {T} is NOT the default, must use {T} explicitly
{=} =Bad.TriggerWithoutMarker
   [=] <schedule#string
   [ ] ✗ — parsed as {=}[exe], but has no [T]/[Q]/[W] — Rule B fires
```

Note: `{T}`, `{W}`, `{Q}` do NOT need extra markers — they already imply `[T]`, `[W]`, `[Q]`.
Note: `{=}` defaults to `{=}[exe]` — only execution gets the implicit default.

### Rule F: Base vs Derived pipeline constraints

```polyglot
[ ] ✗ — base pipeline cannot have execution body
{=}[exe] =Bad.BaseWithBody
   [%] .baseCode << #BaseCode.Rust.File.Text.Read
   [=] <path#path
   [=] >content#string
   [r] =SomeWork                   [ ] ✗ — base pipelines are bodyless

[ ] ✗ — derived pipeline cannot have .baseCode
{=}[exe] =Bad.DerivedWithBase
   [%] .baseCode << #BaseCode.Rust.DoNothing
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoSomething                [ ] ✗ — has body, cannot also be base

[ ] ✗ — derived {W} wrapper cannot have .baseCode
{W} =W.Bad.DerivedWithBase
   [%] .baseCode << #BaseCode.Rust.W.Polyglot
   [{] $input#string
   [\]
      [r] =DoNothing               [ ] ✗ — has body, cannot also be base

[ ] ✗ — base wrapper cannot have [{]/[}]/[\]/[/]
{W} =W.Bad.BaseWithBody
   [%] .baseCode << #BaseCode.Rust.W.Polyglot
   [{] $input#string               [ ] ✗ — base wrappers are bodyless

[ ] ✗ — .baseCode references non-existent variant
{=}[exe] =Bad.InvalidBase
   [%] .baseCode << #BaseCode.Rust.NotARealThing
   [=] <x#string                   [ ] ✗ — no such #BaseCode variant

[ ] ✗ — .baseCode uses wrong base language
{=}[exe] =Bad.WrongBase
   [%] .baseCode << #BaseCode.Go.File.Text.Read
   [=] <path#path                  [ ] ✗ — config says base: Rust
```

### Rule G: Trigger output constraint
**PGE01032 — Missing Trigger Boolean Output**

`>IsTriggered#bool` is mandatory. Additional outputs are allowed — they wire
into the execution pipeline's inputs, supplying data alongside the fire signal.

```polyglot
[ ] ✓ — {T} with mandatory output + additional data outputs
{T} =T.Good.WithData
   [=] <config#string
   [=] >IsTriggered#bool
   [=] >payload#serial

[ ] ✓ — trigger with body still needs >IsTriggered#bool
{T} =T.WithBody
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >IsTriggered#bool
   [r] =CheckCondition
      [=] >ready >> >IsTriggered

[ ] ✗ PGE01032 — {T} missing >IsTriggered#bool
{T} =T.Bad.NoOutput
   [=] <config#string
   [=] >payload#serial

[ ] ✗ PGE01032 — >IsTriggered must be #bool, found #string
{T} =T.Bad.WrongType
   [=] >IsTriggered#string
```

---

## 5. Foreign Language Execution — `=RT.*` and Language Wrappers

Polyglot executes foreign code (Python, JS, Shell, etc.) through two mechanisms:
- **Language wrappers** (`{W} =W.<Language>.<Version>`) — set up the runtime environment
- **Runtime pipelines** (`=RT.<Language>.*`) — execute code within that environment

### `=RT.*` Patterns

Two standard patterns per language:

```polyglot
[ ] Run an external script file
{=}[exe] =RT.Python.Script
   [%] .baseCode << #BaseCode.Rust.RT.Python.Script
   [=] <script#path
   [=] <env#Code.Environment
   [=] >stdout#string
   [=] >stderr#string
   [=] >exitCode#int

[ ] Call a specific function from a file
{=}[exe] =RT.Python.Function
   [%] .baseCode << #BaseCode.Rust.RT.Python.Function
   [=] <file#path
   [=] <function#string
   [=] <env#Code.Environment
   [=] <args#array:serial
   [=] <kwargs#map:serial
   [=] >stdout#string
   [=] >stderr#string
   [=] >returnValue#serial
```

Same pattern for other languages: `=RT.JS.Script`, `=RT.Shell.Script`, etc.

### Language Wrapper Definition

```polyglot
[ ] Base — sets up Python 3.19 environment
{W} =W.Python.3.19
   [{] $dependency#path
   [}] $env#Code.Environment
   [\]
      [r] =RT.Python.SetupEnv
         [=] <version << "3.19"
         [=] <dependency << $dependency
         [=] >env >> $env
   [/]
      [r] =RT.Python.TeardownEnv
         [=] <env << $env
```

### `[C]` Inline Code Marker

For inline foreign code within a pipeline, `[C]` carries code lines passed to `=RT.*`:

```polyglot
[r] =RT.Python.Script
   [=] <env << $env
   [=] <script <<
      [C] with open(path, 'w') as f:
      [C]     f.write('Hello World from Python')
   [=] >stdout >> $output
```

`[C]` is a block element (not a block type). Each `[C]` line is one line of foreign code.
The `<script` input accepts either a `#path` to a file OR inline `[C]` lines.

### Hello World — Full Example

```polyglot
{@} @Local:001.HelloWorld:v1.0.0

[ ] Requirements file for Python dependencies
{#} #HelloConfig
   [.] .pythonDeps#path ~> "./requirements.txt"

{=}[exe] =HelloWorld.Python
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Python.3.19
      [=] $dependency << $config.pythonDeps
      [=] $env >> $env
   [=] <config#HelloConfig
   [=] >greeting#string ~> ""

   [ ] Run external Python script
   [r] =RT.Python.Script
      [=] <env << $env
      [=] <script << "{$config.pythonDeps:directory}/hello.py"
      [=] >stdout >> >greeting
      [!] !RT.ScriptFailed
         [r] >greeting << "Hello World (fallback)"

{=}[exe] =HelloWorld.Python.Inline
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Python.3.19
      [=] $dependency << "./requirements.txt"
      [=] $env >> $env
   [=] >greeting#string ~> ""

   [ ] Inline Python code via [C] marker
   [r] =RT.Python.Script
      [=] <env << $env
      [=] <script <<
         [C] import sys
         [C] print("Hello World from Python " + sys.version)
      [=] >stdout >> >greeting
      [!] !RT.ScriptFailed
         [r] >greeting << "Hello World (fallback)"

{=}[exe] =HelloWorld.Python.Function
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Python.3.19
      [=] $dependency << "./requirements.txt"
      [=] $env >> $env
   [=] <name#string
   [=] >greeting#string ~> ""

   [ ] Call a specific function from a Python file
   [r] =RT.Python.Function
      [=] <env << $env
      [=] <file << "./greetings.py"
      [=] <function << "greet"
      [=] <args << [$name]
      [=] <kwargs << {}
      [=] >returnValue >> >greeting
      [!] !RT.FunctionFailed
         [r] >greeting << "Hello {$name} (fallback)"
```

Three variants:
1. **Script** — runs an external `.py` file
2. **Inline** — embeds Python code directly via `[C]` lines
3. **Function** — calls a specific function with args/kwargs

---

## 6. Resolved Questions

1. **Zero-IO wrappers** — valid for both base and derived. `=W.Polyglot` (base) and user wrappers with no `[{]`/`[}]` are both allowed.
2. **`#BaseCode` location** — lives in stdlib.
3. **`=RT.*`** — RT = RunTime. Bridges to other programming languages and shell/bash/cmd commands. Available for `{=}[exe]` pipelines.
4. **`{Q} #QueueName`** — defines queue behavior and configuration data (how the queue behaves, its settings).
5. **`=RT.*` patterns** — two standard patterns per language: `=RT.<Lang>.Script` (run file) and `=RT.<Lang>.Function` (call function). Both are `{=}[exe]` base pipelines backed by native code.
6. **`[C]` inline code** — block element (not block type) for embedding foreign code lines. Passed as input to `=RT.*` pipelines.

## 7. Open Questions

1. **{M} macros** — do they need markers? They're invoked via `[M]` inside `{#}`. As a `{#}` subtype, they inherit "no markers." But could be `{M}[M]` for consistency.
2. **File-wide warning suppression** — what syntax for suppressing the `{=}` implicit `[exe]` warning?
