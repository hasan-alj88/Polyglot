---
rule: "3.5"
code: PGW-301
name: "[b] Called Pipeline Has Discarded Outputs"
severity: warning
---

### Rule 3.5w — `[b]` Called Pipeline Has Discarded Outputs
`PGW-301`

**Statement:** When a `[b]` call invokes a pipeline that declares output parameters, those outputs are silently discarded. PGW-301 warns that the called pipeline produces data that will be lost. Use `$*` to explicitly acknowledge the discard, or `[ ] Ignore PGW-301` to suppress.
**Rationale:** Silently discarding output is often unintentional — the developer may have meant to use `[p]` instead. The warning surfaces this before it becomes a runtime surprise.
**Detection:** The compiler inspects the called pipeline's signature. If it declares output parameters and the caller is `[b]`, PGW-301 fires.

**See also:** PGE-305 (error when `[b]` output is referenced)

**WARNING:**
```polyglot
[ ] ⚠ PGW-301 — called pipeline has outputs but [b] discards them
[ ] =Audit.Log declares >auditId — silently lost via [b]
[b] =Audit.Log
   [=] <event << $event            [ ] ⚠ PGW-301 — =Audit.Log output discarded
```

```polyglot
[ ] ✓ acknowledged with $* — no warning
[b] =Audit.Log
   [=] <event << $event
   [=] >auditId >> $*              [ ] ✓ explicitly discarded
```

```polyglot
[ ] ✓ suppressed via Ignore comment
[ ] Ignore PGW-301
[b] =Audit.Log
   [=] <event << $event            [ ] no warning — suppressed
```
