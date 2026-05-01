---
audience: developer
rule: "9.23"
code: PGE10009
name: Unresolved Permission Template
severity: error
---

# Rule 9.23 — Unresolved Permission Template
`PGE10009`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->

**Statement:** If a `{_}` permission template (a `{_}` definition with `(_)` input lines) is referenced but not all `(_)` inputs are provided at the reference site, PGE10009 fires. Additionally, if `{<param}` interpolation in a template's field values cannot resolve after input substitution, PGE10009 fires.
**Rationale:** Permission objects must be fully resolved at compile time — "never generic at resolution." A template with unfilled inputs cannot produce a concrete `_` object, so the compiler cannot validate capabilities, compute content hashes, or enforce the grant-within-ceiling rule. Unresolved templates are undefined behavior by design.
**Detection:** When the compiler encounters a permission reference (`(#) _PermName` or `(-) _PermName`) that resolves to a `{_}` template:
1. Check that every `(_)` input declared in the template has a corresponding `(_) <param << value` at the reference site
2. After substitution, scan all `[.]` field values for remaining `{<...}` interpolation markers
3. If any input is missing or any interpolation is unresolved, PGE10009 fires

**See also:** PGE10005 (invalid permission block marker), PGE10004 (undeclared permission), [[permissions/permission-prefixes#__ Generic Permissions]]

**VALID:**
```aljam3
[ ] ✓ template with all inputs provided
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML

{#} #Config
   (#) _YAMLFile
      (_) <file << "/config/secrets.yaml"     [ ] ✓ all inputs filled
   [#] #data << -Yaml.LoadFile
      (-) <source << _YAMLFile
   [.] .connectionString#string <~ #data.db.connectionString
```

**INVALID:**
```aljam3
[ ] ✗ PGE10009 — template input <file not provided
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML

{#} #Config
   (#) _YAMLFile                              [ ] ✗ PGE10009 — missing (_) <file input
   [#] #data << -Yaml.LoadFile
      (-) <source << _YAMLFile
   [.] .connectionString#string <~ #data.db.connectionString
```

**Open point:** None.
