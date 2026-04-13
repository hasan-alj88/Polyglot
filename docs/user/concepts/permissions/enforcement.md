---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Permission Enforcement

## Parallel Write Exclusion

<!-- @c:glossary#Reconciliation -->
Concurrent parallel jobs (`[=]`) may not hold write permission to the same resource path — this is a compile error (PGE10008). Read permission to the same resource is allowed across parallel jobs.

This rule makes [[glossary#Reconciliation|reconciliation]] safe by construction: parallel jobs are pure readers, and only sequential code after collection can write to shared resources. No runtime locks, mutexes, or transactional memory are needed — the permission system eliminates write contention at compile time.

The compiler checks for overlapping write targets by comparing the resource paths in `{_}` grant objects across all `[=]` jobs in the same parallel scope. Overlap is determined by glob intersection — if two grants can match the same concrete path, PGE10008 fires.

```polyglot
{_} _WriteGrant
   [.] .intent << #Grant
   [.] .File.Write "output/result.json"

{ } ✗ PGE10008 — two parallel jobs write to the same file
[=] -Write.PartA
   [_] _WriteGrant
[=] -Write.PartB
   [_] _WriteGrant                      [ ] ✗ same write target as PartA

{ } ✓ Sequential is fine — no contention
[-] -Write.PartA
   [_] _WriteGrant
[-] -Write.PartB
   [_] _WriteGrant                      [ ] ✓ sequential — no overlap
```

See [[technical/compile-rules/PGE/PGE10008-parallel-write-permission-exclusion|PGE10008]] for the full rule with detection algorithm and examples.

## No Instances

Permissions are **compile-time declarations** — they apply across all instances of a pipeline. There are no per-instance permissions. If `-ProcessLogs` has `[_] _LogFileGrant`, every instance of `-ProcessLogs` shares that grant. The `%_` metadata tree branch has no `:{instance}` level (see [[data-is-trees]]).

## Compile-Time Enforcement

All permission checks are **static analysis** — resolved at compile time, not runtime. The compiler verifies:

1. **Grant within ceiling** — every `[_]` grant in a `{-}` must reference a `{_}` object whose capabilities fall within the `{@}` package ceiling (PGE10001)
2. **Import ceiling compatibility** — imported package ceilings must fall within the importer's ceiling (PGE10002)
3. **Pure computation enforced** — any IO call in a pipeline with no `[_]` lines is a compile error
4. **Fully filled** — every `{_}` object must have all leaf fields assigned (no empty leaves)
5. **Intent validation** — `#Ceiling` objects may use glob patterns; `#Grant` objects must use specific narrow values

No runtime permission checks exist. If it compiles, the permissions are satisfied.

## Compile-Time File Binding

<!-- @c:vision#No Dynamic Code -->
Permission grants that reference external files — `<code.file` paths in `-Run.*` pipelines ([[pglib/pipelines/Run/INDEX|u:-Run.*]]), configuration files, data files — are bound to the file's content at compilation time. The compiled output includes a content hash of every referenced file.

If a referenced file changes after compilation:

1. The Polyglot Service **revokes** the associated permission grant
2. The pipeline **refuses to execute** until the developer recompiles with the updated file
3. A **file change watcher trigger** monitors all referenced file paths and notifies the developer that recompilation is required

This ensures that no external code or input runs through the platform without having passed through the compiler's analysis. The principle is simple: compilation is a license to launch, and that license is invalidated when the inputs change.

**Note:** `.pg` source files are covered by the same principle implicitly — changing a `.pg` file has no effect until the developer recompiles, at which point the compiler re-analyses the entire package.
