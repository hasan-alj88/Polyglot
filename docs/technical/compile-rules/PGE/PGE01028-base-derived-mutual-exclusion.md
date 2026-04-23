---
audience: developer
rule: "1.28"
code: PGE01028
name: Native/Derived Block Mutual Exclusion
severity: error
---

# Rule 1.28 — Native/Derived Block Mutual Exclusion
`PGE01028`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Native `{N}` and derived `{-}` block types are mutually exclusive. A `{N}` definition cannot have an execution body; a `{-}` definition cannot have `%Native.*` metadata. A `{N}` definition without `%Native.Kind` is also an error (exception: `{T}` and `{Q}` are IO-only subtypes of `{-}` by design).
**Rationale:** Native definitions delegate to host language code — a Polyglot execution body would conflict with the native implementation. Derived definitions are pure Polyglot — `%Native.*` metadata would create ambiguity about which implementation runs. A `{N}` block without `.Kind` has no subsystem role, making it unresolvable.
**Detection:** The compiler checks for mutual exclusion between `{N}` block type and execution body elements (`[T]`, `[Q]`, `[W]`, `[-]`, `[=]`, `[b]`, `[s]`, `[\]`, `[/]`). It also validates that `{N}` blocks declare `%Native.Kind` with a valid `#NativeKind` variant and have a `.<Language>` field matching the language resolved for the operation's subsystem (via `native.defaults` or `native.overrides` in the service config).

**Sub-conditions:**

| Condition | Trigger | Error |
|-----------|---------|-------|
| a | `{N}` + execution body | Native definition cannot have execution body |
| b | `{-}` + `%Native.Kind` | Derived pipeline cannot have native metadata |
| c | `{N}` without `%Native.Kind` | Native definition must declare Kind |
| d | `%Native.Kind` references non-existent `#NativeKind` variant | Invalid `#NativeKind` variant |
| e | `{N}` without `.<Language>` for resolved subsystem language | Config resolves `Rust` for this operation but `{N}` has no `.Rust` field |

**VALID:**
```polyglot
[ ] ✓ native execution pipeline — {N} with %Native metadata, no body
{N} -File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   (-) <path#path
   (-) >content#string
   (-) !File.NotFound
   (-) !File.PermissionDenied

[ ] ✓ derived execution pipeline — {-} with full Polyglot body
{-} -ProcessData
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -DoWork

[ ] ✓ native trigger — {N} with Trigger kind
{N} -T.Call
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TriggerCall"
   [%] .description << "Pipeline invoked by another pipeline"
   (-) >IsTriggered#bool

[ ] ✓ derived trigger — IO-only {T}, no body, no %Native (OK for {T})
{T} -T.DailyWebhookReady
   [T] -T.Daily"3AM"
   [T] -T.Webhook"/api/ready"
   (-) >IsTriggered#bool

[ ] ✓ native wrapper — {N} with Wrapper kind
{N} -W.Polyglot
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WrapperPolyglot"
   [%] .description << "Default Polyglot wrapper"

[ ] ✓ derived wrapper — {W} with body, no %Native
{W} -W.DB.Connection
   (-) <connectionString;string
   (-) >dbConn
   [\]
      [-] -DB.Connect
         (-) <connStr << $connectionString
         (-) >conn >> $dbConn
   [/]
      [-] -DB.Disconnect
         (-) <conn << $dbConn

[ ] ✓ native intrinsic — compiler built-in, no host function needed
{N} -DoNothing
   [%] .Kind << #NativeKind.Intrinsic
   [%] .description << "No-op pipeline"
```

**INVALID:**
```polyglot
[ ] ✗ PGE01028a — native definition cannot have execution body
{N} -Bad.NativeWithBody
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "BadNativeWithBody"
   (-) <path#path
   (-) >content#string
   [-] -SomeWork

[ ] ✗ PGE01028b — derived pipeline cannot have native metadata
{-} -Bad.DerivedWithNative
   [%] .Kind << #NativeKind.Execution
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -DoSomething

[ ] ✗ PGE01028c — native definition must declare Kind
{N} -Bad.NoKind
   [%] .Rust << "BadNoKind"
   (-) <x#string

[ ] ✗ PGE01028d — invalid #NativeKind variant
{N} -Bad.InvalidKind
   [%] .Kind << #NativeKind.Storage
   [%] .Rust << "BadInvalidKind"
   (-) <x#string

[ ] ✗ PGE01028e — missing language field for resolved subsystem language
{N} -Bad.NoLangField
   [%] .Kind << #NativeKind.Execution
   [%] .description << "No .Rust field but config resolves Rust for this operation"
   (-) <x#string
```
