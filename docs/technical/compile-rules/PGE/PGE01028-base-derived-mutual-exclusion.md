---
rule: "1.28"
code: PGE01028
name: Base/Derived Pipeline Mutual Exclusion
severity: error
---

### Rule 1.28 — Base/Derived Pipeline Mutual Exclusion
`PGE01028`

**Statement:** Base and derived pipeline constraints are mutually exclusive. A definition with `.baseCode` metadata cannot have an execution body; a definition with an execution body cannot have `.baseCode`. A bodyless `{=}` pipeline without `.baseCode` is also an error (exception: `{T}` and `{Q}` are IO-only by design).
**Rationale:** Base pipelines delegate to native code — a Polyglot body would conflict with the native implementation. Derived pipelines are pure Polyglot — `.baseCode` would create ambiguity about which implementation runs. Bodyless `{=}` without `.baseCode` is dead code with no implementation.
**Detection:** The compiler checks for mutual exclusion between `.baseCode` metadata and execution body elements (`[T]`, `[Q]`, `[W]`, `[r]`, `[p]`, `[b]`, `[s]`, `[{]`, `[}]`, `[\]`, `[/]`). It also validates that `.baseCode` references an existing `#BaseCode` variant using the configured base language.

**Sub-conditions:**

| Condition | Trigger | Error |
|-----------|---------|-------|
| a | `.baseCode` + execution body | Base pipeline cannot have execution body |
| b | Execution body + `.baseCode` | Derived pipeline cannot have `.baseCode` |
| c | No body + no `.baseCode` on `{=}` | Bodyless pipeline must be base (except `{T}`/`{Q}`) |
| d | `.baseCode` references non-existent variant | Invalid `#BaseCode` variant |
| e | `.baseCode` uses wrong base language | Config says `base: Rust` but code uses different prefix |

**VALID:**
```polyglot
[ ] ✓ base execution pipeline — bodyless with .baseCode
{=}[exe] =File.Text.Read
   [%] .baseCode << #BaseCode.Rust.File.Text.Read
   [=] <path#path
   [=] >content#string
   [=] !File.NotFound
   [=] !File.PermissionDenied

[ ] ✓ derived execution pipeline — body without .baseCode
{=} =ProcessData
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoWork

[ ] ✓ base trigger — bodyless with .baseCode
{T} =T.Call
   [%] .baseCode << #BaseCode.Rust.T.Call
   [=] >IsTriggered#bool

[ ] ✓ derived trigger — IO-only, no body, no .baseCode (OK for {T})
{T} =T.DailyWebhookReady
   [T] =T.Daily"3AM"
   [T] =T.Webhook"/api/ready"
   [=] >IsTriggered#bool

[ ] ✓ base wrapper — bodyless with .baseCode
{W} =W.Polyglot
   [%] .baseCode << #BaseCode.Rust.W.Polyglot

[ ] ✓ derived wrapper — body without .baseCode
{W} =W.DB.Connection
   [{] $connectionString#string
   [}] $dbConn
   [\]
      [r] =DB.Connect
         [=] <connStr << $connectionString
         [=] >conn >> $dbConn
   [/]
      [r] =DB.Disconnect
         [=] <conn << $dbConn
```

**INVALID:**
```polyglot
[ ] ✗ PGE01028a — base pipeline cannot have execution body
{=}[exe] =Bad.BaseWithBody
   [%] .baseCode << #BaseCode.Rust.File.Text.Read
   [=] <path#path
   [=] >content#string
   [r] =SomeWork

[ ] ✗ PGE01028b — derived pipeline cannot have .baseCode
{=}[exe] =Bad.DerivedWithBase
   [%] .baseCode << #BaseCode.Rust.DoNothing
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =DoSomething

[ ] ✗ PGE01028b — derived wrapper cannot have .baseCode
{W} =W.Bad.DerivedWithBase
   [%] .baseCode << #BaseCode.Rust.W.Polyglot
   [{] $input#string
   [\]
      [r] =DoNothing

[ ] ✗ PGE01028a — base wrapper cannot have [{]/[}]/[\]/[/]
{W} =W.Bad.BaseWithBody
   [%] .baseCode << #BaseCode.Rust.W.Polyglot
   [{] $input#string

[ ] ✗ PGE01028d — .baseCode references non-existent variant
{=}[exe] =Bad.InvalidBase
   [%] .baseCode << #BaseCode.Rust.NotARealThing
   [=] <x#string

[ ] ✗ PGE01028e — .baseCode uses wrong base language
{=}[exe] =Bad.WrongBase
   [%] .baseCode << #BaseCode.Go.File.Text.Read
   [=] <path#path
```
