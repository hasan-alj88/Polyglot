---
audience: user
type: specification
updated: 2026-03-24
status: complete
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

Every `@alias` reference must resolve to a declared `[@]` import (PGE-901). The pipeline name after the alias must exist in the imported package (PGE-904). Referencing a deprecated pipeline emits a warning (PGW-901).

**Note:** Standard library pipelines (`=File.*`, `=T.*`, `=Q.*`, `=W.*`) are built-in and do NOT require `[@]` import — see [[pipelines#Triggers]].

## Import Rules

Each `[@]` import line declares an alias for a package address. The compiler enforces:

- **Unique aliases** — each `@alias` name in a file must be unique (PGE-912). Two `[@]` lines with the same alias make resolution ambiguous.
- **No stdlib shadowing** — an alias must not match a reserved stdlib namespace prefix: `File`, `Path`, `Math`, `Sys`, `T`, `Q`, `W` (PGE-913). See [[stdlib/INDEX|stdlib/INDEX.md]] for the full reserved list.
- **Alias must be used** — an `[@]` import that is never referenced anywhere in the file is flagged as dead code (PGW-902). This typically indicates incomplete refactoring.

## Dependency Rules

Package imports must form a directed acyclic graph. If Package A imports Package B and Package B imports Package A (directly or transitively), the cycle is a compile error (PGE-902). The compiler reports the full cycle path.

Within a package, pipeline calls must also be acyclic — Polyglot has no recursion mechanism. Self-calls and mutual call loops are compile errors (PGE-914). See [[pipelines#Call Site Rules]].

Pipeline references in `[r]`, `[p]`, or `[b]` calls must resolve to either a stdlib pipeline or a `{=}` definition within the same package (PGE-903). Cross-package pipelines must use the `@alias=Pipeline` form with a valid `[@]` import.

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
- **No duplicates** — a `{=}` pipeline name or `{#}` data name must be unique across all files in the package (PGE-907)
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

## Compile Rules Reference

| Code | Name | Section |
|------|------|---------|
| PGE-901 | Undefined Import Alias | [[#Usage]] |
| PGE-902 | Circular Package Dependency | [[#Dependency Rules]] |
| PGE-903 | Unresolved Pipeline Reference | [[#Dependency Rules]] |
| PGE-904 | Unresolved Import Pipeline Reference | [[#Usage]] |
| PGE-905 | Multi-File Version Mismatch | [[#Rules]] |
| PGE-906 | Multi-File Package Name Mismatch | [[#Rules]] |
| PGE-907 | Duplicate Definition | [[#Rules]] |
| PGE-909 | Multi-File Reference Not Found | [[#Rules]] |
| PGE-910 | Multi-File Self-Reference | [[#Rules]] |
| PGE-911 | Asymmetric Multi-File Reference | [[#Rules]] |
| PGE-912 | Duplicate Import Alias | [[#Import Rules]] |
| PGE-913 | Import Alias Shadows Standard Library | [[#Import Rules]] |
| PGE-914 | Circular Pipeline Call | [[#Dependency Rules]] |
| PGW-901 | Deprecated Pipeline Reference | [[#Usage]] |
| PGW-902 | Unused Import | [[#Import Rules]] |
