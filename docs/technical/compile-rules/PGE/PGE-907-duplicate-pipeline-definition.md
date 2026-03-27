---
rule: "9.7"
code: PGE-907
name: Duplicate Definition
severity: error
---

### Rule 9.7 — Duplicate Definition
`PGE-907`

**Statement:** Every named definition — `{=}` pipeline, `{#}` data type, or `{M}` macro — must have a unique name within the same package and version. If two or more definitions share the same name (across files or within a single file), PGE-907 fires on each duplicate.
**Rationale:** Definition names are the unit of reference for calls (`[r]`/`[p]`/`[b]`), type annotations (`#TypeName`), and macro wrappers (`[W]`). Duplicate names create ambiguity the compiler cannot resolve — it would not know which definition a reference intends.
**Detection:** After loading all files in the package, the compiler builds a map of all `{=}`, `{#}`, and `{M}` names to their source files. If any name maps to more than one definition, PGE-907 fires on each duplicate (all definitions after the first occurrence). The diagnostic includes the definition type and the file where the original was defined.

**See also:** PGE-903 (unresolved pipeline reference), PGE-912 (duplicate import alias — analogous for `[@]` aliases)

**VALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{=} =LoadData
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >data#string
   [r] >data << "loaded"

{#} #Config
   [.] .host#string
   [.] .port#int

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{=} =ProcessData
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [r] ...

{#} #User
   [.] .name#string
   [.] .email#string

[ ] ✓ all names unique: =LoadData, =ProcessData, #Config, #User
```

**INVALID:**
```polyglot
[ ] ✗ PGE-907 — duplicate pipeline across files
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [r] >output << $input

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{=} =Transform                              [ ] ✗ PGE-907 — =Transform already defined in file-01.pg
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >result#string
   [r] >result << $data
```

```polyglot
[ ] ✗ PGE-907 — duplicate data type across files
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{#} #Config
   [.] .host#string
   [.] .port#int

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{#} #Config                                 [ ] ✗ PGE-907 — #Config already defined in file-01.pg
   [.] .dbHost#string
   [.] .dbPort#int
```

```polyglot
[ ] ✗ PGE-907 — duplicate macro within same file
{@} @Local:1000.MyApp:v1.0.0

{M} =W.Setup
   [{] $conn#string
   [}] $handle#string
   [\]
      [r] =Connect
         [=] <conn << $conn
         [=] >handle >> $handle
   [/]
      [r] =Disconnect
         [=] <handle << $handle

{M} =W.Setup                                [ ] ✗ PGE-907 — =W.Setup already defined above
   [{] $input#string
   [}] $output#string
   [\]
      [r] =DoNothing
   [/]
      [r] =DoNothing
```

**Diagnostic:** "Duplicate {definition type} `{name}` in package `{package}:{version}` — first defined in {file}:{line}, duplicate at {file}:{line}"

**Open point:** None.
