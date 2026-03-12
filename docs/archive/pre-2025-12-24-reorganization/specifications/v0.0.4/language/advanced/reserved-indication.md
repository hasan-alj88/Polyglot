---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "reserved-indication"
shard: false

# --- Classification ---
type: spec
topic: "Polyglot v0.0.4 - Reserved Indication System"
summary: "Advanced: Polyglot v0.0.4 - Reserved Indication using . and ; separators"
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
updated: 2025-12-23
version: 0.0.4
tags:
  - "#advanced"
  - "#features"
---
# Polyglot v0.0.4 - Reserved Indication System

**Version:** v0.0.4
**Status:** ✅ Final
**Last Updated:** 2025-12-23

> **⚠️ BREAKING CHANGE FROM v0.0.3**
> v0.0.4 **reverses** the separator meanings:
> - v0.0.3: `;` = reserved, `.` = user-defined
> - v0.0.4: `.` = reserved, `;` = custom/user-defined

---

## Overview

Polyglot uses **two separators** (`.` and `;`) to distinguish reserved segments from custom/user-defined segments in enum and error hierarchies.

**Separator Semantics (v0.0.4):**
- **`.` (dot)** = RESERVED by Polyglot core
- **`;` (semicolon)** = CUSTOM/user-defined

**Applies to:**
- `#` Enum hierarchies
- `!` Error hierarchies

**Does NOT apply to:**
- `:` Type hierarchies (use `.` exclusively)
- Other prefixes (`$`, `|`, `%`, `@`)

---

## Syntax

### Fully Reserved

**All segments use `.` separator:**

```polyglot
#Boolean.True                      // Both segments reserved
#Boolean.False                     // Both segments reserved
!Network.HTTP.Timeout              // All three segments reserved
```

**Pattern:** `#ReservedRoot.ReservedSegment.ReservedNested`

### Fully User-Defined

**All segments use `.` separator (no semicolons):**

```polyglot
#OrderStatus.Pending               // User-defined enum
#OrderStatus.Processing            // User-defined enum
!Application.InvalidInput          // User-defined error
```

**Pattern:** `#UserRoot.UserSegment.UserNested`

**Note:** Both reserved and fully user-defined use `.` - the difference is determined by whether the root is in the reserved registry (enforced by `%Reserved` metadata).

### Mixed (Reserved + Custom)

**Use `;` to indicate custom extensions on reserved types:**

```polyglot
#DT.Business.Week;OurCompanyWeek
//^^^^^^^^^^^^^^^ ^^^^^^^^^^^^^^^^
//|               └─ CUSTOM week definition (semicolon prefix)
//└─ RESERVED namespace (dots)

!Network.HTTP;MyApp;RateLimited
//^^^^^^^^^^^ ^^^^^^^^^^^^^^^^^^^
//|            └─ CUSTOM error extensions (semicolons)
//└─ RESERVED root (dots)
```

**Pattern:** `#Reserved.Path;CustomExtension;MoreCustom`

---

## Rules

### 1. Separator Meanings (v0.0.4)

Each separator indicates the nature of the **following** segment:

- **`.` before segment** = That segment is RESERVED
- **`;` before segment** = That segment is CUSTOM/user-defined

```polyglot
#DT.Business.Week;OurCompanyWeek;RestDays
// ^^       ^^   ^^^^^^^^^^^^^^^^ ^^^^^^^^
// |        |    |                └─ CUSTOM (semicolon before)
// |        |    └─ CUSTOM (semicolon before)
// |        └─ RESERVED (dot before)
// └─ RESERVED (dot before)
```

### 2. Reserved Roots Must Use Dots

Reserved enumerations and errors must use `.` for all reserved segments:

```polyglot
#Boolean.True                      // ✅ Correct: Both reserved (dots)
#Boolean;True                      // ❌ ERROR: True marked as custom!
```

### 3. Extension Pattern

Custom extensions on reserved types use `;` to mark custom parts:

```polyglot
#HTTP.Status;Custom.RateLimited    // ✅ Extend reserved HTTP.Status with custom
#DT.Business.Week;OurCompany;RestDays  // ✅ Multiple custom extensions
```

### 4. Fully Custom Uses Dots Only

User-defined enums/errors with NO reserved components use `.` throughout:

```polyglot
#MyEnum.MyValue                    // ✅ Fully custom (registered as non-reserved)
#OrderStatus.Pending               // ✅ Fully custom
```

**Distinction:** The root (`MyEnum`, `OrderStatus`) is NOT in the reserved registry.

### 5. Privilege Enforcement

**Reserved segments require core dev privileges:**
- Marked via `%Reserved << #Boolean.True` metadata (see [Metadata System](./metadata-system.md))
- Compiler validates: only Polyglot core packages can define reserved roots
- User packages attempting to create reserved roots: **compile error**

---

## Standard Reserved Enums

### Boolean

```polyglot
#Boolean.True                      // Reserved boolean true
#Boolean.False                     // Reserved boolean false
```

**Convenience Aliases:**

For brevity, built-in aliases are provided:
- `#True` → `#Boolean.True`
- `#False` → `#Boolean.False`

Both full and alias forms are valid.

### Optional/Result

```polyglot
#Optional.None
#Optional.Some

#Result.Ok
#Result.Err
```

### Violation Actions

```polyglot
#Violation.Action.Raise
#Violation.Action.Clip
#Violation.Action.Transform
#Violation.Action.Default
#Violation.Action.Warn
#Violation.Action.Ignore
```

### I/O Streams

```polyglot
#IO.Stream.SinglePush
#IO.Stream.SinglePull
#IO.Stream.DoublePush
#IO.Stream.DoublePull
#IO.Stream.TriplePush              // Enables <<< operator
#IO.Stream.TriplePull              // Enables >>> operator

#IO.Stream.Keys.Numeric
#IO.Stream.Keys.Named
```

### Ordering

```polyglot
#Ordering.Less
#Ordering.Equal
#Ordering.Greater
```

### DateTime

```polyglot
#DT.Weekday.Monday
#DT.Weekday.Tuesday
#DT.Weekday.Wednesday
#DT.Weekday.Thursday
#DT.Weekday.Friday
#DT.Weekday.Saturday
#DT.Weekday.Sunday

#DT.Month.January
#DT.Month.February
#DT.Month.March
// ... etc
```

### HTTP

```polyglot
#HTTP.Method.GET
#HTTP.Method.POST
#HTTP.Method.PUT
#HTTP.Method.DELETE
#HTTP.Method.PATCH
#HTTP.Method.HEAD
#HTTP.Method.OPTIONS

#HTTP.Status.Ok                    // 200
#HTTP.Status.Created               // 201
#HTTP.Status.NotFound              // 404
#HTTP.Status.ServerError           // 500
// ... etc
```

---

## Standard Reserved Errors

### Network Errors

```polyglot
!Network.HTTP.Timeout
!Network.HTTP.ConnectionRefused
!Network.HTTP.DNSResolutionFailed
!Network.TCP.ConnectionReset
!Network.TLS.CertificateInvalid
```

### Validation Errors

```polyglot
!Validation.Schema.InvalidJSON
!Validation.Schema.MissingField
!Validation.Type.TypeMismatch
!Validation.Range.OutOfBounds
```

### System Errors

```polyglot
!System.IO.FileNotFound
!System.IO.PermissionDenied
!System.Memory.OutOfMemory
!System.Process.Timeout
```

---

## User Extensions

### Extending Reserved Enums

**Add custom week definition to DT.Business:**

```polyglot
{#} #DT.Business.Week;OurCompanyWeek
[.] .monday :DT.Weekday << #DT.Weekday.Monday
[.] .friday :DT.Weekday << #DT.Weekday.Friday
[.] .rest_days :pg.array.DT.Weekday << {#DT.Weekday.Saturday, #DT.Weekday.Sunday}
{x}

// Usage
[r] $week :pg.enum << #DT.Business.Week;OurCompanyWeek
[r] $start :DT.Weekday << $week.monday
```

**Extend HTTP.Status with custom statuses:**

```polyglot
{#} #HTTP.Status;Custom.RateLimited
[.] .retry_after :pg.int
[.] .limit :pg.int
{x}

// Usage
[f] $status =? #HTTP.Status;Custom.RateLimited
   [r] $wait << $status.retry_after
```

### Extending Reserved Errors

**Add application-specific network errors:**

```polyglot
{!} !Network.HTTP;MyApp.APIRateLimited
[.] .message :pg.string
[.] .retry_after :pg.int
{x}

// Usage
[z]
   [r] $result << |CallAPI
[!] !Network.HTTP;MyApp.APIRateLimited >> $error
   [r] $wait << $error.retry_after
   [r] |Sleep <seconds << $wait
   [r] $retry << |CallAPI          // Retry after wait
```

---

## Privilege Model

### Core Dev Privileges

**Only Polyglot core packages can:**
- Define reserved roots (marked with `%Reserved << #Boolean.True`)
- Create new reserved namespaces under reserved roots
- Extend reserved namespaces with new reserved segments

**Checked via:**
- Package source: Must be from `@Polyglot::Core::*` registry
- Metadata: `%Reserved << #Boolean.True` on enum/error definition
- Compiler validation: Prevents user packages from creating reserved roots

### User Privileges

**Users can:**
- Reference reserved enums/errors: `#Boolean.True`, `!Network.HTTP.Timeout`
- Extend with custom segments: `#HTTP.Status;Custom.RateLimited`
- Create fully user-defined hierarchies: `#MyEnum.MyValue`

**Users cannot:**
- Create reserved roots: `{#} #NewReserved.Value` with `%Reserved` → **compile error** (unless core package)
- Redefine existing reserved enums: `{#} #Boolean.True` → **compile error**

---

## Validation & Errors

### Compile-Time Checks

**1. Reserved Root Redefinition**

```polyglot
{#} #Boolean.True                  // ❌ ERROR: Cannot redefine reserved root
```

**Error:** `ReservedEnumRedefinition: #Boolean.True is reserved by Polyglot core`

**2. Unauthorized Reserved Creation**

```polyglot
// In user package @MyApp::Utils
{#} #NewReserved.Value
[%] %Reserved << #Boolean.True     // ❌ ERROR: Not core package
```

**Error:** `UnauthorizedReservedDefinition: Only core packages can define reserved enums`

**3. Semicolon on Fully Reserved**

```polyglot
#Boolean;True                      // ❌ ERROR: Marking True as custom when it's reserved!
```

**Error:** `InvalidReservedSyntax: #Boolean.True is reserved, cannot use ; separator`

### Runtime Checks

**None required** - all validation happens at compile-time.

---

## IDE & Tooling Support

### Syntax Highlighting

- **Reserved segments (before `.`):** Different color (e.g., purple)
- **Custom segments (after `;`):** Standard color (e.g., blue)
- **Mixed paths:** Gradient or color change at `;` boundary

### Autocomplete

**When typing `#B`:**
- Show reserved enums like `#Boolean.`, `#Business.`
- Show user enums matching pattern

**When typing `#HTTP.Status;`:**
- Show custom extensions of HTTP.Status
- Mark as user-defined

### Hover Information

**On `#Boolean.True`:**
```
Reserved Enum: Boolean.True
Package: @Polyglot::Core::Types:1.0.0.0
Privilege: Core Reserved
Documentation: Represents true boolean value
Alias: #True
```

**On `#HTTP.Status;Custom.RateLimited`:**
```
Extended Enum: HTTP.Status;Custom.RateLimited
Base: #HTTP.Status (reserved)
Extension: Custom.RateLimited (user-defined)
Package: @MyApp::HTTP:2.0.0.0
```

---

## Migration from v0.0.3

> **🚨 BREAKING CHANGE:** v0.0.4 reverses the separator semantics!

### v0.0.3 Syntax (OLD)

```polyglot
#Boolean.True                     // ✅ v0.0.3: Both reserved (semicolons)
#OrderStatus.Processing            // ✅ v0.0.3: User-defined (dots)
#HTTP.Status.Custom.RateLimited   // ✅ v0.0.3: Mixed (semicolons=reserved, dots=custom)
```

### v0.0.4 Syntax (NEW)

```polyglot
#Boolean.True                      // ✅ v0.0.4: Both reserved (dots)
#OrderStatus.Processing            // ✅ v0.0.4: User-defined (dots)
#HTTP.Status;Custom.RateLimited    // ✅ v0.0.4: Mixed (dots=reserved, semicolons=custom)
```

### Automated Conversion

**Step 1: Reserved enums (identified by core package or %Reserved metadata):**

```regex
# Convert reserved enums from v0.0.3 to v0.0.4
FIND:    #;([A-Z]\w*);
REPLACE: #$1.
```

**Example:**
- `#Boolean.True` → `#Boolean.True`
- `#HTTP.Method;GET` → `#HTTP.Method.GET`

**Step 2: Mixed hierarchies - flip separators:**

```regex
# In v0.0.3: #;Reserved;Path.Custom.Extension
# In v0.0.4: #Reserved.Path;Custom.Extension
# Manual review required for mixed cases
```

**Step 3: User enums:**

```
No change needed - user-defined enums already use . separator
```

---

## Examples

### Example 1: Fully Reserved

```polyglot
[r] $flag :pg.bool << #Boolean.True       // Reserved boolean
[r] $status :pg.enum << #HTTP.Status.Ok   // Reserved HTTP status
```

### Example 2: Custom Extension

```polyglot
// Define custom business week
{#} #DT.Business.Week;FourDayWeek
[.] .start :DT.Weekday << #DT.Weekday.Monday
[.] .end :DT.Weekday << #DT.Weekday.Thursday
{x}

// Use it
[r] $week :pg.enum << #DT.Business.Week;FourDayWeek
```

### Example 3: Fully Custom

```polyglot
// User-defined order status
{#} #OrderStatus.Processing
[.] .started_at :pg.datetime
[.] .progress :pg.float
{x}

[r] $status :pg.enum << #OrderStatus.Processing
```

---

## See Also

- [Metadata System](./metadata-system.md) - `%Reserved` metadata property
- [Enum System](../types/enums-serial.md) - Enum definitions and usage
- [Error Handling](../control-flow/error-handling.md) - Error definitions and propagation
- [Prefix System](../syntax/prefix-system.md) - All prefix operators

---

**Status:** ✅ Specification Complete - v0.0.4 Syntax
**Migration Status:** ⚠️ Documentation update in progress from v0.0.3 syntax
