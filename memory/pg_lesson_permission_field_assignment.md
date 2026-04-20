---
name: pg_lesson_permission_field_assignment
description: Permission {_} fields are value fields needing << assignment, not enum field declarations
type: feedback
---

**Rule:** All `{_}` permission object fields use `<<` final assignment: `[.] .category << #File`, not `[.] .category #File`.
**Why:** I treated permission fields as enum field declarations (no operator), but `{_}` is a `##Permission` struct instance "with all leaves filled" — every field is a value field holding an enum value, requiring explicit `<<` assignment.
**How to apply:** When writing `{_}` blocks, always use `<<` on every `[.]` field. Permission fields are value fields assigned with enum values, not enum field declarations.
**Spec file:** docs/user/syntax/identifiers.md (permission identifiers), permissions spec
**Spec updated:** no
