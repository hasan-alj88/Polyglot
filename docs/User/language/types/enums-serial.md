---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: enums-serial
shard: false

# --- Classification ---
type: spec
topic: Enums & Serial Data
summary: Specification for Enums & Serial Data
keywords:
  - types
  - type-system
  - specification

# --- BMAD Agent Routing ---
agents:
  - architect
  - developer
phase: solutioning
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - language-syntax
unlocks:
  - advanced-features

# --- Relationships ---
related:
  []
parent: language-types

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#types"
  - "#spec"
---
# Enums & Serial Data

**What You'll Learn:**
- Difference between compile-time (#Enum) and runtime (:pg.serial) schemas
- Enum definitions with variant subfields and data subfields
- Variant subfield rules and exhaustive matching
- Serial load blocks for file-based initialization
- Enums as containers vs value enums
- Implicit conversion between #Enum and :pg.serial
- Common patterns and best practices

---

## Overview

**Enums** and **Serial data** (`:pg.serial`) are the two primary mechanisms for handling **hierarchical field data** in Polyglot:

| Construct      | Schema       | Purpose                           | Example                |
| -------------- | ------------ | --------------------------------- | ---------------------- |
| **\#Enum**      | Compile-time | Structured data with known schema | `#Address` with fields |
| **:pg.serial** | Runtime      | Dynamic data with unknown schema  | Loaded from files      |

**Key Distinction:**
- **#Enum:** Schema determined at compile time → Compiler can catch errors
- **:pg.serial:** Schema determined at runtime → More flexible but fewer compile-time checks

**Recommendation:** Use **#Enum** whenever the schema is known so the compiler can catch errors more efficiently.

---

## Reserved vs User-Defined Enums

### The Semicolon System

**Polyglot distinguishes between reserved and user-defined enums using separators:**

| Separator | Type | Who Can Define | Example |
|-----------|------|----------------|---------|
| `;` | Reserved | Core language only | `#;Boolean;True` |
| `.` | User-defined | Any developer | `#tatus.Pending` |

**Reserved Enums:**
```polyglot
#;Boolean;True                     // Fully reserved enum
#;Boolean;False                    // Fully reserved enum
```

**User-Defined Enums:**
```polyglot
#OrderStatus.Processing            // User-defined enum
#Result.Success                    // User-defined enum
```

**Mixed (Reserved base, user-defined extensions):**
```polyglot
#;DT;Business.FiscalYear.Q1       // Reserved base (#;DT;Business), user extension (.FiscalYear.Q1)
```

### Why This Matters

**Compile-time vs user-defined:**
- **Reserved enums** (`;`) are part of the core language and guaranteed to exist
- **User-defined enums** (`.`) are defined by your code and must be declared before use

**Pattern:**
- Use reserved enums like `#;Boolean;True` and `#;Boolean;False` directly (no definition needed)
- Define your own enums with `.` separators using `{#}...{x}` blocks

**Example:**
```polyglot
// Use reserved enum directly (no definition needed)
[r] $is_active :bool << #True

// User-defined enum requires definition
{#} #Status
[.] .Pending
[.] .Completed
{x}

[r] $current_status :#Status << #Status.Pending
```

**See:** [Reserved Indication System](../User/language/advanced/reserved-indication.md) for complete details on the semicolon system.

---

## Reserved Boolean Aliases

**For convenience, Polyglot provides shorthand aliases for the reserved Boolean enum:**

```polyglot
{A} #Boolean
[A] #;Boolean;True >> #True
[A] #;Boolean;False >> #False
{x}
```

**This allows you to use:**
- `#True` instead of `#;Boolean;True`
- `#False` instead of `#;Boolean;False`

**Both forms are valid:**
```polyglot
// Full form (always valid):
[r] $is_active :pg.bool << #;Boolean;True
[r] $is_deleted :pg.bool << #;Boolean;False

// Short form via alias (also valid):
[r] $is_active :pg.bool << #True
[r] $is_deleted :pg.bool << #False
```

**Style Recommendation:**
- **Use SHORT form** (`#True` / `#False`) in: code examples, inline expressions, simple assignments
- **Use FULL form** (`#;Boolean;True` / `#;Boolean;False`) in: formal specifications, type system documentation, when emphasizing reserved status

**Example:**
```polyglot
// Metadata (use short form for brevity):
[%] %Inline.Output << #True

// Boolean comparisons (use short form):
[f] $flag =? #True
   [r] $result << "yes"

// Formal type example (use full form to show reserved nature):
[r] $validated :pg.bool << #;Boolean;True  // Explicitly reserved enum
```

**Note:** These aliases are **built into the language core** - no import required.

---

## Understanding Subfields

### The `.` Prefix and `[.]` Marker

**Both `.` prefix and `[.]` marker denote SUBFIELDS:**

- **`.` prefix** - Indicates a subfield name (e.g., `.House`, `.street`, `.type`)
- **`[.]` marker** - Indicates subfield context (definition or access)

**Enums are ONE example of using subfields.** The same subfield concept applies to:
- Enum definitions
- Serial data access
- Struct-like hierarchies

---

## Enum Definitions

### Basic Syntax

**Enums are defined using `{#}...{x}` blocks:**

```polyglot
{#} #MyApp.Info.AddressType
[A] #AddressType                    // Alias for shorthand
[.] .House                          // Subfield (no type = variant)
[.] .Office                         // Subfield (variant)
[.] .Warehouse                      // Subfield (variant)
[.] .BoxOffice                      // Subfield (variant)
[.] .Other                          // Subfield (variant)
   [.] .type :pg.string <~ "Other"  // Subfield (with type = data subfield)
{x}
```

**Terminology:**
- **Subfield without type** - Variant subfield (e.g., `.House`, `.Office`)
- **Subfield with type** - Data subfield (e.g., `.type :pg.string`)
- **`[.]` marker** - Indicates we're working with subfields

---

## Subfield Rules in Enums

### Rule 1: Variant Subfields Can Only Be Siblings with Variant Subfields

**CORRECT:**
```polyglot
{#} #Status
[.] .Pending                        // Variant subfield (no type)
[.] .Processing                     // Variant subfield (sibling)
[.] .Completed                      // Variant subfield (sibling)
{x}
```

**WRONG:**
```polyglot
{#} #Status
[.] .Pending                        // Variant subfield
[.] .message :pg.string             // ❌ ERROR: Data subfield cannot be sibling of variant
{x}
```

**Why this rule?** Variant subfields must be exhaustively matched in conditionals. The compiler must know all possible variants at the same level.

### Rule 2: Variant Subfields Can Have Data Subfield Children

**CORRECT:**
```polyglot
{#} #Result
[.] .Success                        // Variant subfield
   [.] .data :pg.serial             // Data subfield (child)
   [.] .timestamp :pg.string        // Data subfield (child)
[.] .Error                          // Variant subfield (sibling of .Success)
   [.] .code :pg.int                // Data subfield (child)
   [.] .message :pg.string          // Data subfield (child)
{x}
```

**Pattern:** Variant subfields define branches, data subfields define data for those branches.

---

## Value Enums vs Container Enums

### Value Enums (With Initial Values)

**Definition with initial values:**
```polyglot
{#} #MyApp.Info.Address
[A] #Address
[.] .Type :#AddressType <~ #AddressType.House  // Default value
[.] .street :pg.string << "123 Main St"        // Final value
[.] .city :pg.string << "Springfield"          // Final value
[.] .zip :pg.string << "12345"                 // Final value
{x}
```

**Usage:**
```polyglot
[r] $address_type :#AddressType << #Address.Type  // Pull value
[r] $street :pg.string << #Address.street         // Pull value
```

**Lifecycle:**
- `<~` fields are **Default** (can receive one more push to become Final)
- `<<` fields are **Final** (no more pushes allowed, pull as needed)

### Container Enums (Without Initial Values)

**Definition as container:**
```polyglot
{#} #MyApp.Info.Address
[A] #Address
[.] .Type :#AddressType              // No initial value
[.] .street :pg.string               // No initial value
[.] .city :pg.string                 // No initial value
[.] .zip :pg.string                  // No initial value
{x}
```

**Instantiation:**
```polyglot
[r] $address1 :#Address
   [.] .Type :#AddressType << #AddressType.House
   [.] .street :pg.string << "123 Main St"
   [.] .city :pg.string << "Springfield"
   [.] .zip :pg.string << "12345"
```

**Pattern:** Container enums define schema, instantiation provides values.

---

## Serial Load Blocks

### Loading from Files in Enum Definition

**Define file paths:**
```polyglot
{#} #MyApp.Data.Files
[A] #MyFiles
[.] .file1 :pg.path << "\\FileDir\\file1.yaml"
[.] .file2 :pg.path << "\\FileDir\\file2.json"
[.] .file3 :pg.path << "\\FileDir\\file3.toml"
{x}
```

**Load fields from multiple sources:**
```polyglot
{#} #MyApp.Info.Address
[A] #Address
[.] .Type :#AddressType <~ #AddressType.House
[s] |YAML.Load"{#MyFiles.file1}"
   [.] .street :pg.string << .address1.street  // Load from YAML path
   [.] .city :pg.string << .address1.city
[s] |JSON.Load"{#MyFiles.file2}"
   [.] .zip :pg.string << .city1.zip          // Load from JSON path
[s][!] *!                                     // Error handler for all [s] blocks
{x}
```

**Key Features:**
- Multiple `[s]` blocks can load from different sources
- Fields map to paths in loaded files (e.g., `.address1.street`)
- Single `[s][!] *!` handles errors for all serial load blocks

**See:** [Serial Load Block](../User/language/advanced/serial-load-block.md)

---

## Serial Data (:pg.serial)

### Runtime Schema

**Serial data has schema determined at runtime:**

```polyglot
// Load from file (schema unknown at compile time)
[s] $config :pg.serial << |YAML.Load"{$config_path}"
[s][!] *!

// Access fields dynamically
[r] $host :pg.string << $config."database.host"
[r] $port :pg.int << $config."database.port"
```

### Creating Serial Data from Scratch

**Partial copy from existing serial:**
```polyglot
[r] $source :pg.serial << $existing_serial

[r] $data :pg.serial << $source
   [.] field1 << $value1              // Override/add field
   [.] field2 << $value2              // Override/add field
```

**From enum:**
```polyglot
[r] $serial_copy :pg.serial << $address_enum
```

---

## Implicit Conversion

### Enum to Serial

**Full conversion:**
```polyglot
[r] $address :#Address << ...
[r] $serial :pg.serial << $address    // Convert entire enum to serial
```

**Partial conversion:**
```polyglot
[r] $address :#Address << ...
[r] $partial :pg.serial << $address
   [.] << .city                       // Copy only city field
   [.] << .zip                        // Copy only zip field
```

### Serial to Enum

**Load serial from file:**
```polyglot
[s] $serial1 :pg.serial << |TOML.Load"{#MyFiles.file3}"
[s][!] *!
```

**Convert to enum:**
```polyglot
[r] $address2 :#Address << $serial1
```

**Behavior:**
- Matching fields are copied
- Non-matching fields get `!TOML.Load.NoData` error in **Faulted** state
- Fields that don't exist in serial are ignored

---

## Shorthand Syntax

### Overview

**Shorthand allows implicit value assignment when field name matches variable name.**

**Applies to:**
1. `#Enum` construction
2. `#Serial` construction
3. `[s]` Serial Load Blocks

**Syntax:** `[.] .fieldname` implicitly means `[.] .fieldname << $fieldname`

---

### Shorthand for Value Enums

**When enum has initial values:**
```polyglot
{#} #User
[.] .username :pg.string << "guest"
{x}

// Later in pipeline:
[r] $name :pg.string <~ "Alice"
[r] $name << #User.username         // Pull value from enum
```

---

### Shorthand for Enum Construction

**When variable name matches field name:**

```polyglot
{#} #User
[.] .username :pg.string
[.] .email :pg.string
{x}

// Later in pipeline:
[r] $username << "Alice"             // Variable named "username"
[r] $email << "alice@example.com"    // Variable named "email"

[r] $user :#User << #User
   [.] .username                     // Shorthand: implicit << $username
   [.] .email                        // Shorthand: implicit << $email
```

**Expands to:**
```polyglot
[r] $user :#User << #User
   [.] .username << $username        // Explicit
   [.] .email << $email              // Explicit
```

---

### Shorthand for Serial Construction

**Works the same way for `#Serial`:**

```polyglot
[r] $name << "Bob"
[r] $age << 30
[r] $city << "NYC"

[r] $person :pg.serial << #Serial
   [.] .name                         // Shorthand: implicit << $name
   [.] .age                          // Shorthand: implicit << $age
   [.] .city                         // Shorthand: implicit << $city
```

**Expands to:**
```polyglot
[r] $person :pg.serial << #Serial
   [.] .name << $name                // Explicit
   [.] .age << $age                  // Explicit
   [.] .city << $city                // Explicit
```

---

### Shorthand in Serial Load Blocks `[s]`

**Shorthand works in `[s]` blocks when loading from files:**

```polyglot
{#} #Config
[s] |YAML.Load"{\"config.yaml\"}"
   [.] .database                     // Shorthand: loads from .database path in YAML
   [.] .server                       // Shorthand: loads from .server path in YAML
   [.] .logging                      // Shorthand: loads from .logging path in YAML
[s][!] *!
{x}
```

**Expands to:**
```polyglot
{#} #Config
[s] |YAML.Load"{\"config.yaml\"}"
   [.] .database << .database        // Explicit: load from YAML path .database
   [.] .server << .server            // Explicit: load from YAML path .server
   [.] .logging << .logging          // Explicit: load from YAML path .logging
[s][!] *!
{x}
```

---

### Shorthand Rules

**For shorthand to work:**

1. **Variable must exist in scope** (for enum/serial construction)
   - Or path must exist in loaded data (for `[s]` blocks)

2. **Variable must be Final or Default state**
   - Cannot use Pending variables for shorthand

3. **Name must exactly match**
   - Field `.username` requires variable `$username` (exact match)
   - Case-sensitive

4. **Type must be compatible**
   - Variable type must match or be coercible to field type

5. **Shorthand is optional**
   - Can always use explicit `[.] .field << $value` form
   - Shorthand is syntactic sugar for convenience

---

### When to Use Shorthand

**Use shorthand when:**
- Field and variable names naturally match
- Code is clearer without repetition
- Building objects from matching variables

**Use explicit form when:**
- Field name differs from variable name
- Value comes from expression, not variable
- Clarity is more important than brevity

**Example mixing both:**
```polyglot
[r] $first_name << "Alice"
[r] $age << 30

[r] $user :#User << #User
   [.] .username << $first_name      // Explicit: names differ
   [.] .age                          // Shorthand: names match
   [.] .created_at << |DT.Now""      // Explicit: value from expression
```

---

## Exhaustive Matching

### The Requirement

**All variant subfield siblings MUST be covered in pattern matching:**

```polyglot
{#} #Status
[.] .Pending
[.] .Processing
[.] .Completed
[.] .Cancelled
{x}
```

### Match Expressions (`[m]`)

**CORRECT - All variants covered:**
```polyglot
[m] $message << $status
   [?] #Status.Pending ? "Waiting"
   [?] #Status.Processing ? "In progress"
   [?] #Status.Completed ? "Done"
   [?] #Status.Cancelled ? "Cancelled"
```

**CORRECT - Using wildcard:**
```polyglot
[m] $message << $status
   [?] #Status.Completed ? "Done"
   [?] * ? "Not completed"           // Covers all other variants
```

**WRONG - Missing variant:**
```polyglot
[m] $message << $status
   [?] #Status.Pending ? "Waiting"
   [?] #Status.Processing ? "In progress"
   [?] #Status.Completed ? "Done"
   // ❌ COMPILE ERROR: Missing #Status.Cancelled
```

### Forked Branches (`[f]`)

**Exhaustive matching also applies to forked conditional branches:**

**CORRECT - All variants covered with explicit branches:**
```polyglot
[f] $status =? #Status.Pending
   [r] $message << "Waiting"
[&] $status =? #Status.Processing
   [r] $message << "In progress"
[&] $status =? #Status.Completed
   [r] $message << "Done"
[&] $status =? #Status.Cancelled
   [r] $message << "Cancelled"
```

**WRONG - Missing variant:**
```polyglot
[f] $status =? #Status.Pending
   [r] $message << "Waiting"
[&] $status =? #Status.Processing
   [r] $message << "In progress"
[&] $status =? #Status.Completed
   [r] $message << "Done"
   // ❌ COMPILE ERROR: Missing #Status.Cancelled variant
```

**CORRECT - Nested enum matching:**
```polyglot
{#} #Result
[.] .Success
[.] .Error
[.] .Timeout
{x}

[f] $result =? #Result.Success
   [r] $code << 200
[&] $result =? #Result.Error
   [r] $code << 500
[&] $result =? #Result.Timeout
   [r] $code << 504
   // All variants covered - OK
```

### Why Exhaustive Matching?

**Compiler enforces exhaustiveness to prevent logic errors.** You must handle all possible variant subfield cases.

**Benefits:**
1. **Compile-time safety** - Catch missing cases before runtime
2. **Maintainability** - Adding new variant subfields triggers compiler errors in all match sites
3. **Documentation** - Forces explicit handling of all states

---

## Common Patterns

### Pattern 1: Result Type

```polyglot
{#} #Result
[.] .Success
   [.] .data :pg.serial
   [.] .timestamp :pg.string
[.] .Error
   [.] .code :pg.int
   [.] .message :pg.string
{x}

{|} |FetchData
[|] >result :#Result

[t] |T.Call
[W] |W.Polyglot.Scope

   [z] $data :pg.serial << |Database.Query""
   [z][!] *! >> $error :!

   [f] $error.state =? :pg.state.faulted
      [|] >result << #Result.Error
         [.] .code << 500
         [.] .message << "Database error"
   [f] *?
      [|] >result << #Result.Success
         [.] .data << $data
         [.] .timestamp << |DT.Now""

{x}
```

### Pattern 2: Configuration Loading

```polyglot
{#} #Config
[s] |YAML.Load"{\"config.yaml\"}"
   [.] .database :pg.serial << .database
   [.] .server :pg.serial << .server
   [.] .logging :pg.serial << .logging
[s][!] *!
{x}

// Usage:
[r] $db_host :pg.string << #Config.database."host"
[r] $db_port :pg.int << #Config.database."port"
```

### Pattern 3: Nested Enums

```polyglot
{#} #ContactInfo
[.] .email :pg.string
[.] .phone :pg.string
{x}

{#} #User
[.] .name :pg.string
[.] .contact :#ContactInfo
{x}

// Instantiation:
[r] $user :#User
   [.] .name << "Alice"
   [.] .contact :#ContactInfo
      [.] .email << "alice@example.com"
      [.] .phone << "555-1234"

// Access:
[r] $email :pg.string << $user.contact.email
```

### Pattern 4: Conditional Defaults with Enums

```polyglot
{#} #UserRole
[.] .Admin
[.] .User
[.] .Guest
{x}

{#} #User
[.] .username :pg.string
[.] .role :#UserRole <~ #UserRole.Guest  // Default to Guest
{x}

// Override default:
[r] $admin :#User
   [.] .username << "alice"
   [.] .role << #UserRole.Admin       // Override default
```

---

## Best Practices

### 1. Use Enums for Known Schemas

**Good:**
```polyglot
{#} #Order
[.] .order_id :pg.string
[.] .customer_id :pg.string
[.] .items :pg.array.pg.string
[.] .total :pg.float
{x}
```

**Why:** Compiler can validate field access and catch errors early.

### 2. Use :pg.serial for Dynamic Data

**Good:**
```polyglot
[s] $api_response :pg.serial << |HTTP.Get"{$api_url}"
[s][!] *!

[r] $dynamic_field << $api_response."unknown.path"
```

**Why:** Schema may vary or be unknown at compile time.

### 3. Prefer Variant Subfields for State

**Good:**
```polyglot
{#} #OrderStatus
[.] .Pending
[.] .Processing
[.] .Shipped
[.] .Delivered
[.] .Cancelled
{x}
```

**Bad:**
```polyglot
{#} #Order
[.] .status :pg.string              // Stringly-typed, error-prone
{x}
```

**Why:** Variant subfields provide exhaustive checking and type safety.

### 4. Document Variant Subfields

**Good:**
```polyglot
{#} #Priority
[%] %Doc << "Task priority levels"

[.] .Critical
   [%] %description << "Urgent, handle immediately"
[.] .High
   [%] %description << "Important, handle soon"
[.] .Medium
   [%] %description << "Normal priority"
[.] .Low
   [%] %description << "Can be deferred"
{x}
```

### 5. Use Aliases for Long Names

**Good:**
```polyglot
{#} #MyApp.Domain.Business.CustomerAddress
[A] #Address                        // Short alias
[.] .street :pg.string              // Data subfield
{x}

// Use alias:
[r] $addr :#Address
```

### 6. Remember: Subfields Are Universal

**The `[.]` marker and `.` prefix are not enum-specific:**

```polyglot
// In enums:
{#} #Status
[.] .Pending                        // Subfield in enum
{x}

// In serial data:
[r] $data :pg.serial << ...
   [.] field1 << value              // Subfield in serial

// Accessing nested data:
[r] $value << $data."field.subfield"  // Dot notation for subfields
```

**Key concept:** Subfields create hierarchy. Enums are one use case.

---

## Common Pitfalls

### Pitfall 1: Mixing Variant and Data Subfields as Siblings

```polyglot
// ❌ WRONG:
{#} #Status
[.] .Pending                        // Variant subfield
[.] .message :pg.string             // Data subfield - ERROR!
{x}

// ✅ CORRECT:
{#} #Status
[.] .Pending                        // Variant subfield
   [.] .message :pg.string          // Data subfield as child
[.] .Processing
   [.] .message :pg.string
{x}
```

### Pitfall 2: Incomplete Exhaustive Matching

```polyglot
{#} #Status
[.] .A
[.] .B
[.] .C
{x}

// ❌ WRONG:
[m] $result << $status
   [?] #Status.A ? "A"
   [?] #Status.B ? "B"
   // Missing .C - COMPILE ERROR

// ✅ CORRECT:
[m] $result << $status
   [?] #Status.A ? "A"
   [?] #Status.B ? "B"
   [?] * ? "Other"                  // Covers .C
```

### Pitfall 3: Forgetting [.] for Sub-fields

```polyglot
// ❌ WRONG:
[r] $data :pg.serial << $source
   field1 << $value1                // Missing [.]

// ✅ CORRECT:
[r] $data :pg.serial << $source
   [.] field1 << $value1            // [.] indicates sub-field
```

### Pitfall 4: Type Mismatch in Conversion

```polyglot
{#} #User
[.] .age :pg.int
{x}

[r] $serial :pg.serial << ...
   age << "not a number"            // String, not int

// ❌ ERROR when converting:
[r] $user :#User << $serial         // Type mismatch on .age field
```

---

## Comparison Table

| Feature | #Enum | :pg.serial |
|---------|-------|------------|
| **Schema** | Compile-time | Runtime |
| **Type Safety** | Strong (compile-time errors) | Weak (runtime errors) |
| **Field Access** | `$enum.field` | `$serial."field"` or `$serial."path.to.field"` |
| **Definition** | `{#} #Name ... {x}` | No definition needed |
| **Best For** | Known structures | Dynamic/unknown data |
| **Exhaustive Matching** | Required | Not enforced |
| **Conversion** | Can convert to :pg.serial | Can convert to #Enum |
| **File Loading** | Via `[s]` blocks | Via `\|*.Load` pipelines |

---

## Summary

### Enums

**Purpose:** Compile-time schema for structured data

**Syntax:**
```polyglot
{#} #EnumName
[.] .enum_field                     // Variant
   [.] .value_field :type           // Data for variant
{x}
```

**Usage:**
```polyglot
[r] $instance :#EnumName
   [.] .value_field << value
```

### Serial Data

**Purpose:** Runtime schema for dynamic data

**Syntax:**
```polyglot
[r] $data :pg.serial << $source
   [.] field << value
```

**Access:**
```polyglot
[r] $value << $data."field.path"
```

### Key Concepts

- **Variant subfields** define variants (exhaustive matching required)
- **Data subfields** carry data (with types)
- **Serial load blocks** (`[s]`) load from files in parallel
- **Implicit conversion** between #Enum and :pg.serial
- **Use #Enum when schema is known** for better error checking

---

## Related Documentation

- [Prefix System](./prefix-system.md) - `#` prefix for enums
- [Types System](./types.md) - Type notation
- [Reserved Indication System](../User/language/advanced/reserved-indication.md) - Semicolon system
- [Serial Load Block](../User/language/advanced/serial-load-block.md) - `[s]` marker details
- [Variables & Lifecycle](./variables-lifecycle.md) - Variable states

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
