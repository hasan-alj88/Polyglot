---
audience: user
type: specification
updated: 2026-03-25
status: complete
---

# Package Declaration

<!-- @identifiers -->
<!-- @blocks -->
Mandatory first block in every `.pg` file — exactly one `{@}` per file. Multiple `{#}` and `{=}` definitions are allowed, but not multiple `{@}`. See [[blocks]] for `{@}` definition and `[@]` import element. Package addresses use `::` to separate the registry from the package name, with `:` (flexible) separators throughout. Packages live at `%@` in the metadata tree (see [[data-is-trees#How Concepts Connect]]).

```polyglot
{ } Package declaration block
{@} @Local:999::MyPackageName:Sub:v1.0.0
   [ ] imports
   [@] @alias1 << @Community:user123:ProjectName::PackageName:Sub:v1.0.0
   [@] @alias2 << @Local:999::AnotherPackage:Sub:v1.0.0
```

## Address Format

`@<Registry>:<ID>::<PackageName>:<Version>`

Package addresses use `:` for flexible (user-defined) levels and `::` as a registry separator:

- `@` — package prefix
- `Local` / `Community` / `Company` — registry type (fixed)
- `::` — registry separator (separates registry+ID from package name)
- Registry ID (flexible) — format depends on registry type:

| Registry Type | ID Format | Example |
|--------------|-----------|---------|
| `Local` | Port number (unused port) | `@Local:999::` |
| `Community` | Username and project name | `@Community:user123:ProjectName::` |
| `Company` | Registered company domain parts | `@Company:acme:corp::` |

- `:<PackageName>` — package and subpackage (flexible, user-defined, can nest: `:MyPkg:Sub`)
- `:<Version>` — version (flexible)

## Usage

Reference imported packages via their alias:
- `@alias1#SomeData` — access data from imported package. See [[syntax/types/basic-types#User-Defined Types]]
- `@alias1=SomePipeline` — access pipeline from imported package. See [[concepts/pipelines/INDEX|pipelines]]
- `@alias1#DataName.EnumField` — reference enum value cross-package. See [[syntax/types/structs#Enum Fields vs Value Fields]]

Every `@alias` reference must resolve to a declared `[@]` import (PGE09001). The pipeline name after the alias must exist in the imported package (PGE09004). Referencing a deprecated pipeline emits a warning (PGW09001).

**Note:** Standard library pipelines (`=File.*`, `=T.*`, `=Q.*`, `=W.*`) are built-in and do NOT require `[@]` import — see [[concepts/pipelines/io-triggers#Triggers]].

## Import Rules

Each `[@]` import line declares an alias for a package address. The compiler enforces:

- **Unique aliases** — each `@alias` name in a file must be unique (PGE09011). Two `[@]` lines with the same alias make resolution ambiguous.
- **No stdlib shadowing** — an alias must not match a reserved stdlib namespace prefix: `File`, `Path`, `Math`, `Sys`, `T`, `Q`, `W` (PGE09012). See [[stdlib/INDEX|stdlib/INDEX.md]] for the full reserved list.
- **Alias must be used** — an `[@]` import that is never referenced anywhere in the file is flagged as dead code (PGW09002). This typically indicates incomplete refactoring.

## Dependency Rules

Package imports must form a directed acyclic graph. If Package A imports Package B and Package B imports Package A (directly or transitively), the cycle is a compile error (PGE09002). The compiler reports the full cycle path.

Within a package, pipeline calls must also be acyclic — Polyglot has no recursion mechanism. Self-calls and mutual call loops are compile errors (PGE09013). See [[concepts/pipelines/inline-calls#Call Site Rules]].

Pipeline references in `[r]`, `[p]`, or `[b]` calls must resolve to either a stdlib pipeline or a `{=}` definition within the same package (PGE09003). Cross-package pipelines must use the `@alias=Pipeline` form with a valid `[@]` import.

## Multi-File Packages

A single package can span multiple `.pg` files. Each file declares the same `{@}` package address and references the other files using `[@]` with a path string — no alias on the left side.

### Syntax

```polyglot
{ } Explicit file references
{@} @Local:1000::MyApp:v1.0.0
   [@] << "{.}\my-app-02.pg"
   [@] << "{.}\my-app-03.pg"
```

```polyglot
{ } Folder shorthand — include all .pg files in the directory
{@} @Local:1000::MyApp:v1.0.0
   [@] << "{.}"
```

Distinguished from import `[@]` by: no alias on the left, path string on the right. Import syntax is `[@] @alias << @<Registry>:<ID>::<Name>:<Version>`, file reference syntax is `[@] << "path"`.

### Rules

- **Same address** — every file in the package must declare the same `{@}` package name and version (PGE09005, PGE09006)
- **Full mesh** — every file must reference all other files in the package. If file A references B and C, then B must reference A and C, and C must reference A and B (PGE09010)
- **No duplicates** — a `{=}` pipeline name or `{#}` data name must be unique across all files in the package (PGE09007)
- **No self-reference** — a file must not list itself (PGE09009)
- **File must exist** — every referenced path must resolve to an existing `.pg` file (PGE09008)

### Folder Shorthand

`[@] << "{.}"` discovers all `.pg` files in the specified directory. This is equivalent to listing each file explicitly. The folder shorthand satisfies the full mesh rule automatically — the compiler expands it to the full file list before validation.

### Example — Three-File Package

**my-app-01.pg:**
```polyglot
{@} @Local:1000::MyApp:v1.0.0
   [@] << "{.}\my-app-02.pg"
   [@] << "{.}\my-app-03.pg"

{#} #Config
   [.] .host#string
   [.] .port#int
```

**my-app-02.pg:**
```polyglot
{@} @Local:1000::MyApp:v1.0.0
   [@] << "{.}\my-app-01.pg"
   [@] << "{.}\my-app-03.pg"

{=} =LoadConfig
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >config#Config
   [r] >config << ...
```

**my-app-03.pg:**
```polyglot
{@} @Local:1000::MyApp:v1.0.0
   [@] << "{.}\my-app-01.pg"
   [@] << "{.}\my-app-02.pg"

{=} =RunServer
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <config#Config
   [r] ...
```

All three files share the `#Config` data type and can call each other's pipelines as if they were in one file. A file can also reference imports and sibling files together:

```polyglot
{@} @Local:1000::MyApp:v1.0.0
   [ ] sibling files
   [@] << "{.}\my-app-02.pg"
   [ ] external imports
   [@] @utils << @Local:999::Utilities:v1.0.0
```

## Permissions

<!-- @permissions -->
The `{@}` block can declare `[_]` permission lines that set the **permission ceiling** — the maximum IO permissions any definition in the package can request. See [[permissions]] for the full permission system, inline/IO forms, and permission categories.

### Ceiling Syntax

`[_]` lines go after `[@]` imports in the `{@}` block:

```polyglot
{@} @Local:999::LogAnalyzer:v1.0.0
   [@] @http << @Community:devops:HttpClient::HttpClient:v2.1.0
   [_] _File.read"/var/log/*"
   [_] _File.write"/tmp/reports/*"
   [_] _Web.request
      [_] <url#string << "https://alerts.internal/*"
      [_] <method#string << "POST"
   [_] _System.env"LOG_LEVEL"
```

### Ceiling Rules

- **Ceiling, not grant** — `[_]` in `{@}` sets the maximum allowed permissions. Each `{=}` pipeline or `{M}` macro must explicitly request the permissions it needs. Nothing is inherited automatically. See [[permissions#Hierarchical Scoping]].
- **No ceiling = no IO** — if `{@}` has no `[_]` lines, the entire package is pure computation. Any IO call in any pipeline is a compile error (PGE10001).
- **Pipeline subset** — every `[_]` in a pipeline must fall within the package ceiling. A pipeline requesting `_File.read"/etc/shadow"` when the ceiling only allows `_File.read"/var/log/*"` is a compile error (PGE10001).
- **Import ceiling** — the compiler checks each imported package's own `{@}` ceiling against the importer's ceiling. If the imported package declares permissions outside what the importer allows, it is a compile error (PGE10002). Each package declares its own ceiling independently; the compiler validates compatibility.
- **Placement** — `[_]` lines go after `[@]` imports and before any `{=}`/`{#}`/`{M}` definitions in the `{@}` block.

## Compile Rules Reference

| Code | Name | Section |
|------|------|---------|
| PGE09001 | Undefined Import Alias | [[#Usage]] |
| PGE09002 | Circular Package Dependency | [[#Dependency Rules]] |
| PGE09003 | Unresolved Pipeline Reference | [[#Dependency Rules]] |
| PGE09004 | Unresolved Import Pipeline Reference | [[#Usage]] |
| PGE09005 | Multi-File Version Mismatch | [[#Rules]] |
| PGE09006 | Multi-File Package Name Mismatch | [[#Rules]] |
| PGE09007 | Duplicate Definition | [[#Rules]] |
| PGE09008 | Multi-File Reference Not Found | [[#Rules]] |
| PGE09009 | Multi-File Self-Reference | [[#Rules]] |
| PGE09010 | Asymmetric Multi-File Reference | [[#Rules]] |
| PGE09011 | Duplicate Import Alias | [[#Import Rules]] |
| PGE09012 | Import Alias Shadows Standard Library | [[#Import Rules]] |
| PGE09013 | Circular Pipeline Call | [[#Dependency Rules]] |
| PGE10001 | Pipeline Exceeds Package Permission Ceiling | [[#Permissions]] |
| PGE10002 | Imported Package Exceeds Importer Permission Ceiling | [[#Permissions]] |
| PGW09001 | Deprecated Pipeline Reference | [[#Usage]] |
| PGW09002 | Unused Import | [[#Import Rules]] |
