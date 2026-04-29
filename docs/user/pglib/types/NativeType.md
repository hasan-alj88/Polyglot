---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
metadata_definition: "%definition.#:NativeType"
metadata_instance: "%#:NativeType:N"
---

# #NativeType Enum

<!-- @c:types -->
<!-- @c:spec/native-dispatch#Serialization Protocol -->
<!-- @c:pglib/types/NativeKind -->

`#NativeType` represents the set of primitive types available in each supported host language. Used by `#Variable` to tag values with their source or target language type, enabling the Bridge conversion algorithm to marshal data between language pairs.

See [[pglib/types/Variable|#Variable]] for the language-tagged variable type that references `#NativeType` branches.

---

## Definition

```aljam3
{#} #NativeType
   [%] .description << "Primitive type classifier per host language"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] %##Active << #ActiveKind.One
   :Python
      [.] .int
      [.] .float
      [.] .str
      [.] .bool
      [.] .bytes
      [.] .list
      [.] .dict
      [.] .NoneType
   :Rust
      [.] .i32
      [.] .i64
      [.] .f32
      [.] .f64
      [.] .String
      [.] .bool
      [.] .Vec_u8
      [.] .Vec
      [.] .PathBuf
      [.] .HashMap
   :Go
      [.] .int
      [.] .int64
      [.] .float64
      [.] .string
      [.] .bool
      [.] .byte_slice
      [.] .slice
      [.] .map
   :JavaScript
      [.] .number
      [.] .string
      [.] .boolean
      [.] .Uint8Array
      [.] .Array
      [.] .Object
      [.] .null
```

Only one language branch is active per instance (`%##Active.One`). The active branch identifies which language's type system the value belongs to.

---

## Marshalling Table

The Bridge conversion algorithm uses this table to determine how to convert between Aljam3 types and each language's native types. Conversion always passes through the Aljam3 type system as intermediary.

| Aljam3 Type | Python | Rust | Go | JavaScript |
|---------------|--------|------|----|------------|
| `#int` | `int` | `i64` | `int64` | `number` |
| `#float` | `float` | `f64` | `float64` | `number` |
| `#string` | `str` | `String` | `string` | `string` |
| `#bool` | `bool` | `bool` | `bool` | `boolean` |
| `#path` | `str` (OS path) | `PathBuf` | `string` | `string` |
| `#serial` | `dict` | `serde_json::Value` | `map[string]any` | `Object` |
| `#array.T` | `list[T]` | `Vec<T>` | `[]T` | `Array` |
| `#Record` | `dict` | `HashMap<String, _>` | `map[string]any` | `Object` |
| `#bytes` | `bytes` | `Vec<u8>` | `[]byte` | `Uint8Array` |
| `#dt` | `int` | `i64` | `int64` | `number` |
| (none) | `NoneType` | `Option::None` | `nil` | `null` |

**Key rule:** All values cross the boundary as strings using the JSON wire format defined in [[spec/native-dispatch#Serialization Protocol]]. The marshalling table determines which native constructor to use on the receiving side.

---

## Extensibility

New languages add new flexible branches to the enum. Existing branches and the marshalling table are unaffected. To add a language:

1. Add a new `:Language` branch with its primitive type fields
2. Extend the marshalling table with the new column
3. Implement the conversion pair in the Bridge native operation (Rust host code)

No changes to `#NativeType`'s schema properties or `#Variable` are required.

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:NativeType` | Compile-time type template |
| Instance | `%#:NativeType:0` | Runtime instance (enum -- one active field) |

## Related

- [[pglib/types/Variable|#Variable]] -- language-tagged variable using `#NativeType` branches
- [[spec/native-dispatch|Native Dispatch]] -- JSON wire format and serialization protocol
- [[spec/type-identity|Type Identity]] -- "all data is serialized strings" principle
- [[pglib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]] -- pipelines that use native types
- [[pglib/pipelines/Variable/Convert|-Variable.Convert]] -- pairwise type conversion pipeline
