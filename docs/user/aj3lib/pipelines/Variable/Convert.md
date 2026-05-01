---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
metadata_definition: "%definition.-:Variable.Convert"
metadata_instance: "%-:Variable.Convert:N"
---

# -Variable.Convert

<!-- @c:jm3lib/types/Variable -->
<!-- @c:jm3lib/types/NativeType -->
<!-- @c:spec/native-dispatch#Serialization Protocol -->

Convert a `#Variable` from one language's type system to another. The conversion uses the Aljam3 type system as intermediary -- source native type is mapped to a Aljam3 type, then the Aljam3 type is mapped to the target native type.

This pipeline is `{N}` native but user-callable. Bridge pipelines call it internally, but users can also invoke it directly for explicit variable conversion outside a Bridge context.

## Definition

```aljam3
{N} -Variable.Convert
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "VariableConvert"
   [%] .description << "Convert a #Variable between language type systems."
   (-) <from#Variable
   (-) <targetLang#string
   (-) >to#Variable
   (-) !Run.Bridge.IncompatibleType
   (-) !Run.Bridge.UnsupportedPair
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<from` | `#Variable` | Source variable with active language branch and native type |
| `<targetLang` | `#string` | Target language name (must match a `#NativeType` branch: "Python", "Rust", "Go", "JavaScript") |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>to` | `#Variable` | Converted variable with target language branch active |

## Conversion Algorithm

The conversion follows three steps:

```text
Step 1: Read source
   Source #Variable (:Python, .name="count", .value="42", .type=int)

Step 2: Map to Aljam3 type
   Python int → Aljam3 #int
   Value remains string-encoded: "42"

Step 3: Map to target type
   Aljam3 #int → Rust i64
   Target #Variable (:Rust, .name="count", .value="42", .type=i64)
```

The value string does not change during conversion -- only the language branch and native type are remapped. This is possible because all values use the same JSON wire format regardless of language (see [[spec/native-dispatch#Serialization Protocol]]).

See [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] for the full algorithm specification including edge cases and unsupported pairs.

## Errors

| Error | Condition |
|-------|-----------|
| `!Run.Bridge.IncompatibleType` | Source native type has no mapping to the target language (e.g., Python `bytes` to JavaScript has no direct equivalent) |
| `!Run.Bridge.UnsupportedPair` | The language pair is not yet implemented in the native operation registry |

## Permissions

No additional permissions required beyond the calling pipeline's permission scope. `-Variable.Convert` operates on in-memory data only -- no process spawning, file access, or network calls.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Variable.Convert` | Compile-time pipeline template |
| Instance | `%-:Variable.Convert:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/types/Variable|#Variable]] -- the language-tagged variable type
- [[jm3lib/types/NativeType|#NativeType]] -- marshalling table for type mapping
- [[jm3lib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] -- uses Convert internally
- [[jm3lib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] -- uses Convert internally
- [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] -- full algorithm specification
