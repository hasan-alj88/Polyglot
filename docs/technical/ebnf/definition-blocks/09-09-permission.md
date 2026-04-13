---
audience: designer
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.9 Permission Object Definition (`{_}`)

```ebnf
permission_object_def  ::= "{_}" permission_id NEWLINE
                            indent "[.]" ".intent" push_left ( "#Ceiling" | "#Grant" ) NEWLINE
                            { indent permission_field_line NEWLINE }
                            { indent comment_line NEWLINE } ;

permission_field_line  ::= "[.]" "." category_name "." capability_name string_literal ;

category_name          ::= "File" | "Web" | "Database" | "System"
                         | "Crypto" | "IPC" | "Device" | "Memory" ;
```

**Rules:**

- `{_}` defines a named, reusable permission object. The name uses the `_` prefix (e.g., `_DataCeiling`, `_ReportReader`).
- `.intent` must be the first field — either `#Ceiling` (allows glob patterns) or `#Grant` (requires specific narrow values).
- Each `permission_field_line` declares a capability: `.Category.Capability "scope"`. The `category_name` must be one of the 8 predefined categories. Each category has a per-category capability enum (e.g., `#FileCapability`: Read, Write, Execute, Delete, Create).
- **Fully filled** — every `{_}` object must have all leaf fields assigned. Empty leaves are a compile error.
- **No instances** — permissions are compile-time declarations. No `:{instance}` level exists in `%_`.
- **No inline declarations** — `[_]` in `{@}` and `{-}` always references a `{_}` object by name. Inline permission syntax is not valid.
- **Identifier prefixes:** permissions are data trees using `#`/`##`/`###` pattern: `_` = `##Permission` struct instance (all leaves filled), `__` = generic template with `[#]` inputs, `___` = specific field within the permission object.

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.9 `{_}` Permission | [[concepts/permissions\|permissions]] |
