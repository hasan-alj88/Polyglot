---
rule: "3.5"
code: PGE-305
name: "[b] Has No Collectible Output"
severity: error
---

### Rule 3.5 — `[b]` Has No Collectible Output
`PGE-305`

**Statement:** `[b]` fire-and-forget pipelines produce no output variables accessible to the prime pipeline. Any output the `[b]` pipeline produces internally is discarded. Attempting to reference a `[b]` result variable or collect it via `[*]` is a compile error.
**Rationale:** `[b]` exists for side-effect-only work (logging, database writes, notifications). It runs detached from the prime pipeline's data flow. If the caller needs output, it should use `[p]` instead. This enforces a clean separation: `[p]` = parallel with output, `[b]` = parallel without.
**Detection:** The compiler checks that no `[b]` call declares output variables wired to prime-scope variables, and that no `[*]` collector references a `[b]`-scoped variable.

**See also:** PGW-301 (warning when `[b]` called pipeline has discarded outputs)

**VALID:**
```polyglot
[ ] ✓ [b] with no output — side-effect only
[b] =Audit.Log
   [=] <event << $event            [ ] ✓ input is fine — fire and forget
```

```polyglot
[ ] ✓ [b] alongside [p] — only [p] output is collected
[b] =Notify.Admin
   [=] <msg << "processing started"

[p] =Fetch.Data
   [=] <id << $userId
   [=] >result >> $data

[*] *All
   [*] << $data                      [ ] ✓ only $data (from [p]) is collected
```

**INVALID:**
```polyglot
[ ] ✗ PGE-305 — [b] output wired to prime variable
[b] =Audit.Log
   [=] <event << $event
   [=] >auditId >> $auditId        [ ] ✗ PGE-305 — [b] cannot produce output
```

```polyglot
[ ] ✗ PGE-305 — attempting to collect [b] output
[b] =Background.Task
   [=] >result >> $bgResult

[*] *All
   [*] << $bgResult                  [ ] ✗ PGE-305 — $bgResult is from [b]
```

### See Also

- [[concepts/collections/collect|Collect]] — documents `[b]` fire-and-forget having no collectible output (references PGE-305)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE-305 to example scenarios
