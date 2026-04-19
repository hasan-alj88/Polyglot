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
<!-- @c:pglib/types/schemas/Inf -->
<!-- @c:pglib/types/schemas/Nullable -->
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

## Per-Language Encode/Decode

<!-- @c:spec/native-dispatch#Value Encoding -->
Related: [[native-dispatch#Value Encoding]]

The Primitive Type Mapping Table above shows type correspondences between Polyglot and each supported language. This section shows how each SDK actually serializes and deserializes values — the concrete encode and decode functions that implement the universal string algorithm.

### Python

**String representation:**

| Polyglot Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `str` | Value as-is | `"hello world"` |
| `#string` | `str` | Value as-is | `"hello world"` |
| `#int` | `int` | `str(value)` | `"42"`, `"-7"`, `"0"` |
| `#float` | `float` | `repr(value)` (full precision) | `"3.14"`, `"-0.001"`, `"inf"`, `"nan"` |
| `#bool` | `bool` | `"True"` / `"False"` (Python capitalization) | `"True"` |
| `#path` | `str` | OS-native path string | `"/home/user/file.txt"` |
| `#dt` | `int` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `dict` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `list[T]` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `bytes` | Base64-encoded string | `"SGVsbG8="` |
| enum | `str` | Variant name as string | `"Active"` |
| (none) | `NoneType` | Empty string `""` | `""` |

**Encode (Python -> Polyglot String):**

```python
def to_polyglot(value, polyglot_type: str) -> str:
    """Serialize a Python value to Polyglot universal string."""
    match polyglot_type:
        case "RawString" | "string" | "path":
            return str(value)
        case "int":
            return str(int(value))
        case "float":
            return repr(float(value))  # full precision
        case "bool":
            return "True" if value else "False"
        case "bytes":
            return base64.b64encode(value).decode("ascii")
        case "dt":
            return str(int(value))  # epoch seconds
        case "none":
            return ""
        case _:
            # enum — variant name as string
            # value is a Python str matching the enum variant
            return str(value)
```

**Decode (Polyglot String -> Python):**

```python
def from_polyglot(envelope: dict) -> Any:
    """Deserialize Polyglot universal string to Python value."""
    t, v = envelope["type"], envelope["value"]
    match t:
        case "RawString" | "string" | "path":
            return v
        case "int":
            return int(v)
        case "float":
            return float(v)  # handles "inf", "-inf", "nan"
        case "bool":
            return v == "True"
        case "bytes":
            return base64.b64decode(v)
        case "dt":
            return int(v)  # epoch seconds
        case "none":
            return None
        case "serial":
            return {k: from_polyglot(child) for k, child in v.items()}
        case _ if t.startswith("array:"):
            return [from_polyglot(item) for item in v]
        case _:
            # enum — return variant name as string
            return v
```

**Enum handling:** Python represents enums as plain strings. The Behavior Contract lists valid variants — the SDK can optionally validate against them, but the base decode returns the string. Users who want `enum.Enum` types wrap the result themselves.

### Rust

**String representation:**

| Polyglot Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `String` | Value as-is | `"hello world"` |
| `#string` | `String` | Value as-is | `"hello world"` |
| `#int` | `i64` | `value.to_string()` | `"42"`, `"-7"`, `"0"` |
| `#float` | `f64` | Full-precision decimal (`format!("{}", value)`) | `"3.14"`, `"-0.001"` |
| `#bool` | `bool` | `"True"` / `"False"` (Polyglot convention, NOT Rust's `"true"`) | `"True"` |
| `#path` | `PathBuf` | `value.to_string_lossy()` | `"/home/user/file.txt"` |
| `#dt` | `i64` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `serde_json::Value` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `Vec<T>` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `Vec<u8>` | Base64-encoded string | `"SGVsbG8="` |
| enum | `String` | Variant name as string | `"Active"` |
| (none) | `Option::None` | Empty string `""` | `""` |

**Encode (`ToPolyglot` trait):**

```rust
pub trait ToPolyglot {
    fn to_polyglot(&self, polyglot_type: &str) -> Result<String, PolyglotError>;
}

impl ToPolyglot for i64 {
    fn to_polyglot(&self, _: &str) -> Result<String, PolyglotError> {
        Ok(self.to_string())  // "42"
    }
}

impl ToPolyglot for f64 {
    fn to_polyglot(&self, _: &str) -> Result<String, PolyglotError> {
        Ok(format!("{}", self))  // ryu-style shortest round-trip
    }
}

impl ToPolyglot for bool {
    fn to_polyglot(&self, _: &str) -> Result<String, PolyglotError> {
        Ok(if *self { "True" } else { "False" }.to_string())
    }
}

impl ToPolyglot for String {
    fn to_polyglot(&self, _: &str) -> Result<String, PolyglotError> {
        Ok(self.clone())  // string, RawString, path, enum
    }
}

impl ToPolyglot for PathBuf {
    fn to_polyglot(&self, _: &str) -> Result<String, PolyglotError> {
        Ok(self.to_string_lossy().into_owned())
    }
}

impl ToPolyglot for Vec<u8> {
    fn to_polyglot(&self, _: &str) -> Result<String, PolyglotError> {
        Ok(base64::encode(self))
    }
}

impl<T: ToPolyglot> ToPolyglot for Vec<T> {
    fn to_polyglot(&self, polyglot_type: &str) -> Result<String, PolyglotError> {
        // Serialized as JSON array of typed envelopes by the caller
        // Each element calls T::to_polyglot individually
        unimplemented!("array serialization handled by Envelope builder")
    }
}

impl<T: ToPolyglot> ToPolyglot for Option<T> {
    fn to_polyglot(&self, polyglot_type: &str) -> Result<String, PolyglotError> {
        match self {
            Some(v) => v.to_polyglot(polyglot_type),
            None => Ok(String::new()),  // ""
        }
    }
}
// HashMap<String, serde_json::Value> — serialized as nested typed JSON by Envelope builder
```

**Decode (`FromPolyglot` trait):**

```rust
pub fn from_polyglot<T: FromPolyglot>(envelope: &Envelope) -> Result<T, PolyglotError> {
    T::from_polyglot(envelope)
}

pub trait FromPolyglot: Sized {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError>;
}

impl FromPolyglot for i64 {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        match envelope.type_name.as_str() {
            "int" => envelope.value.parse::<i64>()
                .map_err(|_| PolyglotError::TypeOverflow),
            _ => Err(PolyglotError::TypeMismatch),
        }
    }
}

impl FromPolyglot for f64 {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        match envelope.type_name.as_str() {
            "float" => {
                match envelope.value.as_str() {
                    "Infinity" | "inf" | "+Inf" => Ok(f64::INFINITY),
                    "-Infinity" | "-inf" | "-Inf" => Ok(f64::NEG_INFINITY),
                    "NaN" | "nan" => Ok(f64::NAN),
                    v => v.parse::<f64>().map_err(|_| PolyglotError::InvalidValue),
                }
            }
            _ => Err(PolyglotError::TypeMismatch),
        }
    }
}

impl FromPolyglot for bool {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        match envelope.value.as_str() {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => Err(PolyglotError::InvalidValue),
        }
    }
}

impl FromPolyglot for String {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        match envelope.type_name.as_str() {
            "string" | "RawString" | "path" => Ok(envelope.value.clone()),
            _ => Err(PolyglotError::TypeMismatch),
        }
    }
}

impl FromPolyglot for PathBuf {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        match envelope.type_name.as_str() {
            "path" => Ok(PathBuf::from(&envelope.value)),
            _ => Err(PolyglotError::TypeMismatch),
        }
    }
}

impl FromPolyglot for Vec<u8> {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        match envelope.type_name.as_str() {
            "bytes" => base64::decode(&envelope.value)
                .map_err(|_| PolyglotError::InvalidValue),
            _ => Err(PolyglotError::TypeMismatch),
        }
    }
}

impl<T: FromPolyglot> FromPolyglot for Option<T> {
    fn from_polyglot(envelope: &Envelope) -> Result<Self, PolyglotError> {
        if envelope.type_name == "none" || envelope.value.is_empty() {
            Ok(None)
        } else {
            T::from_polyglot(envelope).map(Some)
        }
    }
}
// Vec<T> and HashMap deserialization handled by Envelope parser
// (walks JSON array/object, calls T::from_polyglot per element)
```

**Enum handling:** The compiler code-generates native Rust enum types from the Behavior Contract's variant list. The SDK provides a `#[derive(PolyglotEnum)]` macro to generate both `ToPolyglot` and `FromPolyglot` trait implementations automatically.

**Design note:** No `PolyglotValue` enum is needed. The Behavior Contract tells the Runner which type each port expects, so the SDK always calls with the correct generic parameter. Runtime type switching is unnecessary.

### Go

**String representation:**

| Polyglot Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `string` | Value as-is | `"hello world"` |
| `#string` | `string` | Value as-is | `"hello world"` |
| `#int` | `int64` | `strconv.FormatInt(value, 10)` | `"42"`, `"-7"`, `"0"` |
| `#float` | `float64` | `strconv.FormatFloat(value, 'g', -1, 64)` | `"3.14"`, `"-0.001"` |
| `#bool` | `bool` | `"True"` / `"False"` (Polyglot convention, NOT Go's `"true"`) | `"True"` |
| `#path` | `string` | OS-native path string | `"/home/user/file.txt"` |
| `#dt` | `int64` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `map[string]any` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `[]T` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `[]byte` | Base64-encoded string | `"SGVsbG8="` |
| enum | `string` | Variant name as string | `"Active"` |
| (none) | `nil` | Empty string `""` | `""` |

**Encode (`ToPolyglot` function with type switch):**

```go
func ToPolyglot(value any, polyglotType string) (string, error) {
    switch polyglotType {
    case "RawString", "string", "path":
        return value.(string), nil
    case "int":
        return strconv.FormatInt(value.(int64), 10), nil
    case "float":
        return strconv.FormatFloat(value.(float64), 'g', -1, 64), nil
    case "bool":
        if value.(bool) { return "True", nil }
        return "False", nil
    case "bytes":
        return base64.StdEncoding.EncodeToString(value.([]byte)), nil
    case "dt":
        return strconv.FormatInt(value.(int64), 10), nil  // epoch seconds
    case "none":
        return "", nil
    default:
        // enum — variant name as string
        return value.(string), nil
    }
}
// serial, array encoding handled by Envelope builder
// (walks map/slice recursively, calls ToPolyglot per element)
```

**Decode (per-type functions):**

```go
func FromPolyglotInt(envelope Envelope) (int64, error) {
    if envelope.Type != "int" {
        return 0, ErrTypeMismatch
    }
    return strconv.ParseInt(envelope.Value, 10, 64)
}

func FromPolyglotFloat(envelope Envelope) (float64, error) {
    if envelope.Type != "float" {
        return 0, ErrTypeMismatch
    }
    return strconv.ParseFloat(envelope.Value, 64)
}

func FromPolyglotBool(envelope Envelope) (bool, error) {
    switch envelope.Value {
    case "True":  return true, nil
    case "False": return false, nil
    default:      return false, ErrInvalidValue
    }
}

func FromPolyglotBytes(envelope Envelope) ([]byte, error) {
    if envelope.Type != "bytes" {
        return nil, ErrTypeMismatch
    }
    return base64.StdEncoding.DecodeString(envelope.Value)
}

func FromPolyglotString(envelope Envelope) (string, error) {
    switch envelope.Type {
    case "string", "RawString", "path":
        return envelope.Value, nil
    default:
        return "", ErrTypeMismatch
    }
}

func FromPolyglotDt(envelope Envelope) (int64, error) {
    if envelope.Type != "dt" {
        return 0, ErrTypeMismatch
    }
    return strconv.ParseInt(envelope.Value, 10, 64)
}

func FromPolyglotNone(envelope Envelope) (bool, error) {
    if envelope.Type == "none" || envelope.Value == "" {
        return true, nil
    }
    return false, nil
}

func FromPolyglotEnum(envelope Envelope) (string, error) {
    return envelope.Value, nil
}

// FromPolyglotSerial and FromPolyglotArray require recursive
// envelope parsing — the Envelope parser walks the JSON structure
// and calls the appropriate FromPolyglot* function per element.
```

**Enum handling:** The compiler code-generates `type Status string` with a const block and a validation map from the Behavior Contract's variant list.

### JavaScript

**String representation:**

| Polyglot Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `string` | Value as-is | `"hello world"` |
| `#string` | `string` | Value as-is | `"hello world"` |
| `#int` | `number` | `String(value)` | `"42"`, `"-7"`, `"0"` |
| `#float` | `number` | `String(value)` | `"3.14"`, `"-0.001"` |
| `#bool` | `boolean` | `"True"` / `"False"` (Polyglot convention, NOT JS `"true"`) | `"True"` |
| `#path` | `string` | OS-native path string | `"/home/user/file.txt"` |
| `#dt` | `number` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `Object` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `Array` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `Uint8Array` | Base64-encoded string | `"SGVsbG8="` |
| enum | `string` | Variant name as string | `"Active"` |
| (none) | `null` | Empty string `""` | `""` |

**Encode (`toPolyglot` function):**

```javascript
function toPolyglot(value, polyglotType) {
    switch (polyglotType) {
        case "RawString": case "string": case "path":
            return String(value);
        case "int":
            if (!Number.isSafeInteger(value)) throw new PolyglotError("TypeOverflow");
            return String(value);
        case "float":
            return String(value);
        case "bool":
            return value ? "True" : "False";
        case "bytes":
            // value is Uint8Array
            return btoa(String.fromCharCode(...value));
        case "dt":
            return String(Math.floor(value));  // epoch seconds
        case "none":
            return "";
        default:
            // enum — variant name as string
            return String(value);
    }
}
```

**Decode (`fromPolyglot` function):**

```javascript
function fromPolyglot(envelope) {
    const { type, value } = envelope;
    switch (type) {
        case "RawString": case "string": case "path":
            return value;
        case "int": {
            const n = parseInt(value, 10);
            if (n > Number.MAX_SAFE_INTEGER || n < Number.MIN_SAFE_INTEGER) {
                throw new PolyglotError("TypeOverflow");
            }
            return n;
        }
        case "float":
            return parseFloat(value);  // handles "Infinity", "-Infinity", "NaN"
        case "bool":
            return value === "True";
        case "bytes":
            return Uint8Array.from(atob(value), c => c.charCodeAt(0));
        case "dt":
            return parseInt(value, 10);  // epoch seconds
        case "none":
            return null;
        case "serial":
            return Object.fromEntries(
                Object.entries(value).map(([k, v]) => [k, fromPolyglot(v)])
            );
        default:
            if (type.startsWith("array:")) {
                return value.map(item => fromPolyglot(item));
            }
            // enum — return variant name as string
            return value;
    }
}
```

**Enum handling:** JavaScript represents enums as plain strings (like Python). For optional validation, the SDK can accept a `validVariants` set:

```javascript
function fromPolyglotEnum(envelope, validVariants) {
    if (validVariants && !validVariants.has(envelope.value)) {
        throw new PolyglotError("InvalidVariant");
    }
    return envelope.value;
}

// Usage with Behavior Contract variant list:
const statusVariants = new Set(["Active", "Inactive", "Pending"]);
const status = fromPolyglotEnum(envelope, statusVariants);
```

### Strongly vs Weakly Typed Design Summary

| Concern | Weakly Typed (Python, JS) | Strongly Typed (Rust, Go) |
|---|---|---|
| **Encode** | Single function, `polyglot_type` param selects branch | Trait/interface per type, or type-switch on `any` |
| **Decode** | Single function returning dynamic type, switch on `type` field | Per-type functions or generics, caller specifies target type |
| **Type safety** | Runtime — caller must cast/check the return | Compile-time — wrong type = compile error (Rust) or wrong function call (Go) |
| **Who knows the type?** | The envelope's `type` field at runtime | The Behavior Contract at compile time; envelope `type` validated at runtime |
| **How type is determined** | Runtime switch on envelope | Compiler analyzes foreign AST or user declares type explicitly; Behavior Contract pre-selects the decode function |
| **Error on mismatch** | Possible but rare (dynamic) | Required — `TypeMismatch` error if envelope type != expected |

### Wire Format Conventions

<!-- @c:spec/native-dispatch#Value Encoding -->
Related: [[native-dispatch#Value Encoding]], [[pglib/types/schemas/Inf\|##Inf]], [[pglib/types/schemas/Nullable\|##Nullable]]

The SDK normalizes all values to canonical wire format strings before serialization. Each SDK must produce identical wire output regardless of language-native representations. The canonical forms are defined by [[native-dispatch#Value Encoding]] — the SDK is a consumer of that protocol.

#### Boolean

Canonical wire format: `"True"` / `"False"` (Python capitalization).

All SDKs normalize to this convention regardless of language-native boolean string forms. The canonical form is defined in [[native-dispatch#Value Encoding]].

| Language | Native `true` | Native `false` | SDK Writes |
|---|---|---|---|
| Python | `True` | `False` | `"True"` / `"False"` (no conversion needed) |
| Rust | `true` | `false` | `"True"` / `"False"` |
| Go | `true` | `false` | `"True"` / `"False"` |
| JavaScript | `true` | `false` | `"True"` / `"False"` |

On deserialization (`from_polyglot`), SDKs accept only the canonical forms `"True"` and `"False"`. Any other string (e.g., `"true"`, `"1"`, `"yes"`) produces `!SDK.InvalidValue`.

#### Float Special Values

Float special values are **governed by schema** — not all float fields support them. The schema properties [[pglib/types/schemas/Nullable\|##Nullable]] and [[pglib/types/schemas/Inf\|##Inf]] control which special values a field accepts:

- `##Nullable` enables `""` (empty string = none/null)
- `##Inf` enables `"Infinity"`, `"-Infinity"`, `"NaN"`

A float field without these schema properties rejects special values at compile time.

**Canonical wire forms and schema requirements:**

| Value | Canonical Wire String | Schema Requirement |
|---|---|---|
| None/null | `""` | Field must have `##Nullable` |
| +Infinity | `"Infinity"` | Field must have `##Inf` |
| -Infinity | `"-Infinity"` | Field must have `##Inf` |
| NaN | `"NaN"` | Field must have `##Inf` |

**Per-language normalization (encode — `to_polyglot`):**

SDKs must normalize language-native representations to the canonical wire forms on serialization:

| Value | Canonical | Python produces | Rust produces | Go produces | JS produces |
|---|---|---|---|---|---|
| +Infinity | `"Infinity"` | `"inf"` | `"inf"` | `"+Inf"` | `"Infinity"` |
| -Infinity | `"-Infinity"` | `"-inf"` | `"-inf"` | `"-Inf"` | `"-Infinity"` |
| NaN | `"NaN"` | `"nan"` | `"NaN"` | `"NaN"` | `"NaN"` |

Python, Rust, and Go SDKs must detect their native special value strings and replace them with the canonical form before writing to the wire. JavaScript already produces canonical forms natively.

**Per-language acceptance (decode — `from_polyglot`):**

On deserialization, SDKs accept all common variants and convert to the native special value:

| Canonical | Also Accepted | Python Result | Rust Result | Go Result | JS Result |
|---|---|---|---|---|---|
| `"Infinity"` | `"inf"`, `"+Inf"`, `"+inf"` | `float('inf')` | `f64::INFINITY` | `math.Inf(1)` | `Infinity` |
| `"-Infinity"` | `"-inf"`, `"-Inf"` | `float('-inf')` | `f64::NEG_INFINITY` | `math.Inf(-1)` | `-Infinity` |
| `"NaN"` | `"nan"`, `"NAN"` | `float('nan')` | `f64::NAN` | `math.NaN()` | `NaN` |

#### Null/None

Wire format: empty string `""` with type `"none"`. This is consistent with `###None` field type semantics — the absence of a value is represented as an empty string, not a JSON `null`.

| Language | Native Null | Wire Encoding |
|---|---|---|
| Python | `None` | `{ "type": "none", "value": "" }` |
| Rust | `Option::None` | `{ "type": "none", "value": "" }` |
| Go | `nil` | `{ "type": "none", "value": "" }` |
| JavaScript | `null` | `{ "type": "none", "value": "" }` |

On deserialization, SDKs detect either `type == "none"` or an empty `value` string and return the language-native null type. Both conditions independently signal none — this provides resilience against partial envelope construction.

#### Bytes

Base64-encoded string. The SDK converts between native byte types (`bytes`, `Vec<u8>`, `[]byte`, `Uint8Array`) and the Base64 string representation transparently.

#### DateTime

Epoch seconds as string (e.g., `"1712494800"`). UTC only, seconds precision.

| Language | Encode | Decode | Precision |
|---|---|---|---|
| Python | `str(int(value.timestamp()))` | `int(value)` | seconds |
| Rust | `value.to_string()` (i64 epoch) | `value.parse::<i64>()` | seconds |
| Go | `strconv.FormatInt(value, 10)` | `strconv.ParseInt(value, 10, 64)` | seconds |
| JavaScript | `String(Math.floor(value / 1000))` (Date.now() is ms) | `parseInt(value, 10)` | seconds |

#### JavaScript Integer Overflow

The SDK errors on values exceeding 2^53 - 1 (`Number.MAX_SAFE_INTEGER`) rather than allowing silent precision loss.

### Behavior Contract Type Selection

<!-- @c:spec/behavior-contract#Type Mapping Descriptors -->
Related: [[behavior-contract#Type Mapping Descriptors]]

The encode/decode functions above are not selected at random — the Behavior Contract drives the selection:

1. The compiler emits type mapping descriptors in the Behavior Contract for every IO port in every pipeline.
2. At job dispatch time, the Runner reads these descriptors to determine the Polyglot type of each port.
3. The SDK uses the descriptor's type to call the correct encode or decode function for that port.
4. Strongly typed SDKs (Rust, Go) never need runtime type guessing — the Behavior Contract determines the type at compile time and the SDK calls the appropriate typed function directly.

This means the `type` field in the JSON envelope is redundant for strongly typed SDKs — it serves as a runtime validation check, not a dispatch mechanism. For weakly typed SDKs (Python, JS), the `type` field is the primary dispatch mechanism since these languages lack compile-time type selection.

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
