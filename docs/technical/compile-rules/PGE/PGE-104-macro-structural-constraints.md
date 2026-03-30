---
rule: "1.4"
code: PGE-104
name: Macro Structural Constraints
---

### Rule 1.4 — Macro Structural Constraints
`PGE-104`

**Statement:** A `{M}` macro definition is restricted to setup/cleanup elements only. The complete element whitelist:

| Element | Allowed | Scope |
|---------|---------|-------|
| `[{]` / `[}]` | Yes | Macro IO — inputs and outputs |
| `[\]` / `[/]` | Yes | Setup and cleanup scopes |
| `[r]` | Yes | Pipeline calls within `[\]` or `[/]` |
| `[W]` | Yes | Composite wrapper calls within `[\]` or `[/]` |
| `[?]` | Yes | Conditionals within `[\]` or `[/]` — for branching setup logic |
| `[!]` | Yes | Error handlers scoped under `[r]` calls within `[\]` or `[/]` |
| `[t]` | **No** | Triggers are pipeline-only |
| `[Q]` | **No** | Queues are pipeline-only |
| `[=]` | **No** | Pipeline-level IO is pipeline-only |
| `[p]` | **No** | Parallel execution — macros are sequential |
| `[b]` | **No** | Fire-and-forget — macros are sequential |
| `[*]` | **No** | Collectors — no parallel to collect from |

**Rationale:** Macros exist to encapsulate reusable setup/cleanup logic. Allowing triggers, queues, or parallel execution inside a macro would make it behave like a pipeline, breaking the structural separation between lifecycle wrappers and executable pipelines. Conditionals and error handlers are allowed because setup logic may need to branch or handle failures.

**VALID:**
```polyglot
[ ] ✓ macro with [{]/[}] IO, [\]/[/] scopes, and composite [W] inside
{M} =W.DB.Transaction
   [{] $connStr#string
   [}] $txHandle#string

   [\]
      [ ] ✓ composite wrapper inside macro setup
      [W] =W.DB.Connection
         [=] $connStr << $connStr
         [=] $conn >> $conn
      [r] =DB.BeginTransaction
         [=] <conn << $conn
         [=] >handle >> $txHandle

   [/]
      [r] =DB.Commit
         [=] <handle << $txHandle
```

```polyglot
[ ] ✓ conditional inside macro setup — branching on input
{M} =W.Cache.Init
   [{] $backend#string
   [}] $cacheHandle#string

   [\]
      [?] $backend
         [?] =? "redis"
            [r] =Cache.Redis.Connect
               [=] >handle >> $cacheHandle
         [?] =? "memory"
            [r] =Cache.Memory.Init
               [=] >handle >> $cacheHandle
         [?] *?
            [r] =Cache.Default.Init
               [=] >handle >> $cacheHandle

   [/]
      [r] =Cache.Disconnect
         [=] <handle << $cacheHandle
```

```polyglot
[ ] ✓ error handler inside macro setup
{M} =W.Service.Init
   [{] $config#string
   [}] $serviceHandle#string

   [\]
      [r] =Service.Connect
         [=] <config << $config
         [=] >handle >> $serviceHandle
      [!] !Connect.Failed
         [r] =Log.Error
            [=] <msg << "Service connection failed"

   [/]
      [r] =Service.Disconnect
         [=] <handle << $serviceHandle
```

**INVALID:**
```polyglot
[ ] ✗ PGE-104 — [t] inside a macro
{M} =W.Bad
   [{] $input#string
   [}] $output#string

   [\]
      [t] =T.Call    [ ] ✗ PGE-104 — triggers not allowed in macros
```

```polyglot
[ ] ✗ PGE-104 — [Q] inside a macro
{M} =W.Bad
   [\]
      [Q] =Q.Default    [ ] ✗ PGE-104 — queues not allowed in macros
```

```polyglot
[ ] ✗ PGE-104 — [p] inside a macro
{M} =W.Bad
   [{] $input#string
   [}] $output#string

   [\]
      [p] =Fetch.Data    [ ] ✗ PGE-104 — parallel not allowed in macros
         [=] <in << $input
         [=] >out >> $output
```

### See Also

- [[concepts/pipelines/wrappers|Wrappers]] — user-facing wrapper documentation (references PGE-104)
- [[concepts/macros|Macros]] — user-facing macro concept overview
