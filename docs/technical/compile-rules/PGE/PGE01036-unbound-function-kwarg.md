---
audience: developer
rule: "1.36"
code: PGE01036
name: Unbound Function Kwarg
severity: error
---

### Rule 1.36 — Unbound Function Kwarg
`PGE01036`

<!-- @u:pglib/pipelines/Run/INDEX -->
<!-- @u:pglib/pipelines/Run/Function -->

**Statement:** Every `<kwarg#Record` field in a `-Run.<Lang>.Function` call must correspond to a keyword parameter of the `<func` function. A field name with no matching keyword parameter is a compile error.
**Rationale:** `<kwarg` Record field names map to keyword arguments by name. If a field name doesn't match a keyword parameter, the function call will fail at runtime with an unexpected keyword argument error — catchable at compile time.
**Detection:** The compiler parses the function signature of `<func` in the `[C]` block and checks that each `<kwarg#Record` field name matches a keyword parameter (or `**kwargs` catch-all). Applies at compile time for `<code.inline` only; deferred to runtime for `<code.file`.

**VALID:**
```polyglot
[ ] kwarg field names match function keyword parameters
[-] -Run.Python.Function
   (-) <env#PyEnv << $pyenv
   (-) <func#string << "format_name"
   (-) <kwarg#Record
      [.] .first#string << $firstName
      [.] .last#string << $lastName
   (-) <code.inline <<
      [C] def format_name(first="", last=""):
      [C]     return f"{last}, {first}"
```

**INVALID:**
```polyglot
[ ] PGE01036 — .surname not a keyword parameter of format_name()
[-] -Run.Python.Function
   (-) <env#PyEnv << $pyenv
   (-) <func#string << "format_name"
   (-) <kwarg#Record
      [.] .first#string << $firstName
      [.] .surname#string << $lastName
   (-) <code.inline <<
      [C] def format_name(first="", last=""):
      [C]     return f"{last}, {first}"
```

**Diagnostic:** "Unbound function kwarg — `<kwarg` field `.surname` not found as keyword parameter of function `format_name`"
