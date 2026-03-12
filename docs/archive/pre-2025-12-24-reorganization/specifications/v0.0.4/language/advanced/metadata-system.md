---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "metadata-system"
shard: false

# --- Classification ---
type: spec
topic: "Polyglot v0.0.4 - Metadata System Specif"
summary: "Advanced: Polyglot v0.0.4 - Metadata System S"
keywords:
  - advanced
  - features
  - specification

# --- BMAD Agent Routing ---
agents:
  - architect
phase: solutioning
workflow: greenfield
module: bmm
complexity: high

# --- Dependency Chain ---
prereqs:
  - language-syntax
  - type-system
  - control-flow
unlocks:
  - stdlib

# --- Relationships ---
related:
  []
parent: "language-advanced"

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#advanced"
  - "#features"
---
# Polyglot v0.0.4 - Metadata System Specification

**Version:** v0.0.4
**Status:** ✅ Final
**Last Updated:** 2025-12-14

---

## Overview

Polyglot's metadata system uses the `%` prefix to attach structured annotations to packages, pipelines, enums, errors, and parameters.

**Purpose:**
- Documentation generation
- Deprecation warnings
- Author tracking
- Reserved indication (core privilege)
- Tooling configuration (e.g., variadic input via `%InStream`)

---

## Metadata Tree Structure

### Complete Metadata Schema

```
MetaData [%]
├── Package {@}
│   ├── Doc :pg.string
│   ├── Author :pg.array.enum.Author
│   │    ├── 0
│   │    │   ├── Name :pg.string
│   │    │   ├── Since :pg.string
│   │    │   └── Email :pg.string
│   │    ├── 1
│   │    │   ├── Name :pg.string
│   │    │   ├── Since :pg.string
│   │    │   └── Email :pg.string
│   │    ...
│   └── Deprecated
│        ├── message :pg.string
│        └── new :pg.string
├── Pipeline {|}
│   ├── Doc :pg.string
│   ├── Author :pg.array.enum.Author
│   │    ├── 0
│   │    │   ├── Name :pg.string
│   │    │   ├── Since :pg.string
│   │    │   └── Email :pg.string
│   │    ├── 1
│   │    │   ├── Name :pg.string
│   │    │   ├── Since :pg.string
│   │    │   └── Email :pg.string
│   │    ...
│   └── Deprecated
│        ├── message :pg.string
│        └── new :pg.string
├── Enum {#}
│   ├── Doc :pg.string
│   ├── Author :pg.array.enum.Author
│   │    ├── 0
│   │    │   ├── Name :pg.string
│   │    │   ├── Since :pg.string
│   │    │   └── Email :pg.string
│   │    ├── 1
│   │    │   ├── Name :pg.string
│   │    │   ├── Since :pg.string
│   │    │   └── Email :pg.string
│   │    ...
│   ├── Deprecated
│   │    ├── message :pg.string
│   │    └── new :pg.string
│   └── Reserved :pg.bool
├── EnumField [.]
│   └── Reserved :pg.bool
└── Error {!}
    ├── Doc :pg.string
    ├── Author :pg.array.enum.Author
    │    ├── 0
    │    │   ├── Name :pg.string
    │    │   ├── Since :pg.string
    │    │   └── Email :pg.string
    │    ├── 1
    │    │   ├── Name :pg.string
    │    │   ├── Since :pg.string
    │    │   └── Email :pg.string
    │    ...
    ├── Deprecated
    │    ├── message :pg.string
    │    └── new :pg.string
    ├── Reserved :pg.bool
    ├── Message :pg.string
    ├── TraceBack :pg.array.pg.string
    └── Object :#PolyglotObject
```

**Author Enum Structure:**
```
#Author
├── Name :pg.string
├── Since :pg.string
└── Email :pg.string
```

**PolyglotObject Enum:**
```
#PolyglotObject
├── Package
├── Pipeline
├── Enum
└── Variable
```

---

## Metadata Syntax

### Package Metadata

```polyglot
{@} @Local::MyApp:1.0.0.0
[A] @App
[<] @PgTypes                       // Import alias registry

[%] %Doc <<
[+] +"Advanced application for"
[+] +"managing user workflows"

[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "0.1.0"

[%] %Author                        // Multiple authors
   [.] .name << "Bob <bob@example.com>"
   [.] .since << "0.1.4"

[%] %License << "MIT"
[%] %Repository << "https://github.com/myapp/repo"
{x}
```

### Pipeline Metadata

```polyglot
{|} |FetchUser
[%] %PublicTo << {@MyApp}          // Export visibility
[%] %PrivacyGroup << "Authentication"

[%] %Errors
   [.] .error << @MyApp!Database.Timeout
   [.] .when << "Database unreachable"

[<] i<user_id:string
   [%] %description << "Unique user identifier"
   // %Params.0.name and %Params.0.type inferred from declaration

[<] i<include_profile:bool
   [%] %description << "Include full profile data"
   [%] %default << #False

[>] o>user:serial
   [%] %description << "User data with optional profile"

{x}
```

### Enum Metadata

```polyglot
{#} #EnumDef
[%] %Reserved << #Boolean.True     // Enum-level: this enum is reserved
[%] %Doc << "Core boolean enumeration"

[.] .enumfield                     // Reserved field (dots in reserved enum)
[.] .enumfield_also_reserved       // Another reserved field
   [%] %Reserved << #Boolean.True  // Field-level reserved (privileged!)
{#}
```

**Reserved enum field usage:**
```polyglot
#EnumDef.enumfield                 // All dots (fully reserved enum and field)
#EnumDef.enumfield_also_reserved   // All dots (fully reserved enum and field)
#EnumDef;custom_field              // Semicolon (reserved enum, custom field extension)
```

### Deprecation Metadata

```polyglot
{|} |FetchUser
[%] %Deprecated
   [.] .new_pipeline << |FetchUserV2
   [.] .message << "Use |FetchUserV2 instead"
   [.] .auto_migrate << #True    // Auto-switch in tooling
{x}
```

**Compiler behavior:**
```polyglot
[r] |FetchUser                     // ⚠️  WARNING: FetchUser is deprecated
                                   //     Use |FetchUserV2 instead
```

---

## Standard Metadata Attributes

### Common Attributes (All Constructs)

| Attribute | Type | Description |
|-----------|------|-------------|
| `%Doc` | :pg.string or multi-line | Documentation |
| `%Author` | :array.enum.Author | Author info (repeatable) |
| `%Deprecated` | struct | Deprecation notice with migration |

### Package-Specific

| Attribute | Type | Description |
|-----------|------|-------------|
| `%License` | :pg.string | License (e.g., "MIT", "Apache-2.0") |
| `%Repository` | :pg.string | Source repository URL |

### Pipeline-Specific

| Attribute | Type | Description |
|-----------|------|-------------|
| `%PublicTo` | :array.registry | Export visibility control |
| `%PrivacyGroup` | :pg.string | Privacy/security grouping |
| `%Errors` | :array.struct | Expected errors with conditions |

### Parameter-Specific

| Attribute | Type | Description |
|-----------|------|-------------|
| `%description` | :pg.string | Parameter documentation |
| `%default` | varies | Default value for parameter |

### Enum-Specific

| Attribute | Type | Description |
|-----------|------|-------------|
| `%Reserved` | :pg.bool | Enum-level reserved flag |

### EnumField-Specific

| Attribute | Type | Description |
|-----------|------|-------------|
| `%Reserved` | :pg.bool | Field-level reserved (privileged!) |

### Error-Specific

| Attribute | Type | Description |
|-----------|------|-------------|
| `%Reserved` | :pg.bool | Error-level reserved flag |
| `%Message` | :pg.string | Error message template |
| `%TraceBack` | :array.pg.string | Stack trace info |
| `%Object` | :#PolyglotObject | Object that caused error |

### Variadic Input Configuration

| Attribute | Type | Description |
|-----------|------|-------------|
| `%InStream` | enum | Enables shorthand operator |
| `%Keys` | enum | Indexing scheme (numeric, named) |
| `%ItemType` | :pg.type | Type validation for items |

---

## Variadic Input via Metadata

### Enabling `<<<` Operator

```polyglot
{|} |Set.Intersection
[<] i<type:pg.type
[<] i<sets:serial                  // Dynamic array (type unknown at compile)
   [%] %InStream << #IO.Stream;TriplePull    // Enable <<< operator
   [%] %Keys << #IO.Stream;Keys;numeric      // Numeric indexing (0, 1, 2...)
   [%] %ItemType << $type                     // Item type for validation

[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~Recursive.Tournament.RandomPairs   // Tournament reduction
   [~] <array << $sets
   [~] >item1 >> $set1
   [~] >item2 >> $set2
   [r] $result << |Set.IntersectTwo
      [|] <set1 << $set1
      [|] <set2 << $set2
      [|] >intersection >> $result
   [v] *Into.Winner
   [*] >winner << $result

[|] >intersection << $result:set
{x}
```

**Usage:**
```polyglot
[r] |Set.Intersection
   <<< $set1                       // Appends to sets array (index 0)
   <<< $set2                       // Appends to sets array (index 1)
   <<< $set3                       // Appends to sets array (index 2)
   >>> $intersection               // Captures output
```

**How it works:**
1. `%InStream << #IO.Stream;TriplePull` enables `<<<` operator
2. `%Keys << #IO.Stream;Keys;numeric` means items indexed 0, 1, 2...
3. `%ItemType << $type` validates each item matches type
4. Compiler transforms `<<<` into array appends

---

## Reserved Indication Metadata

### Enum-Level Reserved

```polyglot
{#} #Boolean
[%] %Reserved << #Boolean.True     // Mark entire enum as reserved
[.] .True                          // Reserved field
[.] .False                         // Reserved field
{#}
```

**Compiler validation:**
- Only `@Polyglot::Core::*` packages can set `%Reserved` on enums
- User packages attempting to set `%Reserved`: **compile error**

### Field-Level Reserved

```polyglot
{#} #DT.Business
[%] %Reserved << #Boolean.True     // Enum is reserved

[.] .FiscalYear                    // Reserved field
   [%] %Reserved << #Boolean.True  // Field-level reserved (privileged!)

[.] ;CustomYear                    // Custom field extension (semicolon for custom)
{#}
```

**Privilege escalation:**
- `%Enumeration.Reserved` → enum-level
- `%EnumField.Reserved` → field-level (higher privilege, core devs only)

**See:** [Reserved Indication System](./reserved-indication-system.md) for complete details

---

## Metadata References

### Referencing Metadata

```polyglot
{@MyApp%PrivacyGroup.Authentication}    // Reference metadata value
@OrderProc!InvalidOrder                  // Reference error from registry
```

### Querying Metadata (Future)

**Planned feature for tooling:**
```polyglot
[r] $doc << |Meta.Get
   [|] <target << |MyPipeline
   [|] <property << "%Doc"
   [|] >value >> $documentation

[r] $authors << |Meta.Query
   [|] <registry << @MyApp
   [|] <filter << "%Author.name =? 'Alice'"
   [|] >results >> $alice_contributions
```

---

## Multi-line Metadata

### Multi-line Documentation

```polyglot
[%] %Doc <<
[+] +"This pipeline processes orders by:"
[+] +"1. Validating input"
[+] +"2. Enriching with customer data"
[+] +"3. Calculating totals"
[+] +""
[+] +"Returns enriched order object."
```

### Multi-line Error Messages

```polyglot
{!} !Validation.ComplexError
[%] %Message <<
[+] +"Validation failed due to multiple issues:"
[+] +"- {$error.field_name}: {$error.constraint}"
[+] +"- See documentation for valid ranges"
{!}
```

---

## Compiler & Tooling Integration

### Documentation Generation

**From metadata to docs:**
```bash
$ polyglot docs generate --registry @MyApp
```

**Generates:**
- API reference from `%Doc` attributes
- Author attribution from `%Author`
- Deprecation warnings
- Parameter documentation

### IDE Hover Information

**Hover on |FetchUser:**
```
Pipeline: FetchUser
Documentation: Fetches user data from database
Authors:
  - Alice <alice@example.com> (since 0.1.0)
  - Bob <bob@example.com> (since 0.1.4)
Privacy Group: Authentication
Errors:
  - Database.Timeout: Database unreachable
Parameters:
  - user_id (string): Unique user identifier
  - include_profile (bool, default: false): Include full profile data
Returns:
  - user (serial): User data with optional profile
```

### Deprecation Warnings

**Compile-time:**
```polyglot
[r] |OldPipeline                   // ⚠️  WARNING: OldPipeline deprecated
                                   //     Use |NewPipeline instead
                                   //     Auto-migration available
```

**Auto-migration flag:**
```bash
$ polyglot migrate --auto
```
Transforms code using `%Deprecated.auto_migrate` metadata.

---

## Standard Patterns

### Complete Package with Metadata

```polyglot
{@} @Local::OrderProcessing:1.0.0.0
[A] @OrderProc
[<] @PgTypes
[<] @OrderData << @Local::OrderDBOps:1.0.0.0

[%] %Doc <<
[+] +"Order processing module with"
[+] +"validation and error handling"

[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "1.0.0"

[%] %License << "MIT"
[%] %Repository << "https://github.com/myapp/orders"
{x}
```

### Complete Pipeline with Metadata

```polyglot
{|} |ProcessOrder
[%] %PublicTo << {@OrderProc}
[%] %PrivacyGroup << "OrderManagement"

[%] %Doc <<
[+] +"Processes order with validation,"
[+] +"enrichment, and error handling"

[%] %Errors
   [.] .error << @OrderProc!InvalidOrder
   [.] .when << "Order validation fails"

[%] %Errors
   [.] .error << @OrderProc!Database.Timeout
   [.] .when << "Database unreachable after retries"

[<] i<order_id:string
   [%] %description << "Unique order identifier"

[<] i<priority:string
   [%] %description << "Order priority: high, medium, or low"
   [%] %default << "medium"

[t] |T.Call
[W] |W.Polyglot.Scope

// ... pipeline implementation ...

[|] >order_record << $order_record:#OrderRecord
[|] >processing_type << $processing_type:string
{x}
```

---

## See Also

- [Reserved Indication](./reserved-indication-system.md) - `%Reserved` metadata usage
- [Variadic Input](../features/variadic-input.md) - `%InStream` configuration
- [Documentation Generation](../../tooling/doc-generation.md) - Using metadata for docs
- [Deprecation Strategy](../../patterns/deprecation.md) - Managing breaking changes

---

**Status:** ✅ Specification Complete - Implementation Ready
