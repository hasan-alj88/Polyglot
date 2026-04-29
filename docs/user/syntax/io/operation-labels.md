---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
---

# Operation Labels

<!-- @u:operation-labels -->
`(-) $Label` labels a pipeline call's IO, allowing downstream operations to access outputs via `$Label>outputParam` without intermediate variables. The `(-)` marker mirrors the `[-]` pipeline call context. See [[operation-labels]] for full syntax, chain step labels `(.)`, IO comments `( )`, and compile rules.

```aljam3
[-] -ReadFile
   (-) $Read
   (-) <path << "input.csv"
   (-) >content

[-] -ParseCSV
   (-) $Parse
   (-) <data << $Read>content         ( ) access Read's output directly
```

In chain IO addressing, step labels replace numeric/leaf-name step refs:

```aljam3
[-] -ReadFile->-ParseCSV->-ValidateRows
   (-) $Pipeline
      (.) $Read
      (.) $Parse
      (.) $Validate
   (-) >$Read.path << "input.csv"
   (-) <$Parse.rows >> >result
```
