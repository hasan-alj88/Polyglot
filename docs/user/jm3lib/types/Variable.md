---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
metadata_definition: "%definition.#:Variable"
metadata_instance: "%#:Variable:N"
---

# #Variable Struct (##Enum)

<!-- @c:types -->
<!-- @c:jm3lib/types/NativeType -->
<!-- @c:jm3lib/pipelines/Run/INDEX -->

`#Variable` represents a named variable from a specific host language, tagged with its native type. Used by `-Run.Bridge.*` pipelines to pass arguments and return values across language boundaries.

Each `#Variable` instance has exactly one active language branch (`%##Active.One`) -- a Python variable cannot simultaneously be a Rust variable.

---

## Definition

```aljam3
{#} #Variable
   [%] .description << "Language-tagged variable for cross-language bridge calls"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] %##Active << #ActiveKind.One
   :Python
      [.] .name#string
      [.] .value#serial
      [.] .type#NativeType.Python
   :Rust
      [.] .name#string
      [.] .value#serial
      [.] .type#NativeType.Rust
   :Go
      [.] .name#string
      [.] .value#serial
      [.] .type#NativeType.Go
   :JavaScript
      [.] .name#string
      [.] .value#serial
      [.] .type#NativeType.JavaScript
```

---

## Fields

Each language branch contains the same three fields:

| Field | Type | Description |
|-------|------|-------------|
| `.name` | `#string` | Variable name in the source language |
| `.value` | `#serial` | Serialized value (JSON wire format) |
| `.type` | `#NativeType.<Lang>` | Native type classifier from the source language |

The `.type` field references the corresponding language branch of [[jm3lib/types/NativeType|#NativeType]]. This constrains `.type` to only valid types for the active language -- a `:Python` variable cannot have `.type#NativeType.Rust`.

---

## Instance Examples

**Valid** -- single active branch (Python):

```json
{
  "branch": "Python",
  "name": "user_count",
  "value": { "type": "int", "value": "42" },
  "type": "int"
}
```

**Valid** -- single active branch (Rust):

```json
{
  "branch": "Rust",
  "name": "file_path",
  "value": { "type": "path", "value": "/data/output.csv" },
  "type": "PathBuf"
}
```

**Invalid** -- two active branches (compile error):

```json
{
  "branch": ["Python", "Rust"],
  "error": "PGE04001 -- ##Enum with %##Active.One cannot have multiple active branches"
}
```

---

## Usage in Bridge Pipelines

`#Variable` appears as array elements in `-Run.Bridge.*` IO ports:

| Pipeline | IO Port | Type | Purpose |
|----------|---------|------|---------|
| `-Run.Bridge.Function` | `<arg` | `#array.Variable` | Positional arguments to the callee function |
| `-Run.Bridge.Function` | `<kwarg` | `#Record.String.Variable` | Named arguments to the callee function |
| `-Run.Bridge.Function` | `>Bind` | `#array.Variable` | Return values from the callee |
| `-Run.Bridge.Script` | `<Bind` | `#array.Variable` | Input variables injected into callee |
| `-Run.Bridge.Script` | `>Bind` | `#array.Variable` | Output variables read back from callee |
| `-Variable.Convert` | `<from` | `#Variable` | Source variable to convert |
| `-Variable.Convert` | `>to` | `#Variable` | Converted variable in target language |

---

## Conversion

The `-Variable.Convert` pipeline converts a `#Variable` from one language branch to another. Conversion passes through the Aljam3 type system as intermediary:

```text
Source #Variable (:Python, .type=int, .value="42")
    → Aljam3 #int ("42")
    → Target #Variable (:Rust, .type=i64, .value="42")
```

See [[jm3lib/pipelines/Variable/Convert|-Variable.Convert]] for the pipeline definition and [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] for the full algorithm.

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Variable` | Compile-time type template |
| Instance | `%#:Variable:0` | Runtime instance (enum -- one active field) |

## Related

- [[jm3lib/types/NativeType|#NativeType]] -- primitive type classifier per host language
- [[jm3lib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] -- cross-language function call
- [[jm3lib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] -- cross-language variable binding
- [[jm3lib/pipelines/Variable/Convert|-Variable.Convert]] -- pairwise type conversion
- [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] -- conversion algorithm and dual-wrapper lifecycle
