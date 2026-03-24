---
audience: user
type: specification
updated: 2026-03-24
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
   [.] .Name;RawString
   [.] .ErrorAlias;RawString
   [.] .Message;RawString
   [.] .Info;serial
```

| Field | Filled by | Purpose |
|-------|-----------|---------|
| `.Name` | Runtime (auto) | Full error identifier (e.g., `"File.NotFound"`) |
| `.ErrorAlias` | Author | Short display name for logging/UI |
| `.Message` | Author (at raise site) | Human-readable description |
| `.Info` | Author (at raise site) | Arbitrary context data as `;serial` |

## Defining Custom Errors (`{!}`)

`{!}` defines an error tree. Each leaf is typed `;#Error`:

```polyglot
{!} !Validation
   [.] .Empty;#Error
   [.] .TooLong;#Error
   [.] .InvalidEmail;#Error
```

This creates `!Validation.Empty`, `!Validation.TooLong`, `!Validation.InvalidEmail` — all carrying the `#Error` struct.

`{!}` creates entries at `%!:Namespace.Error` in the metadata tree. See [[data-is-trees#How Concepts Connect]].

## Built-in Error Namespaces

No `[@]` import needed. Stdlib errors are defined as `{!}` blocks by the runtime:

```polyglot
{!} !File
   [.] .NotFound;#Error
   [.] .ReadError;#Error
   [.] .WriteError;#Error

{!} !No
   [.] .Input;#Error
   [.] .Output;#Error

{!} !Timeout
   [.] .Connection;#Error
   [.] .Read;#Error

{!} !Math
   [.] .DivideByZero;#Error

{!} !Validation
   [.] .Error;#Error
```

## Pipeline Error Associations

Each stdlib pipeline declares the errors it can raise via `[=] !ErrorName` (see [[pipelines#Error Trees]]):

```
=File.Text.Read
   [=] !File.NotFound
   [=] !File.ReadError

=File.Text.Write
   [=] !File.NotFound
   [=] !File.WriteError

=File.Text.Append
   [=] !File.NotFound
   [=] !File.WriteError

=Math.Divide
   [=] !Math.DivideByZero

=Math.Modulo
   [=] !Math.DivideByZero
```
