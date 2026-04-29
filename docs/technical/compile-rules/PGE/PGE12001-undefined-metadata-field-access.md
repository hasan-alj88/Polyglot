---
audience: developer
rule: "12.1"
code: PGE12001
name: Undefined Metadata Field Access
severity: error
---

# Rule 12.1 — Undefined Metadata Field Access
`PGE12001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A metadata access expression (`$var%field`, `-Pipeline%field`, `#Data%field`) must reference a field that exists in the metadata schema for that object type. Each object type has a fixed set of valid metadata fields — accessing a field outside that schema is a compile error.
**Rationale:** The `%` tree has a well-defined schema per object type. An invalid field access like `$count%description` (variables have no `.description` metadata) is always a logic error. Catching this at compile time prevents silent empty reads or undefined behavior at runtime.
**Detection:** The compiler resolves the object type from the `%` access expression and checks field existence. Fixed metadata fields (`%state`, `%status`, etc.) and user-declared `[%]` fields are a direct schema lookup. The `%info` field (`#serial`) requires fetching from the metadata tree to confirm the subfield path exists. Any field reference not found is rejected.

**See also:** PGE02006 (metadata pull-only — `live` fields cannot be pushed), PGE04006 (undefined variable reference), PGE04005 (undefined field reference)

---

## Valid Metadata Fields by Object Type

### Variable (`$`)

| Field | Type |
|-------|------|
| `%state` | `#live.#VarState` |
| `%sourceError` | `#live.error` |

### Pipeline (`{-}`)

| Field | Type |
|-------|------|
| `%status` | `#live.#PipelineStatus` |
| `%errors` | `#live.array.error` |
| `%isSuccess` | `#live.#Boolean` |
| `%instanceCount` | `#live.int` |
| `%lastRun` | `#live.string` |
| `%duration` | `#live.string` |
| `%triggerCount` | `#live.int` |
| `%description` | `#string` (user-declared `[%]`) |
| `%version` | `#string` (user-declared `[%]`) |
| `%authors` | `#array:string` (user-declared `[%]`) |
| `%license` | `#string` (user-declared `[%]`) |
| `%deprecated` | `#bool` (user-declared `[%]`) |
| `%deprecatedMessage` | `#string` (user-declared `[%]`) |
| `%alias` | — (user-declared `[%]`) |
| `%info` | `#serial` (user-declared `[%]`) |

### Data (`{#}`)

| Field | Type |
|-------|------|
| `%lastModified` | `#live.string` |
| `%files` | `#live.array.path` |
| `%errors` | `#live.array.error` |
| `%usageCount` | `#live.int` |
| `%description` | `#string` (user-declared `[%]`) |
| `%version` | `#string` (user-declared `[%]`) |
| `%authors` | `#array:string` (user-declared `[%]`) |
| `%license` | `#string` (user-declared `[%]`) |
| `%deprecated` | `#bool` (user-declared `[%]`) |
| `%deprecatedMessage` | `#string` (user-declared `[%]`) |
| `%alias` | — (user-declared `[%]`) |
| `%info` | `#serial` (user-declared `[%]`) |

---

**VALID:**
```polyglot
[ ] ✓ variable live metadata — %state exists for all $variables
{-} -CheckState
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [ ]
   [?] $input%state
      [?] =? .Final
         [-] >out << $input
      [?] *?
         [-] >out << "not ready"
```

```polyglot
[ ] ✓ pipeline live metadata — %status exists for all pipelines
{-} -Monitor
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [ ]
   [?] -Worker%status
      [?] =? .Running
         [-] >out << "active"
      [?] *?
         [-] >out << "idle"
```

```polyglot
[ ] ✓ user-declared metadata — %description exists when declared in [%]
{-} -Documented
   [%] .description << "Handles payments"
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [ ]
   [-] >out << -Documented%description
```

**INVALID:**
```polyglot
[ ] ✗ PGE12001 — $variable has no %description field
{-} -BadAccess
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <count#int
   (-) >out#string
   [ ]
   [-] >out << $count%description       [ ] ✗ PGE12001 — $var has no .description
```

```polyglot
[ ] ✗ PGE12001 — pipeline has no %memory field
{-} -BadPipeline
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [ ]
   [-] >out << -Worker%memory           [ ] ✗ PGE12001 — no .memory in pipeline schema
```

```polyglot
[ ] ✗ PGE12001 — %isSuccess is pipeline-only, not variable
{-} -WrongScope
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >out#string
   [ ]
   [?] $data%isSuccess                  [ ] ✗ PGE12001 — .isSuccess is pipeline metadata
      [?] =? .True
         [-] >out << "ok"
      [?] *?
         [-] >out << "fail"
```

**Open point:** None.
