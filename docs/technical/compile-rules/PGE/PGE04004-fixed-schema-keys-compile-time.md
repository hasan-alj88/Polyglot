---
audience: developer
rule: "4.4"
code: PGE04004
name: Fixed-Schema Keys Are Compile-Time Only
severity: error
---

### Rule 4.4 — Fixed-Schema Keys Are Compile-Time Only
`PGE04004`

**Statement:** Fields declared with `.` (fixed separator) in a `{#}` block are compile-time defined. The set of `.` keys is frozen by the schema declaration — at runtime, you cannot add new fixed fields, remove existing ones, or change the schema shape of a fixed level. Changing the schema of a fixed field level is a compile error. Flexible (`:`) field levels are open — new `:` keys can be added at runtime, and the set of `:` keys is not frozen.
**Rationale:** Fixed fields represent a closed, known schema — they exist for type safety and compile-time validation. If fixed fields could be added or removed dynamically, the compiler could not verify field access. Flexible fields exist precisely for dynamic, open-ended data where the key set is not known ahead of time.
**Detection:** The compiler tracks which fields belong to fixed (`.`) schema levels. Any attempt to access or assign a `.`-separated field not declared in the `{#}` definition triggers PGE04004. For `:` fields, no such check is performed — they are open by design.

**VALID:**
```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✓ accessing only declared fixed fields
[r] $user#UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30
```

```polyglot
[ ] ✓ flexible fields — adding keys at runtime is allowed
[r] $meta#serial
   [r] $meta:source << "api"
   [r] $meta:timestamp << "2026-03-18"
   [r] $meta:custom_tag << "urgent"     [ ] ✓ new : key added dynamically
```

```polyglot
{#} #Config
   [.] .timeout#int
   [.] .info#serial

[ ] ✓ fixed level frozen, but :info opens a flexible level
[r] $cfg#Config
   [r] $cfg.timeout << 30
   [r] $cfg.info:author << "admin"      [ ] ✓ : level under .info is flexible
   [r] $cfg.info:version << "1.0"       [ ] ✓ new : key is fine
```

**INVALID:**
```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✗ PGE04004 — .email not declared in fixed schema
[r] $user#UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30
   [r] $user.email << "alice@example.com" [ ] ✗ PGE04004 — fixed schema is closed
```

```polyglot
{#} #Point
   [.] .x#float
   [.] .y#float

[ ] ✗ PGE04004 — cannot add .z to a fixed schema at runtime
[r] $p#Point
   [r] $p.x << 1.0
   [r] $p.y << 2.0
   [r] $p.z << 3.0                       [ ] ✗ PGE04004 — .z not in #Point
```
