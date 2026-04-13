---
audience: developer
rule: "1.35"
code: PGE01035
name: Unbound Function Argument
severity: error
---

### Rule 1.35 — Unbound Function Argument
`PGE01035`

<!-- @u:pglib/pipelines/Run/INDEX -->
<!-- @u:pglib/pipelines/Run/Function -->

**Statement:** Every `<arg#Record` field in a `-Run.<Lang>.Function` call must correspond to a positional parameter of the `<func` function. A field with no matching parameter is a compile error.
**Rationale:** `<arg` Record field order maps to positional arguments. If a field doesn't match a parameter, the function call will fail at runtime with an argument count or name error — catchable at compile time.
**Detection:** The compiler parses the function signature of `<func` in the `[C]` block and checks that `<arg#Record` field count and names match the positional parameters. Applies at compile time for `<code.inline` only; deferred to runtime for `<code.file`.

**VALID:**
```polyglot
[ ] field matches function parameter
[-] -Run.Python.Function
   (-) <env#PyEnv << $pyenv
   (-) <func#string << "greet"
   (-) <arg#Record
      [.] .name#string << $userName
   (-) <code.inline <<
      [C] def greet(name):
      [C]     return f"Hello, {name}!"
```

**INVALID:**
```polyglot
[ ] PGE01035 — .full_name not a parameter of greet()
[-] -Run.Python.Function
   (-) <env#PyEnv << $pyenv
   (-) <func#string << "greet"
   (-) <arg#Record
      [.] .full_name#string << $userName
   (-) <code.inline <<
      [C] def greet(name):
      [C]     return f"Hello, {name}!"
```

**Diagnostic:** "Unbound function argument — `<arg` field `.full_name` not found as parameter of function `greet`"
