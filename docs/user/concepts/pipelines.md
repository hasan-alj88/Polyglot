---
audience: user
type: specification
updated: 2026-03-15
status: draft
---

# Pipeline Structure

<!-- @blocks -->
<!-- @io -->
<!-- @operators -->
<!-- @variable-lifecycle -->
Every pipeline definition `{=}` (see [[blocks]]) must contain these elements in order. IO lines use [[io]] parameters with [[operators]] for assignment. Variable states follow [[variable-lifecycle]].

| Order | Element | Marker | Required |
|-------|---------|--------|----------|
| 0 | Metadata | `[%]` | Optional |
| 1 | Trigger / IO | `[t]`, `[=]` | `[t]` mandatory, `[=]` optional |
| 2 | Queue | `[Q]` | Mandatory |
| 3 | Wrapper | `[W]` | Mandatory |
| 4 | Execution | `[r]`, `[p]`, `[b]`, `[s]`, `[?]` | Yes |

**Metadata:** `[%]` lines declare description, version, authors, license, deprecation, and aliases. `.info;serial` holds custom metadata. See [[blocks#Metadata]].

**Note:** `[t]` triggers and `[=]` IO declarations form one section. IO declarations must appear **before** any trigger that pushes into them — the variable must exist before assignment. When a trigger produces outputs (e.g., `=T.Folder.NewFiles`), its `[=]` IO lines are indented under the `[t]` line and wire trigger outputs to pipeline inputs.

## Pipeline Metadata

Every pipeline carries implicit `live` metadata fields populated by the Polyglot runtime. Query built-in metadata via the `%` accessor instead of creating custom booleans. See [[metadata]] for the full metadata tree, field listings, and access patterns.

### Error Trees

Every pipeline exposes an error tree -- a structured list of every error it can raise, organized as sub-items under the pipeline's full path. Error trees allow callers to handle specific failure modes without guessing. For stdlib pipeline error trees, see [[STDLIB#Standard Library Error Trees]].

## Error Handling

`[!]` error blocks are scoped to the specific `[r]` call that can produce them, indented under the call (after its `[=]` IO lines):

```polyglot
[r] @FS=File.Text.Read
   [=] <path << <filepath
   [=] >content >> >content
   [!] !File.NotFound
      [r] >content << "Error: file not found"
   [!] !File.ReadError
      [r] >content << "Error: could not read file"
```

### Default behavior: `[!]` ends the pipeline

By default, if an `[!]` handler does **not** push a replacement value into the output variable, the pipeline **terminates on error**. No downstream code runs. This is the safe default.

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [ ] pipeline ends here on error
[r] =Process
   [=] <input << >data           [ ] only reachable if >data is Final
```

### Continuing after error: `[*] *Continue`

To continue the prime pipeline after an error, place `[*] *Continue` inside the `[!]` block. `*Continue` is a collector that produces a boolean `>IsFailed` output, wired directly on the `[*]` line. If the `>IsFailed` output is not handled, the compiler emits PGW-205.

Note: pipelines won't trigger if their input is Failed (IO implicit gate), so downstream steps with Failed inputs simply don't fire.

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [*] *Continue >IsFailed >> $fetchFailed
[?] $fetchFailed =? true
   [r] =HandleMissing
[?] *?
   [r] =Process
      [=] <input << >data        [ ] safe — >data is Final on this path
```

Three patterns for error handling:

| Pattern | Pipeline continues? | Variable state |
|---------|-------------------|---------------|
| `[!]` pushes replacement (`<<`/`>>`) | Yes | Always Final |
| `[!]` without replacement (default) | No — ends on error | Never Failed |
| `[!]` with `[*] *Continue >IsFailed >> $var` | Yes | May be Failed — handle via `$var` boolean |

## IO as Implicit Triggers

IO inputs act as implicit trigger gates based on their assignment operator:

| Assignment | Behavior |
|------------|----------|
| `<input << "value"` | **Constant** — always satisfied, value is locked |
| `<input <~ "value"` | **Has default** — uses default if all other triggers fire but nothing fills this input |
| `<input` (no assignment) | **Must be filled externally** — via caller or trigger wiring. Pipeline will not fire until this input reaches Final state |

There is no need to validate inputs with `[?]` checks — unfilled required inputs prevent the pipeline from triggering.

## Triggers

- `=T.Call` — invoked when called from another pipeline
- Standard library triggers live under `=T.*` namespace — no `[@]` import needed (see [[packages#Usage]])
- Triggers with arguments: `=T.Daily"3AM"`, `=T.Webhook"/path"`, `=T.Folder.NewFiles"/dir/"`
- Triggers that produce outputs wire them to pipeline inputs via indented `[=]` IO lines:

```polyglot
[=] <NewFiles;array.path
[t] =T.Folder.NewFiles"/inbox/"
   [=] >NewFiles >> <NewFiles
```

## Queue

- `[Q] =Q.Default` — use default queuing configuration

## Wrappers

Wrappers invoke a macro (see [[blocks]] `{M}`) that provides setup/cleanup scope. Every pipeline requires `[W]` — the compiler rejects pipelines without it.

- `[\]` — macro setup, runs before the execution body
- `[/]` — macro cleanup, runs after the execution body
- `[{]` — macro input (typed variable from pipeline scope)
- `[}]` — macro output (variable exposed back to pipeline scope)

At the `[W]` usage site, macro IO is wired using `[=]` with `$` variables:

```polyglot
[W] =W.DB.Connection
   [=] $connectionString << $connStr
   [=] $dbConn >> $dbConn
```

After `[W]` wiring, the macro's `[}]` outputs (e.g., `$dbConn`) become available as `$` variables in the execution body.

Execution order: `[t],[=]` → `[Q]` → `[\]` → Execution Body → `[/]`

### Parallel Forking in Setup

`[p]` or `[b]` inside `[\]` forks a parallel execution path:

- **`[p]` with no `[*] *All` in setup** — the forked path outlives setup and runs **concurrently with the execution body**. `[/]` uses `[*] *All` with `[*] << $var` to collect the result before proceeding.
- **`[b]` in setup** — fire-and-forget. No collection in `[/]` is possible.
- Variables produced in `[\]` (including by `[p]`) are accessible in `[/]` — same principle as `$dbConn` flowing from `[\]` to `[/]` in `=W.DB.Connection`.

**Pairing constraint:** A `[p]` started in `[\]` and its `[*] *All` collector form an exclusive pair — the collection **must** appear in `[/]`, never in the execution body. The execution body runs while the `[p]` is still in-flight; only `[/]` runs after execution completes and can safely collect.

| Started in | Collected in | Valid? |
|------------|--------------|--------|
| `[\]` `[p]` | `[/]` `[*] *All` | ✓ |
| `[\]` `[p]` | Execution body `[*] *All` | ✗ — body runs while `[p]` is still in-flight |
| Execution body `[p]` | Execution body `[*] *All` | ✓ — normal parallel pattern |

```polyglot
{M} =W.Tracing
   [{] $traceId;string
   [}] $duration;string

   [\]
      [ ] Sequential: open session — blocks before body starts
      [r] =Tracer.Open
         [=] <id << $traceId
         [=] >session >> $session

      [ ] Parallel: no *All after — timer runs concurrently with body
      [p] =Tracer.StartTimer
         [=] <session << $session
         [=] >handle >> $timerHandle

   [ ] body executes here while timer is running

   [/]
      [ ] Collect the timer started in setup
      [*] *All
         [*] << $timerHandle

      [r] =Tracer.StopTimer
         [=] <handle << $timerHandle
         [=] >elapsed >> $duration

      [r] =Tracer.Close
         [=] <session << $session
```

Common wrappers:
- `[W] =W.Polyglot` — default, pure Polyglot Code (calls `=DoNothing` for setup/cleanup)
- `[W] =W.DB.Transaction` — database connection + transaction lifecycle
- `[W] =W.HTTP.Session` — HTTP client lifecycle

See [[STDLIB#Wrappers]] for the full wrapper catalog.

## Execution

The execution section contains `[r]`, `[p]`, `[b]`, `[s]`, `[?]` lines — see [[blocks#Execution]]. For collection operations within execution, see [[collections]].

## Chain Execution

<!-- @io:Chain IO Addressing -->
<!-- @operators -->
Chain execution wires multiple pipelines in sequence on a single `[r]` line, with `>>` separating each step. IO lines under the chain address individual steps by **numeric index** (0-based) or **leaf name** (the last segment of the pipeline's dotted name).

```polyglot
[r] =Pipeline1 >> =Pipeline2 >> =Pipeline3
   [=] >0.inputParam;path << $file
   [=] <0.outputResult >> <1.inputParam
   [=] <2.outputResult >> >output
```

### Step Addressing

IO parameters in a chain are prefixed with a step reference and `.`:

| Syntax | Meaning |
|--------|---------|
| `>N.param` | Push into step N's input (caller perspective) |
| `<N.param` | Pull from step N's output (caller perspective) |
| `>LeafName.param` | Same as `>N` but using pipeline's leaf name |
| `<LeafName.param` | Same as `<N` but using pipeline's leaf name |

The direction convention is **caller-perspective**: `>` means data flows *toward* the step (its input), `<` means data flows *from* the step (its output). This is consistent with how `[=]` IO works in regular pipeline calls. See [[io#Chain IO Addressing]].

**Leaf name alternative:** When pipeline names are long, use the leaf name (last segment) instead of numeric index. Leaf names must be unambiguous within the chain — duplicate leaf names require numeric indices.

```polyglot
[r] =File.List >> =Data.Transform.Rows >> =Report.Format
   [=] >List.folder;path << $folder
   [=] <List.files >> <Rows.input
   [=] <Format.result >> >report
```

Numeric and leaf name references can be mixed in the same chain.

### Auto-Wire

When a step has exactly one output and the next step has exactly one input, and both share the same data type, the wire between them is implicit — no `[=]` line is needed. Only entry IO (first step's inputs) and exit IO (last step's outputs) must be declared.

```polyglot
[r] =File.Text.Read >> =Text.Transform >> =Text.Format
   [ ] Each step: one output;string → one input;string — auto-wired
   [=] >0.path;path << $path
   [=] <2.formatted;string >> >formatted
```

Auto-wire requires:
- Exactly one output on the source step
- Exactly one input on the target step
- Matching data types between them

If any condition is not met, explicit `[=]` wiring is required.

### Error Handling in Chains

Errors in chains use the `!` prefix with a step index or leaf name, followed by the error name. `[!]` blocks are scoped under the chain `[r]` call, after the `[=]` IO lines. See [[pipelines#Error Handling]] for standard error scoping rules.

**Prefer numeric indices** — they are always unambiguous:

```polyglot
[r] =File.Text.Read >> =Text.Parse.CSV
   [=] >0.path;path << $path
   [=] <1.rows;string >> >content
   [!] !0.File.NotFound
      [r] >content << "Error: file not found"
   [!] !1.Parse.InvalidFormat
      [r] >content << "Error: invalid CSV"
```

**Leaf name ambiguity:** When a leaf name shares a segment with the error name, the boundary is ambiguous. For example, `!Read.File.NotFound` is unclear — is the step `Read` (with error `File.NotFound`) or `Read.File` (with error `NotFound`)? In these cases, extend the step ref by one level up to disambiguate:

```polyglot
[ ] Ambiguous — "Read" + error "File.NotFound" looks like step "Read.File"
[!] !Read.File.NotFound

[ ] Unambiguous — extended step ref "Text.Read" is distinct from error "File.NotFound"
[!] !Text.Read.File.NotFound

[ ] Always safe — numeric index avoids all ambiguity
[!] !0.File.NotFound
```

### Type Annotations on Wires

Type annotations (`;type`) on chain IO lines are **optional**. When present, the compiler validates that connected ports have matching types. When omitted, types are inferred from the pipeline definitions.
