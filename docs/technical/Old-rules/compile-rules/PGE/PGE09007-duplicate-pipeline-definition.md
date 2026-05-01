---
audience: developer
rule: "9.7"
code: PGE09007
name: Duplicate Definition
severity: error
---

# Rule 9.7 — Duplicate Definition
`PGE09007`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Every named definition — `{-}` pipeline, `{#}` data type, `{W}` wrapper, `{T}` trigger, `{Q}` queue, or `{!}` error namespace — must have a unique name within the same package and version. If two or more definitions share the same name (across files or within a single file), PGE09007 fires on each duplicate.
**Rationale:** Definition names are the unit of reference for calls (`[-]`/`[=]`/`[b]`), type annotations (`#TypeName`), and wrappers (`[W]`). Duplicate names create ambiguity the compiler cannot resolve — it would not know which definition a reference intends.
**Detection:** After loading all files in the package, the compiler builds a map of all definition names to their source files. If any name maps to more than one definition, PGE09007 fires on each duplicate (all definitions after the first occurrence). The diagnostic includes the definition type and the file where the original was defined.

**See also:** PGE09003 (unresolved pipeline reference), PGE09011 (duplicate import alias — analogous for `[@]` aliases)

**VALID:**
```aljam3
{ } file-01.jm3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.jm3"

{-} -LoadData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) >data#string
   [ ]
   [-] >data << "loaded"

{#} #Config
   [.] .host#string
   [.] .port#int

{ } file-02.jm3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.jm3"

{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <data#string
   [ ]
   [-] ...

{#} #User
   [.] .name#string
   [.] .email#string

[ ] ✓ all names unique: -LoadData, -ProcessData, #Config, #User
```

**INVALID:**
```aljam3
[ ] ✗ PGE09007 — duplicate pipeline across files
{ } file-01.jm3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.jm3"

{-} -Transform
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >output#string
   [ ]
   [-] >output << $input

{ } file-02.jm3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.jm3"

{-} -Transform                              [ ] ✗ PGE09007 — -Transform already defined in file-01.jm3
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <data#string
   (-) >result#string
   [ ]
   [-] >result << $data
```

```aljam3
[ ] ✗ PGE09007 — duplicate data type across files
{ } file-01.jm3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.jm3"

{#} #Config
   [.] .host#string
   [.] .port#int

{ } file-02.jm3
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.jm3"

{#} #Config                                 [ ] ✗ PGE09007 — #Config already defined in file-01.jm3
   [.] .dbHost#string
   [.] .dbPort#int
```

```aljam3
[ ] ✗ PGE09007 — duplicate wrapper within same file
{@} @Local:1000.MyApp:v1.0.0

{W} -W.Setup
   (-) <conn;string
   (-) >handle;string
   [\]
      [-] -Connect
         (-) <conn << $conn
         (-) >handle >> $handle
   [/]
      [-] -Disconnect
         (-) <handle << $handle

{W} -W.Setup                                [ ] ✗ PGE09007 — -W.Setup already defined above
   (-) <input;string
   (-) >output;string
   [\]
      [-] -DoNothing
   [/]
      [-] -DoNothing
```

**Diagnostic:** "Duplicate {definition type} `{name}` in package `{package}:{version}` — first defined in {file}:{line}, duplicate at {file}:{line}"

## See Also

- [[user/syntax/packages|Packages]] — references PGE09007 in multi-file package rules

**Open point:** None.
