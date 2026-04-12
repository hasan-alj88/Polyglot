---
audience: developer
rule: "3.25"
code: PGE03025
name: Not All Jobs Released
severity: error
---

### Rule 3.25 — Not All Jobs Released
`PGE03025`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** Every code path through a `{*}` collector definition must release ALL jobs `[0,N]`. If any code path exits without releasing every job, this is a compile error.
**Rationale:** Unreleased jobs become orphans — the Trigger Monitor cannot terminate them, and their resources are never reclaimed. This is the collector equivalent of a resource leak.
**Detection:** The compiler performs reachability analysis on all `[T]` blocks within the `{*}` definition. For each possible execution path (including error handlers), it verifies that a release covering `[0,N]` is reached. If any path exits without full release, PGE03025 fires.

**VALID:**
```polyglot
{*} *First
   ...
   [T] *Arrive"0"
      (T) >var
      (T) >job
      >> >winner << $var.value
      [*] *Job.Release"[0,N]"            [ ] ✓ releases all jobs
```

**INVALID:**
```polyglot
[ ] ✗ PGE03025 — only releases the winner's job
{*} *BadFirst
   ...
   [T] *Arrive"0"
      (T) >var
      (T) >job
      >> >winner << $var.value
      [*] *Job.Release"0"                [ ] ✗ only releases job 0
```

### See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — Ground Rule 5
