---
audience: pg-coder
type: specification
updated: 2026-03-30
status: complete
---

# Error System

<!-- @errors -->
<!-- @blocks -->

Errors use the `!` prefix and hierarchical dot names. Every error leaf is typed `#Error`. Custom errors are defined with `{!}` blocks; stdlib errors are built-in and require no `[@]` import.

## `#Error` Struct

All errors — stdlib and user-defined — share the same struct:

```polyglot
{#} #Error
   [.] .Name#RawString
   [.] .ErrorAlias#RawString
   [.] .Message#RawString
   [.] .Info#serial
```

| Field | Filled by | Purpose |
|-------|-----------|---------|
| `.Name` | Runtime (auto) | Full error identifier (e.g., `"File.NotFound"`) |
| `.ErrorAlias` | Author | Short display name for logging/UI |
| `.Message` | Author (at raise site) | Human-readable description |
| `.Info` | Author (at raise site) | Arbitrary context data as `#serial` |

## Defining Custom Errors (`{!}`)

All user-defined errors live under the `!Error` namespace. `{!} !Name` implicitly creates `!Error:Name.*` in the metadata tree. Use `[:]` for extensible branches and `[.]` for terminal leaves (typed `#Error`):

```polyglot
{!} !Error
   [:] :Validation
      [.] .Empty#Error
      [.] .TooLong#Error
      [.] .InvalidEmail#Error
```

This creates `!Error:Validation.Empty`, `!Error:Validation.TooLong`, `!Error:Validation.InvalidEmail` — all carrying the `#Error` struct. Note: the stdlib `!Validation` namespace (shown in [[stdlib/errors/errors#Built-in Error Namespaces]]) is separate — it has fixed leaves defined by the runtime, not user code.

`{!}` creates entries at `%!.Error:Name.*` in the metadata tree. See [[data-is-trees#How Concepts Connect]].

## Built-in Error Namespaces

No `[@]` import needed. Stdlib errors are defined as `{!}` blocks by the runtime:

```polyglot
{!} !File
   [.] .NotFound#Error
   [.] .ReadError#Error
   [.] .WriteError#Error
   [.] .ParseError#Error

{!} !No
   [.] .Input#Error
   [.] .Output#Error

{!} !Timeout
   [.] .Connection#Error
   [.] .Read#Error

{!} !Math
   [.] .DivideByZero#Error

{!} !Validation
   [.] .Schema#Error
   [.] .Type#Error
   [.] .Regex#Error

{!} !Field
   [.] .NotFound#Error
   [.] .PathError#Error

{!} !Alias
   [.] .Clash#Error

{!} !Permission
   [.] .File.Denied#Error
   [.] .Web.Denied#Error
   [.] .Database.Denied#Error
   [.] .System.Denied#Error
   [.] .Crypto.Denied#Error
   [.] .IPC.Denied#Error
   [.] .Device.Denied#Error
   [.] .Memory.Denied#Error

{!} !RT
   [.] .CompileError#Error
   [.] .RuntimeError#Error
   [.] .Timeout#Error
   [.] .EnvironmentError#Error
```

### `!Error` — User-Extensible Namespace

`!Error` is the only namespace with user-extensible children. All other namespaces (`!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission`, `!RT`) have Polyglot-defined fixed leaves.

Users extend `!Error` via `{!}` blocks using `[:]` for extensible branches and `[.]` for terminal leaves. Siblings at the same level must all use the same separator (sibling homogeneity rule).

```polyglot
{!} !Error
   [:] :MyApp
      [:] :Auth
         [.] .Expired#Error
         [.] .Invalid#Error
      [:] :Data
         [.] .Corrupt#Error
         [.] .Missing#Error
      [:] :GeneralFailure#Error
```

This creates `!Error:MyApp:Auth.Expired`, `!Error:MyApp:Auth.Invalid`, `!Error:MyApp:Data.Corrupt`, `!Error:MyApp:Data.Missing`, and `!Error:MyApp:GeneralFailure`.

Tree path: `%!.Error:MyApp:Auth.Expired` — `.Error` is Polyglot-defined (fixed), `:MyApp:Auth` are user-extensible (flexible), `.Expired` is a terminal leaf (fixed).

## Pipeline Error Associations

Each stdlib pipeline declares the errors it can raise via `[=] !ErrorName` (see [[concepts/pipelines/metadata#Error Trees]]):

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
```

## `!Alias.Clash` — Compile Error

`!Alias.Clash` is a compile error raised when an alias collides with an existing name in the target namespace. Aliases place definitions at multiple locations in the `%` metadata tree; when a target location is already occupied, this error fires.

### `[<] !Alias.Clash` Fallback Chain

In `{M}` type macros, the `[#] <Alias` parameter can provide a fallback chain of alternative alias values using `[<] !Alias.Clash`. The compiler tries each value in order until one succeeds:

```polyglot
[#] <Alias << "int"
   [<] !Alias.Clash << "integer"
   [<] !Alias.Clash << "Integer"
```

- First, the compiler tries `"int"` as the alias
- If `"int"` clashes with an existing name in the target namespace, `!Alias.Clash` fires and the compiler tries `"integer"`
- If `"integer"` also clashes, the compiler tries `"Integer"`
- If all alternatives are exhausted, the compile error propagates (unrecoverable)

This pattern is used in `{M} #String.Subtype` to provide robust alias resolution for scalar type definitions like `##Int`, `##Float`, etc.
