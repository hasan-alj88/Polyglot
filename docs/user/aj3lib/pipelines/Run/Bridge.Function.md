---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
metadata_definition: "%definition.-:Run.Bridge.Function"
metadata_instance: "%-:Run.Bridge.Function:N"
---

# -Run.Bridge.Function

<!-- @c:aj3lib/pipelines/Run/INDEX -->
<!-- @c:aj3lib/types/Variable -->
<!-- @c:aj3lib/types/NativeType -->
<!-- @c:spec/native-dispatch#Serialization Protocol -->

Call a named function in foreign code where the **caller** and **callee** run in different language environments. The Bridge handles variable conversion between the two languages automatically using [[aj3lib/pipelines/Variable/Convert|-Variable.Convert]].

This is the cross-language counterpart of [[aj3lib/pipelines/Run/Function|-Run.\<Lang\>.Function]], which operates within a single language. `-Run.Bridge.Function` requires two `-W.Env` wrappers -- one for each language environment.

## Definition

```aljam3
{N} -Run.Bridge.Function
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunBridgeFunction"
   [%] .description << "Call a named function across language boundaries."
   (-) <func#string
   (-) <arg#array.Variable
   (-) <kwarg#Record.String.Variable
   (-) >Bind#array.Variable
   (-) >output#Code:Output
   (-) <code#Code:Source
   (-) !Run.Bridge.IncompatibleType
   (-) !Run.Bridge.UnsupportedPair
   (-) !Run.Bridge.ConversionFailed
   (-) !Run.FunctionNotFound
   (-) !Run.ExecutionError
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<func` | `#string` | Function name in the callee code (compiler-validated against `<code`) |
| `<arg` | `#array.Variable` | Positional arguments -- each `#Variable` tagged with caller's language type |
| `<kwarg` | `#Record.String.Variable` | Named arguments -- field names = parameter names, values are `#Variable` instances |
| `<code` | `#Code:Source` | Function definition in the callee language (inline `[C]` or file) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#array.Variable` | Return values from the callee, tagged with callee's language type |
| `>output` | `#Code:Output` | `.stdout`, `.stderr` capture from the callee environment |

## Dual-Wrapper Requirement

Bridge pipelines require two `-W.Env` wrappers -- one for the caller's language environment and one for the callee's. The two environments are specified using the `;Caller;Callee` dual-env syntax on the `[-]` call line:

```aljam3
[-] -Run.Bridge.Function;PyEnv;RsEnv
```

The first `;` environment is the **caller** (source of arguments). The second `;` environment is the **callee** (target where the function runs). The compiler validates that the two environments reference different languages (PGE01041).

### Wrapper Lifecycle

The dual-wrapper follows ordered setup and cleanup:

- **Setup** (top-to-bottom): Caller env starts first, then callee env
- **Cleanup** (bottom-to-top): Callee env shuts down first, then caller env
- **Skip-if-running**: If an environment is already active from an outer wrapper scope, setup is skipped for that environment

See [[technical/algorithms/bridge-conversion#Dual-Wrapper Lifecycle]] for the full lifecycle algorithm.

## Compiler Validation

| Code | Name | Condition |
|------|------|-----------|
| PGE01035 | Unbound Function Argument | `<func` name not found in callee `<code` |
| PGE01041 | Same Language Bridge | Both `;Caller` and `;Callee` resolve to the same language |
| PGE01042 | Incompatible Native Type | A `<arg`/`<kwarg` `#Variable` type has no mapping to callee language |
| PGE01043 | Missing Bridge Env | Bridge pipeline missing second `;` environment on `[-]` call line |

**Note:** PGE01035 validation applies at compile time for `<code.inline` only. When `<code.file` is used, validation is deferred to runtime.

## Example

A Python pipeline calls a Rust function to process data:

```aljam3
{@} @Registry:PG<CrossLangDemo<1.0
   [.] .name << "CrossLangDemo"

{;} ;PyData
   [.] .language << #BaseCode.Python
   [.] .version << "3.14"

{;} ;RsProc
   [.] .language << #BaseCode.Rust
   [.] .version << "1.85"

{_} _BridgeGrant
   [.] .intent << #Grant
   [.] .System.Process "*"

{-} =CrossLangProcess
   (-) _BridgeGrant
   (-) <inputData#array.float
   (-) >processedResult#serial
   (-) >log#Code:Output
   (-) ;PyData
   (-) ;RsProc
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env;PyData
   [W] -W.Env;RsProc

   [ ]
   [-] -Run.Bridge.Function;PyData;RsProc
      (-) <func#string << "process_batch"
      (-) <arg#array.Variable
         [.] .0#Variable
            [.] .name#string << "data"
            [.] .value#serial << $inputData
            [.] .type#NativeType.Python << .list
      (-) >Bind#array.Variable >> >processedResult
      (-) >output#Code:Output >> >log
      (-) <code.inline <<
         [C] use serde_json::Value;
         [C] fn process_batch(data: Vec<f64>) -> Value {
         [C]     let sum: f64 = data.iter().sum();
         [C]     serde_json::json!({"sum": sum, "count": data.len()})
         [C] }
```

In this example:
- `$inputData` originates in a Python context (caller is `;PyData`)
- The `<arg` `#Variable` is tagged with `.type#NativeType.Python.list`
- The Bridge converts `Python list → Aljam3 #array.float → Rust Vec<f64>` internally
- The callee function runs in the `;RsProc` Rust environment
- Return values in `>Bind` are tagged with `.type#NativeType.Rust`

### SDK Caller Side

From the Python application, the same pipeline is invoked through the [[aljam3-interface|c:aljam3-interface]] SDK:

```python
import aljam3_sdk as pg

result = await pg.call("CrossLangProcess", {
    "inputData": [1.5, 2.7, 3.14, 4.0]
})
# result["processedResult"] — the Rust function's return value
# result["log"] — stdout/stderr from the Rust environment
```

The SDK serializes `inputData` using the universal string algorithm, publishes a NATS request to `-T.Call`, and deserializes the response. The Bridge conversion between Python and Rust happens inside the Aljam3 Service -- the SDK caller does not manage language environments directly.

## When to Use

| Scenario | Use | Why |
|----------|-----|-----|
| Call a function in a **different** language with type conversion | `-Run.Bridge.Function` | Pairwise conversion avoids JSON round-trip overhead |
| Call a function in the **same** language | `-Run.<Lang>.Function` | No cross-language conversion needed |
| Call a Aljam3 pipeline from external code | SDK `call()` | Universal string, zero per-pair setup |
| Performance-critical cross-language integration | `-Run.Bridge.Function` | Direct type mapping, no string serialization |
| Universal integration across many languages | SDK `call()` | Works for all supported languages without per-pair config |

## Relationship to Single-Language -Run.Function

| Aspect | `-Run.<Lang>.Function` | `-Run.Bridge.Function` |
|--------|------------------------|------------------------|
| Environments | One (`<env`) | Two (`;Caller;Callee`) |
| Argument types | `#Record` (native fields) | `#array.Variable` (language-tagged) |
| Type conversion | Implicit (single language) | Explicit via `-Variable.Convert` |
| Wrappers | One `[W] -W.Env` | Two `[W] -W.Env` |
| Compiler validation | PGE01035-01038 | PGE01035, PGE01041-01043 |

## Errors

| Error | Condition |
|-------|-----------|
| `!Run.Bridge.IncompatibleType` | A `#Variable` type has no mapping to the target language |
| `!Run.Bridge.UnsupportedPair` | The language pair is not implemented |
| `!Run.Bridge.ConversionFailed` | Value conversion failed at runtime (e.g., overflow) |
| `!Run.FunctionNotFound` | `<func` name not found in callee code at runtime |
| `!Run.ExecutionError` | Callee function raised an exception or panicked |

## Permissions

Requires `System.Process` capability for **both** language environments.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.Bridge.Function` | Compile-time pipeline template |
| Instance | `%-:Run.Bridge.Function:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]] -- parent namespace
- [[aj3lib/pipelines/Run/Function|-Run.\<Lang\>.Function]] -- single-language function call
- [[aj3lib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] -- cross-language variable binding
- [[aj3lib/pipelines/Variable/Convert|-Variable.Convert]] -- variable conversion pipeline
- [[aj3lib/types/Variable|#Variable]] -- language-tagged variable type
- [[aj3lib/types/NativeType|#NativeType]] -- native type marshalling
- [[aj3lib/pipelines/W/Env|-W.Env]] -- environment wrapper
- [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]]
