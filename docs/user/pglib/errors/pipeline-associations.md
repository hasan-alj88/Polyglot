---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# Pipeline Error Associations

<!-- @c:errors -->

Each pglib pipeline declares the errors it can raise via `[=] !ErrorName` (see [[concepts/pipelines/metadata#Error Trees]]):

```aljam3
=File.Text.Read
   [=] !File.NotFound
   [=] !File.ReadError
   [=] !Permission.File.Denied

=File.Text.Write
   [=] !File.NotFound
   [=] !File.WriteError
   [=] !Permission.File.Denied

=File.Text.Append
   [=] !File.NotFound
   [=] !File.WriteError
   [=] !Permission.File.Denied

=File.Serial.Read
   [=] !File.NotFound
   [=] !File.ReadError
   [=] !File.ParseError
   [=] !Permission.File.Denied

=File.Serial.Write
   [=] !File.NotFound
   [=] !File.WriteError
   [=] !Permission.File.Denied

=File.Serial.Read.Field
   [=] !File.NotFound
   [=] !File.ReadError
   [=] !File.ParseError
   [=] !Field.NotFound
   [=] !Permission.File.Denied

=#.Field
   [=] !Field.NotFound
   [=] !Field.PathError

=#.Column
   [=] !Field.NotFound

=Math.Divide
   [=] !Math.DivideByZero

=Math.Modulo
   [=] !Math.DivideByZero

=RT.<Lang>.Function.Inline
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Function.File
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Script.Inline
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Script.File
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.CLI
   [=] !RT.RuntimeError
   [=] !RT.Timeout

=RT.<Lang>.Bind.Inline
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=RT.<Lang>.Bind.File
   [=] !RT.CompileError
   [=] !RT.RuntimeError
   [=] !RT.EnvironmentError

=Text.Diff
   [=] !Text.Diff.EmptyInput

=ForEach.Text.Lines
   [=] !Text.Lines.Empty

*Into.Text.Append
   [=] !Storage.Space
   [=] !Text.Append.EmptyResult

*Into.Text.Merge
   [=] !Storage.Space
   [=] !Text.Merge.InvalidLineNumber
   [=] !Text.Merge.EmptyBase

=ForEach.CSV.Rows
   [=] !CSV.Parse.MalformedRow
   [=] !CSV.Parse.Empty
   [=] !CSV.Parse.InvalidDelimiter

*Into.CSV.Rows
   [=] !Storage.Space
   [=] !CSV.Collect.SchemaMismatch
   [=] !CSV.Collect.EmptyResult

*Into.CSV.Merge
   [=] !Storage.Space
   [=] !Text.Merge.InvalidLineNumber
   [=] !Text.Merge.EmptyBase
   [=] !CSV.Merge.HeaderConflict

-W.Env
   [=] !Env.NotFound
   [=] !Env.VersionMismatch
   [=] !Env.SetupFailed
   [=] !Env.TeardownFailed
   [=] !Env.Dependency.Missing
   [=] !Env.Dependency.VersionConflict
   [=] !Env.Dependency.InstallFailed
```
