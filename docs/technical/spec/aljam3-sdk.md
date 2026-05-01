---
audience: design
type: spec
updated: 2026-04-19
status: draft
---

# Aljam3 SDK

<!-- @c:spec/native-dispatch -->
<!-- @c:spec/behavior-contract -->
<!-- @c:spec/type-identity -->
<!-- @c:aj3lib/types/schemas/Inf -->
<!-- @c:aj3lib/types/schemas/Nullable -->
<!-- @c:spec/compiler-floor -->
<!-- @c:glossary#Aljam3 Service -->
<!-- @c:glossary#Runner -->
<!-- @c:glossary#Trigger Monitor -->
<!-- @c:glossary#Cross-Language Integration -->
Related: [[native-dispatch]], [[behavior-contract]], [[compiler-floor]], [[aljam3-interface]], [[integrator-internals]]

## Purpose

The Aljam3 SDK is the interface between external applications and the [[glossary#Aljam3 Service|c:Aljam3 Service]]. It provides per-language libraries for:

- **Serializing** native variables into Aljam3's typed JSON wire format
- **Triggering** Aljam3 pipelines from foreign code
- **Exchanging** data with running Aljam3 jobs in real time

The SDK reuses the same JSON wire format defined in [[native-dispatch#Serialization Protocol]] — the SDK is a consumer of that protocol, not a separate serialization system.

### SDK Packages

| Language | Package Name | Registry |
|----------|-------------|----------|
| Python | `aljam3-sdk` | PyPI |
| Rust | `aljam3-sdk` | crates.io |
| Go | `aljam3.dev/sdk` | Go modules |
| JavaScript | `@aljam3/sdk` | npm |

## Universal String Algorithm

All data crossing language boundaries follows a single serialization path. This is the core algorithm the SDK implements.

### Flow

```text
Foreign Variable
    → to_aljam3(var, type)
        → JSON envelope (typed value)
            → NoSQL variable store
                → from_aljam3(str, type)
                    → Foreign Variable
```

### Principles

1. **Type fidelity is a compiler problem.** The wire format carries values only. The compiler emits [[#Type Mapping Descriptors]] in the [[behavior-contract|c:Behavior Contract]] that tell the Runner and SDK how to serialize each IO port — no runtime type inspection needed.

2. **All leaf values are strings.** This preserves Aljam3's "all data is serialized strings" principle (see [[spec/type-identity]]) across every language boundary. Numeric types use string representation to avoid floating-point precision loss.

3. **The JSON envelope is the same one used by native dispatch.** Each value is a typed JSON object matching the [[native-dispatch#Value Encoding]] format:

```json
{
  "type": "<aljam3_type>",
  "value": "<string_encoded_value>"
}
```

### Shared Memory Model

The NoSQL variable store is the shared memory between Aljam3 and foreign code. Variables written by `to_aljam3` are stored in the NoSQL DB; variables read by `from_aljam3` are retrieved from it. There is no OS-level shared memory — all exchange goes through the store.

## SDK Public Interface

The SDK exposes four functions. Each function has a consistent contract across all supported languages.

### `to_aljam3(value, type) → str`

Serialize a native variable to a Aljam3 JSON envelope string.

**Behavior:**
1. Accept a native value and a Aljam3 type identifier
2. Apply the [[#Primitive Type Mapping Table]] to determine the JSON encoding
3. Return a JSON string matching the [[native-dispatch#Value Encoding]] format

**Language Signatures:**

```python
# Python
def to_aljam3(value: Any, aljam3_type: str) -> str:
    """Serialize native Python value to Aljam3 JSON envelope."""
```

```rust
// Rust
pub fn to_aljam3(value: impl Into<Aljam3Value>, aljam3_type: &str) -> Result<String, SdkError>
```

```go
// Go
func ToAljam3(value interface{}, aljam3Type string) (string, error)
```

```javascript
// JavaScript
function toAljam3(value, aljam3Type) // returns string
```

**Errors:**
- `!SDK.TypeMismatch` — native value cannot be converted to the requested Aljam3 type
- `!SDK.SerializationError` — JSON encoding failed

**Example:**

```python
envelope = to_aljam3(42, "int")
# Returns: '{"type": "int", "value": "42"}'

envelope = to_aljam3([1, 2, 3], "array:int")
# Returns: '{"type": "array:int", "value": [{"type": "int", "value": "1"}, {"type": "int", "value": "2"}, {"type": "int", "value": "3"}]}'
```

### `from_aljam3(str) → value`

Deserialize a Aljam3 JSON envelope string to a native variable.

**Behavior:**
1. Parse the JSON envelope
2. Extract `type` and `value` fields
3. Apply the [[#Primitive Type Mapping Table]] in reverse to produce a native value

**Language Signatures:**

```python
# Python
def from_aljam3(envelope: str) -> Any:
    """Deserialize Aljam3 JSON envelope to native Python value."""
```

```rust
// Rust
pub fn from_aljam3<T: FromAljam3>(envelope: &str) -> Result<T, SdkError>
```

```go
// Go
func FromAljam3(envelope string, target interface{}) error
```

```javascript
// JavaScript
function fromAljam3(envelope) // returns native value
```

**Errors:**
- `!SDK.DeserializationError` — JSON parsing failed or envelope malformed
- `!SDK.TypeMismatch` — envelope type cannot be converted to the target native type

**Example:**

```python
value = from_aljam3('{"type": "int", "value": "42"}')
# Returns: 42 (Python int)

value = from_aljam3('{"type": "path", "value": "/home/user/data.txt"}')
# Returns: "/home/user/data.txt" (Python str)
```

### `call(pipeline, bindings) → result`

Trigger a Aljam3 pipeline and wait for the result. This is a fire-and-wait operation.

**Behavior:**
1. Serialize `bindings` using `to_aljam3` for each input
2. Publish a NATS request on `aljam3.call.<pipeline_name>` with the serialized bindings
3. The [[glossary#Trigger Monitor|c:Trigger Monitor]] receives the request and activates the pipeline's `-T.Call` trigger
4. Wait for the response on `aljam3.result.<correlation_id>`
5. Deserialize outputs using `from_aljam3`
6. Return the result

**NATS Protocol:**

```text
Request topic:  aljam3.call.<pipeline_name>
Response topic: aljam3.result.<correlation_id>

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
    """Trigger a Aljam3 pipeline via -T.Call and wait for result."""
```

```rust
// Rust
pub async fn call(pipeline: &str, bindings: HashMap<String, Aljam3Value>) -> Result<HashMap<String, Aljam3Value>, SdkError>
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
- `!SDK.PipelineNotFound` — pipeline name not registered in the Aljam3 Service
- `!SDK.Timeout` — no response within configured timeout
- `!SDK.PipelineError` — pipeline returned an error (wraps the pipeline's error ID and detail)

**Example:**

```python
result = await call("ProcessData", {"input_path": "/data/raw.csv"})
# Sends NATS request to aljam3.call.ProcessData
# Waits for aljam3.result.<uuid>
# Returns: {"output_path": "/data/processed.csv", "row_count": 1500}
```

### `pull(name) → value` / `push(name, value)`

Exchange data with a running Aljam3 job during `-Run.*.Bind` execution. Unlike `call()` which is fire-and-wait, `pull`/`push` provide ongoing data exchange while foreign code is executing inside a Aljam3 pipeline.

**Behavior:**
- `pull(name)` reads a variable from the NoSQL store that Aljam3 placed there for the current job
- `push(name, value)` writes a variable to the NoSQL store for Aljam3 to consume

Both functions operate within the context of a `-Run.<Lang>.Bind` job. The Runner sets up the store keys before invoking the foreign code, and reads them after execution completes.

**Language Signatures:**

```python
# Python
def pull(name: str) -> Any:
    """Read a Aljam3 variable placed for this Bind job."""

def push(name: str, value: Any) -> None:
    """Write a variable for Aljam3 to consume from this Bind job."""
```

```rust
// Rust
pub fn pull(name: &str) -> Result<Aljam3Value, SdkError>
pub fn push(name: &str, value: impl Into<Aljam3Value>) -> Result<(), SdkError>
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
from aljam3_sdk import pull, push

data = pull("input_data")        # Read what Aljam3 placed
result = process(data)           # Do work in Python
push("result", result)           # Write back for Aljam3
```

**Relationship to `-Run.*.Bind`:**
The Bind protocol is symmetric — direction is inferred from the trigger source. When Aljam3 triggers `-Run.<Lang>.Bind`, it places `<Bind` inputs in the store; the foreign code reads them with `pull()`. The foreign code writes results with `push()`; Aljam3 reads them as `>Bind` outputs. See [[aj3lib/pipelines/Run/Bind]] for the pipeline definition.

## Primitive Type Mapping Table

Bindings use only primitive types — no custom or user-defined types cross the SDK boundary.

| Aljam3 | Python | Rust | Go | JavaScript |
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
- `#serial` is Aljam3's unconstrained nested key-value type. It maps to the most general dictionary/map type in each language.
- `#array:T` preserves element type information. The `T` must be one of the other primitive types in this table.
- JavaScript's `number` type covers both `#int` and `#float`. The SDK uses the envelope's `type` field to distinguish them during deserialization.
- `#bytes` is Base64-encoded in the JSON envelope. The SDK converts to/from native byte types (e.g., Python `bytes`, Rust `Vec<u8>`).
- `#dt` represents DateTime as epoch seconds (integer string, e.g., `"1712494800"`). UTC only, seconds precision. The SDK converts to/from native integer types.

## Per-Language Encode/Decode

<!-- @c:spec/native-dispatch#Value Encoding -->
Related: [[native-dispatch#Value Encoding]]

The Primitive Type Mapping Table above shows type correspondences between Aljam3 and each supported language. This section shows how each SDK actually serializes and deserializes values — the concrete encode and decode functions that implement the universal string algorithm.

### Python

**String representation:**

| Aljam3 Type | Native Type | String Representation | Example |
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

**Encode (Python -> Aljam3 String):**

```python
def to_aljam3(value, aljam3_type: str) -> str:
    """Serialize a Python value to Aljam3 universal string."""
    match aljam3_type:
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

**Decode (Aljam3 String -> Python):**

```python
def from_aljam3(envelope: dict) -> Any:
    """Deserialize Aljam3 universal string to Python value."""
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
            return {k: from_aljam3(child) for k, child in v.items()}
        case _ if t.startswith("array:"):
            return [from_aljam3(item) for item in v]
        case _:
            # enum — return variant name as string
            return v
```

**Enum handling:** Python represents enums as plain strings. The Behavior Contract lists valid variants — the SDK can optionally validate against them, but the base decode returns the string. Users who want `enum.Enum` types wrap the result themselves.

### Rust

**String representation:**

| Aljam3 Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `String` | Value as-is | `"hello world"` |
| `#string` | `String` | Value as-is | `"hello world"` |
| `#int` | `i64` | `value.to_string()` | `"42"`, `"-7"`, `"0"` |
| `#float` | `f64` | Full-precision decimal (`format!("{}", value)`) | `"3.14"`, `"-0.001"` |
| `#bool` | `bool` | `"True"` / `"False"` (Aljam3 convention, NOT Rust's `"true"`) | `"True"` |
| `#path` | `PathBuf` | `value.to_string_lossy()` | `"/home/user/file.txt"` |
| `#dt` | `i64` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `serde_json::Value` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `Vec<T>` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `Vec<u8>` | Base64-encoded string | `"SGVsbG8="` |
| enum | `String` | Variant name as string | `"Active"` |
| (none) | `Option::None` | Empty string `""` | `""` |

**Encode (`ToAljam3` trait):**

```rust
pub trait ToAljam3 {
    fn to_aljam3(&self, aljam3_type: &str) -> Result<String, Aljam3Error>;
}

impl ToAljam3 for i64 {
    fn to_aljam3(&self, _: &str) -> Result<String, Aljam3Error> {
        Ok(self.to_string())  // "42"
    }
}

impl ToAljam3 for f64 {
    fn to_aljam3(&self, _: &str) -> Result<String, Aljam3Error> {
        Ok(format!("{}", self))  // ryu-style shortest round-trip
    }
}

impl ToAljam3 for bool {
    fn to_aljam3(&self, _: &str) -> Result<String, Aljam3Error> {
        Ok(if *self { "True" } else { "False" }.to_string())
    }
}

impl ToAljam3 for String {
    fn to_aljam3(&self, _: &str) -> Result<String, Aljam3Error> {
        Ok(self.clone())  // string, RawString, path, enum
    }
}

impl ToAljam3 for PathBuf {
    fn to_aljam3(&self, _: &str) -> Result<String, Aljam3Error> {
        Ok(self.to_string_lossy().into_owned())
    }
}

impl ToAljam3 for Vec<u8> {
    fn to_aljam3(&self, _: &str) -> Result<String, Aljam3Error> {
        Ok(base64::encode(self))
    }
}

impl<T: ToAljam3> ToAljam3 for Vec<T> {
    fn to_aljam3(&self, aljam3_type: &str) -> Result<String, Aljam3Error> {
        // Serialized as JSON array of typed envelopes by the caller
        // Each element calls T::to_aljam3 individually
        unimplemented!("array serialization handled by Envelope builder")
    }
}

impl<T: ToAljam3> ToAljam3 for Option<T> {
    fn to_aljam3(&self, aljam3_type: &str) -> Result<String, Aljam3Error> {
        match self {
            Some(v) => v.to_aljam3(aljam3_type),
            None => Ok(String::new()),  // ""
        }
    }
}
// HashMap<String, serde_json::Value> — serialized as nested typed JSON by Envelope builder
```

**Decode (`FromAljam3` trait):**

```rust
pub fn from_aljam3<T: FromAljam3>(envelope: &Envelope) -> Result<T, Aljam3Error> {
    T::from_aljam3(envelope)
}

pub trait FromAljam3: Sized {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error>;
}

impl FromAljam3 for i64 {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        match envelope.type_name.as_str() {
            "int" => envelope.value.parse::<i64>()
                .map_err(|_| Aljam3Error::TypeOverflow),
            _ => Err(Aljam3Error::TypeMismatch),
        }
    }
}

impl FromAljam3 for f64 {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        match envelope.type_name.as_str() {
            "float" => {
                match envelope.value.as_str() {
                    "Infinity" | "inf" | "+Inf" => Ok(f64::INFINITY),
                    "-Infinity" | "-inf" | "-Inf" => Ok(f64::NEG_INFINITY),
                    "NaN" | "nan" => Ok(f64::NAN),
                    v => v.parse::<f64>().map_err(|_| Aljam3Error::InvalidValue),
                }
            }
            _ => Err(Aljam3Error::TypeMismatch),
        }
    }
}

impl FromAljam3 for bool {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        match envelope.value.as_str() {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => Err(Aljam3Error::InvalidValue),
        }
    }
}

impl FromAljam3 for String {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        match envelope.type_name.as_str() {
            "string" | "RawString" | "path" => Ok(envelope.value.clone()),
            _ => Err(Aljam3Error::TypeMismatch),
        }
    }
}

impl FromAljam3 for PathBuf {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        match envelope.type_name.as_str() {
            "path" => Ok(PathBuf::from(&envelope.value)),
            _ => Err(Aljam3Error::TypeMismatch),
        }
    }
}

impl FromAljam3 for Vec<u8> {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        match envelope.type_name.as_str() {
            "bytes" => base64::decode(&envelope.value)
                .map_err(|_| Aljam3Error::InvalidValue),
            _ => Err(Aljam3Error::TypeMismatch),
        }
    }
}

impl<T: FromAljam3> FromAljam3 for Option<T> {
    fn from_aljam3(envelope: &Envelope) -> Result<Self, Aljam3Error> {
        if envelope.type_name == "none" || envelope.value.is_empty() {
            Ok(None)
        } else {
            T::from_aljam3(envelope).map(Some)
        }
    }
}
// Vec<T> and HashMap deserialization handled by Envelope parser
// (walks JSON array/object, calls T::from_aljam3 per element)
```

**Enum handling:** The compiler code-generates native Rust enum types from the Behavior Contract's variant list. The SDK provides a `#[derive(Aljam3Enum)]` macro to generate both `ToAljam3` and `FromAljam3` trait implementations automatically.

**Design note:** No `Aljam3Value` enum is needed. The Behavior Contract tells the Runner which type each port expects, so the SDK always calls with the correct generic parameter. Runtime type switching is unnecessary.

### Go

**String representation:**

| Aljam3 Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `string` | Value as-is | `"hello world"` |
| `#string` | `string` | Value as-is | `"hello world"` |
| `#int` | `int64` | `strconv.FormatInt(value, 10)` | `"42"`, `"-7"`, `"0"` |
| `#float` | `float64` | `strconv.FormatFloat(value, 'g', -1, 64)` | `"3.14"`, `"-0.001"` |
| `#bool` | `bool` | `"True"` / `"False"` (Aljam3 convention, NOT Go's `"true"`) | `"True"` |
| `#path` | `string` | OS-native path string | `"/home/user/file.txt"` |
| `#dt` | `int64` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `map[string]any` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `[]T` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `[]byte` | Base64-encoded string | `"SGVsbG8="` |
| enum | `string` | Variant name as string | `"Active"` |
| (none) | `nil` | Empty string `""` | `""` |

**Encode (`ToAljam3` function with type switch):**

```go
func ToAljam3(value any, aljam3Type string) (string, error) {
    switch aljam3Type {
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
// (walks map/slice recursively, calls ToAljam3 per element)
```

**Decode (per-type functions):**

```go
func FromAljam3Int(envelope Envelope) (int64, error) {
    if envelope.Type != "int" {
        return 0, ErrTypeMismatch
    }
    return strconv.ParseInt(envelope.Value, 10, 64)
}

func FromAljam3Float(envelope Envelope) (float64, error) {
    if envelope.Type != "float" {
        return 0, ErrTypeMismatch
    }
    return strconv.ParseFloat(envelope.Value, 64)
}

func FromAljam3Bool(envelope Envelope) (bool, error) {
    switch envelope.Value {
    case "True":  return true, nil
    case "False": return false, nil
    default:      return false, ErrInvalidValue
    }
}

func FromAljam3Bytes(envelope Envelope) ([]byte, error) {
    if envelope.Type != "bytes" {
        return nil, ErrTypeMismatch
    }
    return base64.StdEncoding.DecodeString(envelope.Value)
}

func FromAljam3String(envelope Envelope) (string, error) {
    switch envelope.Type {
    case "string", "RawString", "path":
        return envelope.Value, nil
    default:
        return "", ErrTypeMismatch
    }
}

func FromAljam3Dt(envelope Envelope) (int64, error) {
    if envelope.Type != "dt" {
        return 0, ErrTypeMismatch
    }
    return strconv.ParseInt(envelope.Value, 10, 64)
}

func FromAljam3None(envelope Envelope) (bool, error) {
    if envelope.Type == "none" || envelope.Value == "" {
        return true, nil
    }
    return false, nil
}

func FromAljam3Enum(envelope Envelope) (string, error) {
    return envelope.Value, nil
}

// FromAljam3Serial and FromAljam3Array require recursive
// envelope parsing — the Envelope parser walks the JSON structure
// and calls the appropriate FromAljam3* function per element.
```

**Enum handling:** The compiler code-generates `type Status string` with a const block and a validation map from the Behavior Contract's variant list.

### JavaScript

**String representation:**

| Aljam3 Type | Native Type | String Representation | Example |
|---|---|---|---|
| `#RawString` | `string` | Value as-is | `"hello world"` |
| `#string` | `string` | Value as-is | `"hello world"` |
| `#int` | `number` | `String(value)` | `"42"`, `"-7"`, `"0"` |
| `#float` | `number` | `String(value)` | `"3.14"`, `"-0.001"` |
| `#bool` | `boolean` | `"True"` / `"False"` (Aljam3 convention, NOT JS `"true"`) | `"True"` |
| `#path` | `string` | OS-native path string | `"/home/user/file.txt"` |
| `#dt` | `number` | Epoch seconds as string | `"1712494800"` |
| `#serial` | `Object` | Nested typed JSON object | `{ "key": { "type": "string", "value": "v" } }` |
| `#array:T` | `Array` | JSON array of typed values | `[{ "type": "int", "value": "1" }, ...]` |
| `#bytes` | `Uint8Array` | Base64-encoded string | `"SGVsbG8="` |
| enum | `string` | Variant name as string | `"Active"` |
| (none) | `null` | Empty string `""` | `""` |

**Encode (`toAljam3` function):**

```javascript
function toAljam3(value, aljam3Type) {
    switch (aljam3Type) {
        case "RawString": case "string": case "path":
            return String(value);
        case "int":
            if (!Number.isSafeInteger(value)) throw new Aljam3Error("TypeOverflow");
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

**Decode (`fromAljam3` function):**

```javascript
function fromAljam3(envelope) {
    const { type, value } = envelope;
    switch (type) {
        case "RawString": case "string": case "path":
            return value;
        case "int": {
            const n = parseInt(value, 10);
            if (n > Number.MAX_SAFE_INTEGER || n < Number.MIN_SAFE_INTEGER) {
                throw new Aljam3Error("TypeOverflow");
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
                Object.entries(value).map(([k, v]) => [k, fromAljam3(v)])
            );
        default:
            if (type.startsWith("array:")) {
                return value.map(item => fromAljam3(item));
            }
            // enum — return variant name as string
            return value;
    }
}
```

**Enum handling:** JavaScript represents enums as plain strings (like Python). For optional validation, the SDK can accept a `validVariants` set:

```javascript
function fromAljam3Enum(envelope, validVariants) {
    if (validVariants && !validVariants.has(envelope.value)) {
        throw new Aljam3Error("InvalidVariant");
    }
    return envelope.value;
}

// Usage with Behavior Contract variant list:
const statusVariants = new Set(["Active", "Inactive", "Enqueued"]);
const status = fromAljam3Enum(envelope, statusVariants);
```

### Strongly vs Weakly Typed Design Summary

| Concern | Weakly Typed (Python, JS) | Strongly Typed (Rust, Go) |
|---|---|---|
| **Encode** | Single function, `aljam3_type` param selects branch | Trait/interface per type, or type-switch on `any` |
| **Decode** | Single function returning dynamic type, switch on `type` field | Per-type functions or generics, caller specifies target type |
| **Type safety** | Runtime — caller must cast/check the return | Compile-time — wrong type = compile error (Rust) or wrong function call (Go) |
| **Who knows the type?** | The envelope's `type` field at runtime | The Behavior Contract at compile time; envelope `type` validated at runtime |
| **How type is determined** | Runtime switch on envelope | Compiler analyzes foreign AST or user declares type explicitly; Behavior Contract pre-selects the decode function |
| **Error on mismatch** | Possible but rare (dynamic) | Required — `TypeMismatch` error if envelope type != expected |

### Wire Format Conventions

<!-- @c:spec/native-dispatch#Value Encoding -->
Related: [[native-dispatch#Value Encoding]], [[aj3lib/types/schemas/Inf\|##Inf]], [[aj3lib/types/schemas/Nullable\|##Nullable]]

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

On deserialization (`from_aljam3`), SDKs accept only the canonical forms `"True"` and `"False"`. Any other string (e.g., `"true"`, `"1"`, `"yes"`) produces `!SDK.InvalidValue`.

#### Float Special Values

Float special values are **governed by schema** — not all float fields support them. The schema properties [[aj3lib/types/schemas/Nullable\|##Nullable]] and [[aj3lib/types/schemas/Inf\|##Inf]] control which special values a field accepts:

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

**Per-language normalization (encode — `to_aljam3`):**

SDKs must normalize language-native representations to the canonical wire forms on serialization:

| Value | Canonical | Python produces | Rust produces | Go produces | JS produces |
|---|---|---|---|---|---|
| +Infinity | `"Infinity"` | `"inf"` | `"inf"` | `"+Inf"` | `"Infinity"` |
| -Infinity | `"-Infinity"` | `"-inf"` | `"-inf"` | `"-Inf"` | `"-Infinity"` |
| NaN | `"NaN"` | `"nan"` | `"NaN"` | `"NaN"` | `"NaN"` |

Python, Rust, and Go SDKs must detect their native special value strings and replace them with the canonical form before writing to the wire. JavaScript already produces canonical forms natively.

**Per-language acceptance (decode — `from_aljam3`):**

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
2. At job dispatch time, the Runner reads these descriptors to determine the Aljam3 type of each port.
3. The SDK uses the descriptor's type to call the correct encode or decode function for that port.
4. Strongly typed SDKs (Rust, Go) never need runtime type guessing — the Behavior Contract determines the type at compile time and the SDK calls the appropriate typed function directly.

This means the `type` field in the JSON envelope is redundant for strongly typed SDKs — it serves as a runtime validation check, not a dispatch mechanism. For weakly typed SDKs (Python, JS), the `type` field is the primary dispatch mechanism since these languages lack compile-time type selection.

## Type Mapping Descriptors

The compiler emits **type mapping descriptors** as part of the [[behavior-contract|c:Behavior Contract]]. These descriptors tell the Runner and SDK exactly how to serialize/deserialize each IO port — no runtime type inspection needed.

### Descriptor Source

The type mapping descriptors are derived from the native registry's IO schema (see [[native-dispatch#Registry Entry Schema]]). The `inputs` and `outputs` arrays in each registry entry serve as the descriptors — they specify each port's name and Aljam3 type.

### Descriptor Schema

For each IO port in a pipeline, the compiler emits:

```json
{
  "port_name": "path",
  "direction": "input",
  "aljam3_type": "path",
  "is_array": false,
  "element_type": null
}
```

| Field | Type | Description |
|-------|------|-------------|
| `port_name` | string | IO port name from pipeline definition |
| `direction` | string | `"input"` or `"output"` |
| `aljam3_type` | string | Aljam3 type identifier (from [[#Primitive Type Mapping Table]]) |
| `is_array` | boolean | Whether this port carries an array type |
| `element_type` | string or null | Element type if `is_array` is true |

### Example

For a pipeline with mixed IO types:

```aljam3
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
    { "port_name": "input_path", "direction": "input", "aljam3_type": "path", "is_array": false, "element_type": null },
    { "port_name": "batch_size", "direction": "input", "aljam3_type": "int", "is_array": false, "element_type": null },
    { "port_name": "rows", "direction": "output", "aljam3_type": "array:serial", "is_array": true, "element_type": "serial" },
    { "port_name": "count", "direction": "output", "aljam3_type": "int", "is_array": false, "element_type": null }
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
| Python | `aljam3-sdk` | `pip install aljam3-sdk` |
| Rust | `aljam3-sdk` | `cargo add aljam3-sdk` |
| Go | `aljam3.dev/sdk` | `go get aljam3.dev/sdk` |
| JavaScript | `@aljam3/sdk` | `npm install @aljam3/sdk` |

### Version Locking

The SDK version is locked to the Aljam3 compiler version. Both use the same version number. This ensures:

- Type mapping descriptors match what the SDK expects
- Wire format compatibility between compiler output and SDK serialization
- NATS topic structure matches between Trigger Monitor and SDK

A version mismatch between SDK and compiler produces an `!SDK.VersionMismatch` error at connection time.

### Codebase Location

SDK implementations live under `lib/<language>/integrator/` in the Aljam3 source tree. See [[native-dispatch#lib/ Folder Structure]] for the full directory layout.

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| Aljam3's type system is the bridge | No per-language type catalog. Both sides map through Aljam3 primitives. The [[#Primitive Type Mapping Table]] is the single source of truth. |
| All leaf values are strings | Preserves [[spec/type-identity\|Aljam3's "all data is serialized strings"]] principle across every boundary. Numeric precision is never lost. |
| Shared memory = NoSQL variable store | Not OS-level shared memory. All data exchange goes through the same NoSQL store the [[glossary#Aljam3 Service\|c:Aljam3 Service]] uses for pipeline state. |
| No dynamic code generation | All SDK code is pre-compiled. The [[#Pluggable Wire Format]] selects implementations by configuration, not by generating code at runtime. |
| Error propagation maps to `!Error` tree | SDK errors (`!SDK.*`) follow the same structure as Aljam3's error system. Callers handle SDK errors the same way they handle pipeline errors. |
| SDK version locked to compiler version | Prevents wire format drift. A single version number covers compiler output, SDK serialization, and NATS protocol. |

### Why No FFI in the SDK

FFI (Foreign Function Interface) between language pairs is significantly faster than serialized string exchange — it avoids JSON serialization overhead entirely. However, the SDK deliberately excludes FFI for three reasons:

1. **Dynamic code generation.** FFI requires generating language-specific bindings between arbitrary language pairs at build time or runtime. This violates the SDK's "no dynamic code generation" design principle — all SDK code must be pre-compiled.

2. **Not universally available.** FFI is feasible between some language pairs (e.g., Rust↔C, Python↔C via ctypes) but not all. A universal SDK cannot depend on a mechanism that only works for some combinations.

3. **Complexity vs. universality.** The universal string algorithm works identically for every supported language. FFI would require per-pair implementations, testing matrices, and maintenance — complexity that scales quadratically with language count.

**The trade-off is acknowledged:** FFI is faster. For performance-critical pairwise integration, Aljam3 provides `-Run.Bridge` — a separate pipeline designed for direct code-to-code binding between specific language pairs. See [[aj3lib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] for cross-language function calls and [[aj3lib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] for cross-language variable binding.

The SDK and `-Run.Bridge` serve different needs: the SDK provides universal, zero-configuration integration; `-Run.Bridge` provides optimized, per-pair integration where performance justifies the setup cost. See [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] for the full conversion algorithm.

## Related

| Document | Relationship |
|----------|-------------|
| [[native-dispatch]] | Wire format definition, serialization protocol, native function contract |
| [[behavior-contract]] | Where type mapping descriptors live in compiled output |
| [[compiler-floor]] | Native operation categories ({N} block semantics) |
| [[aljam3-interface]] | User-facing SDK guide (integrator audience) |
| [[integrator-internals]] | Data casting methods and library architecture |
| [[spec/type-identity]] | "All data is serialized strings" foundation |
| [[aj3lib/pipelines/T/Call]] | `-T.Call` trigger used by `call()` |
| [[aj3lib/pipelines/Run/Bind]] | `-Run.*.Bind` pipeline used by `pull()`/`push()` |
| [[aj3lib/pipelines/Run/Bridge.Function]] | `-Run.Bridge.Function` — cross-language function call |
| [[aj3lib/pipelines/Run/Bridge.Script]] | `-Run.Bridge.Script` — cross-language variable binding |
| [[technical/algorithms/bridge-conversion]] | Bridge conversion algorithm for pairwise type mapping |
