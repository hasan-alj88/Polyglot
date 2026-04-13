---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Permission Prefixes: _ / __ / ___

<!-- @c:identifiers -->

Permission objects are data trees — the same `#`/`##`/`###` pattern applied to permissions:

| Prefix | Is a | Purpose | Mirrors |
|--------|------|---------|---------|
| `_` | `#` struct instance | `##Permission` instance, all leaves filled (Final or Default->Final on pull) | `#` |
| `__` | `##` schema template | Generic permission with `[#]` inputs — fills missing fields to yield a valid `_` object | `##` |
| `___` | `###` field | A specific field within the permission object | `###` |

## _ Permission Objects

A `_` object is a `#` struct instance whose schema is `##Permission`. Every leaf must be filled — either Final or Default (which becomes Final when pulled). This is a data tree, not a special construct.

```polyglot
{_} _WebAccess
   [.] .intent << #Ceiling
   [.] .Web.Request "https://api.example.com/*"
   [.] .Web.Socket "wss://stream.example.com/*"
```

## __ Generic Permissions

A `__` descriptor is a `##` schema template with `[#]` inputs. It fills missing fields to produce a concrete `_` object at compile time — syntax sugar so you don't write the full `{_}` block each time.

pglib ships generic permissions at two levels:

- **Category-level** (`__File`, `__Web`, ...) — takes capability + scope
- **Capability-level** (`__File.Read`, `__Web.Request`, ...) — takes only scope, sugar over category-level

```polyglot
[ ] Category-level — specify capability and scope
[_] __File
   (_) <capability << .Read
   (_) <scope << "/var/log/*"

[ ] Capability-level — scope only (capability is baked in)
[_] __File.Read
   (_) <scope << "/var/log/app/*.log"
```

See [[pglib/permissions/INDEX|pglib Generic Permissions]] for the full list of 8 category generics and 31 capability generics.

Users can also define custom `__` generics:

```polyglot
{_} __ApiAccess
   [#] <endpoint;string

   [.] .intent << #Grant
   [.] .Web.Request "{$endpoint}"

[_] __ApiAccess
   (_) <endpoint << "https://api.example.com/*"
```

**Compile-time resolution:** All generic permissions are fully resolved at compile time. The resulting `_` object has all leaves in Final or Default state. When a Default leaf is pulled, it transitions to Final (see [[variable-lifecycle#Default]]). No runtime permission evaluation occurs — the compiler validates all grants against ceilings statically.

## ___ Permission Fields

A `___` identifier names a specific field within the permission data tree — mirroring `###` field types. These restrict permission behavior based on environment or policy, and like `__` descriptors, resolve entirely at compile time.

Examples:

- `___Unix` — Unix-specific permission constraints (file modes, signals)
- `___Sandboxed` — sandboxed environment restrictions
- `___ReadOnly` — read-only access constraint
