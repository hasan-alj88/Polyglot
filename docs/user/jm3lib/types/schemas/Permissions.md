---
audience: automation-builder
type: specification
updated: 2026-05-04
status: complete
metadata_definition: "%definition.##:Permissions"
metadata_instance: "%##:Permissions:N"
---

# `##Permissions` — Capability Handles Schema

<!-- @c:schemas -->

In Aljam3, authorized capability handles (`_`) are strictly organized into the `{_}` **Permissions Data Tree**, governed by the `##Permissions` schema. 

This schema relies on the strict **Zero Trust Atomic Capability Model**.

## 1. Structural Definition

The `##Permissions` schema leverages `%##Active << #ActiveKind.One` (or its syntax sugar `##OneActive`) to mathematically enforce that a capability handle (`_`) represents exactly **one atomic intent**. 

There are no compound permissions (e.g., no `ReadWrite`), and no recursive inheritance.

```aljam3
{#} #FileMode
   [ ] No Datatypes = Enum
   [.] .Read
   [.] .Write
   [.] .Append

{#} #FileCreation
   [ ] No Datatypes = Enum
   [.] .MustExist
   [.] .CreateIfMissing
   [.] .CreateOrFail
   [.] .Overwrite

{#} #FilePermission
   [ ] syntax sugar for [#] %##Active << #ActiveKind.One
   [#] << ##OneActive  
   
   [.] .Path#UnixFilePath
   [.] .Mode#FileMode
   [.] .Creation#FileCreation

{#} #FolderPermission
   [#] << ##OneActive  
   
   [.] .ListFiles
      [.] .folder#UnixFolder
   [.] .ListSubfolders
      [.] .folder#UnixFolder
   [.] .ReadFiles
      [.] .folder#UnixFolder
      [.] .FileNameRegex#RawString
   [.] .AppendFiles
      [.] .folder#UnixFolder
      [.] .FileNameRegex#RawString
   [.] .OverwriteFiles
      [.] .folder#UnixFolder
      [.] .FileNameRegex#RawString
   [.] .CreateFiles
      [.] .folder#UnixFolder
      [.] .FileNameRegex#RawString
   [.] .CreateFolders
      [.] .folder#UnixFolder
      [.] .FileNameRegex#RawString
   [.] .DeleteFiles
      [.] .folder#UnixFolder
      [.] .FileNameRegex#RawString

{#} #DatabasePermission
   [#] << ##OneActive  
   
   [.] .QueryRecords
      [.] .connectionUri#String
      [.] .TableNameRegex#RawString
   [.] .InsertRecords
      [.] .connectionUri#String
      [.] .TableNameRegex#RawString
   [.] .UpdateRecords
      [.] .connectionUri#String
      [.] .TableNameRegex#RawString
   [.] .DeleteRecords
      [.] .connectionUri#String
      [.] .TableNameRegex#RawString
   [.] .ExecuteProcedure
      [.] .connectionUri#String
      [.] .ProcedureNameRegex#RawString
   [.] .ExecuteRawQuery
      [.] .connectionUri#String
      [ ] Note: Raw queries bypass structural checks (Absolute Trust)

{#} ##Permissions
   [#] << ##OneActive
   [.] .File#FilePermission
   [.] .Folder#FolderPermission
   [.] .Database#DatabasePermission
   [.] .Runtime
      [.] .Execute
      [.] .Compile
```

## 2. Populating the `{_}` Tree

The `{_}` tree natively consumes configuration variables from the `{;}` Environments tree to ensure capabilities can adapt across staging and production without modifying pipeline logic. It is populated using standard struct syntax `[.]`.

```aljam3
[ ] Fill the Permissions Data Tree natively
{_} _DataCsvReader
   [.] .Folder.ReadFiles
      [.] .folder << {;}<DataPath
      [.] .FileNameRegex << "\\.csv$"
```

## 3. Enforcement via Rust TM (Task Manager)

Aljam3 does not pass raw Network Sockets or unbounded File Descriptors to the pipeline. It uses an **In-Process Trusted Proxy Model**.

1. **Setup Phase**: When the TM evaluates the capability, it establishes the raw connection (e.g., a `sqlx::Pool` or a `dir_fd`).
2. **Proxy Generation**: The TM wraps the raw connection and the `.Regex` constraint into a secure Rust Struct (e.g., `Aljam3DbProxy`) where all internal fields are `private`.
3. **Execution**: The async pipeline function is forced to call methods on the Proxy Struct. The Proxy enforces the structural intent and regex strings, logging OpenTelemetry security violations if the pipeline attempts unauthorized access.
4. **SQL Injection Prevention**: Because pipelines use proxy structs, all database writes are automatically parameterized (Prepared Statements), mathematically neutralizing SQL injection.

## 4. Host Protections (Compile Time)

The Aljam3 Compiler actively protects the underlying Unix host. It will raise a Compile Error and halt if a capability attempts to violate host boundaries:

1. **System Directory Block**: Targeting `/etc/`, `/bin/`, `/proc/` raises `!CompileError.RestrictedSystemPath`.
2. **Host Ownership Mismatch**: If the user running the script does not have native OS access to the folder, it raises `!CompileError.HostPermissionDenied`.
3. **Invalid Regex**: Raises `!CompileError.InvalidRegex`.

## Related
- [[jm3lib/types/schemas/Environments|##Environments]]
- [[jm3lib/types/schemas/ErrorsTree|##ErrorsTree]]
