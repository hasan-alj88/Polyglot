---
audience: developer
rule: "4.9"
code: PGE04009
name: Unhandled Serial→Struct Conversion
severity: error
---

### Rule 4.9 — Unhandled Serial→Struct Conversion
`PGE04009`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/errors -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** When a `serial` value is pushed into a `struct`-typed target and the compiler cannot statically prove that the serial's fields satisfy the struct's schema, the push must be wrapped in `[!]` error handling with `!< fallback`. If the error handling is absent, the compiler raises PGE04009.
**Rationale:** Murphy's Law — if a serial→struct conversion can fail, it will fail. Serial is unconstrained; struct has a fixed schema. When the compiler cannot guarantee compatibility, the user must handle the mismatch explicitly. This ensures every uncertain conversion has a recovery path with a known-good fallback value. No warnings, no hoping for the best.
**Detection:** The compiler identifies every assignment where a `#serial` source flows into a `#StructType` target. If the serial's fields are all statically known and provably match → no action needed. If the serial's fields are all statically known and provably wrong → PGE04002. Otherwise (partial knowledge, dynamic origin, conditional fields), the compiler checks for an enclosing `[!]` block with `!< fallback`. Absent → PGE04009.

**Three outcomes for serial→struct push:**

| Compiler knowledge | Result |
|--------------------|--------|
| Provably matches | No handling needed |
| Provably mismatches | PGE04002 (schema mismatch) |
| Cannot prove match | `[!]` + `!< fallback` required — PGE04009 if absent |

**VALID:**
```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✓ statically provable — all serial fields known and match
[-] $data#serial
   [-] $data:name << "Alice"
   [-] $data:age << 30
[-] $user#UserRecord << $data
```

```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✓ dynamic serial — handled with [!] + !< fallback
[-] $defaultUser#UserRecord
   [-] $defaultUser.name << "Unknown"
   [-] $defaultUser.age << 0

[-] $user#UserRecord << $dynamicSerial
   (>) !> $defaultUser                [ ] catch-all fallback
   [!] !SchemaMismatch
      [-] >user << $defaultUser
```

```polyglot
{#} #Config
   [.] .timeout#int
   [.] .retries#int

[ ] ✓ extra fields in serial are allowed — superset is fine
[-] $raw#serial
   [-] $raw:timeout << 30
   [-] $raw:retries << 3
   [-] $raw:debug << "true"
[-] $cfg#Config << $raw              [ ] ✓ :debug ignored, schema satisfied
```

**INVALID:**
```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✗ PGE04009 — dynamic serial pushed to struct without error handling
[-] $user#UserRecord << $dynamicSerial
```

```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] ✗ PGE04009 — conditionally built serial, compiler cannot prove completeness
[-] $data#serial
   [-] $data:name << "Alice"
   [?] $hasAge =? true
      [-] $data:age << 30
   [?] *?
      [ ] .age not assigned in this branch
[-] $user#UserRecord << $data        [ ] ✗ PGE04009 — .age may be missing
```

**Partial matching rules:**
- Serial has **more** fields than struct requires → **allowed**. Extra `:` fields are ignored.
- Serial has **fewer** fields than struct requires → **error**. Missing fields mean the struct schema cannot be fulfilled.

**See also:**
- [PGE04002 — Schema Mismatch](PGE04002-schema-mismatch.md) — provable static mismatch
- [PGE02005 — Failed Must Resolve](PGE02005-failed-is-terminal.md) — compiler-enforced error handling
- [TYPE-IDENTITY](../TYPE-IDENTITY.md) — rule 6 (serial→struct)
- [serial-to-struct-matching](../../brainstorming/serial-to-struct-matching.md) — decision rationale

### See Also

- [[syntax/types/conversions|Type Conversions]] — user-facing summary of serial-to-struct rules (this rule is authoritative)
- [[concepts/pipelines/error-handling|Error Handling]] — `[!]` blocks and `!<`/`!>` fallback operators
