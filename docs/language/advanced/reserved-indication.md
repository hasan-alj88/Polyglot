---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "reserved-indication"
shard: false

# --- Classification ---
type: spec
topic: "Polyglot v0.0.4 - Reserved Indication Sy"
summary: "Advanced: Polyglot v0.0.4 - Reserved Indicati"
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
# Polyglot v0.0.4 - Reserved Indication System

**Version:** v0.0.4
**Status:** ✅ Final
**Last Updated:** 2025-12-14

---

## Overview

Polyglot uses **semicolon (`;`)** to mark reserved segments in enum and error hierarchies, enabling granular control over which parts are reserved by Polyglot core vs. user-defined.

**Applies to:**
- `#` Enum hierarchies
- `!` Error hierarchies

**Does NOT apply to:**
- `:` Type hierarchies (deferred, may change in future)
- Other prefixes (use `.` for all hierarchy)

---

## Syntax

### Fully Reserved

```polyglot
#;Boolean;True                     // Both segments reserved
#;Boolean;False                    // Both segments reserved
!;Network;HTTP;Timeout             // All three segments reserved
```

**Pattern:** All hierarchy segments use `;` separator

### Fully User-Defined

```polyglot
#OrderStatus.Pending               // User-defined enum
#OrderStatus.Processing            // User-defined enum
!Application.InvalidInput          // User-defined error
```

**Pattern:** All hierarchy segments use `.` separator

### Mixed (Reserved + User)

```polyglot
#;DT;Business.FiscalYear.Q1
//^^ ^^^^^^^^ ^^^^^^^^^^^ ^^
//|  |        |           └─ User-defined field
//|  |        └─ User-defined namespace
//|  └─ Reserved namespace
//└─ Reserved root

!;Network;HTTP.Custom.RateLimited
//^^^^^^^  ^^^^ ^^^^^^^^^^^^^^^^
//|        |    └─ User-defined error types
//|        └─ User-defined category
//└─ Reserved root and domain
```

**Pattern:** Mix `;` (reserved) and `.` (user) to create extensions

---

## Rules

### 1. Segment-Level Indication

Each hierarchy segment is independently marked:
- `;` = Reserved (Polyglot core only)
- `.` = User-defined (anyone can create)

```polyglot
#;Reserved;Segment.UserSegment.AnotherUser
```

### 2. Consistent Separators

Cannot mix separators arbitrarily:
```polyglot
#;Boolean.True                     // ❌ ERROR: Mixed without reason
#;Boolean;True                     // ✅ Correct: Both reserved
#Boolean.Custom                    // ✅ Correct: Both user-defined
```

### 3. Extension Pattern

Reserved namespaces can be extended with user segments:
```polyglot
#;HTTP;Status.Custom.MyStatus      // ✅ Extend reserved HTTP.Status
!;Network.MyApp.CustomError        // ✅ Extend reserved Network
```

### 4. Privilege Enforcement

**Reserved segments require core dev privileges:**
- Marked via `%Enumeration.Reserved` metadata (see [Metadata System](./metadata-system.md))
- Compiler validates: only Polyglot core packages can define reserved segments
- User packages attempting to define reserved segments: **compile error**

---

## Standard Reserved Enums

### Boolean

```polyglot
#;Boolean;True
#;Boolean;False
```

**Convenience Aliases:**
For brevity, built-in aliases are provided:
- `#True` → `#;Boolean;True`
- `#False` → `#;Boolean;False`

Both full and short forms are valid. See [Reserved Boolean Aliases](../types/enums-serial.md#reserved-boolean-aliases).

### Optional/Result

```polyglot
#;Optional;None
#;Optional;Some

#;Result;Ok
#;Result;Err
```

### Violation Actions

```polyglot
#;Violation;Action;Raise
#;Violation;Action;Clip
#;Violation;Action;Transform
#;Violation;Action;Default
#;Violation;Action;Warn
#;Violation;Action;Ignore
```

### I/O Streams

```polyglot
#;IO;Stream;SinglePush
#;IO;Stream;SinglePull
#;IO;Stream;DoublePush
#;IO;Stream;DoublePull
#;IO;Stream;TriplePush              // Enables <<< operator
#;IO;Stream;TriplePull              // Enables >>> operator

#;IO;Stream;Keys;Numeric
#;IO;Stream;Keys;Named
```

### Ordering

```polyglot
#;Ordering;Less
#;Ordering;Equal
#;Ordering;Greater
```

### DateTime

```polyglot
#;DT;Weekday;Monday
#;DT;Weekday;Tuesday
#;DT;Weekday;Wednesday
#;DT;Weekday;Thursday
#;DT;Weekday;Friday
#;DT;Weekday;Saturday
#;DT;Weekday;Sunday

#;DT;Month;January
#;DT;Month;February
#;DT;Month;March
// ... etc
```

### HTTP

```polyglot
#;HTTP;Method;GET
#;HTTP;Method;POST
#;HTTP;Method;PUT
#;HTTP;Method;DELETE
#;HTTP;Method;PATCH
#;HTTP;Method;HEAD
#;HTTP;Method;OPTIONS

#;HTTP;Status;Ok               // 200
#;HTTP;Status;Created          // 201
#;HTTP;Status;NotFound         // 404
#;HTTP;Status;ServerError      // 500
// ... etc
```

---

## Standard Reserved Errors

### Network Errors

```polyglot
!;Network;HTTP;Timeout
!;Network;HTTP;ConnectionRefused
!;Network;HTTP;DNSResolutionFailed
!;Network;TCP;ConnectionReset
!;Network;TLS;CertificateInvalid
```

### Validation Errors

```polyglot
!;Validation;Schema;InvalidJSON
!;Validation;Schema;MissingField
!;Validation;Type;TypeMismatch
!;Validation;Range;OutOfBounds
```

### System Errors

```polyglot
!;System;IO;FileNotFound
!;System;IO;PermissionDenied
!;System;Memory;OutOfMemory
!;System;Process;Timeout
```

---

## User Extensions

### Extending Reserved Enums

**Add custom statuses to HTTP:**
```polyglot
{#} #;HTTP;Status.Custom.RateLimited
[.] .retry_after:int
[.] .limit:int
{#}

// Usage
[y] $status =? #;HTTP;Status.Custom.RateLimited
   [r] $wait << $status.retry_after
```

**Extend datetime with business logic:**
```polyglot
{#} #;DT;Business.FiscalYear
[.] .Q1
[.] .Q2
[.] .Q3
[.] .Q4
{#}

// Mixed reserved/user navigation
#;DT;Business.FiscalYear.Q1
//^^  ^^^^^^^^ ^^^^^^^^^^
//|   |        └─ User-defined
//|   └─ User-defined namespace under reserved DT
//└─ Reserved root
```

### Extending Reserved Errors

**Add application-specific network errors:**
```polyglot
{!} !;Network.MyApp.APIRateLimited
[.] .message:string
[.] .retry_after:int
{!}

// Usage
[z]
   [r] $result << |CallAPI
[!] !;Network.MyApp.APIRateLimited >> $error
   [r] $wait << $error.retry_after
   [r] |Sleep <seconds << $wait
   [r] $retry << |CallAPI          // Retry after wait
```

---

## Privilege Model

### Core Dev Privileges

**Only Polyglot core packages can:**
- Define segments with `;` separator
- Create new reserved roots (`#;NewRoot`)
- Extend reserved namespaces with new reserved segments

**Checked via:**
- Package source: Must be from `@Polyglot::Core::*` registry
- Metadata: `%Enumeration.Reserved` or `%Error.Reserved` set to `#;Boolean;True`
- Compiler validation: Prevents user packages from using `;` for new definitions

### User Privileges

**Users can:**
- Reference reserved enums/errors: `#;Boolean;True`
- Extend with user segments: `#;HTTP;Status.Custom.MyStatus`
- Create fully user-defined hierarchies: `#MyEnum.MyValue`

**Users cannot:**
- Redefine reserved segments: `{#} #;Boolean;True` → **compile error**
- Create new reserved roots: `{#} #;MyRoot;Value` → **compile error** (unless core package)

---

## Validation & Errors

### Compile-Time Checks

**1. Reserved Redefinition**
```polyglot
{#} #;Boolean;True                 // ❌ ERROR: Cannot redefine reserved
```
**Error:** `ReservedEnumRedefinition: #;Boolean;True is reserved by Polyglot core`

**2. Unauthorized Reserved Creation**
```polyglot
// In user package @MyApp::Utils
{#} #;NewReserved;Value            // ❌ ERROR: Not core package
```
**Error:** `UnauthorizedReservedDefinition: Only core packages can define reserved segments`

**3. Inconsistent Separators**
```polyglot
#;Boolean.True                     // ⚠️  WARNING: Inconsistent separators
```
**Warning:** `InconsistentSeparators: Mixing ; and . without clear extension pattern`

### Runtime Checks

**None required** - all validation happens at compile-time.

---

## IDE & Tooling Support

### Syntax Highlighting

- **Reserved segments (`;`):** Different color (e.g., purple)
- **User segments (`.`):** Standard color (e.g., blue)
- **Mixed paths:** Gradient or alternating colors

### Autocomplete

**When typing `#;`:**
- Show only reserved enums from core
- Filter by privilege level

**When typing `#;HTTP;Status.`:**
- Show user extensions of HTTP.Status
- Mark reserved vs. user-defined

### Hover Information

**On `#;Boolean;True`:**
```
Reserved Enum: Boolean.True
Package: @Polyglot::Core::Types:1.0.0.0
Privilege: Core
Documentation: Represents true boolean value
```

**On `#;HTTP;Status.Custom.RateLimited`:**
```
Extended Enum: HTTP.Status.Custom.RateLimited
Base: #;HTTP;Status (reserved)
Extension: Custom.RateLimited (user-defined)
Package: @MyApp::HTTP:2.0.0.0
```

---

## Migration from v0.0.3

### v0.0.3 Syntax (No Indication)

```polyglot
#Boolean.True                      // Ambiguous: reserved or user?
#OrderStatus.Processing            // Ambiguous: reserved or user?
```

### v0.0.4 Syntax (Clear Indication)

```polyglot
#;Boolean;True                     // Clear: reserved
#OrderStatus.Processing            // Clear: user-defined
```

### Automated Conversion

**Reserved enums (core packages only):**
```regex
FIND:    #(Boolean|Optional|Result|Ordering|HTTP|DT)\.
REPLACE: #;\1;
```

**User enums (all other packages):**
```
No change needed - already use `.` separator
```

---

## Future Considerations

### Type Indication (Deferred)

**Current:** Types use `.` exclusively
```polyglot
:pg.int                            // No reserved indication
:pg.array.pg.string               // No reserved indication
```

**Possible future:** Apply `;` to core types
```polyglot
:;pg;int                           // Reserved core type
:;pg;array.:;pg;string            // Reserved collection of reserved type
```

**Status:** Deferred - types are subject to larger redesign

### Error Taxonomy

Reserved error hierarchy could be standardized:
```polyglot
!;Category;Domain;Specific         // 3-level taxonomy
!;Network;HTTP;Timeout             // Category.Domain.Specific
```

**Benefit:** Consistent error handling and documentation

---

## See Also

- [Metadata System](./metadata-system.md) - `%Reserved` metadata property
- [Enum System](../core-syntax/enums.md) - Enum definitions and usage
- [Error Handling](./error-handling.md) - Error definitions and propagation
- [Prefix System](../language/syntax/prefix-system.md) - All prefix operators

---

**Status:** ✅ Specification Complete - Implementation Ready
