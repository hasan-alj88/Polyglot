---
audience: [architect, designer]
type: spec
updated: 2026-04-05
---

# Path Grammar

<!-- @source:metadata-tree/INDEX -->

The general path patterns:

```ebnf
schema_path     ::= "%" "definition" "." type_prefix ":" ref
                   | "%" "definition" ".##:" ref
                   | "%" "definition" ".###:" ref
instance_path   ::= "%" type_prefix ":" ref ":" instance { "." field }
permission_path ::= "%" "_" ":" name { "." field }
error_path      ::= "%" "!" "." namespace { "." leaf }
                   | "%" "!" ".Error" { ":" user_path }
package_path    ::= "%" "@" ":" registry { ":" id_part } "::" name { ":" segment }
schema_prop     ::= "%" "##" property_name { "." sub_property }
field_type_prop ::= "%" "###" property_name
```

| Element | Rule |
|---------|------|
| `type_prefix` | Follows `instance_path`: `#`, `##`, `###`, `=`, `T`, `W`, `Q`, `~`, `*`, `$`, `M` |
| | Own grammar rule: `_` → `permission_path`, `!` → `error_path`, `@` → `package_path` |
| `ref` | Object name — flexible field (`:`). Applies to `instance_path` prefixes only. |
| `instance` | Instance number — flexible field (`:`). Applies to `instance_path` prefixes only. |
| `field` | Fixed field path (`.`) within the instance |

**Exception branch rules:**
- `%_` uses `permission_path` — `:` for named `{_}` object, then `.` fixed fields. No `:{instance}` level (permissions are compile-time).
- `%!` uses `error_path` — `.` for Polyglot-defined namespaces and leaves. `.Error` children use `:` (user-extensible).
- `%@` uses `package_path` — `::` separates registry+ID from package name. `::` is treated as `:` in the tree.

## Shorthand in User Code

User code uses shorthand accessors that resolve to full instance paths:

| Shorthand | Resolves to |
|-----------|-------------|
| `=MyPipeline%status` | `%=:MyPipeline:<current>.status` |
| `$myVar%state` | `%$:myVar:<current>.state` |
| `#Record%lastModified` | `%#:Record:<current>.lastModified` |
| `=W.DB.Connection%status` | `%W:DB.Connection:<current>.status` |
| `#Queue:GPUQueue%activeCount` | `%Q:GPUQueue:<current>.activeCount` |

The `:<current>` segment is implicit — the runtime resolves it to the calling context's instance.

See also: [[object-types|Object Type Branches]], [[instance-lifecycle|Instance Lifecycle]]
