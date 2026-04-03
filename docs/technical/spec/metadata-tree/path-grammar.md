---
audience: [architect, designer]
type: spec
updated: 2026-04-03
---

# Path Grammar

<!-- @source:metadata-tree/INDEX -->

The general path patterns:

```ebnf
schema_path     ::= "%" "definition" "." type_prefix ":" ref
                   | "%" "definition" ".##:" ref
                   | "%" "definition" ".###:" ref
instance_path   ::= "%" type_prefix ":" ref ":" instance { "." field }
permission_path ::= "%" "_" { "." field }
error_path      ::= "%" "!" "." namespace { "." leaf }
                   | "%" "!" ".Error" { ":" user_path }
package_path    ::= "%" "@" ":" registry { ":" id_part } "::" name { ":" segment }
schema_prop     ::= "%" "##" property_name { "." sub_property }
field_type_prop ::= "%" "###" property_name
```

| Element | Rule |
|---------|------|
| `type_prefix` | One of: `#`, `##`, `###`, `=`, `T`, `W`, `Q`, `~`, `*`, `$`, `M`, `!`, `@`, `_` |
| `ref` | Object name — flexible field (`:`) |
| `instance` | Instance number — flexible field (`:`) |
| `field` | Fixed field path (`.`) within the instance |

**Branch-specific rules:**
- `%_` — all `.` fixed fields, no `:` anywhere. No ref or instance levels.
- `%!` — `.` for Polyglot-defined namespaces and leaves. `.Error` children use `:` (user-extensible).
- `%@` — `::` separates registry+ID from package name. `::` is treated as `:` in the tree.

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
