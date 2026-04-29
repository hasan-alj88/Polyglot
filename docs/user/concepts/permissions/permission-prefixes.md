---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# Permission Prefixes: _ / __ / ___

<!-- @c:identifiers -->

Permission objects are data trees — the same `#`/`##`/`###` pattern applied to permissions:

| Prefix | Is a | Purpose | Mirrors |
|--------|------|---------|---------|
| `_` | `#` struct instance | `##Permission` instance, all leaves filled (Final or Default->Final on pull) | `#` |
| `__` | `##` schema template | Generic permission with inputs — fills missing fields to yield a valid `_` object | `##` |
| `___` | `###` field | A specific field within the permission object | `###` |

## _ Permission Objects

A `_` object is a `#` struct instance whose schema is `##Permission`. Every leaf must be filled — either Final or Default (which becomes Final when pulled). This is a data tree, not a special construct.

```aljam3
{_} _WebAccess
   [.] .intent << #Ceiling
   [.] .category #Web
   [.] .capability #Request
   [.] .scope "https://api.example.com/*"
   [.] .host "api.example.com"
   [.] .port 443
   [.] .endpoint "/v2/*"
```

## __ Generic Permissions

A `__` descriptor is a `##` schema template with `(_)` inputs. It fills missing fields to produce a concrete `_` object at compile time.

### pglib Generic Permissions

pglib ships generic permissions at two levels:

- **Category-level** (`__File`, `__Web`, ...) — takes capability + scope + resource fields
- **Capability-level** (`__File.Read`, `__Web.Request`, ...) — takes only scope + resource fields (capability is baked in)

```aljam3
[ ] Category-level — specify capability, scope, and resource fields
{_} __File
   (_) <capability#Capability
   (_) <scope#string
   (_) <path#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability "{<capability}"
   [.] .scope "{<scope}"
   [.] .path "{<path}"

[ ] Capability-level — scope and resource only (capability is baked in)
{_} __File.Read
   (_) <scope#string
   (_) <path#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<scope}"
   [.] .path "{<path}"
```

See [[pglib/permissions/INDEX|pglib Generic Permissions]] for the full list of 8 category generics and 31 capability generics.

### User-Defined Templates

Users define custom `{_}` templates with `(_)` input lines:

```aljam3
[ ] Template definition — parameterized file permission
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML

[ ] Usage in {#} — caller fills the path
{#} #Config
   (#) _YAMLFile
      (_) <file << "/config/secrets.yaml"
   [#] #data << -Yaml.LoadFile
      (-) <source << _YAMLFile
   [.] .connectionString#string <~ #data.db.connectionString
```

**Compile-time resolution:** All templates are fully resolved at compile time. The compiler substitutes `(_)` inputs, resolves `{<param}` interpolation in field values, and validates the resulting `_` object has all leaves filled. For file-category permissions, the compiler reads the file at `.path` and computes a content hash. See [[enforcement#Compile-Time File Binding]]. No runtime permission evaluation occurs — the compiler validates all grants against ceilings statically.

## ___ Permission Fields

A `___` identifier names a specific field within the permission data tree — mirroring `###` field types. These restrict permission behavior based on environment or policy, and like `__` descriptors, resolve entirely at compile time.

Examples:

- `___Unix` — Unix-specific permission constraints (file modes, signals)
- `___Sandboxed` — sandboxed environment restrictions
- `___ReadOnly` — read-only access constraint
