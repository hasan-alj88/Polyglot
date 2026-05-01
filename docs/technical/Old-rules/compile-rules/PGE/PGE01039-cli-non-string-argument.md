---
audience: developer
rule: "1.39"
code: PGE01039
name: CLI Non-String Argument
severity: error
---

# Rule 1.39 — CLI Non-String Argument
`PGE01039`

<!-- @u:jm3lib/pipelines/Run/INDEX -->
<!-- @u:jm3lib/pipelines/Run/CLI -->

**Statement:** In a `-Run.<Lang>.CLI` call, all `<arg#Record` and `<kwarg#Record` fields must be typed `#string`. A non-string field type is a compile error.
**Rationale:** CLI arguments are shell strings. The runtime passes them as command-line tokens to the binary — there is no structured type marshalling for CLI invocations. Non-string types would require implicit conversion with no defined semantics.
**Detection:** The compiler checks that every field in `<arg#Record` and `<kwarg#Record` has type `#string`.

**VALID:**
```aljam3
[ ] all CLI argument fields are #string
[-] -Run.Rust.CLI
   (-) <binary#path << -Path"/usr/local/bin/mytool"
   (-) <arg#Record
      [.] .input#string << "{$inputPath}"
   (-) <kwarg#Record
      [.] .format#string << "json"
      [.] .verbose#string << "true"
```

**INVALID:**
```aljam3
[ ] PGE01039 — .count is #int, not #string
[-] -Run.Rust.CLI
   (-) <binary#path << -Path"/usr/local/bin/mytool"
   (-) <arg#Record
      [.] .input#string << "{$inputPath}"
      [.] .count#int << $count
```

**Diagnostic:** "CLI non-string argument — `.CLI` `<arg` field `.count` is `#int`; all CLI argument fields must be `#string`"
