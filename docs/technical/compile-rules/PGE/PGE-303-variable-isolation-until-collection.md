---
rule: "3.3"
code: PGE-303
name: Variable Isolation Until Collection
severity: error
---

### Rule 3.3 — Pull Isolation Until Collection
`PGE-303`

**Statement:** A variable produced by a `[p]` parallel pipeline cannot be pulled from in the prime pipeline before the `[*]` collector for that parallel has executed. Pulling such a variable before collection is a compile error. Variables discarded via `$*` are exempt (they are never accessible). Push violations are caught by PGE-301 regardless of collection status.
**Rationale:** Parallel pipelines run concurrently with the prime pipeline. Until a collector synchronizes, the variable may not yet have a value — or may be mid-write. Enforcing pull isolation ensures the prime pipeline only sees fully resolved values.
**Detection:** The compiler tracks which variables originate from `[p]` scopes and verifies that any pull targeting them appears only after the corresponding `[*]` collector block.

**VALID:**
```polyglot
[ ] ✓ variable used only after *All collection
[p] =Fetch.Data
   [=] >result >> $data

[*] *All
   [*] << $data

[r] =Process
   [=] <input << $data             [ ] ✓ $data is collected — safe to pull
```

```polyglot
[ ] ✓ two parallels, both collected before use
[p] =Fetch.A
   [=] >result >> $a

[p] =Fetch.B
   [=] >result >> $b

[*] *All
   [*] << $a
   [*] << $b

[r] =Combine
   [=] <first << $a               [ ] ✓ both collected
   [=] <second << $b
```

**INVALID:**
```polyglot
[ ] ✗ PGE-303 — pulling parallel output before collection
[p] =Fetch.Data
   [=] >result >> $data

[r] =Process
   [=] <input << $data             [ ] ✗ PGE-303 — $data not yet collected

[*] *All
   [*] << $data
```

```polyglot
[ ] ✗ PGE-301 — pushing into parallel output (push violation, not PGE-303)
[p] =Fetch.Data
   [=] >result >> $data

[r] $data << "override"           [ ] ✗ PGE-301 — push across parallel boundary

[*] *All
   [*] << $data
```

**Note:** `live` metadata (`$data%state`) **can** be inspected before collection. PGE-303 isolates user-assignable push/pull operations only. `live` fields are runtime-managed and read-only (PGE-206), so no data race is possible. See resolved design issue 003 (git history: `docs/technical/compiler_issues/003-metadata-access-before-collection.md`).
