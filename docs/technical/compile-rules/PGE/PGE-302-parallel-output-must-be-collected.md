---
rule: "3.2"
code: PGE-302
name: Parallel Output Must Be Collected
severity: error
---

### Rule 3.2 — Parallel Output Must Be Collected
`PGE-302`

**Statement:** If a `[p]` parallel pipeline produces any output variable, there must be a `[*]` collector (`*All`, `*First`, `*Nth`, `*Ignore`, etc.) that references it. An uncollected parallel output is a compile error. `*Ignore` explicitly discards unwanted outputs — it satisfies this rule without consuming the value. This rule does not apply to `[b]` fire-and-forget — `[b]` outputs are explicitly disallowed (see Rule 3.5, PGE-305).
**Rationale:** Parallel output variables exist in an isolated scope until collected. Without a collector, the output is inaccessible and the parallel work is wasted. Requiring collection ensures every parallel pipeline's results are explicitly consumed or intentionally discarded via `*Ignore` (or the pipeline uses `[b]` if no output is needed at all).
**Detection:** After parsing the full pipeline body, the compiler checks that every `[p]` output variable appears as a `[*] <<` input in a `[*]` collector block.

**VALID:**
```polyglot
[ ] ✓ parallel output collected via *All
[p] =Fetch.Data
   [=] <id << $userId
   [=] >data >> $data

[*] *All
   [*] << $data

[r] =Process
   [=] <input << $data
```

```polyglot
[ ] ✓ multiple parallels, all collected
[p] =Fetch.A
   [=] >result >> $a

[p] =Fetch.B
   [=] >result >> $b

[*] *All
   [*] << $a
   [*] << $b
```

```polyglot
[ ] ✓ race collector — all candidates listed
[p] =Search.Fast
   [=] >result >> $fast

[p] =Search.Slow
   [=] >result >> $slow

[*] *First
   [*] << $fast
   [*] << $slow
   [*] >> $winner
```

```polyglot
[ ] ✓ $* inline discard — no variable created
[p] =Audit.Log
   [=] <event << $event
   [=] >auditId >> $*              [ ] ✓ discarded at declaration site
```

```polyglot
[ ] ✓ *Ignore explicitly discards named variable
[p] =Audit.Log
   [=] <event << $event
   [=] >auditId >> $auditId

[*] *Ignore
   [*] << $auditId                   [ ] ✓ acknowledged and discarded
```

**INVALID:**
```polyglot
[ ] ✗ PGE-302 — parallel output with no collector
[p] =Fetch.Data
   [=] <id << $userId
   [=] >data >> $data              [ ] ✗ PGE-302 — $data never collected

[r] =NextStep
   [=] <input << $data             [ ] also PGE-303 — use before collection
```

```polyglot
[ ] ✗ PGE-302 — one of two parallel outputs not collected
[p] =Fetch.A
   [=] >result >> $a

[p] =Fetch.B
   [=] >result >> $b

[*] *All
   [*] << $a                          [ ] ✓ $a collected
                                      [ ] ✗ PGE-302 — $b not in any collector
```

### See Also

- [[concepts/collections/collect|Collect]] — documents collector requirement for `[p]` outputs (references PGE-302)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE-302 to example scenarios
- [[syntax/identifiers|Identifiers]] — documents `$*` discard variable satisfying PGE-302
