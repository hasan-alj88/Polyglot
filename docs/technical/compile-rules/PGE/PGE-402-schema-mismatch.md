---
rule: "4.2"
code: PGE-402
name: Schema Mismatch
severity: error
---

### Rule 4.2 — Schema Mismatch
`PGE-402`

**Statement:** When a `{#}` data instance is used where a specific schema is expected, all required fields declared in the `{#}` definition must be satisfied — either by explicit assignment or by having a default value (`<~`). A required field that is never assigned and has no default is a compile error. This rule checks field **completeness** — whether the instance satisfies its schema. Field **presence** on fixed schemas (extra undeclared `.` fields) is checked by PGE-404; field **type** correctness and schema matching are checked per [TYPE-IDENTITY](../TYPE-IDENTITY.md).
**Rationale:** An incomplete data instance cannot be safely consumed by downstream pipelines that expect all schema fields to be available. Catching missing fields at compile time prevents runtime null/missing-field errors.
**Detection:** The compiler resolves the `{#}` definition, enumerates all required fields (those without `<~` defaults), and verifies each is assigned before the instance is consumed. Unassigned required fields trigger PGE-402.

**Boundary with PGE-401:** PGE-402 fires when a data instance has the **correct schema** but is **incomplete** (missing required field values). If the schemas themselves are structurally different (different fields or field types), see PGE-401.

**VALID:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✓ all required fields assigned
[r] $user;UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30
```

```polyglot
{#} #Config
   [.] .timeout;int <~ 30
   [.] .retries;int <~ 3

[ ] ✓ defaults satisfy all fields — no explicit assignment needed
[r] $cfg;Config
```

```polyglot
{#} #Request
   [.] .url;string
   [.] .method;string <~ "GET"

[ ] ✓ .method has default, only .url requires assignment
[r] $req;Request
   [r] $req.url << "https://example.com"
```

**INVALID:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✗ PGE-402 — missing required field .age (no default, never assigned)
[r] $user;UserRecord
   [r] $user.name << "Alice"
   [ ] .age has no default and is never assigned — ✗ PGE-402
```

```polyglot
{#} #Connection
   [.] .host;string
   [.] .port;int
   [.] .protocol;string

[ ] ✗ PGE-402 — .host and .protocol never assigned
[r] $conn;Connection
   [r] $conn.port << 8080
   [ ] .host and .protocol missing — ✗ PGE-402
```
