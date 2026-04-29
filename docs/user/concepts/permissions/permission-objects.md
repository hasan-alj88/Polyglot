---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# {_} Permission Objects

<!-- @c:identifiers -->
<!-- @c:blocks -->
<!-- @u:permissions/permission-prefixes -->
<!-- @u:permissions/permission-schema -->

Permissions are declared as named `{_}` blocks — first-class, reusable permission objects. Each `{_}` block defines a permission policy with a name, intent, and one or more capability grants. The permission object carries both the **grant** (what you're allowed to do) and the **resource locator** (where — file path, DB connection, etc.).

```polyglot
{_} _DataCeiling
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "data/*.csv"
   [.] .path "data/*.csv"

{_} _ReportReader
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "data/reports/q1.csv"
   [.] .path "data/reports/q1.csv"
```

## Intent: Ceiling vs Grant

Every `{_}` object declares an `.intent` field — either `#Ceiling` or `#Grant`:

| Intent | Purpose | Scope values |
|--------|---------|--------------|
| `#Ceiling` | Maximum permissions a package **allows** | Glob patterns permitted (`"data/*.csv"`, `"/var/log/*"`) |
| `#Grant` | Specific permissions a pipeline **requests** | Narrow, specific values only (`"data/reports/q1.csv"`) |

- **Ceiling** — referenced by `{@}` packages via `(@)` IO. Sets the upper bound. Allows but does not grant.
- **Grant** — referenced by `{-}` pipelines via `(-)` IO and `{#}` definitions via `(#)` IO. Requests specific capabilities within the ceiling.
- **Compiler validates: Grant must be a subset of Ceiling** — every grant value must fall within a ceiling pattern. A grant requesting `"data/secret.csv"` when the ceiling only allows `"data/reports/*"` is a compile error (PGE10001).
- **Narrowing allowed, expanding NOT** — a grant can request less than the ceiling allows, but never more.

## Fully Filled Requirement

Every `{_}` object must be **fully filled** — every leaf field must have a value assigned. Empty leaves are a compile error. This prevents accidental "allow everything" policies from incomplete declarations.

## Instance vs Template

Permission objects come in two forms:

### Instance — Fully Filled

A `{_}` instance has all fields resolved to concrete values. It points to one specific resource:

```polyglot
{_} _Secrets
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/secrets.yaml"
   [.] .path "/config/secrets.yaml"
   [.] .format #YAML
```

### Template — Parameterized

A `{_}` template has `(_)` input lines. The caller provides the missing values, and the compiler resolves the template into a concrete instance at compile time:

```polyglot
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML
```

Templates are **never generic at resolution** — by compile time, every `_` template resolves to a specific resource with all fields filled.

## Permissions as IO

Permission objects are referenced through the **IO markers** of the block that uses them. Every block type uses its own IO marker:

- `{#}` definitions use `(#) _PermName`
- `{-}` pipelines use `(-) _PermName`
- `{@}` packages use `(@) _PermName`

### Usage in {#} Definitions

A `{#}` data definition declares its file dependencies via `(#)`:

```polyglot
[ ] Instance — no inputs needed
{#} #Config
   (#) _Secrets
   [#] #data << -Yaml.LoadFile
      (-) <source << _Secrets
   [.] .connectionString#string <~ #data.db.connectionString

[ ] Template — caller fills the path
{#} #Config2
   (#) _YAMLFile
      (_) <file << "/config/secrets2.yaml"
   [#] #data << -Yaml.LoadFile
      (-) <source << _YAMLFile
   [.] .connectionString#string <~ #data.db.connectionString
```

### Usage in {-} Pipelines

A `{-}` pipeline declares its resource dependencies via `(-)`:

```polyglot
{_} _ProductionDB
   [.] .intent << #Grant
   [.] .category #Database
   [.] .capability #Read
   [.] .scope "analytics.postgres"
   [.] .host "db.internal"
   [.] .port 5432
   [.] .database "analytics"

{-} -ProcessOrders
   (-) _ProductionDB
   [T] -T.Schedule.Cron "0 * * * *"
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ]
   [-] -DB.Query
      (-) <connection << _ProductionDB
      (-) <sql << "SELECT * FROM orders WHERE status = 'pending'"
      (-) >rows >> $orders
```

The pipeline receives the whole `_` object — `-DB.Query` extracts `.host`, `.port`, `.database`, and `.credentials` from the permission object.

### No Undeclared Access

If a block accesses an external resource without declaring the corresponding `_` dependency in its IO, the compiler raises PGE10004. No `_` IO declaration = pure computation only.
