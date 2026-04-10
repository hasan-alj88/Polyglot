---
audience: developer
rule: "3.3"
code: PGE03003
name: Variable Isolation Until Collection
severity: error
---

### Rule 3.3 — Pull Isolation Until Collection
`PGE03003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** A variable produced by a `[=]` parallel pipeline cannot be pulled from in the prime pipeline before the `(*)` collector for that parallel has executed. Pulling such a variable before collection is a compile error. Variables discarded via `$*` are exempt (they are never accessible). Push violations are caught by PGE03001 regardless of collection status.
**Rationale:** Parallel pipelines run concurrently with the prime pipeline. Until a collector synchronizes, the variable may not yet have a value — or may be mid-write. Enforcing pull isolation ensures the prime pipeline only sees fully resolved values.
**Detection:** The compiler tracks which variables originate from `[=]` scopes and verifies that any pull targeting them appears only after the corresponding `(*)` collector block.

**VALID:**
```polyglot
[ ] ✓ variable used only after *All collection
[=] -Fetch.Data
   (-) >result >> $data

(*) *All
   (*) << $data

[-] -Process
   (-) <input << $data             [ ] ✓ $data is collected — safe to pull
```

```polyglot
[ ] ✓ two parallels, both collected before use
[=] -Fetch.A
   (-) >result >> $a

[=] -Fetch.B
   (-) >result >> $b

(*) *All
   (*) << $a
   (*) << $b

[-] -Combine
   (-) <first << $a               [ ] ✓ both collected
   (-) <second << $b
```

**INVALID:**
```polyglot
[ ] ✗ PGE03003 — pulling parallel output before collection
[=] -Fetch.Data
   (-) >result >> $data

[-] -Process
   (-) <input << $data             [ ] ✗ PGE03003 — $data not yet collected

(*) *All
   (*) << $data
```

```polyglot
[ ] ✗ PGE03001 — pushing into parallel output (push violation, not PGE03003)
[=] -Fetch.Data
   (-) >result >> $data

[-] $data << "override"           [ ] ✗ PGE03001 — push across parallel boundary

(*) *All
   (*) << $data
```

**Note:** `live` metadata (`$data%state`) **can** be inspected before collection. PGE03003 isolates user-assignable push/pull operations only. `live` fields are runtime-managed and read-only (PGE02006), so no data race is possible. See resolved design issue 003 (git history: `docs/technical/compiler_issues/003-metadata-access-before-collection.md`).

### See Also

- [[concepts/collections/collect|Collect]] — documents pull isolation constraint for `[=]` outputs (references PGE03003)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03003 to example scenarios
