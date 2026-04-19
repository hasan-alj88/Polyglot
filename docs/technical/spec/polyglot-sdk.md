---
audience: architect
type: spec
updated: 2026-04-19
status: draft
---

# Polyglot SDK

<!-- @c:spec/native-dispatch -->
<!-- @c:spec/behavior-contract -->
<!-- @c:spec/type-identity -->
<!-- @c:spec/compiler-floor -->
<!-- @c:glossary#Polyglot Service -->
<!-- @c:glossary#Runner -->
<!-- @c:glossary#Trigger Monitor -->
<!-- @c:glossary#Cross-Language Integration -->
Related: [[native-dispatch]], [[behavior-contract]], [[compiler-floor]], [[polyglot-interface]], [[integrator-internals]]

## Purpose

The Polyglot SDK is the interface between external applications and the [[glossary#Polyglot Service|c:Polyglot Service]]. It provides per-language libraries for:

- **Serializing** native variables into Polyglot's typed JSON wire format
- **Triggering** Polyglot pipelines from foreign code
- **Exchanging** data with running Polyglot jobs in real time

The SDK reuses the same JSON wire format defined in [[native-dispatch#Serialization Protocol]] — the SDK is a consumer of that protocol, not a separate serialization system.

### SDK Packages

| Language | Package Name | Registry |
|----------|-------------|----------|
| Python | `polyglot-sdk` | PyPI |
| Rust | `polyglot-sdk` | crates.io |
| Go | `polyglot.dev/sdk` | Go modules |
| JavaScript | `@polyglot/sdk` | npm |

## Universal String Algorithm

All data crossing language boundaries follows a single serialization path. This is the core algorithm the SDK implements.

### Flow

```text
Foreign Variable
    → to_polyglot(var, type)
        → JSON envelope (typed value)
            → NoSQL variable store
                → from_polyglot(str, type)
                    → Foreign Variable
```

### Principles

1. **Type fidelity is a compiler problem.** The wire format carries values only. The compiler emits [[#Type Mapping Descriptors]] in the [[behavior-contract|c:Behavior Contract]] that tell the Runner and SDK how to serialize each IO port — no runtime type inspection needed.

2. **All leaf values are strings.** This preserves Polyglot's "all data is serialized strings" principle (see [[spec/type-identity]]) across every language boundary. Numeric types use string representation to avoid floating-point precision loss.

3. **The JSON envelope is the same one used by native dispatch.** Each value is a typed JSON object matching the [[native-dispatch#Value Encoding]] format:

```json
{
  "type": "<polyglot_type>",
  "value": "<string_encoded_value>"
}
```

### Shared Memory Model

The NoSQL variable store is the shared memory between Polyglot and foreign code. Variables written by `to_polyglot` are stored in the NoSQL DB; variables read by `from_polyglot` are retrieved from it. There is no OS-level shared memory — all exchange goes through the store.

## SDK Public Interface

The SDK exposes four functions. Each function has a consistent contract across all supported languages.

### `to_polyglot(value, type) → str`

Serialize a native variable to a Polyglot JSON envelope string.

**Behavior:**
1. Accept a native value and a Polyglot type identifier
2. Apply the [[#Primitive Type Mapping Table]] to determine the JSON encoding
3. Return a JSON string matching the [[native-dispatch#Value Encoding]] format

**Language Signatures:**

```python
# Python
def to_polyglot(value: Any, polyglot_type: str) -> str:
    """Serialize native Python value to Polyglot JSON envelope."""
```

```rust
// Rust
pub fn to_polyglot(value: impl Into<PolyglotValue>, polyglot_type: &str) -> Result<String, SdkError>
```

```go
// Go
func ToPolyglot(value interface{}, polyglotType string) (string, error)
```

```javascript
// JavaScript
function toPolyglot(value, polyglotType) // returns string
```

**Errors:**
- `!SDK.TypeMismatch` — native value cannot be converted to the requested Polyglot type
- `!SDK.SerializationError` — JSON encoding failed

**Example:**

```python
envelope = to_polyglot(42, "int")
# Returns: '{"type": "int", "value": "42"}'

envelope = to_polyglot([1, 2, 3], "array:int")
# Returns: '{"type": "array:int", "value": [{"type": "int", "value": "1"}, {"type": "int", "value": "2"}, {"type": "int", "value": "3"}]}'
```

### `from_polyglot(str) → value`

Deserialize a Polyglot JSON envelope string to a native variable.

**Behavior:**
1. Parse the JSON envelope
2. Extract `type` and `value` fields
3. Apply the [[#Primitive Type Mapping Table]] in reverse to produce a native value

**Language Signatures:**

```python
# Python
def from_polyglot(envelope: str) -> Any:
    """Deserialize Polyglot JSON envelope to native Python value."""
```

```rust
// Rust
pub fn from_polyglot<T: FromPolyglot>(envelope: &str) -> Result<T, SdkError>
```

```go
// Go
func FromPolyglot(envelope string, target interface{}) error
```

```javascript
// JavaScript
function fromPolyglot(envelope) // returns native value
```

**Errors:**
- `!SDK.DeserializationError` — JSON parsing failed or envelope malformed
- `!SDK.TypeMismatch` — envelope type cannot be converted to the target native type

**Example:**

```python
value = from_polyglot('{"type": "int", "value": "42"}')
# Returns: 42 (Python int)

value = from_polyglot('{"type": "path", "value": "/home/user/data.txt"}')
# Returns: "/home/user/data.txt" (Python str)
```

### `call(pipeline, bindings) → result`

Trigger a Polyglot pipeline and wait for the result. This is a fire-and-wait operation.

**Behavior:**
1. Serialize `bindings` using `to_polyglot` for each input
2. Publish a NATS request on `polyglot.call.<pipeline_name>` with the serialized bindings
3. The [[glossary#Trigger Monitor|c:Trigger Monitor]] receives the request and activates the pipeline's `-T.Call` trigger
4. Wait for the response on `polyglot.result.<correlation_id>`
5. Deserialize outputs using `from_polyglot`
6. Return the result

**NATS Protocol:**

```text
Request topic:  polyglot.call.<pipeline_name>
Response topic: polyglot.result.<correlation_id>

Request payload:
{
  "correlation_id": "<uuid>",
  "pipeline": "<pipeline_name>",
  "inputs": {
    "<port_name>": { "type": "<type>", "value": "<value>" },
    ...
  }
}

Response payload (success):
{
  "correlation_id": "<uuid>",
  "status": "ok",
  "outputs": {
    "<port_name>": { "type": "<type>", "value": "<value>" },
    ...
  }
}

Response payload (error):
{
  "correlation_id": "<uuid>",
  "status": "error",
  "error": {
    "id": "<error_id>",
    "detail": "<human-readable message>"
  }
}
```

For the Trigger Monitor's processing of these NATS messages — including pipeline matching, binding storage in the NoSQL variable store, and result collection — see [[native-dispatch#-T.Call Signal Path]].

**Language Signatures:**

```python
# Python
async def call(pipeline: str, bindings: dict[str, Any]) -> dict[str, Any]:
    """Trigger a Polyglot pipeline via -T.Call and wait for result."""
```

```rust
// Rust
pub async fn call(pipeline: &str, bindings: HashMap<String, PolyglotValue>) -> Result<HashMap<String, PolyglotValue>, SdkError>
```

```go
// Go
func Call(ctx context.Context, pipeline string, bindings map[string]interface{}) (map[string]interface{}, error)
```

```javascript
// JavaScript
async function call(pipeline, bindings) // returns Promise<object>
```

**Errors:**
- `!SDK.PipelineNotFound` — pipeline name not registered in the Polyglot Service
- `!SDK.Timeout` — no response within configured timeout
- `!SDK.PipelineError` — pipeline returned an error (wraps the pipeline's error ID and detail)

**Example:**

```python
result = await call("ProcessData", {"input_path": "/data/raw.csv"})
# Sends NATS request to polyglot.call.ProcessData
# Waits for polyglot.result.<uuid>
# Returns: {"output_path": "/data/processed.csv", "row_count": 1500}
```

### `pull(name) → value` / `push(name, value)`

Exchange data with a running Polyglot job during `-Run.*.Bind` execution. Unlike `call()` which is fire-and-wait, `pull`/`push` provide ongoing data exchange while foreign code is executing inside a Polyglot pipeline.

**Behavior:**
- `pull(name)` reads a variable from the NoSQL store that Polyglot placed there for the current job
- `push(name, value)` writes a variable to the NoSQL store for Polyglot to consume

Both functions operate within the context of a `-Run.<Lang>.Bind` job. The Runner sets up the store keys before invoking the foreign code, and reads them after execution completes.

**Language Signatures:**

```python
# Python
def pull(name: str) -> Any:
    """Read a Polyglot variable placed for this Bind job."""

def push(name: str, value: Any) -> None:
    """Write a variable for Polyglot to consume from this Bind job."""
```

```rust
// Rust
pub fn pull(name: &str) -> Result<PolyglotValue, SdkError>
pub fn push(name: &str, value: impl Into<PolyglotValue>) -> Result<(), SdkError>
```

```go
// Go
func Pull(name string) (interface{}, error)
func Push(name string, value interface{}) error
```

```javascript
// JavaScript
function pull(name) // returns value
function push(name, value) // returns void
```

**Errors:**
- `!SDK.VariableNotFound` — `pull` requested a name not placed by the Runner
- `!SDK.NotInBindContext` — `pull`/`push` called outside a `-Run.*.Bind` job
- `!SDK.SerializationError` — `push` value cannot be serialized

**Example:**

```python
# Inside a -Run.Python.Bind job:
from polyglot_sdk import pull, push

data = pull("input_data")        # Read what Polyglot placed
result = process(data)           # Do work in Python
push("result", result)           # Write back for Polyglot
```

**Relationship to `-Run.*.Bind`:**
The Bind protocol is symmetric — direction is inferred from the trigger source. When Polyglot triggers `-Run.<Lang>.Bind`, it places `<Bind` inputs in the store; the foreign code reads them with `pull()`. The foreign code writes results with `push()`; Polyglot reads them as `>Bind` outputs. See [[pglib/pipelines/Run/Bind]] for the pipeline definition.

## Primitive Type Mapping Table

Bindings use only primitive types — no custom or user-defined types cross the SDK boundary.

| Polyglot | Python | Rust | Go | JavaScript |
|----------|--------|------|----|------------|
| `#int` | `int` | `i64` | `int64` | `number` |
| `#float` | `float` | `f64` | `float64` | `number` |
| `#string` | `str` | `String` | `string` | `string` |
| `#bool` | `bool` | `bool` | `bool` | `boolean` |
| `#path` | `str` | `PathBuf` | `string` | `string` |
| `#array:T` | `list[T]` | `Vec<T>` | `[]T` | `T[]` |
| `#serial` | `dict` | `serde_json::Value` | `map[string]any` | `object` |
| `#bytes` | `bytes` | `Vec<u8>` | `[]byte` | `Uint8Array` |
| `#dt` | `int` | `i64` | `int64` | `number` |

**Notes:**
- `#int` and `#float` are encoded as strings in the JSON envelope to preserve precision. The SDK converts to/from native numeric types.
- `#path` maps to `str`/`string` in most languages. Rust uses `PathBuf` for OS path semantics.
- `#serial` is Polyglot's unconstrained nested key-value type. It maps to the most general dictionary/map type in each language.
- `#array:T` preserves element type information. The `T` must be one of the other primitive types in this table.
- JavaScript's `number` type covers both `#int` and `#float`. The SDK uses the envelope's `type` field to distinguish them during deserialization.
- `#bytes` is Base64-encoded in the JSON envelope. The SDK converts to/from native byte types (e.g., Python `bytes`, Rust `Vec<u8>`).
- `#dt` represents DateTime as epoch seconds (integer string, e.g., `"1712494800"`). UTC only, seconds precision. The SDK converts to/from native integer types.

## Type Mapping Descriptors

The compiler emits **type mapping descriptors** as part of the [[behavior-contract|c:Behavior Contract]]. These descriptors tell the Runner and SDK exactly how to serialize/deserialize each IO port — no runtime type inspection needed.

### Descriptor Source

The type mapping descriptors are derived from the native registry's IO schema (see [[native-dispatch#Registry Entry Schema]]). The `inputs` and `outputs` arrays in each registry entry serve as the descriptors — they specify each port's name and Polyglot type.

### Descriptor Schema

For each IO port in a pipeline, the compiler emits:

```json
{
  "port_name": "path",
  "direction": "input",
  "polyglot_type": "path",
  "is_array": false,
  "element_type": null
}
```

| Field | Type | Description |
|-------|------|-------------|
| `port_name` | string | IO port name from pipeline definition |
| `direction` | string | `"input"` or `"output"` |
| `polyglot_type` | string | Polyglot type identifier (from [[#Primitive Type Mapping Table]]) |
| `is_array` | boolean | Whether this port carries an array type |
| `element_type` | string or null | Element type if `is_array` is true |

### Example

For a pipeline with mixed IO types:

```polyglot
{-} -ProcessData
   (-) <input_path#path
   (-) <batch_size#int
   (-) >rows#array:serial
   (-) >count#int
   (-) !File.NotFound
```

The compiler emits these descriptors in the Behavior Contract:

```json
{
  "pipeline": "ProcessData",
  "io_descriptors": [
    { "port_name": "input_path", "direction": "input", "polyglot_type": "path", "is_array": false, "element_type": null },
    { "port_name": "batch_size", "direction": "input", "polyglot_type": "int", "is_array": false, "element_type": null },
    { "port_name": "rows", "direction": "output", "polyglot_type": "array:serial", "is_array": true, "element_type": "serial" },
    { "port_name": "count", "direction": "output", "polyglot_type": "int", "is_array": false, "element_type": null }
  ]
}
```

The SDK reads these descriptors when handling `call()` requests — it knows how to serialize each input binding and deserialize each output value without inspecting the values at runtime.

## Pluggable Wire Format

The SDK abstracts serialization behind a pluggable interface. JSON is the current (and only) implementation. The interface exists to allow future adoption of more efficient binary formats.

### Interface

```text
serialize(value, port_descriptor) → bytes
deserialize(bytes, port_descriptor) → value
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `value` | native | The value to serialize (for `serialize`) or the deserialized result (for `deserialize`) |
| `port_descriptor` | object | The [[#Type Mapping Descriptors\|type mapping descriptor]] for this IO port |
| `bytes` | byte array | The serialized representation |

### JSON Implementation (Current)

The default implementation uses the JSON wire format from [[native-dispatch#Serialization Protocol]]. All values are wrapped in typed envelopes. Leaf values are strings.

### Future Candidates

| Format | Advantage | Status |
|--------|-----------|--------|
| JSON | Universal, human-readable, debuggable | **Current** |
| CBOR | Binary, compact, schema-compatible with JSON | Future candidate |
| MessagePack | Binary, faster serialization than JSON | Future candidate |

No dynamic code generation is required to switch wire formats — all implementations are pre-compiled and selected by configuration. The wire format is a deployment-time choice, not a compile-time one.

## SDK Distribution

### Package Names

| Language | Package | Install Command |
|----------|---------|-----------------|
| Python | `polyglot-sdk` | `pip install polyglot-sdk` |
| Rust | `polyglot-sdk` | `cargo add polyglot-sdk` |
| Go | `polyglot.dev/sdk` | `go get polyglot.dev/sdk` |
| JavaScript | `@polyglot/sdk` | `npm install @polyglot/sdk` |

### Version Locking

The SDK version is locked to the Polyglot compiler version. Both use the same version number. This ensures:

- Type mapping descriptors match what the SDK expects
- Wire format compatibility between compiler output and SDK serialization
- NATS topic structure matches between Trigger Monitor and SDK

A version mismatch between SDK and compiler produces an `!SDK.VersionMismatch` error at connection time.

### Codebase Location

SDK implementations live under `lib/<language>/integrator/` in the Polyglot source tree. See [[native-dispatch#lib/ Folder Structure]] for the full directory layout.

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| Polyglot's type system is the bridge | No per-language type catalog. Both sides map through Polyglot primitives. The [[#Primitive Type Mapping Table]] is the single source of truth. |
| All leaf values are strings | Preserves [[spec/type-identity\|Polyglot's "all data is serialized strings"]] principle across every boundary. Numeric precision is never lost. |
| Shared memory = NoSQL variable store | Not OS-level shared memory. All data exchange goes through the same NoSQL store the [[glossary#Polyglot Service\|c:Polyglot Service]] uses for pipeline state. |
| No dynamic code generation | All SDK code is pre-compiled. The [[#Pluggable Wire Format]] selects implementations by configuration, not by generating code at runtime. |
| Error propagation maps to `!Error` tree | SDK errors (`!SDK.*`) follow the same structure as Polyglot's error system. Callers handle SDK errors the same way they handle pipeline errors. |
| SDK version locked to compiler version | Prevents wire format drift. A single version number covers compiler output, SDK serialization, and NATS protocol. |

### Why No FFI in the SDK

FFI (Foreign Function Interface) between language pairs is significantly faster than serialized string exchange — it avoids JSON serialization overhead entirely. However, the SDK deliberately excludes FFI for three reasons:

1. **Dynamic code generation.** FFI requires generating language-specific bindings between arbitrary language pairs at build time or runtime. This violates the SDK's "no dynamic code generation" design principle — all SDK code must be pre-compiled.

2. **Not universally available.** FFI is feasible between some language pairs (e.g., Rust↔C, Python↔C via ctypes) but not all. A universal SDK cannot depend on a mechanism that only works for some combinations.

3. **Complexity vs. universality.** The universal string algorithm works identically for every supported language. FFI would require per-pair implementations, testing matrices, and maintenance — complexity that scales quadratically with language count.

**The trade-off is acknowledged:** FFI is faster. For performance-critical pairwise integration, Polyglot provides `-Run.Bridge` — a separate pipeline designed for direct code-to-code binding between specific language pairs. See [[pglib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] for cross-language function calls and [[pglib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] for cross-language variable binding.

The SDK and `-Run.Bridge` serve different needs: the SDK provides universal, zero-configuration integration; `-Run.Bridge` provides optimized, per-pair integration where performance justifies the setup cost. See [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] for the full conversion algorithm.

## Related

| Document | Relationship |
|----------|-------------|
| [[native-dispatch]] | Wire format definition, serialization protocol, native function contract |
| [[behavior-contract]] | Where type mapping descriptors live in compiled output |
| [[compiler-floor]] | Native operation categories ({N} block semantics) |
| [[polyglot-interface]] | User-facing SDK guide (integrator audience) |
| [[integrator-internals]] | Data casting methods and library architecture |
| [[spec/type-identity]] | "All data is serialized strings" foundation |
| [[pglib/pipelines/T/Call]] | `-T.Call` trigger used by `call()` |
| [[pglib/pipelines/Run/Bind]] | `-Run.*.Bind` pipeline used by `pull()`/`push()` |
| [[pglib/pipelines/Run/Bridge.Function]] | `-Run.Bridge.Function` — cross-language function call |
| [[pglib/pipelines/Run/Bridge.Script]] | `-Run.Bridge.Script` — cross-language variable binding |
| [[technical/algorithms/bridge-conversion]] | Bridge conversion algorithm for pairwise type mapping |
