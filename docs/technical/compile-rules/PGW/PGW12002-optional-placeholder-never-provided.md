---
audience: developer
rule: "10.2"
code: PGW12002
name: Optional Placeholder Never Provided
severity: warning
---

# Rule 10.2 — Optional Placeholder Never Provided
`PGW12002`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When a pipeline's `%InlineString` template contains an optional placeholder `{name?}` and across all known inline call sites the placeholder matches empty (the default is always used), the compiler emits a warning.
**Rationale:** An optional placeholder that is never filled at any call site may indicate dead template complexity. The pipeline author may want to remove the placeholder and hardcode the default, or callers may be unaware the option exists. This is informational — the code is correct but may benefit from simplification.
**Detection:** The compiler tracks all inline call sites for the pipeline. After template extraction at each site, if `{name?}` resolved to empty in every case, PGW12002 is emitted on the pipeline definition.

**See also:** PGW08002 (unaddressed input with default — similar concept for normal calls)

**WARNING:**
```aljam3
{-} -DB.Connect
   (-) %InlineString << "{host}:{port?}/{db}"
   (-) <host#string
   (-) <port#string <~ "5432"
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ] ...

[ ] ⚠ PGW12002 — {port?} always empty; $port always uses default "5432"
[ ]
[-] $conn1 << -DB.Connect"host1:/db1"
[-] $conn2 << -DB.Connect"host2:/db2"
[-] $conn3 << -DB.Connect"host3:/db3"
```

**Open point:** None.
