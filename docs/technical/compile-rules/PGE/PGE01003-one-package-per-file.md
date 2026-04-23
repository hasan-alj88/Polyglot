---
audience: developer
rule: "1.3"
code: PGE01003
name: One Package Declaration Per File
---

# Rule 1.3 — One Package Declaration Per File
`PGE01003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** Every `.pg` file must contain exactly one `{@}` block, and it must be the first block in the file. A missing `{@}` block and multiple `{@}` blocks are both errors.
**Rationale:** The package declaration establishes the file's identity, registry address, and imports. Without it the runtime cannot resolve references; with duplicates the file has conflicting identities.

**VALID:**
```polyglot
[ ] ✓ exactly one {@} as the first block
{@}
   .address #string << "Registry:com.example.MyPkg:1.0.0"

{-} -MyPipeline
   ...
```

**INVALID:**
```polyglot
[ ] ✗ PGE01003 — no {@} block
{-} -MyPipeline    [ ] ✗ PGE01003 — file has no package declaration
   ...
```

```polyglot
[ ] ✗ PGE01003 — two {@} blocks
{@}
   .address #string << "Registry:com.example.MyPkg:1.0.0"

{-} -MyPipeline
   ...

{@}                [ ] ✗ PGE01003 — second package declaration
   .address #string << "Registry:com.example.Other:1.0.0"
```
