---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# Permission Enforcement

## Parallel Write Exclusion

<!-- @c:glossary#Reconciliation -->
Concurrent parallel jobs (`[=]`) may not hold write permission to the same resource path — this is a compile error (PGE10008). Read permission to the same resource is allowed across parallel jobs.

This rule makes [[glossary#Reconciliation|reconciliation]] safe by construction: parallel jobs are pure readers, and only sequential code after collection can write to shared resources. No runtime locks, mutexes, or transactional memory are needed — the permission system eliminates write contention at compile time.

The compiler checks for overlapping write targets by comparing the `.scope` and `.path` fields in `{_}` grant objects across all `[=]` jobs in the same parallel scope. Overlap is determined by glob intersection — if two grants can match the same concrete path, PGE10008 fires.

```polyglot
{_} _WriteGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Write
   [.] .scope "output/result.json"
   [.] .path "output/result.json"

[ ] ✗ PGE10008 — two parallel jobs write to the same file
[=] -Write.PartA
   (-) _WriteGrant
[=] -Write.PartB
   (-) _WriteGrant                      [ ] ✗ same write target as PartA

[ ] ✓ Sequential is fine — no contention
[-] -Write.PartA
   (-) _WriteGrant
[-] -Write.PartB
   (-) _WriteGrant                      [ ] ✓ sequential — no overlap
```

See [[technical/compile-rules/PGE/PGE10008-parallel-write-permission-exclusion|PGE10008]] for the full rule with detection algorithm and examples.

## No Instances

Permissions are **compile-time declarations** — they apply across all instances of a pipeline. There are no per-instance permissions. If `-ProcessLogs` declares `(-) _LogFileGrant`, every instance of `-ProcessLogs` shares that grant. The `%_` metadata tree branch has no `:{instance}` level (see [[data-is-trees]]).

## Compile-Time Enforcement

All permission checks are **static analysis** — resolved at compile time, not runtime. The compiler verifies:

1. **Grant within ceiling** — every `_` grant in a `{-}` must reference a `{_}` object whose capabilities fall within the `{@}` package ceiling (PGE10001)
2. **Import ceiling compatibility** — imported package ceilings must fall within the importer's ceiling (PGE10002)
3. **Pure computation enforced** — any IO call in a pipeline with no `_` IO declarations is a compile error (PGE10004)
4. **Fully filled** — every `{_}` object must have all leaf fields assigned (no empty leaves)
5. **Intent validation** — `#Ceiling` objects may use glob patterns; `#Grant` objects must use specific narrow values
6. **Template resolution** — all `(_)` inputs must be provided; unresolved template inputs are a compile error (PGE10009)
7. **Resource validation** — file-category `{_}` objects must point to files that exist at compile time (PGE10010)

No runtime permission checks exist. If it compiles, the permissions are satisfied.

## Compile-Time File Binding

<!-- @c:vision#No Dynamic Code -->

Compile-Time File Binding is a **natural consequence** of the `{_}` permission-as-resource model. Because every external file access is mediated through a `{_}` permission object that carries a `.path` field, the compiler always knows which files are referenced. Content hashing is automatic — not a separate mechanism.

### Compiler Detection Flow

When the compiler encounters a `_` permission reference in any block's IO:

1. **Resolve the `{_}` definition** — if it's a template, substitute all `(_)` inputs and resolve `{<param}` interpolation in field values
2. **Validate completeness** — ensure all required fields for the category are present (e.g., `.path` for File)
3. **For file-category permissions** (`.category #File`):
   - Read the file at `.path`
   - Compute a content hash (SHA-256)
   - Store the hash in the compiled output alongside the permission grant
4. **For credentials paths** — `.credentials` fields on Database permissions are also content-hashed (credential rotation requires recompilation)
5. **For DB/Web permissions** — validate connection parameters are well-formed (host format, port range, endpoint syntax)

### Runtime Enforcement

If a referenced file changes after compilation:

1. The Polyglot Service **revokes** the associated permission grant
2. The pipeline **refuses to execute** until the developer recompiles with the updated file
3. A **file change watcher trigger** monitors all referenced file paths and notifies the developer that recompilation is required

This ensures that no external code or input runs through the platform without having passed through the compiler's analysis. The principle is simple: **compilation is a license to launch**, and that license is invalidated when the inputs change.

### What Gets Hashed

| Source | Hash Trigger | Example |
|--------|-------------|---------|
| `{_}` with `.category #File` | `.path` field | `_Secrets` with `.path "/config/secrets.yaml"` |
| `{_}` with `.credentials` | `.credentials` field | `_ProductionDB` with `.credentials "/keys/db.json"` |
| `-Run.*` pipelines | `<code.file` input | `-Run.Script.File` with `<code.file "/scripts/etl.py"` |

Templates contribute to hashing after resolution — each instantiation produces its own hash based on the resolved `.path`.

**Note:** `.pg` source files are covered by the same principle implicitly — changing a `.pg` file has no effect until the developer recompiles, at which point the compiler re-analyses the entire package.
