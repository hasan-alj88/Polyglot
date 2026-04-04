---
audience: pg-coder
type: specification
updated: 2026-03-21
status: complete
---

# Identifiers

<!-- @types -->
<!-- @packages -->
ALL Polyglot identifiers require a prefix — see [[packages]] for `@` address format, [[syntax/types/INDEX|types]] for `#` type annotations:

| Prefix | Type | Example |
|--------|------|---------|
| `@` | Packages | `@Local:999::MyPackage:Sub:v1.0.0` |
| `#` | Struct definitions | `#UserRecord`, `#Boolean.True` |
| `=` | Pipelines | `=ProcessData`, `=Pipeline.Name` |
| `$` | Variables | `$name`, `$result:status`, `$*` (discard) |
| `!` | Errors | `!No.Input`, `!Timeout:Connection` |
| `_` | Permissions | `_File.read`, `_Web.request` |
| `%` | Metadata accessor | `=Pipeline%status`, `$var%state` |

**Permission identifiers (`_`)** — declare IO capabilities using `[_]` block elements. `_` prefixed identifiers use `.` fixed-field navigation for categories and subfields (`_File.read`, `_Database.connect`). No `[_]` declarations = pure computation, zero IO. See [[permissions]] for the full permission system.

## Serialized Identifiers

ALL identifiers are **serialized data**. Two field separators distinguish schema types:

| Separator | Schema | Meaning | Example |
|-----------|--------|---------|---------|
| `.` | Fixed | Predefined keys (schema-defined) | `#Boolean.True` — only `{True, False}` |
| `:` | Flexible | User-defined keys (open schema) | `$user:name` — any field name |
| `%` | Metadata | Read-only runtime metadata | `=Pipeline%status` — live pipeline status |

**Fixed fields (`.`)** — keys predefined by either:
- **Polyglot standard** — built-in types, errors, enums (`#Boolean.True`, `pg.string`, `!No.Input`)
- **User-defined structs** — fields declared via `{#}` blocks (`#UserRecord.name`, `#UserRecord.age`)

**Flexible fields (`:`)** — user-defined, any key accepted:
- `$user:name` — custom variable field
- `$config:timeout:value` — nested custom fields
- `$result:data:items` — arbitrary depth

**Metadata fields (`%`)** — Polyglot-managed, read-only:
- `=ProcessInvoice%status` — pipeline instance status
- `$myVar%state` — variable lifecycle state
- `#Record%lastModified` — data type metadata

The `%` accessor reads `live`-typed metadata that the runtime populates. Users cannot assign to `%` fields. See [[syntax/types/hierarchy#Live Type Modifier]].

**Package addresses** use `::` to separate registry from package name, with `:` flexible throughout:
- `@Local:999::PackageName:Sub:v1.0.0` — `:999` flexible registry ID, `::` registry separator, `:PackageName:Sub` flexible package name, `:v1.0.0` flexible version

**Discard variable (`$*`)** — a reserved identifier that immediately releases any value pushed into it. Use `$*` when a pipeline produces output you intentionally do not need. `$*` satisfies PGE03002 (parallel output must be collected) without naming the variable. For debugging or later use, prefer `*Ignore` with a named variable instead — see [[concepts/collections/collect#*Ignore — Explicit Discard]].

The prefix (`$`, `@`, `!`, `#`, `=`, `_`) identifies the type. The separators (`.` fixed, `:` flexible) navigate within it. For how separators apply to struct definitions, see [[syntax/types/structs#Enum Fields vs Value Fields]]. For collection types that use these schemas, see [[concepts/collections/INDEX#Collection Types]].

These serialized paths — `#Boolean.True`, `$user:name`, `=Pipeline%status` — are all branches on one unified tree. Every Polyglot object lives in the `%` metadata tree, organized by its prefix. After learning the core concepts, see [[data-is-trees]] for how everything connects.

## Serialization Rules

1. **Sibling homogeneity** — all siblings at the same level must use the same separator. No mixing `.` and `:` among siblings.

```polyglot
[ ] VALID:   $user:name, $user:age        [ ] all flexible
[ ] VALID:   #Boolean.True, #Boolean.False [ ] all fixed
[ ] INVALID: $user.name, $user:age         [ ] mixed separators at same level
```

2. **Sibling kind homogeneity** — all siblings at the same level must be the same kind: all enum fields or all value fields (have `#type`). Assignment within value fields is individually optional — unassigned value fields are in **Declared** state.

```polyglot
[ ] VALID:   all value fields, all assigned
[.] .timeout#int <~ 30
[.] .retries#int <~ 3

[ ] VALID:   all value fields, none assigned (Declared state)
[.] .timeout#int
[.] .retries#int

[ ] VALID:   all value fields, mixed assignment (some assigned, some declared)
[.] .timeout#int <~ 30
[.] .retries#int

[ ] INVALID: mixed kinds (enum + value at same level)
[.] .Active
[.] .count#int <~ 0
```

**Declared value fields:** A value field with no assignment is in **Declared** state. It can be pushed to (final or default) in usage, but pulling from a Declared variable is a compile error. See [[variable-lifecycle#Declared]].

3. **Leaf-only assignment** — only leaf fields (no children) can have values assigned. Branch fields are structural only. More broadly, the serialized tree (struct schema) must match — pushing serialized data into a mismatched schema is a compile error. The field type (`.` fixed vs `:` flexible) determines schema compatibility.

```polyglot
[ ] VALID:   assign to leaf
[r] $user:name << "Alice"

[ ] INVALID: assign to branch that has children
[r] $user << "Alice"
   [r] $user:name << "Alice"
```
