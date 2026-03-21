---
rule: "4.9"
code: PGE-409
name: Unhandled Serial→Struct Conversion
severity: error
---

### Rule 4.9 — Unhandled Serial→Struct Conversion
`PGE-409`

**Statement:** When a `serial` value is pushed into a `struct`-typed target and the compiler cannot statically prove that the serial's fields satisfy the struct's schema, the push must be wrapped in `[!]` error handling with `*Continue >FallBack`. If the error handling is absent, the compiler raises PGE-409.
**Rationale:** Murphy's Law — if a serial→struct conversion can fail, it will fail. Serial is schema-free; struct has a fixed schema. When the compiler cannot guarantee compatibility, the user must handle the mismatch explicitly. This ensures every uncertain conversion has a recovery path with a known-good fallback value. No warnings, no hoping for the best.
**Detection:** The compiler identifies every assignment where a `;serial` source flows into a `;StructType` target. If the serial's fields are all statically known and provably match → no action needed. If the serial's fields are all statically known and provably wrong → PGE-402. Otherwise (partial knowledge, dynamic origin, conditional fields), the compiler checks for an enclosing `[!]` block with `*Continue >FallBack`. Absent → PGE-409.

**Three outcomes for serial→struct push:**

| Compiler knowledge | Result |
|--------------------|--------|
| Provably matches | No handling needed |
| Provably mismatches | PGE-402 (schema mismatch) |
| Cannot prove match | `[!]` + `*Continue >FallBack` required — PGE-409 if absent |

**VALID:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✓ statically provable — all serial fields known and match
[r] $data;serial
   [r] $data:name << "Alice"
   [r] $data:age << 30
[r] $user;UserRecord << $data
```

```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✓ dynamic serial — handled with [!] + *Continue >FallBack
[r] $defaultUser;UserRecord
   [r] $defaultUser.name << "Unknown"
   [r] $defaultUser.age << 0

[r] $user;UserRecord << $dynamicSerial
   [!] !SchemaMismatch
      [*] *Continue >FallBack << $defaultUser
```

```polyglot
{#} #Config
   [.] .timeout;int
   [.] .retries;int

[ ] ✓ extra fields in serial are allowed — superset is fine
[r] $raw;serial
   [r] $raw:timeout << 30
   [r] $raw:retries << 3
   [r] $raw:debug << "true"
[r] $cfg;Config << $raw              [ ] ✓ :debug ignored, schema satisfied
```

**INVALID:**
```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✗ PGE-409 — dynamic serial pushed to struct without error handling
[r] $user;UserRecord << $dynamicSerial
```

```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✗ PGE-409 — conditionally built serial, compiler cannot prove completeness
[r] $data;serial
   [r] $data:name << "Alice"
   [?] $hasAge =? true
      [r] $data:age << 30
   [?] *?
      [ ] .age not assigned in this branch
[r] $user;UserRecord << $data        [ ] ✗ PGE-409 — .age may be missing
```

**Partial matching rules:**
- Serial has **more** fields than struct requires → **allowed**. Extra `:` fields are ignored.
- Serial has **fewer** fields than struct requires → **error**. Missing fields mean the struct schema cannot be fulfilled.

**See also:**
- [PGE-402 — Schema Mismatch](PGE-402-schema-mismatch.md) — provable static mismatch
- [PGE-207 — Continue After Error](PGE-207-continue-after-error.md) — `*Continue >FallBack` rules
- [TYPE-IDENTITY](../TYPE-IDENTITY.md) — rule 6 (serial→struct)
- [serial-to-struct-matching](../../brainstorming/serial-to-struct-matching.md) — decision rationale
