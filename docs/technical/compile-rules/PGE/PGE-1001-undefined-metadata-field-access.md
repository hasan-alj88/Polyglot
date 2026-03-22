---
rule: "10.1"
code: PGE-1001
name: Undefined Metadata Field Access
severity: error
---

### Rule 10.1 — Undefined Metadata Field Access
`PGE-1001`

**Statement:** A metadata access expression (`$var%field`, `=Pipeline%field`, `#Data%field`) must reference a field that exists in the metadata schema for that object type. Each object type has a fixed set of valid metadata fields — accessing a field outside that schema is a compile error.
**Rationale:** The `%` tree has a well-defined schema per object type. An invalid field access like `$count%description` (variables have no `.description` metadata) is always a logic error. Catching this at compile time prevents silent empty reads or undefined behavior at runtime.
**Detection:** The compiler resolves the object type from the `%` access expression, looks up the metadata schema for that type, and rejects any field reference not in the schema. Both `live` fields and user-declared `[%]` fields are checked.

**See also:** PGE-206 (metadata pull-only — `live` fields cannot be pushed), PGE-406 (undefined variable reference), PGE-405 (undefined field reference)

---

#### Valid Metadata Fields by Object Type

##### Variable (`$`)

| Field | Type |
|-------|------|
| `%state` | `;live.#VarState` |
| `%sourceError` | `;live.error` |

##### Pipeline (`{=}`)

| Field | Type |
|-------|------|
| `%status` | `;live.#PipelineStatus` |
| `%errors` | `;live.array.error` |
| `%isSuccess` | `;live.#Boolean` |
| `%instanceCount` | `;live.int` |
| `%lastRun` | `;live.string` |
| `%duration` | `;live.string` |
| `%triggerCount` | `;live.int` |
| `%description` | `;string` (user-declared `[%]`) |
| `%version` | `;string` (user-declared `[%]`) |
| `%authors` | `;array.string` (user-declared `[%]`) |
| `%license` | `;string` (user-declared `[%]`) |
| `%deprecated` | `;bool` (user-declared `[%]`) |
| `%deprecatedMessage` | `;string` (user-declared `[%]`) |
| `%alias` | — (user-declared `[%]`) |
| `%info` | `;serial` (user-declared `[%]`) |

##### Data (`{#}`)

| Field | Type |
|-------|------|
| `%lastModified` | `;live.string` |
| `%files` | `;live.array.path` |
| `%errors` | `;live.array.error` |
| `%usageCount` | `;live.int` |
| `%description` | `;string` (user-declared `[%]`) |
| `%version` | `;string` (user-declared `[%]`) |
| `%authors` | `;array.string` (user-declared `[%]`) |
| `%license` | `;string` (user-declared `[%]`) |
| `%deprecated` | `;bool` (user-declared `[%]`) |
| `%deprecatedMessage` | `;string` (user-declared `[%]`) |
| `%alias` | — (user-declared `[%]`) |
| `%info` | `;serial` (user-declared `[%]`) |

---

**VALID:**
```polyglot
[ ] ✓ variable live metadata — %state exists for all $variables
{=} =CheckState
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [?] $input%state
      [?] =? .Final
         [r] >out << $input
      [?] *?
         [r] >out << "not ready"
```

```polyglot
[ ] ✓ pipeline live metadata — %status exists for all pipelines
{=} =Monitor
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [?] =Worker%status
      [?] =? .Running
         [r] >out << "active"
      [?] *?
         [r] >out << "idle"
```

```polyglot
[ ] ✓ user-declared metadata — %description exists when declared in [%]
{=} =Documented
   [%] .description << "Handles payments"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] >out << =Documented%description
```

**INVALID:**
```polyglot
[ ] ✗ PGE-1001 — $variable has no %description field
{=} =BadAccess
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <count;int
   [=] >out;string
   [r] >out << $count%description       [ ] ✗ PGE-1001 — $var has no .description
```

```polyglot
[ ] ✗ PGE-1001 — pipeline has no %memory field
{=} =BadPipeline
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] >out << =Worker%memory           [ ] ✗ PGE-1001 — no .memory in pipeline schema
```

```polyglot
[ ] ✗ PGE-1001 — %isSuccess is pipeline-only, not variable
{=} =WrongScope
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >out;string
   [?] $data%isSuccess                  [ ] ✗ PGE-1001 — .isSuccess is pipeline metadata
      [?] =? .True
         [r] >out << "ok"
      [?] *?
         [r] >out << "fail"
```

**Open point:** None.
