---
audience: architect
type: spec
updated: 2026-04-19
status: draft
---

# Native Dispatch

<!-- @c:spec/type-identity -->
<!-- @c:spec/polyglot-sdk -->
<!-- @concepts/pipelines/INDEX -->
<!-- @c:pglib/types/NativeKind -->

The native dispatch layer bridges Polyglot's runtime and host-language functions. When a subsystem encounters a call to a `{N}` native pipeline, the dispatch layer resolves the function, serializes inputs, executes the native function, and deserializes outputs back into Polyglot variables.

## Terminology

| Term | Definition |
|------|-----------|
| Native pipeline | A `{N}` definition — implemented in a host language, not Polyglot Code |
| Derived pipeline | A `{-}` definition — full Polyglot execution body |
| Host language | The programming language implementing a native function (Rust, Go, etc.) |
| Native registry | Compile-time lookup table mapping pipeline names to native function bindings |
| Wire format | JSON — the serialization format for data crossing the native boundary |
| Dispatch | The act of resolving a native function name, serializing inputs, calling the function, and deserializing outputs |
| Subsystem | One of four Polyglot Service components: Trigger Monitor, Queue Handler, Runner, PGCompiler |

See [[reference/glossary]] for canonical Polyglot terminology.

## Subsystem Architecture

<!-- @u:queue-manager/end-to-end-flow -->

Four subsystems dispatch native operations. Each subsystem dispatches its own `#NativeKind` operations:

| Subsystem | Dispatches | #NativeKind |
|-----------|-----------|-------------|
| Trigger Monitor (TM) | Event evaluation | `.Trigger` |
| Queue Handler (QH) | Scheduling decisions | `.Queue` |
| Runner | Pipeline execution + wrapper lifecycle | `.Execution`, `.Wrapper` |
| PGCompiler | Lexing, parsing, compile rule enforcement | (not dispatched — see below) |

**PGCompiler** is a special case. It does not dispatch `{N}` operations at runtime. The compiler reads `{N}` definitions to build the native registry and validates them against compile rules (PGE01028). The PGCompiler itself is a native implementation — the lexer, parser, and compile rule engine are written in a host language. The YAML config selects which host language implementation runs the compiler.

**Intrinsic** (`#NativeKind.Intrinsic`) operations are never dispatched. The compiler inlines their behavior at compile time. See [[#Intrinsic Catalog]].

### Dispatch Ownership

Each subsystem owns the full dispatch cycle for its operations:

```text
Trigger Monitor
   → dispatches -T.Call, -T.Daily, -T.Folder.NewFiles, -T.Webhook, ...
   → evaluates trigger conditions
   → emits NATS signals on trigger fire

Queue Handler
   → dispatches -Q.Default, -Q.Pause.Hard, -Q.Kill.Graceful, ...
   → manages queue strategy and active controls

Runner
   → dispatches -File.Text.Read, -Math.Add, -DB.Query, ... (Execution)
   → dispatches -W.Polyglot, -W.DB.Connection, ... (Wrapper — at [\] and [/] boundaries)

PGCompiler
   → not runtime dispatch — reads {N} definitions at compile time
   → lexer, parser, compile rules are native implementations selected by config
```

## YAML Configuration

The Polyglot service configuration file selects which host language implements each native operation. The config uses **subsystem defaults with per-operation overrides**.

### Schema

```yaml
native:
  defaults:
    tm: <Language>           # default for all Trigger operations
    qh: <Language>           # default for all Queue operations
    runner: <Language>       # default for all Execution + Wrapper operations
    pgcompiler: <Language>   # compiler implementation language

  overrides:
    # Override specific operations — pipeline name without = prefix
    # "File.Text.Read": <Language>
    # "T.Daily": <Language>
    # "Q.Pause.Hard": <Language>
```

### Resolution Order

1. Check `overrides` for exact pipeline name match
2. Fall back to `defaults` for the operation's subsystem (determined by `#NativeKind`)
3. If no default — configuration error at service startup

### Example

```yaml
native:
  defaults:
    tm: Rust
    qh: Rust
    runner: Rust
    pgcompiler: Rust

  overrides:
    "Math.Add": Go
    "DB.Query": Go
```

This configuration runs most operations in Rust, but dispatches `-Math.Add` and `-DB.Query` to Go implementations.

### Validation

At service startup, each subsystem validates its registry entries:

- Every `{N}` definition resolved to this subsystem must have a `.<Language>` binding matching the configured language
- Missing binding → startup error (not a runtime error)
- Unknown pipeline name in `overrides` → startup warning (ignored)

## Native Registry

### Compile-Time Construction

The compiler scans all pglib `.pg` files, collects `{N}` definitions, and emits a **native registry** — a lookup table included in the compiled output.

For each `{N}` definition, the compiler extracts:

```text
{N} -File.Text.Read
   [%] .Kind << #NativeKind.Execution       → kind
   [%] .Rust << "FileTextRead"              → bindings["Rust"]
   [%] .Go << "file_text_read"              → bindings["Go"]  (if present)
   [%] .description << "Read text file..."  → description
   (-) <path#path                           → inputs
   (-) >content#string                      → outputs
   (-) !File.NotFound                       → errors
   (-) !File.PermissionDenied               → errors
```

### Registry Entry Schema

```json
{
  "name": "File.Text.Read",
  "kind": "Execution",
  "description": "Read text file contents",
  "bindings": {
    "Rust": "FileTextRead",
    "Go": "file_text_read"
  },
  "inputs": [
    { "name": "path", "type": "path" }
  ],
  "outputs": [
    { "name": "content", "type": "string" }
  ],
  "errors": [
    "File.NotFound",
    "File.PermissionDenied"
  ]
}
```

The `inputs` and `outputs` arrays in the registry entry serve as **type mapping descriptors** — the [[polyglot-sdk|c:Polyglot SDK]] uses these same descriptors to determine how to serialize/deserialize values for each IO port. See [[polyglot-sdk#Type Mapping Descriptors]] for how the SDK consumes these descriptors.

### Runtime Loading

At startup, each subsystem:

1. Loads the full native registry from the compiled output
2. Filters entries by `#NativeKind` matching its dispatch responsibility
3. Resolves the active language for each entry using the YAML config
4. Validates that each entry has a binding for the resolved language
5. Builds an in-memory dispatch table: `pipeline_name → (function_name, io_schema)`

## Dispatch Flow

When a subsystem encounters a call to a `{N}` pipeline, it executes six steps:

```text
Step 1: Read function name
   │  Subsystem looks up pipeline name in its dispatch table
   │  Resolves: "File.Text.Read" → function "FileTextRead" (Rust)
   ▼
Step 2: Registry lookup
   │  Retrieves IO schema (inputs, outputs, errors) from registry entry
   │  Validates call-site IO matches registered IO
   ▼
Step 3: Serialize inputs
   │  Converts Polyglot $ variables → JSON data tree
   │  Applies type annotations from IO schema
   ▼
Step 4: Execute
   │  Calls the native function with serialized JSON input
   │  Function runs in host language runtime
   ▼
Step 5: Receive output
   │  Native function returns: Success(JSON) or Error(id, detail)
   │  Subsystem reads the result
   ▼
Step 6: Deserialize
   │  Parses JSON output → Polyglot $ variables
   │  Applies type annotations from IO schema
   │  Variables become Final in the pipeline
```

### Step 1 — Read Function Name

The subsystem extracts the function name from its dispatch table. The dispatch table was built at startup from the native registry + YAML config:

```text
dispatch_table["File.Text.Read"] = {
  function: "FileTextRead",
  language: "Rust",
  io: { inputs: [...], outputs: [...], errors: [...] }
}
```

### Step 2 — Registry Lookup

The subsystem retrieves the IO schema for the pipeline. This schema defines:
- Expected input parameter names and types
- Expected output parameter names and types
- Declared error identifiers

The compiler has already validated call-site IO at compile time (PGE08008, PGE08009, PGE08010). At runtime, the schema is used for serialization/deserialization, not validation.

### Step 3 — Serialize Inputs

All Polyglot data is serialized strings (see [[spec/type-identity]]). The dispatch layer serializes pipeline inputs into a JSON envelope:

```json
{
  "inputs": {
    "path": {
      "type": "path",
      "value": "/home/user/data.txt"
    }
  }
}
```

See [[#Serialization Protocol]] for the full wire format.

### Step 4 — Execute

The subsystem invokes the native function. The invocation mechanism is host-language-specific:

| Language | Mechanism |
|----------|-----------|
| Rust | Direct function call (linked at compile time) |
| Go | Function pointer from shared library or plugin |
| C/C++ | FFI call via C ABI |

The native function receives the JSON input as a string (or byte buffer). It runs entirely within its host language runtime. The function has no access to Polyglot runtime state — only the serialized inputs.

### Step 5 — Receive Output

The native function returns one of two results:

**Success:**
```json
{
  "status": "ok",
  "outputs": {
    "content": {
      "type": "string",
      "value": "file contents here"
    }
  }
}
```

**Error:**
```json
{
  "status": "error",
  "error": {
    "id": "File.NotFound",
    "detail": "Path /home/user/data.txt does not exist"
  }
}
```

### Step 6 — Deserialize

On success, the subsystem deserializes JSON outputs into Polyglot `$` variables. Each output becomes Final in the pipeline's IO state.

On error, the subsystem validates the `error.id` against the declared errors for this `{N}` definition:
- **Declared error** → propagates to the pipeline's `[!]` error handler or `!<` / `!>` fallback
- **Undeclared error** → runtime fatal `!RT.NativeUndeclaredError` — the native function violated its contract

## Serialization Protocol

<!-- @c:spec/type-identity -->

All data crossing the native boundary uses JSON. This section defines the wire format.

### Envelope

Every native call uses a request/response envelope:

**Request:**
```json
{
  "pipeline": "File.Text.Read",
  "inputs": { ... }
}
```

**Response (success):**
```json
{
  "status": "ok",
  "outputs": { ... }
}
```

**Response (error):**
```json
{
  "status": "error",
  "error": {
    "id": "File.NotFound",
    "detail": "human-readable message"
  }
}
```

### Value Encoding

Each value in `inputs` and `outputs` is a typed JSON object:

```json
{
  "type": "<polyglot_type>",
  "value": <json_value>
}
```

| Polyglot Type | JSON `value` | Example |
|---------------|-------------|---------|
| `#RawString` | string | `"hello"` |
| `#string` | string | `"hello"` |
| `#int` | string (preserves precision) | `"42"` |
| `#float` | string (preserves precision) | `"3.14"` |
| `#bool` | string | `"True"` or `"False"` |
| `#path` | string | `"/home/user/file.txt"` |
| `#bytes` | string (Base64) | `"SGVsbG8gV29ybGQ="` |
| `#dt` | string (epoch seconds) | `"1712494800"` |
| `#serial` | object (nested key-value) | `{ "key": { "type": "string", "value": "val" } }` |
| `#array:T` | array of typed values | `[{ "type": "string", "value": "a" }, ...]` |
| enum | string (variant name) | `"Active"` |

**Key rule:** All leaf values are strings. This preserves Polyglot's "all data is serialized strings" principle (see [[spec/type-identity]]) across the native boundary. Numeric types use string representation to avoid floating-point precision loss.

### Nested Data Trees

`#serial` types serialize as nested JSON objects where each key maps to a typed value:

```json
{
  "type": "serial",
  "value": {
    "database": {
      "type": "serial",
      "value": {
        "host": { "type": "string", "value": "localhost" },
        "port": { "type": "int", "value": "5432" }
      }
    }
  }
}
```

Fixed fields (`.`) and flexible fields (`:`) are both represented as JSON object keys. The distinction is a compile-time concern — at the wire level, they are identical.

## Native Function Contract

Every native function, regardless of host language, must implement the same logical contract.

### Signature (Pseudocode)

```
function execute(request_json: string) -> string
```

- **Input:** JSON string matching the request envelope
- **Output:** JSON string matching the response envelope (success or error)
- The function is pure from Polyglot's perspective — no side-channel access to Polyglot state

### Language-Specific Signatures

Each host language implements the contract idiomatically:

**Rust:**
```rust
pub fn file_text_read(request: &str) -> Result<String, String>
// Ok(response_json) or Err(response_json)
```

**Go:**
```go
func FileTextRead(request string) (string, error)
```

**Python:**
```python
def file_text_read(request: str) -> str:
    # returns response JSON (success or error envelope)
```

### Error Reporting

Native functions report errors by returning an error envelope. The `error.id` must match one of the `(-) !` error declarations in the `{N}` definition.

| Scenario | Behavior |
|----------|----------|
| Declared error returned | Propagates to `[!]` handler in calling pipeline |
| Undeclared error returned | Runtime fatal: `!RT.NativeUndeclaredError` |
| Native function panics/crashes | Runtime fatal: `!RT.NativeCrash` |
| Serialization failure on input | Runtime fatal: `!RT.SerializationError` |
| Deserialization failure on output | Runtime fatal: `!RT.DeserializationError` |

## #NativeKind Routing

<!-- @c:pglib/types/NativeKind -->

Each `#NativeKind` variant routes to a specific subsystem and dispatch context.

### Execution

**Dispatched by:** Runner
**When:** Pipeline body encounters `[-]`, `[=]`, or `[b]` call to a `{N}` Execution pipeline
**Examples:** `-File.Text.Read`, `-Math.Add`, `-DB.Query`, `-DT.Now`, `-Run.Bridge.Function`, `-Run.Bridge.Script`

```text
Runner receives job.start signal
   → Runner executes pipeline body
   → Body contains: [-] -File.Text.Read
   → Runner dispatches to native: FileTextRead(JSON)
   → Result flows back into pipeline variables
```

**Bridge dispatch note:** `-Run.Bridge.Function` and `-Run.Bridge.Script` are Execution-kind operations dispatched by the Runner. The Runner handles dual-env setup by dispatching two environment wrappers (one per language) before executing the Bridge call. See [[pglib/pipelines/Run/Bridge.Function]] and [[pglib/pipelines/Run/Bridge.Script]] for the pipeline definitions, and [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] for the type conversion flow.

### Trigger

**Dispatched by:** Trigger Monitor
**When:** TM evaluates trigger conditions for a pipeline
**Examples:** `-T.Call`, `-T.Daily`, `-T.Folder.NewFiles`, `-T.Webhook`

```text
Trigger Monitor evaluates pipeline triggers
   → Pipeline declares: [T] -T.Daily"3AM"
   → TM dispatches to native: TriggerDaily(JSON)
   → Native returns: { IsTriggered: true/false }
   → If triggered: TM emits command.enqueue via NATS
```

Trigger natives receive trigger configuration as input and return `>IsTriggered#bool` plus any additional outputs.

### -T.Call Signal Path

<!-- @c:spec/polyglot-sdk -->
<!-- @u:queue-manager/nats-namespace -->
<!-- @u:queue-manager/reactive-signals -->
Related: [[polyglot-sdk]], [[queue-manager/nats-namespace]], [[queue-manager/reactive-signals]]

`-T.Call` is unique among triggers. Other triggers (`-T.Daily`, `-T.Webhook`, `-T.Folder.NewFiles`) are evaluated by the TM in its polling loop — fire-and-forget events that enqueue jobs with no caller waiting for a response. `-T.Call` uses **NATS request-reply**: the [[polyglot-sdk|c:Polyglot SDK]] publishes a call request, the TM processes it, and the TM publishes the result back to the SDK. The caller blocks until the response arrives.

#### TM Subscription

At startup, the Trigger Monitor subscribes to the NATS wildcard subject:

```text
polyglot.call.*
```

When a message arrives on `polyglot.call.{pipeline_name}`, the TM extracts the pipeline name from the subject suffix and begins processing the call request.

#### Pipeline Matching

The TM searches its trigger registry for a pipeline whose trigger declaration matches the requested name:

```text
Requested subject: polyglot.call.ProcessData
    → TM looks for pipeline with: [T] -T.Call
    → Pipeline name matches "ProcessData"
    → Found: proceed to binding storage
```

If no pipeline with `-T.Call` matches the requested name, the TM publishes an error response immediately:

```json
{
  "correlation_id": "<from request>",
  "status": "error",
  "error": {
    "id": "SDK.PipelineNotFound",
    "detail": "No pipeline 'ProcessData' with -T.Call trigger"
  }
}
```

The response is published to `polyglot.result.{correlation_id}` using the correlation ID from the request payload. See [[polyglot-sdk#call]] for the full request/response payload schema.

#### Binding Storage

The TM writes SDK-provided input bindings to the NoSQL variable store (Redis). Each input binding from the SDK request payload is stored as a separate key:

```text
job:{UID}:port:{port_name}    — one key per input binding (typed JSON envelope)
job:{UID}:meta:correlation_id  — the SDK's correlation UUID for result routing
job:{UID}:meta:caller_topic    — polyglot.result.{correlation_id} (pre-computed)
```

The `{UID}` is generated by the TM at this point — the SDK does not control job IDs. Each port value is stored as the typed JSON envelope exactly as received from the SDK (see [[native-dispatch#Value Encoding]]).

**Example:** For a call with two inputs:

```text
HSET job:abc123:port:input_path  '{"type": "path", "value": "/data/raw.csv"}'
HSET job:abc123:port:batch_size  '{"type": "int", "value": "100"}'
SET  job:abc123:meta:correlation_id  "550e8400-e29b-41d4-a716-446655440000"
SET  job:abc123:meta:caller_topic    "polyglot.result.550e8400-e29b-41d4-a716-446655440000"
```

#### Enqueue

After storing bindings, the TM emits the standard `polyglot.command.enqueue` signal with the job metadata. From this point, the job follows the normal QH → Runner execution path — the QH and Runner have no knowledge that this job originated from an SDK call.

```text
polyglot.command.enqueue {
  jobId: "abc123",
  pipeline: "ProcessData",
  queue: <from pipeline's {Q} declaration>,
  params: {},
  priority: <from pipeline's queue priority>
}
```

See [[queue-manager/reactive-signals#command.enqueue]] for the full signal schema and QH state write.

#### Result Collection

The TM subscribes to Runner acknowledgment signals for the job:

```text
polyglot.runner.completed.{jobId}   — job finished successfully
polyglot.runner.failed.{jobId}      — job failed with error
```

**On success (`runner.completed`):**

1. TM reads output port values from `job:{UID}:port:{name}` (output-direction ports)
2. TM retrieves `job:{UID}:meta:caller_topic`
3. TM constructs the success response envelope:

```json
{
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "ok",
  "outputs": {
    "output_path": { "type": "path", "value": "/data/processed.csv" },
    "row_count": { "type": "int", "value": "1500" }
  }
}
```

4. TM publishes to `polyglot.result.{correlation_id}`

**On failure (`runner.failed`):**

1. TM reads the error from the `runner.failed` signal payload
2. TM retrieves `job:{UID}:meta:caller_topic`
3. TM constructs the error response envelope:

```json
{
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "error",
  "error": {
    "id": "File.NotFound",
    "detail": "/data/raw.csv does not exist"
  }
}
```

4. TM publishes to `polyglot.result.{correlation_id}`

The SDK wraps pipeline errors as `!SDK.PipelineError` — see [[polyglot-sdk#call]] for error handling on the SDK side.

#### Sequence Diagram

```text
SDK              NATS                  TM                Redis              QH              Runner
 │                │                    │                   │                 │                │
 │  call("ProcessData", bindings)      │                   │                 │                │
 │  serialize via to_polyglot()        │                   │                 │                │
 │                │                    │                   │                 │                │
 │  PUBLISH polyglot.call.ProcessData  │                   │                 │                │
 │───────────────>│                    │                   │                 │                │
 │                │  deliver           │                   │                 │                │
 │                │───────────────────>│                   │                 │                │
 │                │                    │                   │                 │                │
 │  SUBSCRIBE polyglot.result.{uuid}   │  match pipeline   │                 │                │
 │───────────────>│                    │  "ProcessData"    │                 │                │
 │                │                    │  has [T] -T.Call   │                 │                │
 │                │                    │                   │                 │                │
 │                │                    │  HSET job:{UID}:port:*              │                │
 │                │                    │──────────────────>│                 │                │
 │                │                    │  SET job:{UID}:meta:*               │                │
 │                │                    │──────────────────>│                 │                │
 │                │                    │                   │                 │                │
 │                │                    │  PUBLISH command.enqueue             │                │
 │                │                    │──────────────────────────────────── >│                │
 │                │                    │                   │                 │                │
 │                │                    │                   │  dispatch        │                │
 │                │                    │                   │                 │  control.start  │
 │                │                    │                   │                 │───────────────>│
 │                │                    │                   │                 │                │
 │                │                    │                   │                 │  ... execute ...│
 │                │                    │                   │                 │                │
 │                │                    │                   │                 │  runner.completed
 │                │  runner.completed.{jobId}              │                 │<───────────────│
 │                │───────────────────>│                   │                 │                │
 │                │                    │                   │                 │                │
 │                │                    │  HGET job:{UID}:port:* (outputs)    │                │
 │                │                    │──────────────────>│                 │                │
 │                │                    │  GET job:{UID}:meta:caller_topic    │                │
 │                │                    │──────────────────>│                 │                │
 │                │                    │                   │                 │                │
 │                │  PUBLISH polyglot.result.{uuid}        │                 │                │
 │                │<───────────────────│                   │                 │                │
 │                │                    │                   │                 │                │
 │  receive result│                    │                   │                 │                │
 │<───────────────│                    │                   │                 │                │
 │                │                    │                   │                 │                │
 │  deserialize via from_polyglot()    │                   │                 │                │
 │  return to foreign code             │                   │                 │                │
```

#### Comparison: -T.Call vs Other Triggers

| Aspect | -T.Call | Other Triggers (-T.Daily, -T.Webhook, ...) |
|--------|--------|---------------------------------------------|
| Signal source | SDK via NATS `polyglot.call.*` | TM evaluation loop (polling/event) |
| Response | Synchronous request-reply via NATS | None (fire-and-forget) |
| Input bindings | SDK provides via request payload | Trigger native provides `>IsTriggered` + outputs |
| Correlation | Per-call UUID tracks request→response | None — no caller waiting |
| TM subscription | `polyglot.call.*` wildcard | Internal evaluation schedule |
| Redis metadata | `job:{UID}:meta:correlation_id`, `caller_topic` | None — no result routing needed |
| Error routing | Errors published to SDK via NATS | Errors logged, job state updated |

### Queue

**Dispatched by:** Queue Handler
**When:** QH evaluates queue control conditions (pause, resume, kill)
**Examples:** `-Q.Default`, `-Q.Pause.Hard.RAM.LessThan`, `-Q.Resume.RAM.MoreThan`, `-Q.Kill.Graceful`

```text
Trigger Monitor evaluates resource condition
   → Condition met for -Q.Pause.Hard.RAM.LessThan
   → TM sends command.pause.hard to QH via NATS
   → QH dispatches native queue control function
```

### Wrapper

**Dispatched by:** Runner
**When:** Runner enters `[\]` setup or `[/]` cleanup phase of a wrapper
**Examples:** `-W.Polyglot`, `-W.DB.Connection`, `-W.RT.Python`

```text
Runner starts pipeline execution
   → Pipeline declares: [W] -W.DB.Connection
   → Runner dispatches wrapper setup: WrapperDBConnectionSetup(JSON)
   → ... pipeline body executes ...
   → Runner dispatches wrapper cleanup: WrapperDBConnectionCleanup(JSON)
```

Wrapper natives expose `(-)` inputs and outputs through the standard serialization protocol.

### Intrinsic

**Dispatched by:** Nobody — compiler-inlined
**When:** Never dispatched at runtime
**Examples:** `-DoNothing`, `-#.JSON.Parse`, `-#.Validate`

Intrinsic operations have no `.<Language>` binding. The compiler recognizes them by name and emits specialized behavior directly into the compiled output. The native dispatch layer never sees them.

See [[#Intrinsic Catalog]] for the full list.

## Intrinsic Catalog

Intrinsics are `{N}` definitions with `#NativeKind.Intrinsic`. They have no host-language function — the compiler inlines their behavior.

### -DoNothing

No-op pipeline. Produces no output, performs no action. Used as an explicit empty branch (`[-] -DoNothing` in `[?]` conditionals) and as the default wrapper body (`-W.Polyglot` calls `-DoNothing` for setup/cleanup).

```polyglot
{N} -DoNothing
   [%] .Kind << #NativeKind.Intrinsic
   [%] .description << "No-op pipeline"
```

**Compiler behavior:** Emits no instructions. The job completes immediately with no output.

### -#.JSON.Parse

Parses JSON text into a `#serial` data tree.

| IO | Type |
|----|------|
| `<raw` | `#RawString` |
| `>data` | `#serial` |

**Compiler behavior:** Emits the built-in JSON parser. No errors — invalid input handled by the calling pipeline.

### -#.YAML.Parse

Parses YAML text into a `#serial` data tree. Same IO as `-#.JSON.Parse`.

**Compiler behavior:** Emits the built-in YAML parser.

### -#.TOML.Parse

Parses TOML text into a `#serial` data tree. Same IO as `-#.JSON.Parse`.

**Compiler behavior:** Emits the built-in TOML parser.

### -#.Match

Boolean schema check — does data match a type?

| IO | Type |
|----|------|
| `<data` | `#serial` |
| `<#type` | type tree |
| `>match` | `#bool` |

**Compiler behavior:** Emits structural comparison against the type's metadata tree.

### -#.Validate

Detailed validation — checks data against type, reports mismatches.

| IO | Type |
|----|------|
| `<data` | `#serial` |
| `<#type` | type tree |
| `>valid` | `#bool` |
| `>errors` | `#array:string` |

**Compiler behavior:** Emits structural validation with error collection.

### -#.Describe

Schema introspection — returns a type's schema as a data tree.

| IO | Type |
|----|------|
| `<#type` | type tree |
| `>schema` | `#serial` |

**Compiler behavior:** Emits metadata tree traversal for the given type.

### -#.Coerce

Best-effort type conversion — keeps matching fields, reports dropped fields.

| IO | Type |
|----|------|
| `<data` | `#serial` |
| `<#type` | type tree |
| `>result` | `#serial` |
| `>dropped` | `#array:string` |

**Compiler behavior:** Emits structural filtering against target type schema.

### -#.Field

Extracts a single field from a data tree by path.

| IO | Type |
|----|------|
| `<data` | `#serial` |
| `<path` | `#RawString` |
| `>value` | `#serial` |

Errors: `!Field.NotFound`, `!Field.PathError`

**Compiler behavior:** Emits tree path traversal with error handling.

### -#.Column

Extracts all values for a column in a Dataframe.

| IO | Type |
|----|------|
| `<data` | `#dataframe` |
| `<column` | `#enum` |
| `>values` | `#array` |

Errors: `!Field.NotFound`

**Compiler behavior:** Emits row iteration with column extraction.

## lib/ Folder Structure

The `lib/` directory contains host-language packages organized by language and subsystem role.

```text
lib/
├── rust/
│   ├── integrator/        SDK: call Polyglot + be called by Polyglot
│   ├── tm/                Trigger Monitor native operations
│   ├── qh/                Queue Handler native operations
│   ├── runner/            Runner native operations (Execution + Wrapper)
│   └── pgcompiler/        Compiler: lexer, parser, compile rules
├── go/
│   ├── integrator/
│   ├── tm/
│   ├── qh/
│   ├── runner/
│   └── pgcompiler/
└── <language>/
    └── (same structure)
```

### Subsystem Bundles

Each subsystem folder contains the native function implementations for that subsystem's `#NativeKind`:

| Folder | Contains | #NativeKind |
|--------|----------|-------------|
| `tm/` | Trigger evaluation functions | `.Trigger` |
| `qh/` | Queue strategy and control functions | `.Queue` |
| `runner/` | Data operations + wrapper lifecycle | `.Execution`, `.Wrapper` |
| `pgcompiler/` | Lexer, parser, compile rule engine | N/A (compiler, not runtime) |

Each native function in these folders must implement the [[#Native Function Contract]] — accept JSON request, return JSON response.

### Integrator SDK

The `integrator/` folder contains the bidirectional SDK for each language:

**Host → Polyglot (call direction):**
- Connect to the Polyglot Service
- Submit pipeline execution requests
- Receive pipeline results

**Polyglot → Host (callback direction):**
- Register native functions callable by `-RT.*` runtime pipelines
- Receive serialized inputs from Polyglot
- Return serialized outputs to Polyglot

The integrator SDK uses the same JSON wire format as native dispatch. This means an integrator's callback functions follow the identical [[#Native Function Contract]]. For the SDK's public API and serialization algorithm, see [[polyglot-sdk]].

### Adding a New Language

To add support for a new host language (e.g., Go):

1. Create `lib/go/` with the standard subfolder structure
2. Implement the native function contract in Go (JSON in → JSON out)
3. Implement each native function listed in the native registry for the target subsystem(s)
4. Add `.<Language>` bindings to `{N}` definitions in pglib `.pg` files (e.g., `[%] .Go << "FileTextRead"`)
5. Update the YAML config to reference the new language in `defaults` or `overrides`
6. Build the integrator SDK for bidirectional communication

The JSON wire format ensures all languages use the same boundary protocol. No changes to the dispatch layer or other subsystems are required.

## Failure Modes

| Failure | When | Severity | Recovery |
|---------|------|----------|----------|
| Registry miss | Pipeline name not in dispatch table | Startup error | Fix YAML config or add `{N}` definition |
| Missing binding | `{N}` has no `.<Language>` for configured language | Startup error (PGE01028e at compile time) | Add `.<Language>` field to `{N}` definition |
| Serialization failure | Polyglot variable cannot be encoded to JSON | Runtime fatal: `!RT.SerializationError` | Bug in serializer — fix type mapping |
| Deserialization failure | Native output JSON cannot be parsed to Polyglot types | Runtime fatal: `!RT.DeserializationError` | Native function returned malformed JSON |
| Undeclared error | Native function returns error not in `(-) !` declarations | Runtime fatal: `!RT.NativeUndeclaredError` | Fix native function to only return declared errors |
| Native crash | Host function panics, segfaults, or throws unhandled exception | Runtime fatal: `!RT.NativeCrash` | Fix native function implementation |
| Timeout | Native function exceeds execution time limit | Runtime fatal: `!RT.NativeTimeout` | Fix native function or increase timeout |

All runtime fatals halt the current job. The Queue Handler is notified via NATS signal and updates job state accordingly. Parent jobs react based on error propagation rules.

## Sequence Diagrams

### Runner Dispatches Execution

```text
Runner                 Dispatch Table        Native Function
  │                        │                       │
  │  lookup("File.Text.    │                       │
  │   Read")               │                       │
  │───────────────────────>│                       │
  │  {fn: "FileTextRead",  │                       │
  │   io: {...}}           │                       │
  │<───────────────────────│                       │
  │                        │                       │
  │  serialize($path)      │                       │
  │  → JSON request        │                       │
  │                        │                       │
  │  FileTextRead(json)    │                       │
  │────────────────────────────────────────────────>│
  │                        │                       │
  │  { status: "ok",       │                       │
  │    outputs: {...} }    │                       │
  │<────────────────────────────────────────────────│
  │                        │                       │
  │  deserialize → $content│                       │
  │  $content becomes Final│                       │
```

### Trigger Monitor Dispatches Trigger

```text
Trigger Monitor        Dispatch Table       Native Function        NATS
  │                        │                      │                  │
  │  lookup("T.Daily")     │                      │                  │
  │───────────────────────>│                      │                  │
  │  {fn: "TriggerDaily"}  │                      │                  │
  │<───────────────────────│                      │                  │
  │                        │                      │                  │
  │  TriggerDaily(json)    │                      │                  │
  │───────────────────────────────────────────────>│                  │
  │  { IsTriggered: true } │                      │                  │
  │<───────────────────────────────────────────────│                  │
  │                        │                      │                  │
  │  command.enqueue                               │                  │
  │───────────────────────────────────────────────────────────────────>│
```

### Error Flow

```text
Runner                 Native Function       Error Handler
  │                         │                      │
  │  execute(json)          │                      │
  │────────────────────────>│                      │
  │  { status: "error",     │                      │
  │    error: {             │                      │
  │      id: "File.NotFound"│                      │
  │      detail: "..." }}   │                      │
  │<────────────────────────│                      │
  │                         │                      │
  │  validate error.id      │                      │
  │  against declared errors│                      │
  │                         │                      │
  │  (declared) propagate   │                      │
  │  to [!] handler         │                      │
  │─────────────────────────────────────────────── │
  │                         │                      │
```

## Related

| Document | Relationship |
|----------|-------------|
| [[technical/ebnf/09-definition-blocks#9.4c\|EBNF §9.4c]] | Formal `{N}` grammar |
| [[technical/compile-rules/PGE/PGE01028\|PGE01028]] | Native/derived mutual exclusion rules |
| [[pglib/types/NativeKind\|#NativeKind]] | The 5 kind variants |
| [[concepts/pipelines/INDEX#Native vs Derived\|Native vs Derived]] | High-level distinction |
| [[spec/type-identity\|Type Identity]] | "All data is serialized strings" foundation |
| [[spec/polyglot-sdk\|Polyglot SDK]] | SDK public API, type mapping, and serialization algorithm |
| [[spec/metadata-tree/branches\|Metadata Tree Branches]] | Pipeline/job structure |
| [[queue-manager/end-to-end-flow\|End-to-End Flow]] | Where native dispatch fits in execution |
| [[pglib/pipelines/#\|-#.* Pipelines]] | Intrinsic catalog source |
| [[pglib/pipelines/Run/Bridge.Function]] | `-Run.Bridge.Function` — cross-language function call |
| [[pglib/pipelines/Run/Bridge.Script]] | `-Run.Bridge.Script` — cross-language variable binding |
| [[technical/algorithms/bridge-conversion]] | Bridge conversion algorithm for pairwise type mapping |
