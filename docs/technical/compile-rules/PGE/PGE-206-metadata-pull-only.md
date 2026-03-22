---
rule: "2.6"
code: PGE-206
name: Live Metadata Fields Are Pull-Only
severity: error
---

### Rule 2.6 — `live` Metadata Fields Are Pull-Only
`PGE-206`

**Statement:** `live`-typed `%` metadata fields are managed by the Polyglot runtime and can only be pulled from. Any push into a `live` `%` field is a compile error. This rule applies only to `live` fields — non-live `%` fields (e.g., `%pipeline.*.description`) are user-assignable and follow normal lifecycle rules.
**Rationale:** `live` fields reflect runtime state that only the runtime can change (e.g., a variable's lifecycle stage, a pipeline's execution status). Allowing user code to push into `live` metadata would break runtime invariants. Non-live metadata fields are user-defined and follow standard push/pull rules.
**Detection:** At any assignment statement where the target uses the `%` accessor on a `live`-typed field — the compiler rejects the push regardless of operator (`<<`, `>>`, `<~`, `~>`).

**Metadata access on Failed-state variables:**
1. **Static metadata** (`%description`, `%version`, etc.) is always accessible regardless of variable state — these are compile-time constants.
2. **`%state`** is always accessible and returns the current state, including `"failed"`.
3. **`live` metadata** on Failed variables is frozen at the point of failure — last known values remain readable. This allows `[!]` error handlers to inspect the failed variable's metadata for recovery decisions.

**`%` schema tree:** Metadata is organized by block type — `%pipeline.*` for `{=}` pipelines, `%#.*` for `{#}` data definitions, `%M.*` for `{M}` macros. The `*` in the path is the instance reference — one pipeline definition may span several concurrent instances, each with its own metadata. The schema is fixed per block type (all instances share the same field structure). See resolved design issue 002 (git history: `docs/technical/compiler_issues/002-metadata-schema-tree.md`) for full schema documentation status.

**VALID:**
```polyglot
[ ] ✓ pulling live %state in a conditional
[=] >data;string
[r] =Fetch
   [=] >payload >> >data
[?] >data%state
   [?] #Final
      [r] =Process
         [=] <input << >data
   [?] *?
      [r] =Log
         [=] <msg << "not ready"
```

```polyglot
[ ] ✓ pulling live %status from a pipeline reference
[?] =Fetch%status
   [?] #Running
      [r] =Log
         [=] <msg << "still running"
   [?] *?
      [r] =Log
         [=] <msg << "done or failed"
```

```polyglot
[ ] ✓ pushing into a non-live % field is allowed
[%] .description;string << "Processes incoming invoices"
```

**INVALID:**
```polyglot
[ ] ✗ PGE-206 — pushing into a live %state field
[=] >data;string
[r] >data%state << #Final          [ ] ✗ PGE-206 — %state is live, cannot push
```

```polyglot
[ ] ✗ PGE-206 — default-pushing into a live %status field
[r] =Pipeline%status <~ #Running   [ ] ✗ PGE-206 — %status is live, cannot push
```
