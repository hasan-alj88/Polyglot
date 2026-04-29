# Rust — Runner Native Operations

Native implementations for `#NativeKind.Execution` and `#NativeKind.Wrapper` operations, dispatched by the Runner.

## Execution Operations

| Pipeline | Function | Description |
|----------|----------|-------------|
| `=File.Text.Read` | `FileTextRead` | Read text file contents |
| `=File.Text.Write` | `FileTextWrite` | Write text to file |
| `=File.Text.Append` | `FileTextAppend` | Append text to file |
| `=Math.Add` | `MathAdd` | Numeric addition |
| `=DB.Query` | `DbQuery` | Database query execution |
| `=DT.Now` | `DtNow` | Current system time |

## Wrapper Operations

| Pipeline | Function | Description |
|----------|----------|-------------|
| `=W.Aljam3` | `WrapperAljam3` | Default no-op wrapper |
| `=W.DB.Connection` | `WrapperDbConnection` | Database connection lifecycle |
| `=W.RT.Python` | `WrapperRtPython` | Python runtime lifecycle |

## Contract

```rust
pub fn file_text_read(request: &str) -> Result<String, String>
```

See `docs/technical/spec/native-dispatch.md#Execution` and `#Wrapper` for dispatch details.
