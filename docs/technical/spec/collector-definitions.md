---
audience: design
type: spec
updated: 2026-04-11
status: draft
---

<!-- @c:glossary#Reconciliation -->
<!-- @c:glossary#Trigger Monitor -->
<!-- @u:technical/ebnf/16-collector-definitions -->
<!-- @u:concepts/collections/collect -->
<!-- @u:technical/plan/queue-manager/overflow -->

# Collector Definitions (`{*}`)

`{*}` is the 10th definition block type. It makes collectors ([[glossary#Reconciliation|c:Reconciliation]]) a first-class definable entity — users can define custom collector logic, not just invoke jm3lib-native operators.

`{N}` remains for native pipelines. `{*}` is exclusively for collector definitions. Existing jm3lib collectors (`*First`, `*All`, etc.) are rewritten as `{*}` blocks — the collector logic lives in `{*}`, while the underlying job management and queue handling remain native.

`{*}` creates a branch on the `%` metadata tree at `%*`.

## Ground Rules

These axioms make `{*}` fundamentally different from other definition blocks.

### Ground Rule 1: Inputs Do NOT Require Final State

Unlike `{-}` pipelines where all inputs must reach Final before the pipeline triggers, `{*}` collector inputs can be **in-progress**. The collector observes variables as they arrive — a non-Final input tells the collector the associated job is still running.

The collector's execution is **continuous** — it runs as a [[glossary#Trigger Monitor|c:Trigger Monitor]] program from the moment the first `(*)` variable is registered, not as a one-shot triggered pipeline.

### Ground Rule 2: Arrival Order Is Axiomatic

**Axiom:** Every incoming job result has a deterministic arrival position, even when jobs complete simultaneously. The system assigns a total order — there is no "tie." This is the foundation of all collector addressing.

### Ground Rule 3: Dual-Path Access — Variable-Centric and Job-Centric

Collectors have two parallel access paths because a single job can produce multiple variables, and a single variable comes from exactly one job:

**Variable-centric: `*Arrive`** — what showed up

| Operation | Description |
|-----------|-------------|
| `*Arrive"0"` | The first arrived variable (0-indexed) |
| `*Arrive"{$n}"` | The nth arrived variable (interpolation) |
| `*Arrive"[0,N]"` | ALL arrived variables (N = total count) |
| `*Arrive.Job.Release"0"` | Release the job that produced this arrival |
| `*Arrive.Job.Release"[1,N]"` | Release jobs for a range of arrivals |

**Job-centric: `*Job.Arrive`** — which job completed

| Operation | Description |
|-----------|-------------|
| `*Job.Arrive"0"` | The first arrived job (0-indexed) |
| `*Job.Arrive"[0,N]"` | ALL arrived jobs |
| `*Job.Arrive.Vars"0"` | All variables produced by this job |
| `*Job.Release"0"` | Release this job |
| `*Job.Release"[0,N]"` | Release all jobs |

**`N` is special:** Resolves to the total count (of variables for `*Arrive`, of jobs for `*Job.Arrive`). Known at collector registration time.

### Ground Rule 4: Arrivals ARE Triggers — `[T]` Mini-Pipelines

`{*}` blocks have **no external trigger declarations** (no `=T.Call`, `=T.Folder.NewFiles`, etc.). The triggers are arrivals, and each `[T]` block is a **mini-pipeline** with its own IO:

```aljam3
[T] *Arrive"0"
   (T) >var       (* the arrived variable — becomes $var *)
   (T) >job       (* the associated job — becomes $job *)
   >> >winner << $var.value
   [*] *Job.Release"[0,N]"
```

**`(T)` IO declarations** make the arrived data available as proper `$` variables inside the trigger block:

| `(T)` Output | Type | Description |
|--------------|------|-------------|
| `(T) >var` | `#Record` | The arrived variable (.value, .variable, .status) |
| `(T) >job` | `#Job` | The associated job (.uid, .host, .queue, .status, .startTime, .duration) |

For **range triggers**, the outputs are collections processable via `=ForEach`:

```aljam3
[T] *Arrive"[0,N]"
   (T) >vars      (* all arrived variables — iterable *)
   (T) >jobs       (* all associated jobs — iterable *)
   [-] =ForEach.Dataframe << $vars
      [~] >row
      [-] $accumulator <~ $accumulator + $row.value
   >> >sum << $accumulator
   [*] *Job.Release"[0,N]"
```

For **job-centric triggers**:

```aljam3
[T] *Job.Arrive"0"
   (T) >job        (* the arrived job *)
   (T) >vars       (* all variables from this job *)
   (* process job and its variables *)
   [*] *Job.Release"[0,N]"
```

**`[T]` trigger patterns:**

| Pattern | Fires when | `(T)` outputs |
|---------|-----------|---------------|
| `[T] *Arrive"N"` | Nth variable arrives | `>var`, `>job` (single) |
| `[T] *Arrive"[lo,hi]"` | All variables in range arrived | `>vars`, `>jobs` (collections) |
| `[T] *Job.Arrive"N"` | Nth job completes | `>job`, `>vars` (single job, its vars) |
| `[T] *Job.Arrive"[lo,hi]"` | All jobs in range completed | `>jobs`, `>vars` (collections) |

### Ground Rule 5: All Jobs Must Be Released — PGE03025

Every code path through the `{*}` block must release ALL jobs `[0,N]`. No orphaned jobs.

```aljam3
[*] *Job.Release"[0,N]"              (* release all — job-centric *)
[*] *Arrive.Job.Release"[0,N]"       (* release all — variable-centric *)
```

Even if a collector only triggers on `*Arrive"0"`, it must still release every job — including the winner's job and jobs that were never triggered on.

**Compound reconciliation:** `*Job.Release` releases THIS collector's claim only. The [[glossary#Trigger Monitor|c:Trigger Monitor]] checks all active collectors — a job is actually cancelled only when ALL collectors holding claims on it have released.

### Ground Rule 6: All Operations Are Sequential — No `[=]`

`{*}` blocks are **strictly sequential**. The `[=]` parallel execution marker is forbidden inside collectors (PGE03021).

All work inside `{*}` uses `[-]` sequential execution. Variable declarations also use `[-]`:

```aljam3
[-] $accumulator <~ 0
```

### Ground Rule 7: Use `=ForEach` Expanders on `<Incoming`

The `<Incoming#IncomingDataFrame` is processed using standard `=ForEach` expanders.

### Ground Rule 8: Mandatory `<Incoming#IncomingDataFrame`

Every `{*}` block receives a system-provided input:

```aljam3
(*) <Incoming#IncomingDataFrame
```

`(*) <Incoming` maps to the **nameless parameters** at the collector invocation site — the `(*) $Var` declarations. These arrive in **arrival order**, NOT declaration order.

`#IncomingDataFrame` is a **subtype of `#Dataframe`** — a 2-level data tree where the 1st level is enumeration (arrival index) and the 2nd level is a `#Record` with the arrived variable info:

| Field | Type | Description |
|-------|------|-------------|
| `.arrival` | `#int` | Arrival sequence number (0-indexed) |
| `.variable` | `#RawString` | Canonical parameter name from `{*}` definition |
| `.value` | `#Serial` | The variable's current value |
| `.status` | `#JobStatus` | Running, Completed, Failed, Cancelled |
| `.jobUid` | `#RawString` | UID of the producing job |

### Ground Rule 9: Overflow — Three-Tier Chain

<!-- @u:technical/plan/queue-manager/overflow -->
`{*}` collectors participate in the PPTD (Parallel Processing Temporary Directory) overflow system ([[technical/plan/queue-manager/overflow|u:overflow]]). Overflow metadata:

```aljam3
[%] .overflow << #OverflowStrategy.Append
[%] .batchable << #bool.True
```

| `.overflow` value | Behavior | Use case |
|-------------------|----------|----------|
| `#OverflowStrategy.InMemoryOnly` | No PPTD — `!Storage.Space` if RAM exceeded | `*First`, `*Nth`, `*All` |
| `#OverflowStrategy.Append` | Per-job temp files; concatenated on collect | `*Into.Text.Append`, `*Into.CSV.Rows` |
| `#OverflowStrategy.Merge` | Per-job diff files; k-way merged on collect | `*Into.Text.Merge`, `*Into.CSV.Merge` |
| `#OverflowStrategy.Custom` | Collector-defined overflow logic | User-defined collectors |

### Ground Rule 10: Error Handling Encouraged

`[T]` blocks CAN and SHOULD have `[!]` error handlers. If `$var.status == #JobStatus.Failed`, the trigger fires with an error context. Collectors are encouraged to handle all their errors.

## `{*}` Metadata

`{*}` blocks use `[%]` metadata with three mandatory fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `.category` | `#CollectorCategory` | Yes | Into, Agg, Sync, Race, or Discard |
| `.scope` | `#CollectorScope` | Yes | Expand (inside `=ForEach`) or Parallel (at `[=]` boundary) |
| `.overflow` | `#OverflowStrategy` | Yes | Overflow behavior |
| `.batchable` | `#bool` | No | Whether batch collection is supported (default: `#bool.False`) |

**Scope semantics:**
- `#CollectorScope.Expand` — collector operates inside `=ForEach` scope, gathering per-item results from mini-pipelines. Used by `*Into.*` and `*Agg.*` operators.
- `#CollectorScope.Parallel` — collector operates at pipeline `[=]` parallel boundary, synchronizing parallel calls. Used by `*All`, `*First`, `*Second`, `*Nth`, and `*Ignore`.

## Complete Examples

### `*First` — Race Collector

```aljam3
{*} *First
   [%] .category << #CollectorCategory.Race
   [%] .scope << #CollectorScope.Parallel
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame
   (*) >winner
   (*) !Collector.NoResult

   [T] *Arrive"0"
      (T) >var
      (T) >job
      >> >winner << $var.value
      [*] *Job.Release"[0,N]"
```

### `*Second` — Race Collector (2nd arrival)

```aljam3
{*} *Second
   [%] .category << #CollectorCategory.Race
   [%] .scope << #CollectorScope.Parallel
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame
   (*) >winner
   (*) !Collector.NoResult

   [T] *Arrive"1"
      (T) >var
      (T) >job
      >> >winner << $var.value
      [*] *Job.Release"[0,N]"
```

### `*All` — Collect-All Barrier

```aljam3
{*} *All
   [%] .category << #CollectorCategory.Sync
   [%] .scope << #CollectorScope.Parallel
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame

   [T] *Job.Arrive"[0,N]"
      (T) >jobs
      (T) >vars
      (* all variables stay accessible *)
      [*] *Job.Release"[0,N]"
```

### `*Nth` — Generic Race Collector

```aljam3
{*} *Nth
   [%] .category << #CollectorCategory.Race
   [%] .scope << #CollectorScope.Parallel
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame
   (*) <n#int
   (*) >winner
   (*) !Collector.NoResult

   [T] *Arrive"{$n}"
      (T) >var
      (T) >job
      >> >winner << $var.value
      [*] *Job.Release"[0,N]"
```

### `*Agg.Sum` — Aggregation Collector

```aljam3
{*} *Agg.Sum
   [%] .category << #CollectorCategory.Agg
   [%] .scope << #CollectorScope.Expand
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame
   (*) <number
   (*) >sum#int

   [-] $accumulator <~ 0

   [T] *Arrive"[0,N]"
      (T) >vars
      (T) >jobs
      [-] =ForEach.Dataframe << $vars
         [~] >row
         [-] $accumulator <~ $accumulator + $row.value
      >> >sum << $accumulator
      [*] *Job.Release"[0,N]"
```

### `*Into.Text.Append` — Data Collector with Overflow

```aljam3
{*} *Into.Text.Append
   [%] .category << #CollectorCategory.Into
   [%] .scope << #CollectorScope.Expand
   [%] .overflow << #OverflowStrategy.Append
   [%] .batchable << #bool.True
   (*) <Incoming#IncomingDataFrame
   (*) <fragment
   (*) <separator#RawString
   (*) <order#CollectOrder
   (*) >text#RawString
   (*) !Text.Append.EmptyResult
   (*) !Storage.Space

   [T] *Arrive"[0,N]"
      (T) >vars
      (T) >jobs
      [-] =ForEach.Dataframe << $vars
         [~] >row
         (* append each fragment with separator *)
      >> >text << $result
      [*] *Job.Release"[0,N]"
      [!] !Text.Append.EmptyResult
         (* handle empty result *)
      [!] !Storage.Space
         (* handle storage exhaustion *)
```

### `*Ignore` — Discard Collector

```aljam3
{*} *Ignore
   [%] .category << #CollectorCategory.Discard
   [%] .scope << #CollectorScope.Parallel
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame

   [T] *Job.Arrive"[0,N]"
      (T) >jobs
      (T) >vars
      (* discard all — no output *)
      [*] *Job.Release"[0,N]"
```

## Compile Rules

| Rule | Description |
|------|-------------|
| PGE03013 | `{*}` block must declare `.category`, `.scope`, and `.overflow` metadata |
| PGE03014 | Expand-scoped collector used outside `=ForEach` scope |
| PGE03015 | Parallel-scoped collector used inside `=ForEach` scope |
| PGE03016 | Collector IO mismatch — `(*)` ports don't match `{*}` definition |
| PGE03017 | `*Arrive`, `*Job.Arrive`, or `*Job.Release` used outside `{*}` context |
| PGE03018 | `{*}` block missing mandatory `<Incoming#IncomingDataFrame` input |
| PGE03019 | Arrival index out of bounds — index exceeds `N` |
| PGE03020 | `{*}` body contains statements outside `[T]` blocks (except `[-]` variable declarations) |
| PGE03021 | `[=]` parallel execution used inside `{*}` collector — forbidden |
| PGE03022 | External trigger source (`=T.*`) used inside `{*}` — only `*Arrive`/`*Job.Arrive` allowed |
| PGE03023 | `{*}` with `.overflow` != `InMemoryOnly` missing `!Storage.Space` error declaration |
| PGE03024 | `*Job.Release` on job with no remaining claims from this collector |
| PGE03025 | Not all jobs released — every code path must release all jobs `[0,N]` |

## See Also

- [[concepts/collections/collect|Collect Operators]] — collector invocation syntax and reconciliation model
- [[concepts/collections/collect#Compound Collector Strategies]] — multi-collector claim semantics
- [[technical/plan/queue-manager/overflow|Overflow System]] — PPTD three-tier overflow chain
- [[technical/ebnf/16-collector-definitions|EBNF §16]] — formal grammar for `{*}` blocks
