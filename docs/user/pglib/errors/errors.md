---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# Error System

<!-- @c:errors -->
<!-- @c:blocks -->

Errors use the `!` prefix and hierarchical dot names. Every error terminal is typed `#Error` — enforced by `%##TerminalType << #Error` (see [[pglib/types/properties/TerminalType|%##TerminalType]]). `{!}` blocks are effectively `{#}` data trees with this terminal constraint. Custom errors are defined with `{!}` blocks; pglib errors are built-in and require no `[@]` import.

## `#NullableRecord`

A `#Record` that can be null — used for optional structured data:

```polyglot
{#} #NullableRecord
   [#] ##Nullable
      (#) <#Record
```

## `#Error` Struct

All errors — pglib and user-defined — share the same struct:

```polyglot
{#} #Error
   [.] .Name#RawString
   [.] .ErrorAlias#RawString
   [.] .MessageTemplate#RawString
   [.] .Info#Record
   [.] .Stderr#NullableRecord
   [.] .StackTrace#NullableRecord
   [.] .ExitCode#NullableRecord
```

| Field | Filled by | Compiler enforced? | Purpose |
|-------|-----------|-------------------|---------|
| `.Name` | Runtime (auto) | — | Full error identifier (e.g., `"File.NotFound"`) |
| `.ErrorAlias` | Author | — | Short display name for logging/UI |
| `.MessageTemplate` | Author (at definition site) | Required on every `{!}` terminal; `{key}` must exist in `.Info` (PGE07008) | Template with `{key}` interpolation (e.g., `"Name exceeds {maxLength} chars"`) |
| `.Info` | Schema at definition site, values at raise site | Raise site must provide all declared keys (PGE07009); extra keys allowed | Structured context data with typed key schema |
| `.Stderr` | Runtime (auto) | — | Captured standard error output (null when not applicable) |
| `.StackTrace` | Runtime (auto) | — | Execution stack trace (null when not applicable) |
| `.ExitCode` | Runtime (auto) | — | Process exit code (null when not applicable) |

`#Error` content is split across two sites:

- **Definition site (`{!}` block):** `.MessageTemplate` is required on every terminal. `.Info` declares a typed key schema — each key with its type (e.g., `[:] :path#path`). The compiler enforces that every `{key}` in `.MessageTemplate` has a matching key in the `.Info` schema (PGE07008).
- **Raise site (`[!] >>` block):** Fills `.Info` values for the declared keys. Must provide all keys from the definition schema (PGE07009). May add extra keys beyond the schema for additional context.

The resolved message is computed at runtime by interpolating `.Info` values into `.MessageTemplate`.

## Defining Custom Errors (`{!}`)

All user-defined errors live under the `!Error` namespace. `{!} !Name` implicitly creates `!Error:Name.*` in the metadata tree. Use `[:]` for extensible branches and `[.]` for terminal leaves (typed `#Error`):

```polyglot
{!} !Error
   [:] :Validation
      [.] .Empty#Error
         (-) .MessageTemplate << "Field {field} is required"
         (-) .Info
            [:] :field#string
      [.] .TooLong#Error
         (-) .MessageTemplate << "{field} exceeds {maxLength} characters"
         (-) .Info
            [:] :field#string
            [:] :maxLength#int
      [.] .InvalidEmail#Error
         (-) .MessageTemplate << "Invalid email format: {email}"
         (-) .Info
            [:] :email#string
```

This creates `!Error:Validation.Empty`, `!Error:Validation.TooLong`, `!Error:Validation.InvalidEmail` — all carrying the `#Error` struct with their `.MessageTemplate` and `.Info` schema defined at the definition site. The raise site fills `.Info` values only. Note: the pglib `!Validation` namespace (shown in [[pglib/errors/errors#Built-in Error Namespaces]]) is separate — it has fixed leaves defined by the runtime, not user code.

`{!}` creates entries at `%!.Error:Name.*` in the metadata tree. See [[data-is-trees#How Concepts Connect]].

## Built-in Error Namespaces

No `[@]` import needed. pglib errors are defined as `{!}` blocks by the runtime:

```polyglot
{!} !File
   [.] .NotFound#Error
      (-) .MessageTemplate << "File not found: {path}"
      (-) .Info
         [:] :path#path
   [.] .ReadError#Error
      (-) .MessageTemplate << "Cannot read file: {path}"
      (-) .Info
         [:] :path#path
   [.] .WriteError#Error
      (-) .MessageTemplate << "Cannot write file: {path}"
      (-) .Info
         [:] :path#path
   [.] .ParseError#Error
      (-) .MessageTemplate << "Parse error in {path}: {reason}"
      (-) .Info
         [:] :path#path
         [:] :reason#string

{!} !No
   [.] .Input#Error
      (-) .MessageTemplate << "Missing required input: {name}"
      (-) .Info
         [:] :name#string
   [.] .Output#Error
      (-) .MessageTemplate << "Missing required output: {name}"
      (-) .Info
         [:] :name#string

{!} !Timeout
   [.] .Connection#Error
      (-) .MessageTemplate << "Connection timed out after {duration}"
      (-) .Info
         [:] :duration#string
   [.] .Read#Error
      (-) .MessageTemplate << "Read timed out after {duration}"
      (-) .Info
         [:] :duration#string

{!} !Math
   [.] .DivideByZero#Error
      (-) .MessageTemplate << "Division by zero: {expression}"
      (-) .Info
         [:] :expression#string

{!} !Validation
   [.] .Schema#Error
      (-) .MessageTemplate << "Schema validation failed: {reason}"
      (-) .Info
         [:] :reason#string
   [.] .Type#Error
      (-) .MessageTemplate << "Type mismatch: expected {expected}, got {actual}"
      (-) .Info
         [:] :expected#string
         [:] :actual#string
   [.] .Regex#Error
      (-) .MessageTemplate << "Value does not match pattern {pattern}: {value}"
      (-) .Info
         [:] :pattern#string
         [:] :value#string

{!} !Field
   [.] .NotFound#Error
      (-) .MessageTemplate << "Field not found: {field}"
      (-) .Info
         [:] :field#string
   [.] .PathError#Error
      (-) .MessageTemplate << "Invalid field path: {path}"
      (-) .Info
         [:] :path#string

{!} !Alias
   [.] .Clash#Error
      (-) .MessageTemplate << "Alias {alias} clashes with existing name in {namespace}"
      (-) .Info
         [:] :alias#string
         [:] :namespace#string

{!} !Permission
   [.] .File.Denied#Error
      (-) .MessageTemplate << "File permission denied: {path}"
      (-) .Info
         [:] :path#path
   [.] .Web.Denied#Error
      (-) .MessageTemplate << "Web permission denied: {url}"
      (-) .Info
         [:] :url#string
   [.] .Database.Denied#Error
      (-) .MessageTemplate << "Database permission denied: {connection}"
      (-) .Info
         [:] :connection#string
   [.] .System.Denied#Error
      (-) .MessageTemplate << "System permission denied: {operation}"
      (-) .Info
         [:] :operation#string
   [.] .Crypto.Denied#Error
      (-) .MessageTemplate << "Crypto permission denied: {operation}"
      (-) .Info
         [:] :operation#string
   [.] .IPC.Denied#Error
      (-) .MessageTemplate << "IPC permission denied: {target}"
      (-) .Info
         [:] :target#string
   [.] .Device.Denied#Error
      (-) .MessageTemplate << "Device permission denied: {device}"
      (-) .Info
         [:] :device#string
   [.] .Memory.Denied#Error
      (-) .MessageTemplate << "Memory permission denied: {operation}"
      (-) .Info
         [:] :operation#string

{!} !RT
   [.] .CompileError#Error
      (-) .MessageTemplate << "Compile error in {language}: {reason}"
      (-) .Info
         [:] :language#string
         [:] :reason#string
   [.] .RuntimeError#Error
      (-) .MessageTemplate << "Runtime error in {language}: {reason}"
      (-) .Info
         [:] :language#string
         [:] :reason#string
   [.] .Timeout#Error
      (-) .MessageTemplate << "Execution timed out after {duration}"
      (-) .Info
         [:] :duration#string
   [.] .EnvironmentError#Error
      (-) .MessageTemplate << "Environment error in {language}: {reason}"
      (-) .Info
         [:] :language#string
         [:] :reason#string

{!} !Env
   [.] .NotFound#Error
      (-) .MessageTemplate << "Environment not found: {name}"
      (-) .Info
         [:] :name#string
   [.] .VersionMismatch#Error
      (-) .MessageTemplate << "Environment {name} version mismatch: expected {expected}, got {actual}"
      (-) .Info
         [:] :name#string
         [:] :expected#string
         [:] :actual#string
   [.] .SetupFailed#Error
      (-) .MessageTemplate << "Environment setup failed: {name}"
      (-) .Info
         [:] :name#string
   [.] .TeardownFailed#Error
      (-) .MessageTemplate << "Environment teardown failed: {name}"
      (-) .Info
         [:] :name#string
   [:] :Dependency
      [.] .Missing#Error
         (-) .MessageTemplate << "Missing dependency: {dependency}"
         (-) .Info
            [:] :dependency#string
      [.] .VersionConflict#Error
         (-) .MessageTemplate << "Dependency {dependency} version conflict: {expected} vs {actual}"
         (-) .Info
            [:] :dependency#string
            [:] :expected#string
            [:] :actual#string
      [.] .InstallFailed#Error
         (-) .MessageTemplate << "Failed to install dependency: {dependency}"
         (-) .Info
            [:] :dependency#string

{!} !Storage
   [.] .Space#Error
      (-) .MessageTemplate << "Insufficient storage space: {required} needed"
      (-) .Info
         [:] :required#string

{!} !Text
   [:] :Diff
      [.] .EmptyInput#Error
         (-) .MessageTemplate << "Diff input is empty: {side}"
         (-) .Info
            [:] :side#string
   [:] :Lines
      [.] .Empty#Error
         (-) .MessageTemplate << "Text has no lines"
   [:] :Append
      [.] .EmptyResult#Error
         (-) .MessageTemplate << "Append produced empty result"
   [:] :Merge
      [.] .InvalidLineNumber#Error
         (-) .MessageTemplate << "Invalid line number: {lineNumber}"
         (-) .Info
            [:] :lineNumber#int
      [.] .EmptyBase#Error
         (-) .MessageTemplate << "Merge base text is empty"

{!} !CSV
   [:] :Parse
      [.] .MalformedRow#Error
         (-) .MessageTemplate << "Malformed CSV row at line {lineNumber}: {reason}"
         (-) .Info
            [:] :lineNumber#int
            [:] :reason#string
      [.] .Empty#Error
         (-) .MessageTemplate << "CSV input is empty"
      [.] .InvalidDelimiter#Error
         (-) .MessageTemplate << "Invalid CSV delimiter: {delimiter}"
         (-) .Info
            [:] :delimiter#string
   [:] :Collect
      [.] .SchemaMismatch#Error
         (-) .MessageTemplate << "Row schema does not match header: {reason}"
         (-) .Info
            [:] :reason#string
      [.] .EmptyResult#Error
         (-) .MessageTemplate << "CSV collection produced empty result"
   [:] :Merge
      [.] .HeaderConflict#Error
         (-) .MessageTemplate << "CSV merge header conflict: {reason}"
         (-) .Info
            [:] :reason#string
```

### `!Error` — User-Extensible Namespace

`!Error` is the only namespace with user-extensible children. All other namespaces (`!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission`, `!RT`, `!Env`, `!Storage`, `!Text`, `!CSV`) have Polyglot-defined fixed leaves.

### `!Env` vs `!RT` — Phase Distinction

| Namespace | Phase | Scope |
|-----------|-------|-------|
| `!Env.*` | `[W]` wrapper setup | Environment availability + dependency resolution |
| `!RT.*` | `[-]` body execution | Foreign code compile/runtime errors |

`!Env.*` errors fire during [[pglib/pipelines/W/Env|-W.Env]] wrapper setup when the environment cannot be established. `!RT.EnvironmentError` fires during execution when the foreign runtime encounters an environment issue within already-running code. Both coexist — they cover different execution phases. See [[environments]] for the `{;}` environment system.

Users extend `!Error` via `{!}` blocks using `[:]` for extensible branches and `[.]` for terminal leaves. Siblings at the same level must all use the same separator (sibling homogeneity rule).

```polyglot
{!} !Error
   [:] :MyApp
      [:] :Auth
         [.] .Expired#Error
            (-) .MessageTemplate << "Token for {userId} expired at {expiredAt}"
            (-) .Info
               [:] :userId#string
               [:] :expiredAt#string
         [.] .Invalid#Error
            (-) .MessageTemplate << "Invalid token format"
      [:] :Data
         [.] .Corrupt#Error
            (-) .MessageTemplate << "Data corrupted in {source}: {reason}"
            (-) .Info
               [:] :source#string
               [:] :reason#string
         [.] .Missing#Error
            (-) .MessageTemplate << "Required data not found: {key}"
            (-) .Info
               [:] :key#string
      [:] :GeneralFailure#Error
         (-) .MessageTemplate << "Application error: {reason}"
         (-) .Info
            [:] :reason#string
```

This creates `!Error:MyApp:Auth.Expired`, `!Error:MyApp:Auth.Invalid`, `!Error:MyApp:Data.Corrupt`, `!Error:MyApp:Data.Missing`, and `!Error:MyApp:GeneralFailure`. Each terminal carries its `.MessageTemplate` and `.Info` schema.

Tree path: `%!.Error:MyApp:Auth.Expired` — `.Error` is Polyglot-defined (fixed), `:MyApp:Auth` are user-extensible (flexible), `.Expired` is a terminal leaf (fixed).

## Pipeline Error Associations

Each pglib pipeline declares the errors it can raise via `[=] !ErrorName` (see [[concepts/pipelines/metadata#Error Trees]]):

```polyglot
=File.Text.Read
   [=] !File.NotFound
   [=] !File.ReadError
   [=] !Permission.File.Denied

=File.Text.Write
   [=] !File.NotFound
   [=] !File.WriteError
   [=] !Permission.File.Denied

=File.Text.Append
   [=] !File.NotFound
   [=] !File.WriteError
   [=] !Permission.File.Denied

=File.Serial.Read
   [=] !File.NotFound
   [=] !File.ReadError
   [=] !File.ParseError
   [=] !Permission.File.Denied

=File.Serial.Write
   [=] !File.NotFound
   [=] !File.WriteError
   [=] !Permission.File.Denied

=File.Serial.Read.Field
   [=] !File.NotFound
   [=] !File.ReadError
   [=] !File.ParseError
   [=] !Field.NotFound
   [=] !Permission.File.Denied

=#.Field
   [=] !Field.NotFound
   [=] !Field.PathError

=#.Column
   [=] !Field.NotFound

=Math.Divide
   [=] !Math.DivideByZero

=Math.Modulo
   [=] !Math.DivideByZero

=RT.<Lang>.Function.Inline
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Function.File
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Script.Inline
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Script.File
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.CLI
   [=] !RT.RuntimeError
   [=] !RT.Timeout

=RT.<Lang>.Bind.Inline
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Bind.File
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=Text.Diff
   [=] !Text.Diff.EmptyInput

=ForEach.Text.Lines
   [=] !Text.Lines.Empty

*Into.Text.Append
   [=] !Storage.Space
   [=] !Text.Append.EmptyResult

*Into.Text.Merge
   [=] !Storage.Space
   [=] !Text.Merge.InvalidLineNumber
   [=] !Text.Merge.EmptyBase

=ForEach.CSV.Rows
   [=] !CSV.Parse.MalformedRow
   [=] !CSV.Parse.Empty
   [=] !CSV.Parse.InvalidDelimiter

*Into.CSV.Rows
   [=] !Storage.Space
   [=] !CSV.Collect.SchemaMismatch
   [=] !CSV.Collect.EmptyResult

*Into.CSV.Merge
   [=] !Storage.Space
   [=] !Text.Merge.InvalidLineNumber
   [=] !Text.Merge.EmptyBase
   [=] !CSV.Merge.HeaderConflict

-W.Env
   [=] !Env.NotFound
   [=] !Env.VersionMismatch
   [=] !Env.SetupFailed
   [=] !Env.TeardownFailed
   [=] !Env.Dependency.Missing
   [=] !Env.Dependency.VersionConflict
   [=] !Env.Dependency.InstallFailed
```

## `!Alias.Clash` — Compile Error

`!Alias.Clash` is a compile error raised when an alias collides with an existing name in the target namespace. Aliases place definitions at multiple locations in the `%` metadata tree; when a target location is already occupied, this error fires.

### `[<] !Alias.Clash` Fallback Chain

In `{#}` generic type definitions, the `(#) <Alias` parameter can provide a fallback chain of alternative alias values using `[<] !Alias.Clash`. The compiler tries each value in order until one succeeds:

```polyglot
(#) <Alias << "int"
   [<] !Alias.Clash << "integer"
   [<] !Alias.Clash << "Integer"
```

- First, the compiler tries `"int"` as the alias
- If `"int"` clashes with an existing name in the target namespace, `!Alias.Clash` fires and the compiler tries `"integer"`
- If `"integer"` also clashes, the compiler tries `"Integer"`
- If all alternatives are exhausted, the compile error propagates (unrecoverable)

This pattern provides robust alias resolution for scalar type definitions like `##Int`, `##Float`, etc.
