---
rule: "9.7"
code: PGE-907
name: Duplicate Pipeline Definition
severity: error
---

### Rule 9.7 — Duplicate Pipeline Definition
`PGE-907`

**Statement:** A `{=}` pipeline name must be unique across all files in a multi-file package. If two or more files define a pipeline with the same name, PGE-907 fires on the duplicate definition(s). Within a single file, the same rule applies — duplicate pipeline names are never allowed.
**Rationale:** Pipeline names are the unit of reference for `[r]`, `[p]`, and chain calls. If two definitions share a name, the compiler cannot determine which one a caller intends. This is always a bug — either a copy-paste error or a naming collision between files.
**Detection:** After loading all files in the package, the compiler builds a map of `{=}` names to their source files. If any name maps to more than one file, PGE-907 fires on each duplicate (all files after the first occurrence).

**See also:** PGE-908 (duplicate data definition), PGE-903 (unresolved pipeline reference)

**VALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{=} =LoadData
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >data;string
   [r] >data << "loaded"

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{=} =ProcessData
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [r] ...

[ ] ✓ =LoadData and =ProcessData are unique names
```

**INVALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >output;string
   [r] >output << $input

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{=} =Transform                              [ ] ✗ PGE-907 — =Transform already defined in file-01.pg
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >result;string
   [r] >result << $data
```

**Open point:** None.
