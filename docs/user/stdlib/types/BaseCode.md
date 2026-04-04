---
audience: pg-coder
type: specification
updated: 2026-04-04
status: complete
---

# #BaseCode Enum

<!-- @types -->
<!-- @concepts/pipelines/INDEX -->

`#BaseCode` is a language-prefixed enum that mirrors stdlib pipeline names. Each leaf maps to exactly one **base pipeline's** native implementation — the compiler resolves `[%] .baseCode << #BaseCode.<language>.<pipeline>` to the backing native code.

See [[concepts/pipelines/INDEX#Base vs Derived|Base vs Derived pipelines]] for when and why `.baseCode` is used.

---

## Definition

```polyglot
{#} #BaseCode
   [%] .description << "Native implementation registry for base pipelines"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [.] .Rust
      [.] .T
         [.] .Call
         [.] .Daily
         [.] .Webhook
         [.] .Folder
            [.] .NewFiles
      [.] .File
         [.] .Text
            [.] .Read
            [.] .Write
         [.] .Binary
            [.] .Read
      [.] .DB
         [.] .Connect
         [.] .Disconnect
         [.] .Query
         [.] .BeginTransaction
         [.] .Commit
         [.] .Rollback
      [.] .Math
         [.] .Add
         [.] .Subtract
         [.] .Multiply
         [.] .Divide
      [.] .DoNothing
      [.] .RT
         [.] .Python
            [.] .Script
            [.] .Function
            [.] .SetupEnv
            [.] .TeardownEnv
         [.] .JS
            [.] .Script
            [.] .Function
            [.] .SetupEnv
            [.] .TeardownEnv
         [.] .Shell
            [.] .Script
      [.] .Q
         [.] .Default
         [.] .Pause
            [.] .Hard
         [.] .Resume
         [.] .Kill
            [.] .Graceful
      [.] .W
         [.] .Polyglot
```

Each variant path mirrors its stdlib pipeline name exactly:
- `#BaseCode.Rust.File.Text.Read` → `=File.Text.Read`
- `#BaseCode.Rust.T.Call` → `=T.Call`
- `#BaseCode.Rust.W.Polyglot` → `=W.Polyglot`

---

## Configuration

The Polyglot config file selects the active base language:

```
base: Rust
```

The compiler validates that all `#BaseCode` references use the configured base language. If a pipeline references `#BaseCode.Go.File.Text.Read` but the config says `base: Rust`, that is a compile error (PGE01028).

Future base languages (e.g., `#BaseCode.Go.*`) can be added by expanding the enum — no pipeline definitions need to change, only the config and enum grow.

---

## Usage

Base pipelines declare `.baseCode` in their `[%]` metadata:

```polyglot
{=}[exe] =File.Text.Read
   [%] .baseCode << #BaseCode.Rust.File.Text.Read
   [=] <path#path
   [=] >content#string
   [=] !File.NotFound
   [=] !File.PermissionDenied
```

Only stdlib base pipelines use `.baseCode`. User-defined pipelines are always **derived** — they have full Polyglot execution bodies and no `.baseCode` field.

---

## Related

- [[concepts/pipelines/INDEX#Base vs Derived|Base vs Derived pipelines]] — base vs derived distinction
- [[enums]] — other stdlib enum types
- [[syntax/types/INDEX|types]] — full type system specification
