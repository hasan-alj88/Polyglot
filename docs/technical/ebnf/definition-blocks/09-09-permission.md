---
audience: designer
type: spec
updated: 2026-04-17
---

<!-- @ebnf/INDEX -->

## 9.9 Permission Object Definition (`{_}`)

```ebnf
permission_object_def  ::= "{_}" permission_id NEWLINE
                            { indent perm_input_line NEWLINE }
                            indent "[.]" ".intent" push_left ( "#Ceiling" | "#Grant" ) NEWLINE
                            { indent permission_field_line NEWLINE }
                            { indent comment_line NEWLINE } ;

perm_input_line        ::= "(_)" typed_io_param ;

permission_field_line  ::= "[.]" "." perm_field_name perm_field_value ;

perm_field_name        ::= "category" | "capability" | "scope"
                         | "path" | "format"
                         | "host" | "port" | "endpoint"
                         | "credentials" | "database" | "table"
                         | "command" | "args" ;

perm_field_value       ::= category_name | capability_ref | string_literal | integer_literal | format_enum ;

category_name          ::= "File" | "Web" | "Database" | "System"
                         | "Crypto" | "IPC" | "Device" | "Memory" ;

format_enum            ::= "#YAML" | "#JSON" | "#TOML" ;
```

**Rules:**

- `{_}` defines a named, reusable permission object. The name uses the `_` prefix (e.g., `_Secrets`, `_ProductionDB`).
- **Template inputs:** `(_)` lines declare parameters for permission templates. These must appear before `.intent`. Template field values may use `{<param}` interpolation (e.g., `.path "{<file}"`). Templates are resolved at compile time — all inputs must be provided at the reference site. See PGE10009.
- `.intent` must be the first `[.]` field — either `#Ceiling` (allows glob patterns) or `#Grant` (requires specific narrow values).
- **Decomposed fields:** Each `permission_field_line` declares a single field. The `.Category.Capability "scope"` shorthand is retired — use `.category`, `.capability`, `.scope`, and resource locator fields (`.path`, `.host`, etc.) separately.
- **Category-dependent resource fields:** File permissions require `.path`; Database permissions require `.host` and `.database`; Web permissions require `.host`. See [[concepts/permissions/capability-enums#Per-Category Resource Fields]].
- **Fully filled** — every `{_}` object must have all leaf fields assigned. Empty leaves are a compile error.
- **No instances** — permissions are compile-time declarations. No `:{instance}` level exists in `%_`.
- **Permissions as IO** — blocks reference `{_}` objects through their IO markers: `(#) _PermName` on `{#}` definitions, `(-) _PermName` on `{-}` pipelines. The receiving block gets the whole `_` object and passes it to called pipelines.
- **Identifier prefixes:** permissions are data trees using `#`/`##`/`###` pattern: `_` = `##Permission` struct instance (all leaves filled), `__` = generic template with `(_)` inputs, `___` = specific field within the permission object.

## Examples

```polyglot
[ ] Instance — fully filled, one specific file
{_} _Secrets
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/secrets.yaml"
   [.] .path "/config/secrets.yaml"
   [.] .format #YAML

[ ] Template — parameterized file permission
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML

[ ] Database permission
{_} _ProductionDB
   [.] .intent << #Grant
   [.] .category #Database
   [.] .capability #Read
   [.] .scope "analytics.postgres"
   [.] .host "db.internal"
   [.] .port 5432
   [.] .database "analytics"
```

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.9 `{_}` Permission | [[concepts/permissions\|permissions]] |
