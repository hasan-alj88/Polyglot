---
audience: designer
rule: "10.8"
code: PGE10008
name: Parallel Write Permission Exclusion
severity: error
---

### Rule 10.8 — Parallel Write Permission Exclusion
`PGE10008`

<!-- @u:syntax/io -->
<!-- @c:permissions -->
<!-- @c:glossary#Reconciliation -->

**Statement:** Concurrent parallel jobs (`[=]`) may not hold write permission to the same resource path. If two or more `[=]` jobs in the same parallel scope reference `{_}` grant objects with overlapping write targets (same file path, database table, network endpoint, etc.), the compiler emits PGE10008. Read permission to the same resource is allowed across parallel jobs.
**Rationale:** Parallel jobs are pure readers by design — write contention is eliminated at compile time through the permission system. This makes [[glossary#Reconciliation|reconciliation]] safe by construction: no runtime locks, mutexes, or transactional memory are needed. Only sequential code after collection can write to shared resources.
**Detection:** The compiler resolves all `{_}` grant objects referenced by `[_]` in each `[=]` job within a parallel scope. For each write capability (`.File.Write`, `.Database.Write`, etc.), the compiler checks whether any two parallel jobs have overlapping resource paths. Overlap is determined by glob intersection — if the grant patterns of two jobs can match the same concrete path, PGE10008 fires.

**VALID:**
```polyglot
{ } ✓ parallel jobs read the same file — no write contention
{_} _ReadGrant
   [.] .intent << #Grant
   [.] .File.Read "data/input.csv"

{-} -ProcessData
   [_] _ReadGrant
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >result#string
   [=] -Analyze.Stats
      (-) <path << "data/input.csv"
      (-) >stats >> $stats
   [=] -Analyze.Schema
      (-) <path << "data/input.csv"
      (-) >schema >> $schema
   (*) *All
      (*) << $stats
      (*) << $schema
   [-] ...
```

```polyglot
{ } ✓ parallel jobs write to DIFFERENT files — no overlap
{_} _WriteGrantA
   [.] .intent << #Grant
   [.] .File.Write "output/stats.json"

{_} _WriteGrantB
   [.] .intent << #Grant
   [.] .File.Write "output/schema.json"

{-} -WriteResults
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [=] -Write.Stats
      [_] _WriteGrantA
      (-) <data << $stats
   [=] -Write.Schema
      [_] _WriteGrantB
      (-) <data << $schema
   (*) *All
      (*) << $statsResult
      (*) << $schemaResult
```

**INVALID:**
```polyglot
{ } ✗ PGE10008 — two parallel jobs write to the same file
{_} _WriteGrant
   [.] .intent << #Grant
   [.] .File.Write "output/result.json"

{-} -ConflictingWrites
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [=] -Write.PartA
      [_] _WriteGrant                   [ ] ✗ writes to output/result.json
      (-) <data << $partA
   [=] -Write.PartB
      [_] _WriteGrant                   [ ] ✗ PGE10008 — same write target
      (-) <data << $partB
   (*) *All
      (*) << $resultA
      (*) << $resultB
```

```polyglot
{ } ✗ PGE10008 — overlapping glob patterns
{_} _WriteAll
   [.] .intent << #Grant
   [.] .File.Write "output/*.json"

{_} _WriteReports
   [.] .intent << #Grant
   [.] .File.Write "output/report-*.json"

{-} -OverlappingWrites
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [=] -Write.General
      [_] _WriteAll                     [ ] ✗ output/*.json
   [=] -Write.Reports
      [_] _WriteReports                 [ ] ✗ PGE10008 — output/report-*.json ⊂ output/*.json
   (*) *All
      (*) << $generalResult
      (*) << $reportResult
```

**Note:** This rule applies only to `[=]` parallel jobs within the same parallel scope. Sequential `[-]` jobs can write to the same resource because they execute in order. `[b]` fire-and-forget jobs are also subject to this rule if they share a parallel scope with other `[=]` or `[b]` jobs.

### See Also

- [[concepts/collections/collect#Permission Safety]] — user-facing explanation of parallel write exclusion
- [[concepts/permissions#Parallel Write Exclusion]] — permission system integration
- [[concepts/collections/collect#Reconciliation]] — reconciliation concept that this rule enables
