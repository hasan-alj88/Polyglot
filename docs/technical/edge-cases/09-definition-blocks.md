---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 9. Definition Blocks (S9)

### EC-9.1: Package with multiple imports

<!-- @packages -->
**EBNF:** `package_block ::= "{@}" package_id NEWLINE { indent import_line NEWLINE }`

**What it tests:** Multiple `[@]` imports in one package block. See [[packages]].

```polyglot
{@} @Local:001.App:v1.0.0
   [@] @AD << @Local:001.ActiveDirectory:v2.0.0
   [@] @Mail << @Local:001.EmailSystem:v1.2.0
   [@] @HR << @Local:001.HRSystem:v2.1.0
```

### EC-9.2: Enum fields — pure enum (no value sub-fields)

<!-- @types:Enum Fields -->
**EBNF:** `enum_field ::= "[.]" fixed_sep name`

**What it tests:** All-enum siblings, no `#type`, no assignment. See [[syntax/types/structs#Enum Fields vs Value Fields]].

```polyglot
{#} #Direction
   [.] .North
   [.] .South
   [.] .East
   [.] .West
```

### EC-9.3: Enum field with nested value sub-fields

**EBNF:** `enum_field ::= "[.]" fixed_sep name NEWLINE { indent data_field NEWLINE }`

**What it tests:** Enum variant carrying typed data underneath.

```polyglot
{#} #Status
   [.] .Failed
      [.] .reason#string <~ "unknown"
      [.] .retries#int <~ 0
   [.] .Success
```

### EC-9.4: Value field data definition — all siblings assigned

<!-- @identifiers:Serialization Rules -->
**EBNF:** `value_field ::= "[.]" fixed_sep name type_annotation [ assignment_op value_expr ]`

**What it tests:** All-or-none assignment rule — all siblings have defaults. See [[identifiers#Serialization Rules]].

```polyglot
{#} #Config
   [.] .timeout#int <~ 30
   [.] .retries#int <~ 3
   [.] .verbose#bool <~ #Boolean.False
```

### EC-9.5: Flexible-field data definition

**EBNF:** `flex_data_field ::= "[:]" flex_sep name type_annotation [ assignment_op value_expr ]`

**What it tests:** `[:]` with `:` separator for open-schema data.

```polyglot
{#} #Metadata
   [:] :author#string <~ ""
   [:] :version#string <~ "0.0.0"
```

### EC-9.6: Pipeline — mandatory structure ordering

<!-- @pipelines -->
**EBNF:** `pipeline_body ::= trigger_section [ io_section ] [ queue_section ] wrapper_section execution_section`

**What it tests:** Correct order: trigger -> IO -> queue -> wrapper -> execution. See [[concepts/pipelines/INDEX|pipelines]].

```polyglot
{=} =Ordered
   [T] =T.Call
   [=] <input#string
   [=] >output#string ~> ""
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] >output << <input
```

### EC-9.7: Pipeline — minimal (no IO, no queue)

**What it tests:** IO and Queue are optional per EBNF.

```polyglot
{=} =Minimal
   [T] =T.Call
   [W] =W.Polyglot
   [r] $x#int << 1
```

### EC-9.8: Trigger with string argument

**EBNF:** `trigger_ref ::= pipeline_ref [ string_literal ]`

**What it tests:** Triggers that take configuration strings.

```polyglot
[T] =T.Daily"3AM"
[T] =T.Webhook"/api/onboarding"
[T] =T.Folder.NewFiles"/inbox/"
```

### EC-9.9: IO as implicit triggers — all three modes

<!-- @pipelines:IO as Implicit Triggers -->
**What it tests:** Constant, default, and required IO. See [[concepts/pipelines/io-triggers#IO as Implicit Triggers]].

```polyglot
[=] <constant#string << "locked"
[=] <fallback#string <~ "default"
[=] <required#string
```

### EC-9.10: `[p]` in `[\]` — parallel fork outlives setup, collected in `[/]`

<!-- @pipelines:Parallel Forking in Setup -->
**EBNF:** `scope_setup ::= "[\]" NEWLINE { indent exec_line NEWLINE }` — `exec_line` includes `parallel_line`.

**What it tests:** `[p]` at end of `[\]` with no `[*] *All` — forked path runs concurrently with body; `[/]` collects via `[*] *All` with `[*] <<` wait inputs. See [[concepts/pipelines/wrappers#Parallel Forking in Setup]].

```polyglot
{W} =W.Tracing
   [{] $traceId#string
   [}] $duration#string
   [\]
      [r] =Tracer.Open
         [=] <id << $traceId
         [=] >session >> $session
      [ ] No *All after [p] — timer runs concurrently with body
      [p] =Tracer.StartTimer
         [=] <session << $session
         [=] >handle >> $timerHandle
   [/]
      [*] *All
         [*] << $timerHandle
      [r] =Tracer.StopTimer
         [=] <handle << $timerHandle
         [=] >elapsed >> $duration
      [r] =Tracer.Close
         [=] <session << $session
```

### EC-9.11: `[b]` in `[\]` — fire-and-forget, no collection in `[/]`

**EBNF:** `scope_setup ::= "[\]" NEWLINE { indent exec_line NEWLINE }` — `exec_line` includes `background_line`.

**What it tests:** `[b]` in setup fires and is never collected — no `[*] *All` in `[/]` for it.

```polyglot
{W} =W.AuditLog
   [{] $userId#string
   [\]
      [r] =Session.Open
         [=] <id << $userId
         [=] >session >> $session
      [ ] Fire audit event — no result needed, no collection
      [b] =Audit.LogEntry
         [=] <userId << $userId
   [/]
      [r] =Session.Close
         [=] <session << $session
```

### EC-9.12: Empty pipeline body — no sections at all

<!-- @pipelines -->
**EBNF ref:** `pipeline_body` — trigger, queue, wrapper, execution sections
**What it tests:** A `{=}` with no content. PGE01005 (missing trigger) fires first. See [[concepts/pipelines/INDEX|pipelines]].

```polyglot
[ ] ✗ PGE01005 — empty pipeline has no trigger
{=} =EmptyPipeline
```

### EC-9.13: Empty `{#}` data definition — no fields

**EBNF ref:** `data_def` — requires at least one `data_body_line`
**What it tests:** A `{#}` with no fields. PGE01021 fires. See [[syntax/types/INDEX|types]].

```polyglot
[ ] ✗ PGE01021 — no fields
{#} #EmptyRecord
```

### EC-9.14: Empty `{!}` error namespace — no leaves

**EBNF ref:** `error_def` — requires at least one `error_leaf_line`
**What it tests:** A `{!}` with no error leaves. PGE01022 fires. See [[concepts/errors|errors]].

```polyglot
[ ] ✗ PGE01022 — no error leaves
{!} !EmptyErrors
```

### EC-9.15: File with only comments after `{@}`

**EBNF ref:** `file ::= package_block { definition }`
**What it tests:** A file that declares a package but defines nothing. PGW01003 fires. See [[syntax/packages|packages]].

```polyglot
[ ] ⚠ PGW01003 — no definitions in file
{@} @Local:999.EmptyPackage:v1.0.0
{ } This file defines nothing
```

### EC-9.16: Non-trigger pipeline used as `[T]` trigger

<!-- @pipelines:Triggers -->
**EBNF ref:** `trigger_ref ::= pipeline_ref [ string_literal ]`
**What it tests:** Using a non-trigger operation (e.g., `=File.Text.Read`) with `[T]`. PGE01024 fires — operations declare allowed markers. See [[concepts/pipelines/INDEX|pipelines]].

```polyglot
[ ] ✗ PGE01024 — =File.Text.Read is not trigger-compatible
{=} =Bad
   [T] =File.Text.Read
```

### EC-9.17: Multiple `[T]` trigger lines — AND semantics

<!-- @pipelines:Triggers -->
**EBNF ref:** `trigger_io_section` — allows multiple `trigger_line` via `{ }` repetition
**What it tests:** Multiple `[T]` lines use AND semantics — all triggers must fire. For OR, use `[|]`. See [[concepts/pipelines/INDEX|pipelines]].

```polyglot
[ ] ✓ AND — both triggers must fire
{=} =DualTrigger
   [T] =T.Daily"3AM"
   [T] =T.Webhook"/api/ready"
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork
```

### EC-9.18: Duplicate `[Q]` or `[W]` sections

**EBNF ref:** `pipeline_body ::= ... queue_section wrapper_section ...`
**What it tests:** Grammar enforces exactly one `[Q]` and one `[W]`. Duplicate sections are a parse error.

```polyglot
[ ] ✗ parse error — grammar requires single [Q] and [W]
{=} =DuplicateQueue
   [T] =T.Call
   [Q] =Q.Default
   [Q] =Q.Custom
   [W] =W.Polyglot
```

### EC-9.19: Discard `$*` in wrapper IO wiring

<!-- @pipelines:Wrappers -->
**EBNF ref:** `wrapper_io_line ::= "[=]" variable_id assignment_op value_expr`
**What it tests:** Using `$*` (discard) in wrapper IO defeats the purpose. PGE01025 fires. See [[concepts/pipelines/wrappers|wrappers]].

```polyglot
[ ] ✗ PGE01025 — $* discards wrapper input
[W] =W.DB.Connection
   [=] $* << $connectionString
```
