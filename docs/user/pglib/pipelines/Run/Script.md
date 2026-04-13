---
audience: automation-builder
type: specification
updated: 2026-04-13
status: draft
metadata_definition: "%definition.-:Run.<Lang>.Script"
metadata_instance: "%-:Run.<Lang>.Script:N"
---

# -Run.\<Lang\>.Script

Run code with Record-typed variable bindings. Field names become native local variables.

> **Supersedes:** `-RT.<Lang>.Script.Inline` and `-RT.<Lang>.Script.File`. See [[pglib/pipelines/RT/Script.Inline|@d:-RT.\<Lang\>.Script.Inline]] and [[pglib/pipelines/RT/Script.File|@d:-RT.\<Lang\>.Script.File]].

## Definition

```polyglot
{N} -Run.<Lang>.Script
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunScript"
   [%] .description << "Run code with Record-typed variable bindings."
   (-) <env#<Lang>Env
   (-) <Bind#Record
   (-) >Bind#Record
   (-) >output#Code:<Lang>.Output
   (-) <code#Code:Source
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.Env` |
| `<Bind` | `#Record` | Input bindings -- field names = native variable names |
| `<code` | `#Code:Source` | Script code (inline `[C]` or file) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#Record` | Output bindings -- field names = native variable names |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Record Binding

`<Bind#Record` field names map **exactly** to native local variables. The compiler validates that every field name exists as an identifier in the code (PGE01033). `>Bind#Record` field names are read back after execution (PGE01034).

This is the primary mechanism for passing Polyglot data into foreign code:

| Polyglot Record Field | Native Variable |
|-----------------------|-----------------|
| `.input_path#path` | `input_path` |
| `.target_w#int` | `target_w` |
| `.target_h#int` | `target_h` |

Field types drive marshalling through the native dispatch JSON wire format. See [[pglib/pipelines/Run/INDEX#Type Marshalling]] for the full type mapping table.

## Compiler Validation

The compiler validates that:
- `<Bind` Record field names exist as identifiers in the code (PGE01033)
- `>Bind` Record field names exist as identifiers in the code (PGE01034)
- Assigned values match Record schema topology (PGE01037)

**Note:** Validation applies at compile time for `<code.inline` only. When `<code.file` is used, validation is deferred to runtime.

## Code Source

Uses `#Code:Source` with `%##Active` one -- provide **either** inline or file, never both (PGE01038):

```polyglot
[ ] inline via [C] blocks
(-) <code.inline <<
   [C] import os
   [C] result = os.listdir(target_dir)
```

```polyglot
[ ] file reference
(-) <code.file#path << "/scripts/cleanup.py"
```

## Example

```polyglot
{;} ;PyML
   [.] .language << "python"
   [.] .version << "3.14"
   [.] .packages << ["Pillow"]

{_} _ImageGrant
   [.] .intent << #Grant
   [.] .System.Process "python3"

{-} =ResizeImage
   [_] _ImageGrant
   (-) <imageFile#path
   (-) <targetWidth#int
   (-) <targetHeight#int
   (-) >result#serial
   (-) >log#Code:Python.Output
   (-) ;PyML
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env
      (-) <env#; << ;PyML

   [ ]
   [-] -Run.Python.Script
      (-) <env#PyEnv << $pyenv
      (-) <Bind#Record
         [.] .input_path#path << $imageFile
         [.] .target_w#int << $targetWidth
         [.] .target_h#int << $targetHeight
      (-) >Bind#Record
         [.] .resized_path#path
      (-) >output#Code:Python.Output >> >log
      (-) <code.inline <<
         [C] from PIL import Image
         [C] img = Image.open(input_path)
         [C] img_resized = img.resize((target_w, target_h))
         [C] resized_path = input_path.with_suffix(".resized.png")
         [C] img_resized.save(resized_path)
```

In this example:
- `input_path`, `target_w`, `target_h` are injected as Python local variables from `<Bind`
- `resized_path` is read back into Polyglot via `>Bind`
- The compiler validates all four names exist in the `[C]` block

## Scope Isolation

Each `-Run.*` call gets a fresh variable scope within the `-W.Env` environment. `<Bind` fields are injected before execution; `>Bind` fields are read back after. Variables from one `-Run.*` call do **not** leak into another.

## Errors

See [[pglib/pipelines/Run/INDEX#Binding Compiler Errors]].

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.<Lang>.Script` | Compile-time pipeline template |
| Instance | `%-:Run.<Lang>.Script:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]]
- [[pglib/pipelines/W/Env|-W.Env]] -- wrapper that manages runtime environments
- [[pglib/types/rt|Runtime types]] -- `#Code`, `#PyEnv`, `#RsEnv`
