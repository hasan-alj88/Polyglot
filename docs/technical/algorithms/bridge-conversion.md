---
audience: design
type: spec
updated: 2026-04-19
status: draft
---

# Bridge Conversion Algorithm

<!-- @c:spec/native-dispatch -->
<!-- @c:spec/native-dispatch#Serialization Protocol -->
<!-- @c:spec/aljam3-sdk -->
<!-- @c:jm3lib/types/Variable -->
<!-- @c:jm3lib/types/NativeType -->
<!-- @c:jm3lib/pipelines/Run/INDEX -->
Related: [[native-dispatch]], [[aljam3-sdk]], [[foreign-code-analysis]]

## Overview

The Bridge enables pairwise cross-language function calls and variable binding by converting `#Variable` instances between language type systems. All conversion passes through the Aljam3 type system as intermediary -- the same string-serialized JSON wire format used by [[native-dispatch|native dispatch]].

Bridge pipelines (`-Run.Bridge.Function`, `-Run.Bridge.Script`) use the conversion algorithm internally. Users can also invoke `-Variable.Convert` directly for explicit variable conversion outside a Bridge context.

### Design Principles

1. **String serialization, not FFI.** All values cross language boundaries as JSON strings. No shared memory, no foreign function interface, no dynamic code generation.
2. **Aljam3 type system as intermediary.** Source native type maps to a Aljam3 type, then Aljam3 type maps to target native type. The two languages never communicate directly.
3. **Same wire format as native dispatch.** The JSON envelope (`{"type": "<aljam3_type>", "value": "<string>"}`) is identical to [[native-dispatch#Value Encoding]].
4. **Extensible by language pair.** New language support adds conversion entries without changing the algorithm.

## Dual-Wrapper Lifecycle

Bridge pipelines require two `-W.Env` wrappers. The Runner manages their lifecycle with strict ordering.

### Setup Order (Top-to-Bottom)

```text
Step 1: Start caller environment
   Runner dispatches -W.Env setup for the caller's {;} definition
   Caller interpreter/runtime starts, dependencies installed
   Caller environment handle becomes available

Step 2: Start callee environment
   Runner dispatches -W.Env setup for the callee's {;} definition
   Callee interpreter/runtime starts, dependencies installed
   Callee environment handle becomes available

Step 3: Bridge execution begins
   Both environments are ready
   Arguments converted and function/script executed
```

### Cleanup Order (Bottom-to-Top)

```text
Step 1: Shut down callee environment
   Runner dispatches -W.Env cleanup for callee
   Callee interpreter/runtime stops

Step 2: Shut down caller environment
   Runner dispatches -W.Env cleanup for caller
   Caller interpreter/runtime stops
```

### Skip-If-Running Semantics

If an environment is already active from an outer wrapper scope, setup is skipped for that environment. The Runner tracks active environments by `{;}` definition name:

```text
Outer pipeline: [W] -W.Env;PyData          ← starts PyData
   Inner Bridge: [-] -Run.Bridge.Function;PyData;RsProc
      Setup PyData: SKIP (already active)
      Setup RsProc: START
      ... execute ...
      Cleanup RsProc: STOP
      Cleanup PyData: SKIP (managed by outer scope)
```

This avoids redundant interpreter startups when the caller environment is already established by a surrounding pipeline.

### Error During Second Env Setup

If the callee environment fails to start after the caller environment is already running:

1. The callee setup error propagates as `!Run.Bridge.EnvSetupFailed`
2. The caller environment remains active (its lifecycle is independent)
3. The Bridge call fails and error propagation follows normal `[!]` rules
4. Cleanup for the caller runs when its owning wrapper scope ends

## Conversion Algorithm

The `-Variable.Convert` pipeline implements the three-step conversion:

### Step 1 -- Read Source Variable

Extract the active language branch, native type, and serialized value from the source `#Variable`:

```text
Input: #Variable {
   branch: "Python",
   name: "user_count",
   value: {"type": "int", "value": "42"},
   type: "int"
}

Extracted:
   source_lang: Python
   source_type: int
   name: "user_count"
   value: "42"
```

### Step 2 -- Map to Aljam3 Type

Look up the source native type in the [[jm3lib/types/NativeType#Marshalling Table|marshalling table]] to find the corresponding Aljam3 type:

```text
Lookup: Python.int → Aljam3 #int

Result:
   aljam3_type: #int
   value: "42" (unchanged -- already string-encoded)
```

The value string does not change during this step. All Aljam3 values are already string-encoded per the [[spec/type-identity|type identity]] principle. The marshalling table lookup only determines the Aljam3 type tag.

### Step 3 -- Map to Target Type

Look up the Aljam3 type in the marshalling table to find the target language's native type:

```text
Lookup: Aljam3 #int → Rust i64

Result: #Variable {
   branch: "Rust",
   name: "user_count",
   value: {"type": "int", "value": "42"},
   type: "i64"
}
```

Again, the value string does not change. The JSON envelope's `"type"` field remains the Aljam3 type (`"int"`) -- the `#NativeType` tag on the `#Variable` tells the receiving Runner which native constructor to use.

### Complete Flow

```text
Source #Variable (:Python, .type=int, .value="42")
    ↓ Step 1: read source
    Python.int
    ↓ Step 2: map to Aljam3
    Aljam3 #int ("42")
    ↓ Step 3: map to target
    Target #Variable (:Rust, .type=i64, .value="42")
```

### Edge Cases

**Same Aljam3 type, different native types:**
Python `str` and Rust `String` both map to Aljam3 `#string`. Conversion is trivial -- only the branch and type tag change.

**No direct mapping (incompatible type):**
Python `bytes` has no direct Aljam3 equivalent. Attempting to convert a `bytes`-typed `#Variable` to JavaScript raises `!Run.Bridge.IncompatibleType`. The user must explicitly convert to `#string` (base64-encoded) before bridging.

**Numeric precision:**
All numeric values are string-encoded, so precision is preserved across conversion. Python `int` (arbitrary precision) → Rust `i64` (64-bit) may overflow at runtime, raising `!Run.Bridge.ConversionFailed`.

**Null/None handling:**
Python `NoneType`, Go `nil`, JavaScript `null`, Rust `Option::None` all map to the absence of a value. The conversion uses an empty string with type `"null"` in the JSON envelope.

## Bridge.Function Call Flow

The complete call flow for `-Run.Bridge.Function` from trigger to result:

```text
Step 1: Pipeline triggers
   [T] fires, job enters queue via QH

Step 2: Runner starts wrapper setup
   -W.Env;CallerEnv setup (or skip if active)
   -W.Env;CalleeEnv setup

Step 3: Convert arguments
   For each <arg #Variable:
      -Variable.Convert(from=arg, targetLang=callee_lang)
   For each <kwarg #Variable:
      -Variable.Convert(from=kwarg.value, targetLang=callee_lang)

Step 4: Dispatch to callee
   Runner serializes converted args to JSON
   Runner dispatches RunBridgeFunction native op
   Native op invokes <func in callee environment
   Function executes in callee language runtime

Step 5: Collect return values
   Native op reads return values from callee
   Values tagged with callee's #NativeType
   Returned as >Bind #array.Variable

Step 6: Convert return values (optional)
   If caller needs values in its own type system:
      For each >Bind #Variable:
         -Variable.Convert(from=result, targetLang=caller_lang)

Step 7: Wrapper cleanup
   -W.Env;CalleeEnv cleanup
   -W.Env;CallerEnv cleanup (or skip if outer scope)
```

### Sequence Diagram

```mermaid
sequenceDiagram
    participant Caller as Caller Pipeline
    participant Runner
    participant Convert as -Variable.Convert
    participant CallerEnv as Caller Env<br>(e.g., Python)
    participant CalleeEnv as Callee Env<br>(e.g., Rust)

    Caller->>Runner: [-] -Run.Bridge.Function;Py;Rs

    Runner->>CallerEnv: -W.Env setup (or skip)
    CallerEnv-->>Runner: ready

    Runner->>CalleeEnv: -W.Env setup
    CalleeEnv-->>Runner: ready

    loop For each <arg / <kwarg
        Runner->>Convert: convert(#Variable:Python → Rust)
        Convert-->>Runner: #Variable:Rust
    end

    Runner->>CalleeEnv: call <func with converted args
    CalleeEnv-->>Runner: return values (#Variable:Rust)

    Runner->>Caller: >Bind, >output

    Runner->>CalleeEnv: -W.Env cleanup
    Runner->>CallerEnv: -W.Env cleanup (or skip)
```

## Bridge.Script Call Flow

The call flow for `-Run.Bridge.Script` is similar but uses variable injection instead of function arguments:

```text
Step 1: Convert <Bind variables
   For each <Bind #Variable:
      -Variable.Convert(from=var, targetLang=callee_lang)
      Converted .name becomes native local variable in callee

Step 2: Inject and execute
   Runner injects converted variables into callee scope
   Callee script executes with injected variables

Step 3: Read back >Bind variables
   Runner reads .name values from callee scope
   Values tagged with callee's #NativeType

Step 4: Convert >Bind back to caller type system
   For each >Bind #Variable:
      -Variable.Convert(from=result, targetLang=caller_lang)
```

## Chaining

Bridge calls are standard `{-}` pipeline operations. Chaining A→B then B→C is two separate `[-]` calls wired through Aljam3 IO:

```text
{-} =PythonToRustToGo
   ...
   [W] -W.Env;PyEnv
   [W] -W.Env;RsEnv
   [W] -W.Env;GoEnv

   [ ]
   [-] -Run.Bridge.Function;PyEnv;RsEnv
      ... Python → Rust call ...
      (-) >Bind#array.Variable >> $rustResult

   [-] -Run.Bridge.Function;RsEnv;GoEnv
      ... Rust → Go call, using $rustResult ...
      (-) <arg#array.Variable << $rustResult
```

Each Bridge call is independent. The intermediate `$rustResult` is a Aljam3 variable holding `#array.Variable` instances tagged with Rust types. The second Bridge call converts these from Rust to Go automatically.

## Runner Dispatch

The Runner handles Bridge pipelines as a specialized form of Execution dispatch:

### Dual Dispatch

Each Bridge invocation involves **two** native dispatch calls (beyond the `-W.Env` wrapper dispatches):

1. **-Variable.Convert dispatch** — one call per argument/variable to convert between language type systems
2. **RunBridgeFunction/RunBridgeScript dispatch** — one call to execute the function/script in the callee environment

The Runner coordinates these calls sequentially. The conversion calls are standard Execution dispatches to the `VariableConvert` native function. The bridge execution call passes already-converted arguments to the callee.

### Subsystem Ownership

| Responsibility | Owner |
|---------------|-------|
| Wrapper lifecycle (setup/cleanup) | Runner (existing `-W.Env` dispatch) |
| Variable conversion | Runner (dispatches `-Variable.Convert` native op) |
| Function/script execution | Runner (dispatches `RunBridgeFunction`/`RunBridgeScript`) |
| Argument validation | Compiler (PGE01041-01043 at compile time) |
| Type mapping | `-Variable.Convert` native implementation |

## Performance Characteristics

### String Serialization Overhead

Bridge calls serialize all values through JSON strings twice: once for the conversion step (source → Aljam3) and once for the dispatch step (Aljam3 → callee). This adds overhead compared to single-language `-Run.*` calls.

| Scenario | Serialization Passes | Relative Cost |
|----------|---------------------|---------------|
| Single-language `-Run.Function` | 1 (Aljam3 → native) | Baseline |
| Bridge `-Run.Bridge.Function` | 2 (source → Aljam3 → target) | ~2x serialization |
| SDK `pg.call()` | 2 (foreign → Aljam3 → foreign, via NATS) | ~2x + NATS round-trip |

### When Bridge is Faster Than SDK

Bridge calls avoid the NATS round-trip that SDK `pg.call()` requires. For tight loops where a Python pipeline repeatedly calls Rust functions, Bridge executes within the Runner process, eliminating network latency:

```text
SDK path:  Python → NATS → TM → QH → Runner → Rust → NATS → Python
Bridge path: Runner(Python env) → Convert → Runner(Rust env) → Convert → Runner
```

### When SDK is Simpler

SDK is simpler for coarse-grained calls from external applications. Bridge is designed for **within-pipeline** cross-language calls where both environments are already managed by Aljam3 wrappers.

## Related

| Document | Relationship |
|----------|-------------|
| [[spec/native-dispatch\|Native Dispatch]] | JSON wire format and dispatch flow |
| [[spec/native-dispatch#Serialization Protocol\|Serialization Protocol]] | Value encoding format shared with Bridge |
| [[spec/aljam3-sdk\|Aljam3 SDK]] | Alternative cross-language mechanism (external to pipeline) |
| [[algorithms/foreign-code-analysis\|Foreign Code Analysis]] | AST analysis for binding validation |
| [[jm3lib/types/Variable\|#Variable]] | Language-tagged variable type |
| [[jm3lib/types/NativeType\|#NativeType]] | Native type marshalling table |
| [[jm3lib/pipelines/Run/Bridge.Function\|-Run.Bridge.Function]] | Pipeline definition |
| [[jm3lib/pipelines/Run/Bridge.Script\|-Run.Bridge.Script]] | Pipeline definition |
| [[jm3lib/pipelines/Variable/Convert\|-Variable.Convert]] | Conversion pipeline |
| [[jm3lib/pipelines/W/Env\|-W.Env]] | Environment wrapper lifecycle |
