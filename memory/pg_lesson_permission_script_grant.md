---
name: pg_lesson_permission_script_grant
description: Script files need dedicated {_} grant with #ScriptExecution; path comes FROM the permission
type: feedback
---

**Rule:** Foreign code script files require a dedicated `{_}` permission grant with `#ScriptExecution` category and `#Execute` capability, scoped to the script path. The `<code.file` input references the permission grant (`_GrantName`), not a string literal — the permission carries the resource locator.
**Why:** I hardcoded the script path as a string literal. The permission-as-resource model means `{_}` carries both the grant (capability) AND the resource locator (path). The script path flows from the permission, not from a literal.
**How to apply:** When calling `-Run.*.Script` with a file: (1) define a `{_}` grant with `.category << #ScriptExecution`, `.capability << #Execute`, `.scope << "/path/to/script.py"`, (2) wire it as `(-) <code.file#path << _GrantName`.
**Spec file:** permissions spec, pg_design_permission_as_resource.md
**Spec updated:** no
