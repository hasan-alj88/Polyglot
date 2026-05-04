---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
metadata_definition: "%definition.-:Run.Bridge.Script"
metadata_instance: "%-:Run.Bridge.Script:N"
---

# -Run.Bridge.Script

<!-- @c:jm3lib/pipelines/Run/INDEX -->
<!-- @c:jm3lib/types/Variable -->
<!-- @c:jm3lib/types/NativeType -->
<!-- @c:spec/native-dispatch#Serialization Protocol -->

Run code in one language with variables originating from another language. Variables are converted between language type systems automatically using [[jm3lib/pipelines/Variable/Convert|-Variable.Convert]].

This is the cross-language counterpart of [[jm3lib/pipelines/Run/Script|-Run.\<Lang\>.Script]], which operates within a single language. `-Run.Bridge.Script` requires two `-W.Env` wrappers -- one for each language environment.

## Definition

```aljam3
{N} -Run.Bridge.Script
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunBridgeScript"
   [%] .description << "Run code with cross-language variable bindings."
   (-) <Bind#array.Variable
   (-) >Bind#array.Variable
   (-) >output#Code:Output
   (-) <code#Code:Source
   (-) !Run.Bridge.IncompatibleType
   (-) !Run.Bridge.UnsupportedPair
   (-) !Run.Bridge.ConversionFailed
   (-) !Run.ExecutionError
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Bind` | `#array.Variable` | Input variables -- each `#Variable` tagged with caller's language type, converted to callee's type before injection |
| `<code` | `#Code:Source` | Script code in the callee language (inline `[C]` or file) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#array.Variable` | Output variables -- tagged with callee's language type, converted back to caller's type after execution |
| `>output` | `#Code:Output` | `.stdout`, `.stderr` capture from the callee environment |

## Dual-Wrapper Requirement

Bridge pipelines require two `-W.Env` wrappers. The `;Caller;Callee` dual-env syntax specifies both on the `[-]` call line:

```aljam3
[-] -Run.Bridge.Script;PyEnv;GoEnv
```

The first `;` environment is the **caller** (source of input variables). The second `;` environment is the **callee** (target where the script runs). The compiler validates that the two environments reference different languages (PGE01041).

### Wrapper Lifecycle

See [[jm3lib/pipelines/Run/Bridge.Function#Dual-Wrapper Requirement]] for the ordered setup/cleanup rules and skip-if-running semantics.

## Variable Binding

`<Bind` variables are converted from the caller's language type to the callee's language type before injection. Each `#Variable` instance's `.name` field becomes a native local variable in the callee code:

| `#Variable` Field | Purpose |
|--------------------|---------|
| `.name` | Native variable name in callee code |
| `.value` | Serialized value (converted to callee type) |
| `.type` | Source language native type (used by `-Variable.Convert`) |

The compiler validates that all `.name` values exist as identifiers in the callee `<code` (PGE01033 -- same rule as single-language Script).

After execution, `>Bind` variables are read back and converted from the callee's language type to the caller's language type.

## Compiler Validation

| Code | Name | Condition |
|------|------|-----------|
| PGE01033 | Unbound Script Variable | `<Bind` `.name` not found in callee `<code` |
| PGE01034 | Unbound Script Output | `>Bind` `.name` not found in callee `<code` |
| PGE01041 | Same Language Bridge | Both `;Caller` and `;Callee` resolve to the same language |
| PGE01042 | Incompatible Native Type | A `<Bind` `#Variable` type has no mapping to callee language |
| PGE01043 | Missing Bridge Env | Bridge pipeline missing second `;` environment on `[-]` call line |

**Note:** PGE01033/PGE01034 validation applies at compile time for `<code.inline` only. When `<code.file` is used, validation is deferred to runtime.

## Example

A Go pipeline injects variables into a Python ML script:

```aljam3
{@} @Registry:PG<MLBridge<1.0
   [.] .name << "MLBridge"

{;} ;GoApp
   [.] .language << #BaseCode.Go
   [.] .version << "1.23"

{;} ;PyML
   [.] .language << #BaseCode.Python
   [.] .version << "3.14"

{_} _BridgeGrant
   [.] .intent << #Grant
   [.] .System.Process "*"

{-} =GoToPythonML
   (-) _BridgeGrant
   (-) <features#array.float
   (-) <modelName#string
   (-) >predictions#array.float
   (-) >log#Code:Output
   (-) ;GoApp
   (-) ;PyML
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env;GoApp
   [W] -W.Env;PyML

   [ ]
   [-] -Run.Bridge.Script;GoApp;PyML
      (-) <Bind#array.Variable
         [.] .0#Variable
            [.] .name#string << "features"
            [.] .value#serial << $features
            [.] .type#NativeType.Go << .Slice
         [.] .1#Variable
            [.] .name#string << "model_name"
            [.] .value#serial << $modelName
            [.] .type#NativeType.Go << .string
      (-) >Bind#array.Variable
         [.] .0#Variable
            [.] .name#string << "predictions"
      (-) >output#Code:Output >> >log
      (-) <code.inline <<
         [C] import numpy as np
         [C] from sklearn.linear_model import LinearRegression
         [C] # features and model_name are injected as Python variables
         [C] X = np.array(features).reshape(-1, 1)
         [C] model = LinearRegression().fit(X, X * 2.5 + 1)
         [C] predictions = model.predict(X).tolist()
```

In this example:
- `$features` and `$modelName` originate in a Go context (caller is `;GoApp`)
- `<Bind` variables are tagged with `#NativeType.Go` types
- The Bridge converts `Go []float64 → Aljam3 #array.float → Python list` and `Go string → Aljam3 #string → Python str`
- The Python script receives `features` and `model_name` as native Python variables
- `>Bind` reads `predictions` back, tagged with `#NativeType.Python`, converted to Go type

### SDK Caller Side

From the Go application, the same pipeline is invoked through the [[aljam3-interface|c:aljam3-interface]] SDK:

```go
import pg "aljam3.dev/sdk"

result, err := pg.Call(ctx, "GoToPythonML", map[string]interface{}{
    "features":  []float64{1.0, 2.0, 3.0, 4.0, 5.0},
    "modelName": "linear_demo",
})
// result["predictions"] — the Python model's output as []float64
// result["log"] — stdout/stderr from the Python environment
```

The SDK serializes inputs using the universal string algorithm, publishes a NATS request to `-T.Call`, and deserializes the response. The Bridge conversion between Go and Python happens inside the Aljam3 Service -- the SDK caller does not manage language environments directly.

## When to Use

| Scenario | Use | Why |
|----------|-----|-----|
| Inject variables into a script in a **different** language | `-Run.Bridge.Script` | Pairwise conversion avoids JSON round-trip overhead |
| Run a script in the **same** language with variable injection | `-Run.<Lang>.Script` | No cross-language conversion needed |
| Call a Aljam3 pipeline from external code | SDK `call()` | Universal string, zero per-pair setup |
| ML/data science workloads across language boundaries | `-Run.Bridge.Script` | Direct type mapping for arrays, no string serialization |
| Universal integration across many languages | SDK `call()` | Works for all supported languages without per-pair config |

## Errors

| Error | Condition |
|-------|-----------|
| `!Run.Bridge.IncompatibleType` | A `#Variable` type has no mapping to the target language |
| `!Run.Bridge.UnsupportedPair` | The language pair is not implemented |
| `!Run.Bridge.ConversionFailed` | Value conversion failed at runtime (e.g., overflow) |
| `!Run.ExecutionError` | Callee script raised an exception or panicked |

## Permissions

Requires `System.Process` capability for **both** language environments.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.Bridge.Script` | Compile-time pipeline template |
| Instance | `%-:Run.Bridge.Script:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]] -- parent namespace
- [[jm3lib/pipelines/Run/Script|-Run.\<Lang\>.Script]] -- single-language script execution
- [[jm3lib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] -- cross-language function call
- [[jm3lib/pipelines/Variable/Convert|-Variable.Convert]] -- variable conversion pipeline
- [[jm3lib/types/Variable|#Variable]] -- language-tagged variable type
- [[jm3lib/types/NativeType|#NativeType]] -- native type marshalling
- [[jm3lib/pipelines/W/Env|-W.Env]] -- environment wrapper
- [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]]
