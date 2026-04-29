---
audience: developer
rule: "9.24"
code: PGE10010
name: Permission Resource Not Found
severity: error
---

# Rule 9.24 — Permission Resource Not Found
`PGE10010`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/permissions/enforcement -->

**Statement:** If a `{_}` permission object with `.category #File` references a `.path` that does not exist at compile time, PGE10010 fires. This also applies to `.credentials` paths on Database permissions. The compiler must be able to read and content-hash every referenced file during compilation.
**Rationale:** Aljam3's Compile-Time File Binding guarantees that no pipeline runs with stale or missing external resources. If a file doesn't exist at compile time, the compiler cannot compute its content hash, and the permission grant is invalid. This is a security property — external files kept outside of git (secrets, config) must be present and hashable at compilation.
**Detection:** After resolving all `{_}` definitions (including template substitution):
1. For each `{_}` with `.category #File`: read the file at `.path`
2. For each `{_}` with a `.credentials` field: read the file at `.credentials`
3. If any file does not exist or is not readable, PGE10010 fires on the `{_}` definition

**See also:** PGE10009 (unresolved permission template — template inputs must resolve before path validation), PGE10004 (undeclared permission), [[permissions/enforcement#Compile-Time File Binding]]

**VALID:**
```aljam3
[ ] ✓ file exists at compile time — compiler reads and hashes it
{_} _AppConfig
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/app.json"
   [.] .path "/config/app.json"              [ ] ✓ file exists — content-hashed
   [.] .format #JSON
```

**INVALID:**
```aljam3
[ ] ✗ PGE10010 — file does not exist at compile time
{_} _MissingConfig
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/nonexistent.yaml"
   [.] .path "/config/nonexistent.yaml"      [ ] ✗ PGE10010 — file not found
   [.] .format #YAML
```

```aljam3
[ ] ✗ PGE10010 — credentials file does not exist
{_} _BadDB
   [.] .intent << #Grant
   [.] .category #Database
   [.] .capability #Read
   [.] .scope "analytics.postgres"
   [.] .host "db.internal"
   [.] .port 5432
   [.] .database "analytics"
   [.] .credentials "/keys/missing.json"     [ ] ✗ PGE10010 — credentials file not found
```

**Open point:** None.
