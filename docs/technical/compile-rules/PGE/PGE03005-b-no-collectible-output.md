---
audience: developer
rule: "3.5"
code: PGE03005
name: "[b] Has No Collectible Output"
severity: error
---

### Rule 3.5 — `[b]` Has No Collectible Output
`PGE03005`

**Statement:** `[b]` fire-and-forget pipelines produce no output variables accessible to the prime pipeline. Any output the `[b]` pipeline produces internally is discarded. Attempting to reference a `[b]` result variable or collect it via `(*)` is a compile error.
**Rationale:** `[b]` exists for side-effect-only work (logging, database writes, notifications). It runs detached from the prime pipeline's data flow. If the caller needs output, it should use `[=]` instead. This enforces a clean separation: `[=]` = parallel with output, `[b]` = parallel without.
**Detection:** The compiler checks that no `[b]` call declares output variables wired to prime-scope variables, and that no `(*)` collector references a `[b]`-scoped variable.

**See also:** PGW03001 (warning when `[b]` called pipeline has discarded outputs)

**VALID:**
```polyglot
[ ] ✓ [b] with no output — side-effect only
[b] -Audit.Log
   (-) <event << $event            [ ] ✓ input is fine — fire and forget
```

```polyglot
[ ] ✓ [b] alongside [=] — only [=] output is collected
[b] -Notify.Admin
   (-) <msg << "processing started"

[=] -Fetch.Data
   (-) <id << $userId
   (-) >result >> $data

(*) *All
   (*) << $data                      [ ] ✓ only $data (from [=]) is collected
```

**INVALID:**
```polyglot
[ ] ✗ PGE03005 — [b] output wired to prime variable
[b] -Audit.Log
   (-) <event << $event
   (-) >auditId >> $auditId        [ ] ✗ PGE03005 — [b] cannot produce output
```

```polyglot
[ ] ✗ PGE03005 — attempting to collect [b] output
[b] -Background.Task
   (-) >result >> $bgResult

(*) *All
   (*) << $bgResult                  [ ] ✗ PGE03005 — $bgResult is from [b]
```

### See Also

- [[concepts/collections/collect|Collect]] — documents `[b]` fire-and-forget having no collectible output (references PGE03005)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03005 to example scenarios
