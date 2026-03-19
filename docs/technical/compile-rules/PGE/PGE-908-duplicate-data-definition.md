---
rule: "9.8"
code: PGE-908
name: Duplicate Data Definition
severity: error
---

### Rule 9.8 — Duplicate Data Definition
`PGE-908`

**Statement:** A `{#}` data name must be unique across all files in a multi-file package. If two or more files define a data type with the same name, PGE-908 fires on the duplicate definition(s). Within a single file, the same rule applies — duplicate data names are never allowed.
**Rationale:** Data names are the unit of reference for type annotations (`;TypeName`), enum access (`#Type.Field`), and schema matching. Duplicate names create ambiguity the compiler cannot resolve.
**Detection:** After loading all files in the package, the compiler builds a map of `{#}` names to their source files. If any name maps to more than one file, PGE-908 fires on each duplicate.

**See also:** PGE-907 (duplicate pipeline definition), PGE-401 (type mismatch)

**VALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{#} #Config
   [.] .host;string
   [.] .port;int

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{#} #User
   [.] .name;string
   [.] .email;string

[ ] ✓ #Config and #User are unique names
```

**INVALID:**
```polyglot
{ } file-01.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-02.pg"

{#} #Config
   [.] .host;string
   [.] .port;int

{ } file-02.pg
{@} @Local:1000.MyApp:v1.0.0
   [@] << "{.}\file-01.pg"

{#} #Config                                 [ ] ✗ PGE-908 — #Config already defined in file-01.pg
   [.] .dbHost;string
   [.] .dbPort;int
```

**Open point:** None.
