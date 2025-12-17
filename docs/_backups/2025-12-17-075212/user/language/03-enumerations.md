# Enumerations

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Enumerations are immutable data structures with fixed schemas defined at compile-time. They provide strong type safety and are a fundamental part of Polyglot's data modeling system.

**Key Characteristics:**
- Immutable (cannot change after definition)
- Fixed schema (all fields known at compile-time)
- Strong type safety
- Hierarchical dot notation
- Support for aliases

---

## Table of Contents

1. [Enumeration Basics](#enumeration-basics)
2. [Definition Syntax](#definition-syntax)
3. [Field Definitions](#field-definitions)
4. [Hierarchical Naming](#hierarchical-naming)
5. [Alias Support](#alias-support)
6. [Regular Enumerations](#regular-enumerations)
7. [Reserved Enumerations](#reserved-enumerations)
8. [Extendable Enumerations](#extendable-enumerations)
9. [Enumeration vs Serial vs Error](#enumeration-vs-serial-vs-error)
10. [Usage Patterns](#usage-patterns)
11. [Best Practices](#best-practices)

---

## Enumeration Basics

### What is an Enumeration?

An **enumeration** is an immutable data structure with a fixed set of fields, similar to:
- Structs in C/Rust
- Records in functional languages
- Immutable objects in OOP
- Constants/configuration objects

**Unlike `:pg.serial`:**
- Schema is fixed at compile-time (not runtime)
- Cannot add or remove fields after definition
- Provides stronger type safety

---

### Why Use Enumerations?

**Use enumerations when:**
- Schema is known at compile-time
- You need strong type safety
- Data represents configuration or constants
- You want compile-time validation
- Fields should never change

**Don't use enumerations when:**
- Schema changes at runtime
- Parsing JSON from external APIs
- Need dynamic keys
- Working with user-provided data structures

---

## Definition Syntax

### Basic Definition

**Block Markers:**
- `[#]` - Start enumeration definition
- `[X]` - End enumeration definition

**Syntax:**
```polyglot
[#] EnumerationName
[<] .field1: type << value
[<] .field2: type << value
[X]
```

---

### Simple Example

```polyglot
[#] Configuration
[<] .host:pg.string << "localhost"
[<] .port:pg.int << 8080
[<] .debug:pg.bool << #False
[X]
```

**Usage:**
```polyglot
// Reference entire enumeration
[r] .config: #Configuration << #Configuration

// Access specific field
[r] .host:pg.string << #Configuration.host
[r] .port:pg.int << #Configuration.port
```

---

## Field Definitions

### Field Syntax

**Format:**
```polyglot
[<] .field_name: type << value
```

**Components:**
- `[<]` - Field definition marker
- `.field_name` - Field name (starts with dot)
- `: type` - Field type
- `<< value` - Field value (required)

---

### Field Types

All Polyglot types can be used as field types:

```polyglot
[#] AllTypes
// Primitives
[<] .integer:pg.int << 42
[<] .unsigned:pg.uint << 100
[<] .float:pg.float << 3.14
[<] .text:pg.string << "example"
[<] .flag:pg.bool << #True

// Special types
[<] .file_path:pg.path << \\DataDir\\file.txt
[<] .timestamp:pg.dt << |DT"2024-01-15 14:30:"

// Collections
[<] .items: pg.array.pg.string << array{"a", "b", "c"}
[<] .unique:pg.set{pg\int} << set{1, 2, 3}

// Complex types
[<] .data:pg.serial << serial{
[^]  "key": "value"
[^]}
[X]
```

---

### Required Values

**All fields MUST have values:**

```polyglot
// ✓ CORRECT - Field with value
[#] Valid
[<] .field:pg.string << "value"
[X]

// ✗ ERROR - Field without value
[#] Invalid
[<] .field:pg.string  // Missing << value
[X]
```

---

### Nested Enumerations

Enumerations can reference other enumerations:

```polyglot
// Define database configuration
[#] DatabaseConfig
[<] .host:pg.string << "db.example.com"
[<] .port:pg.int << 5432
[<] .ssl:pg.bool << #True
[X]

// Define cache configuration
[#] CacheConfig
[<] .enabled:pg.bool << #True
[<] .ttl:pg.int << 3600
[X]

// Combine into application config
[#] AppConfig
[<] .database: #DatabaseConfig << #DatabaseConfig
[<] .cache: #CacheConfig << #CacheConfig
[<] .app_name:pg.string << "MyApp"
[X]
```

**Access nested fields:**
```polyglot
[r] .db_host:pg.string << #AppConfig.database.host
[r] .cache_ttl:pg.int << #AppConfig.cache.ttl
```

---

## Hierarchical Naming

### Dot Notation for Namespacing

Enumerations support hierarchical names using dots:

```polyglot
[#] MyApp.Database.Configuration
[<] .host:pg.string << "localhost"
[<] .port:pg.int << 5432
[X]

[#] MyApp.Cache.Configuration
[<] .host:pg.string << "cache.local"
[<] .port:pg.int << 6379
[X]

[#] MyApp.API.Configuration
[<] .base_url:pg.string << "https://api.example.com"
[<] .timeout:pg.int << 30
[X]
```

---

### Naming Conventions

**Recommended structure:**
```
Package.Context.Name
```

**Examples:**
```polyglot
[#] MyApp.Configuration.Database
[#] MyApp.Configuration.Cache
[#] MyApp.Configuration.API

[#] DataProcessing.Validation.Rules
[#] DataProcessing.Transform.Settings

[#] UI.Theme.Colors
[#] UI.Theme.Fonts
```

---

### Access with Full Path

```polyglot
// Access using full hierarchical name
[r] .db_config: #MyApp.Database.Configuration << #MyApp.Database.Configuration

// Access field
[r] .host:pg.string << #MyApp.Database.Configuration.host
```

---

## Alias Support

### Defining Aliases

**Purpose:** Create shorter names for enumerations within the same package

**Block Marker:** `[A]` - Alias definition

**Syntax:**
```polyglot
[#] Very.Long.Hierarchical.Name
[A] ShortName
[<] .field: type << value
[X]
```

---

### Alias Example

```polyglot
[#] MyApp.Configuration.Database
[A] DBConfig  // Alias
[<] .host:pg.string << "localhost"
[<] .port:pg.int << 5432
[X]
```

**Usage with alias:**
```polyglot
// Use full name
[r] .config1: #MyApp.Configuration.Database << #MyApp.Configuration.Database

// Use alias (package-scoped only)
[r] .config2: #DBConfig << #DBConfig

// Access fields with alias
[r] .host:pg.string << #DBConfig.host
```

---

### Alias Scope

**Important:** Aliases are **package-scoped only**

- Alias can only be used within the same package where it's defined
- Other packages must use full name
- Helps avoid naming conflicts across packages

```polyglot
// Package: MyApp
[#] MyApp.Configuration.Database
[A] DBConfig  // Only usable in MyApp package
[<] .host:pg.string << "localhost"
[X]

// In MyApp package - ✓ OK
[r] .config: #DBConfig << #DBConfig

// In OtherPackage - ✗ ERROR - Must use full name
[r] .config: #MyApp.Configuration.Database << #MyApp.Configuration.Database
```

---

## Regular Enumerations

### User-Defined Enumerations

**Regular enumerations** are defined entirely by users with full control over schema.

**Characteristics:**
- User defines all fields
- User controls all values
- Not extendable
- Complete flexibility

---

### Regular Enumeration Examples

**Application Configuration:**
```polyglot
[#] AppConfig
[<] .version:pg.string << "1.0.0"
[<] .environment:pg.string << "production"
[<] .max_connections:pg.int << 100
[X]
```

**Color Palette:**
```polyglot
[#] UI.Colors
[<] .primary:pg.string << "#3498db"
[<] .secondary:pg.string << "#2ecc71"
[<] .danger:pg.string << "#e74c3c"
[<] .warning:pg.string << "#f39c12"
[X]
```

**API Endpoints:**
```polyglot
[#] API.Endpoints
[<] .users:pg.string << "/api/v1/users"
[<] .posts:pg.string << "/api/v1/posts"
[<] .comments:pg.string << "/api/v1/comments"
[X]
```

---

## Reserved Enumerations

### What are Reserved Enumerations?

**Reserved enumerations** are system-defined enumerations provided by Polyglot with fixed schemas.

**Characteristics:**
- Defined by the Polyglot language (not users)
- Schema is predetermined and documented
- May be extendable or non-extendable
- Provide system functionality

---

### Types of Reserved Enumerations

**1. Non-Extendable Reserved:**
- Users cannot add new variants
- Schema is completely fixed
- Examples: `#Status.*`, `#None`

**2. Extendable Reserved:**
- Users can add new variants with `.*` suffix
- Must follow fixed schema defined by Polyglot
- Examples: `#Path.Identifiers.*`, `#Queues.*`

---

### Reserved Enumeration Examples

**Non-Extendable: `#Status.*`**
```polyglot
// Provided by Polyglot - users cannot extend
#Status.Success
#Status.Failed
#Status.Pending
#Status.Running
#Status.Cancelled

// Usage
[r] .result: #Status << #Status.Success
```

**Non-Extendable: `#None`**
```polyglot
// Represents "no value"
[o] #None  // Pipeline returns no output
```

---

### Extendable Reserved Enumerations

**Extendable reserved enumerations** have the `.*` suffix indicating users can add new variants.

**Examples:**
- `#Path.Identifiers.*` - Cross-platform path definitions
- `#Queues.*` - Custom queue definitions
- `#DT.Business.Week.*` - Business week definitions

---

## Extendable Enumerations

### Extending Reserved Enumerations

**Syntax:**
```polyglot
[#] ReservedEnumeration.{CustomName}
[A] AliasName
[<] .required_field1: type << value
[<] .required_field2: type << value
// Must follow schema defined by Polyglot
[X]
```

---

### Example: Extending `#Path.Identifiers.*`

**Schema (defined by Polyglot):**
- `.unix:pg.path` - Unix/Linux/macOS path (REQUIRED)
- `.windows:pg.path` - Windows path (REQUIRED)

**User Extension:**
```polyglot
[#] Path.Identifiers.MyApp.DataDirectory
[A] DataDir
[<] .unix:pg.path << \\UnixRoot\\opt\myapp\data\
[<] .windows:pg.path << \\C\\ProgramData\MyApp\Data\
[X]
```

**Usage:**
```polyglot
// Use with alias
[r] .data_file:pg.path << \\DataDir\\records.csv

// Use with full name
[r] .config_file:pg.path << \\Path.Identifiers.MyApp.DataDirectory\\config.json
```

---

### Example: Extending `#Queues.*`

**Schema (defined by Polyglot - pending confirmation):**
- `.max_concurrent:pg.int` - Maximum concurrent executions (REQUIRED)
- Other fields TBD

**User Extension:**
```polyglot
[#] Queues.Background
[<] .max_concurrent:pg.int << 5
[X]

[#] Queues.HighPriority
[<] .max_concurrent:pg.int << 10
[X]
```

**Usage:**
```polyglot
[Q] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.Background
```

---

### Rules for Extending

**Must follow reserved schema:**
```polyglot
// ✓ CORRECT - Follows #Path.Identifiers.* schema
[#] Path.Identifiers.MyApp.LogDir
[<] .unix:pg.path << \\UnixRoot\\var\log\myapp\
[<] .windows:pg.path << \\C\\Logs\MyApp\
[X]

// ✗ ERROR - Missing required .windows field
[#] Path.Identifiers.MyApp.Invalid
[<] .unix:pg.path << \\UnixRoot\\var\log\myapp\
// Missing .windows field
[X]

// ✗ ERROR - Wrong field types
[#] Path.Identifiers.MyApp.WrongTypes
[<] .unix:pg.string << "/var/log"  // Should be pg\path
[<] .windows:pg.string << "C:\\Logs"  // Should be pg\path
[X]
```

---

### Special Path Value: `\\NoPath\\`

When a path doesn't exist on a particular OS, use `\\NoPath\\`:

```polyglot
[#] Path.Identifiers.WindowsOnly.Registry
[<] .unix:pg.path << \\NoPath\\
[<] .windows:pg.path << \\HKEY_LOCAL_MACHINE\\Software\
[X]

[#] Path.Identifiers.UnixOnly.DevNull
[<] .unix:pg.path << \\UnixRoot\\dev\null
[<] .windows:pg.path << \\NoPath\\
[X]
```

---

## Enumeration vs Serial vs Error

### Complete Comparison

| Feature | Enumeration (`#Enum`) | Serial (`:pg.serial`) | Error (`!Error`) |
|---------|----------------------|---------------------|------------------|
| **Schema** | Immutable (compile-time) | Mutable (runtime) | Immutable (compile-time) |
| **Definition** | `[#]...[X]` blocks | Inline literals | `[!]...[X]` blocks |
| **Type safety** | Strong | Weaker | Strong |
| **Fields** | Fixed at compile-time | Can change at runtime | 3 reserved + custom |
| **Access** | `#Enum.field` | `.serial["key"]` | `!Error.field` |
| **Use case** | Configuration, constants | JSON data, dynamic | Error handling |
| **Extendable** | Some (reserved only) | N/A (always dynamic) | No (define new types) |
| **Nesting** | Yes | Yes | Yes (custom fields) |
| **Marker** | `#` | N/A (type) | `!` |

---

### When to Use What

**Use Enumerations (`#Enum`) when:**
```polyglot
// Configuration with known schema
[#] DatabaseConfig
[<] .host:pg.string << "db.example.com"
[<] .port:pg.int << 5432
[X]

// Constants
[#] ErrorCodes
[<] .not_found:pg.int << 404
[<] .unauthorized:pg.int << 401
[<] .server_error:pg.int << 500
[X]

// Fixed data structures
[#] Dimensions
[<] .width:pg.int << 1920
[<] .height:pg.int << 1080
[X]
```

---

**Use Serial (`:pg.serial`) when:**
```polyglot
// JSON from API
[r] |HTTP.Get
[<] .url:pg.string << "https://api.example.com/data"
[>] .response:pg.serial >> api_data

// Dynamic data
[r] .user_data:pg.serial << serial{
[^]  "name": "Alice",
[^]  "age": 30,
[^]  "email": "alice@example.com"
[^]}

// Runtime-determined keys
[r] .config:pg.serial << serial{}
[r] .config["dynamic_key"] << "dynamic_value"
```

---

**Use Errors (`!Error`) when:**
```polyglot
// Custom error definitions
[!] !MyApp.ValidationError
[<] .message:pg.string << "Validation failed"
[<] .code:pg.int << 4000
[<] .trace:pg.string << ""
[<] .field:pg.string << ""
[X]

// Error catching
[r] |MightFail
[!] !pg.FileSystem.NotFound
[>] .message:pg.string >> err_msg
```

---

## Usage Patterns

### Pattern 1: Application Configuration

```polyglot
[#] App.Config
[<] .name:pg.string << "MyApplication"
[<] .version:pg.string << "1.0.0"
[<] .environment:pg.string << "production"
[X]

[#] App.Database
[<] .host:pg.string << "localhost"
[<] .port:pg.int << 5432
[<] .name:pg.string << "myapp_db"
[X]

// Use in pipeline
[|] Initialize
[r] |ConnectDatabase
[<] .host:pg.string << #App.Database.host
[<] .port:pg.int << #App.Database.port
[<] .database:pg.string << #App.Database.name
[X]
```

---

### Pattern 2: Feature Flags

```polyglot
[#] Features
[<] .new_ui:pg.bool << #True
[<] .beta_features:pg.bool << #False
[<] .debug_mode:pg.bool << #False
[<] .analytics:pg.bool << #True
[X]

// Use in conditional logic
[|] RenderUI
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] #Features.new_ui =? #Boolean.True
[~][r] |RenderNewUI
[~][o] #None

[?] #Features.new_ui =? #Boolean.False
[~][r] |RenderOldUI
[~][o] #None

[X]
```

---

### Pattern 3: API Endpoints

```polyglot
[#] API.Endpoints.V1
[A] API
[<] .base:pg.string << "https://api.example.com/v1"
[<] .users:pg.string << "/users"
[<] .posts:pg.string << "/posts"
[<] .comments:pg.string << "/comments"
[X]

// Build full URL
[|] FetchUsers
[r] |U.String.Concat
[<] .parts: pg.array.pg.string << array{
[^]  #API.base,
[^]  #API.users
[^]}
[>] .url:pg.string >> full_url

[r] |HTTP.Get
[<] .url:pg.string << full_url
[>] .response:pg.serial >> users_data
[X]
```

---

### Pattern 4: Cross-Platform Paths

```polyglot
// Define path identifiers
[#] Path.Identifiers.MyApp.Logs
[A] LogDir
[<] .unix:pg.path << \\UnixRoot\\var\log\myapp\
[<] .windows:pg.path << \\C\\Logs\MyApp\
[X]

[#] Path.Identifiers.MyApp.Data
[A] DataDir
[<] .unix:pg.path << \\UnixRoot\\opt\myapp\data\
[<] .windows:pg.path << \\C\\ProgramData\MyApp\Data\
[X]

// Use in pipeline
[|] ProcessLogs
[r] |ReadFile
[<] .path:pg.path << \\LogDir\\app.log

[r] |SaveData
[<] .path:pg.path << \\DataDir\\output.csv
[X]
```

---

### Pattern 5: Validation Rules

```polyglot
[#] Validation.User
[<] .min_username_length:pg.int << 3
[<] .max_username_length:pg.int << 20
[<] .min_password_length:pg.int << 8
[<] .require_email:pg.bool << #True
[X]

// Use in validation pipeline
[|] ValidateUser
[i] .username:pg.string
[i] .password:pg.string

// Check username length
[?] .username.length < #Validation.User.min_username_length
[~][r] |ThrowError
[~][<] .message:pg.string << "Username too short"

// Check password length
[?] .password.length < #Validation.User.min_password_length
[~][r] |ThrowError
[~][<] .message:pg.string << "Password too short"
[X]
```

---

## Best Practices

### 1. Use Hierarchical Names

**Good:**
```polyglot
[#] MyApp.Database.Configuration
[#] MyApp.Cache.Configuration
[#] MyApp.API.Configuration
```

**Avoid:**
```polyglot
[#] DBConfig
[#] CacheConfig
[#] APIConfig
```

---

### 2. Provide Meaningful Aliases

**Good:**
```polyglot
[#] MyApplication.Configuration.Database
[A] DBConfig  // Clear, concise
```

**Avoid:**
```polyglot
[#] MyApplication.Configuration.Database
[A] X  // Too short, unclear
[A] MyApplicationConfigurationDatabaseAlias  // Too long
```

---

### 3. Group Related Configuration

**Good:**
```polyglot
[#] App.Database
[<] .host:pg.string << "localhost"
[<] .port:pg.int << 5432
[<] .name:pg.string << "myapp_db"
[<] .ssl:pg.bool << #True
[X]
```

**Avoid:**
```polyglot
// Scattered configuration
[#] DatabaseHost
[<] .value:pg.string << "localhost"
[X]

[#] DatabasePort
[<] .value:pg.int << 5432
[X]
```

---

### 4. Use Appropriate Types

**Good:**
```polyglot
[#] Config
[<] .port:pg.int << 8080  // Integer for port
[<] .enabled:pg.bool << #True  // Boolean for flag
[<] .timeout:pg.int << 30  // Integer for seconds
[X]
```

**Avoid:**
```polyglot
[#] Config
[<] .port:pg.string << "8080"  // Wrong type
[<] .enabled:pg.string << "true"  // Should be boolean
[<] .timeout:pg.string << "30"  // Should be integer
[X]
```

---

### 5. Document Complex Enumerations

```polyglot
// Good: Include comments explaining purpose
[#] Payment.Gateway.Stripe
// Stripe API configuration for production environment
[<] .api_key:pg.string << "sk_live_..."
[<] .webhook_secret:pg.string << "whsec_..."
[<] .api_version:pg.string << "2024-01-01"
[X]
```

---

### 6. Use `\\NoPath\\` for OS-Specific Paths

**Good:**
```polyglot
[#] Path.Identifiers.System.DevNull
[<] .unix:pg.path << \\UnixRoot\\dev\null
[<] .windows:pg.path << \\NoPath\\  // Doesn't exist on Windows
[X]
```

**Avoid:**
```polyglot
// Don't leave fields empty or use placeholder strings
[#] Path.Identifiers.System.DevNull
[<] .unix:pg.path << \\UnixRoot\\dev\null
[<] .windows:pg.path << ""  // Wrong - use \\NoPath\\
[X]
```

---

### 7. Prefer Enumerations Over Serial for Known Schemas

**Good:**
```polyglot
// Known schema - use enumeration
[#] UserProfile
[<] .name:pg.string << "Alice"
[<] .age:pg.int << 30
[<] .email:pg.string << "alice@example.com"
[X]
```

**Avoid:**
```polyglot
// Known schema - don't use serial unnecessarily
[r] .user:pg.serial << serial{
[^]  "name": "Alice",
[^]  "age": 30,
[^]  "email": "alice@example.com"
[^]}
```

---

## See Also

### Language Specification
- [Type System](02-type-system.md) - Enumeration types details
- [Complete Syntax Reference](01-syntax-complete.md) - `[#]` block marker
- [Error Handling](04-error-handling.md) - Error types (special enumerations)

### Standard Library
- [Reserved Enumerations](../standard-library/06-reserved-enumerations.md) - Complete schema reference

### Examples
- [File Operations](../examples/file-operations.md) - Path identifier usage

### Planning
- [Decision Log](../decision-log.md) - Enumeration decisions (#1, #9, #14)
- [Reserved Enumeration Schemas](../reserved-enumeration-schema-decisions.md) - Schema definitions

---

**End of Enumerations Reference**