---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Operation Labels

<!-- @u:operation-labels -->
`($)` labels a pipeline call's IO, allowing downstream operations to access outputs via `$Label>outputParam` without intermediate variables. See [[operation-labels]] for full syntax, chain step labels `(.)`, IO comments `( )`, and compile rules.

```polyglot
[-] -ReadFile
   ($) $Read
   (-) <path << "input.csv"
   (-) >content

[-] -ParseCSV
   ($) $Parse
   (-) <data << $Read>content         ( ) access Read's output directly
```

In chain IO addressing, step labels replace numeric/leaf-name step refs:

```polyglot
[-] -ReadFile->-ParseCSV->-ValidateRows
   ($) $Pipeline
      (.) $Read
      (.) $Parse
      (.) $Validate
   (-) >$Read.path << "input.csv"
   (-) <$Parse.rows >> >result
```
