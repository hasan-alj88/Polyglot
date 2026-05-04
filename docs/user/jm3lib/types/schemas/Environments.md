---
audience: automation-builder
type: specification
updated: 2026-05-04
status: draft
metadata_definition: "%definition.##:Environments"
metadata_instance: "%##:Environments:N"
---

# `##Environments` — System Configuration Schema

<!-- @c:schemas -->

In Aljam3, Environments (`{;}`) are not loose strings or floating variables; they are natively represented as a **Data Tree** strictly governed by the `##Environments` schema.

## Structural Definition

The `##Environments` schema leverages `%##Active << #ActiveType.One` to enforce union-like exclusive typing at the capability level. A specific environment configuration must be exactly one type of resource (e.g., a File, a Database, or a Runtime).

```aljam3
{#} ##Environments
   [#] %##Active << #ActiveType.One
   [ ] Only one of the fields in this level is active 
   [.] .File
      [.] .file#File
      [.] .Mode#String
   [.] .Database
      [.] .Uri#URIString
      [.] .Pool#Int
   [.] .Runtime
      [#] %##Active << #ActiveType.One
      [.] .Python
         [.] .uv#Boolean
         [.] .Requirements#File
      [.] .Rust
         [.] .CargoToml#File
      [.] .Node
         [.] .PackageJson#File
```

## Populating the `{;}` Tree

Developers fill the `{;}` Environments Data Tree to define the raw *values* for a specific deployment scope (e.g., Staging vs. Production).

```aljam3
[ ] Fill the Environments Data Tree for Production
{;}
   (<) <DbUri << "postgres://user:pass@localhost:5432/production_db"
   (<) <DataPath << "/opt/app/data/prod_users/"
```

## Symbiosis with Permissions (`_`)

The `{;}` Environments tree is directly consumed by the `__` compilation blocks to yield authorized `_` capability handles, which populate the `{_}` Permissions Data Tree. 

This separation allows developers to recompile an entire project for a new environment without modifying the internal pipeline logic. The compiler simply reads the new `{;}` tree, verifies the resources exist, and asserts `!NoErrors`.

## Related
- [[jm3lib/types/schemas/Permissions|##Permissions]]
- [[jm3lib/types/schemas/ErrorsTree|##ErrorsTree]]
