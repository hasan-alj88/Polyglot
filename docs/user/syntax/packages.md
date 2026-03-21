---
audience: user
type: specification
updated: 2026-03-21
status: draft
---

# Package Declaration

<!-- @identifiers -->
<!-- @blocks -->
Mandatory first block in every `.pg` file — exactly one `{@}` per file. Multiple `{#}` and `{=}` definitions are allowed, but not multiple `{@}`. See [[blocks]] for `{@}` definition and `[@]` import element. Package addresses are [[identifiers#Serialized Identifiers]] using `.` (fixed) and `:` (flexible) separators. Packages live at `%@` in the metadata tree (see [[data-is-trees#How Concepts Connect]]).

```polyglot
{ } Package declaration block
{@} @Local:999.MyPackageName.Sub:v1.2.3.2
   [ ] imports
   [@] @alias1 << @Community:user123.PackageName.Sub:v1.2.3.2
   [@] @alias2 << @Local:999.AnotherPackage.Sub:v1.2.3.2
```

## Address Format

`@Registry:ID.Name.SubPkg:Version.Major.Minor.Patch`

Package addresses are serialized identifiers using `.` (fixed) and `:` (flexible):

- `@` — package prefix
- `Local` / `Community` / `Registry` — registry type (fixed)
- Registry ID (flexible) — format depends on registry type:

| Registry Type | ID Format | Example |
|--------------|-----------|---------|
| `Local` | Port number (unused port) | `:999`, `:042` |
| `Community` | Username and project name | `:devops.NotificationHub` |
| `Registry` | Registered company name | `:Acme` |

- `.Name.SubPkg` — package and subpackage (fixed)
- `:v1.2.3.2` — version (flexible)

## Usage

Reference imported packages via their alias:
- `@alias1#SomeData` — access data from imported package. See [[types#User-Defined Types]]
- `@alias1=SomePipeline` — access pipeline from imported package. See [[pipelines]]
- `@alias1#DataName.EnumField` — reference enum value cross-package. See [[types#Enum Fields vs Value Fields]]

**Note:** Standard library pipelines (`=File.*`, `=T.*`, `=Q.*`, `=W.*`) are built-in and do NOT require `[@]` import — see [[pipelines#Triggers]].

## Multi-File Packages

A single package can span multiple `.pg` files. Each file declares the same `{@}` package address and references the other files using `[@]` with a path string — no alias on the left side.

### Syntax

```polyglot
{ } Explicit file references
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\my-app-02.pg"
   [@] << "{.}\my-app-03.pg"
```

```polyglot
{ } Folder shorthand — include all .pg files in the directory
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}"
```

Distinguished from import `[@]` by: no alias on the left, path string on the right. Import syntax is `[@] @alias << @Registry:...`, file reference syntax is `[@] << "path"`.

### Rules

- **Same address** — every file in the package must declare the same `{@}` package name and version (PGE-905, PGE-906)
- **Full mesh** — every file must reference all other files in the package. If file A references B and C, then B must reference A and C, and C must reference A and B (PGE-911)
- **No duplicates** — a `{=}` pipeline name or `{#}` data name must be unique across all files in the package (PGE-907, PGE-908)
- **No self-reference** — a file must not list itself (PGE-910)
- **File must exist** — every referenced path must resolve to an existing `.pg` file (PGE-909)

### Folder Shorthand

`[@] << "{.}"` discovers all `.pg` files in the specified directory. This is equivalent to listing each file explicitly. The folder shorthand satisfies the full mesh rule automatically — the compiler expands it to the full file list before validation.

### Example — Three-File Package

**my-app-01.pg:**
```polyglot
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\my-app-02.pg"
   [@] << "{.}\my-app-03.pg"

{#} #Config
   [.] .host;string
   [.] .port;int
```

**my-app-02.pg:**
```polyglot
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\my-app-01.pg"
   [@] << "{.}\my-app-03.pg"

{=} =LoadConfig
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >config;Config
   [r] >config << ...
```

**my-app-03.pg:**
```polyglot
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\my-app-01.pg"
   [@] << "{.}\my-app-02.pg"

{=} =RunServer
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <config;Config
   [r] ...
```

All three files share the `#Config` data type and can call each other's pipelines as if they were in one file. A file can also reference imports and sibling files together:

```polyglot
{@} @Local:1000.MyApp:v1.0.0
   [ ] sibling files
   [@] << "{.}\my-app-02.pg"
   [ ] external imports
   [@] @utils << @Local:999.Utilities:v1.0.0
```
