---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: prefix-system
shard: false

# --- Classification ---
type: reference
topic: Prefix System
summary: Reference for Prefix System
keywords:
  - syntax
  - reference
  - language

# --- BMAD Agent Routing ---
agents:
  - developer
  - architect
phase: planning
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - core-principles
unlocks:
  - control-flow
  - type-system

# --- Relationships ---
related:
  []
parent: language-syntax

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#syntax"
  - "#reference"
---
# Prefix System

**What You'll Learn:**
- All 7 prefix operators in Polyglot
- The subfield operator (`.`)
- How prefixes create namespaces and hierarchies
- When to use each prefix/operator
- Common patterns and examples

---

## The 7 Prefixes + Subfield Operator

Polyglot uses **prefix operators** to identify different construct types and the **subfield operator** (`.`) to access hierarchical fields:

| Prefix/Operator | Name | Purpose | Example |
|-----------------|------|---------|---------|
| `$` | Variable | Variables and parameters | `$user`, `$age` |
| `:` | Type | Type paths and notation | `:pg.string`, `:array.int` |
| `#` | Enum/Struct | Enum and struct definitions/values | `#OrderStatus`, `#User.new` |
| `\|` | Pipeline | Pipeline definitions and calls | `\|Database.Users.Find` |
| `!` | Error | Error types and instances | `!Network.HTTP.Timeout` |
| `@` | Registry | Package/module registry | `@ocal::MyApp:1.0.0.0` |
| `%` | Metadata | Metadata properties | `%Doc`, `%Author` |
| `.` | Subfield | Field access and subfield definition | `.name`, `.profile.email` |

**Key Concepts:**
- Every **prefix** creates a **separate namespace** with its own **hierarchy**
- The **subfield operator** (`.`) navigates hierarchies and defines fields

---

## `$` - Variable Prefix

### Purpose

Identifies **variables** and **parameters** (runtime values).

### Syntax

```polyglot
$identifier
$identifier.field
$identifier.field.nested
```

### Examples

**Variable declaration:**
```polyglot
[r] $name :string << "Alice"
[r] $age :int << 30
[r] $email :string << "alice@example.com"
```

**Field access:**
```polyglot
[r] $user << #User
   [.] .profile
      [.] .name << "Alice"
      [.] .bio << "Software engineer"

[r] $user_name :string << $user.profile.name
[r] $user_bio :string << $user.profile.bio
```

**Parameters:**
```polyglot
{|} |ProcessUser
[|] <user_id :string               // In calls: <user_id << $variable
[|] <max_results :int
{x}
```

### Hierarchy

**Pattern:** `$variable.field.nested.path`

```polyglot
$config.database.connection.host
   ^       ^         ^         ^
   var   field1    field2    field3
```

### Why `$`?

1. **Familiar** - Used in Shell, PHP, Perl, PowerShell
2. **Unambiguous** - Clearly distinguishes from hierarchy separator (`.`)
3. **Greppable** - `grep "\$user"` finds all variable usage
4. **Visual** - Variables stand out in code

### See Also

- [Variables & Lifecycle](./variables-lifecycle.md) - Variable states and lifecycle
- [Core Principles](./core-principles.md#variable-prefix--not-) - Why `$` not `,`

---

## `:` - Type Prefix

### Purpose

Identifies **type paths** and **type notation**.

### Syntax

```polyglot
:namespace.type
:namespace.collection.element_type
```

### Examples

**Built-in types:**
```polyglot
[r] $name :string << "Alice"
[r] $age :int << 30
[r] $price :float << 19.99
[r] $active :bool << #True
```

**Polyglot built-in types:**
```polyglot
[r] $text :pg.string << "Hello"
[r] $count :pg.int << 42
[r] $ratio :pg.float << 3.14
[r] $flag :pg.bool << #True
```

**Collection types:**
```polyglot
[r] $names :array.string << {"Alice", "Bob", "Charlie"}
[r] $scores :array.int << {95, 87, 92}
[r] $matrix :array.array.int << {{1, 2}, {3, 4}}
```

**Custom types:**
```polyglot
[r] $user :User << #User
   [.] .name << "Alice"
   [.] .email << "alice@example.com"

[r] $order :Order << #Order
   [.] .order_id << "ORD-123"
   [.] .status << #OrderStatus.Processing
```

### Hierarchy

**Pattern:** `:namespace.type` or `:collection.element_type`

```polyglot
:pg.array.int
 ^   ^     ^
 ns  coll  elem
```

**Nested collections:**
```polyglot
:array.array.array.string
   ^      ^      ^      ^
 coll1  coll2  coll3  elem
```

### Built-in Type Namespace

| Namespace | Types | Example |
|-----------|-------|---------|
| `pg.*` | Polyglot built-ins | `:pg.string`, `:pg.int` |
| `array.*` | Array collections | `:array.int`, `:array.string` |
| `set.*` | Set collections | `:set.int`, `:set.string` |
| `map.*` | Map collections | `:map.string.int` |

### Type Aliases

**Short form vs full form:**

| Short | Full | Description |
|-------|------|-------------|
| `:string` | `:pg.string` | String type |
| `:int` | `:pg.int` | Integer type |
| `:float` | `:pg.float` | Float type |
| `:bool` | `:pg.bool` | Boolean type |

Both forms are valid - short form is syntactic sugar.

### See Also

- [Types System](./types.md) - Complete type reference
- [Enums & Structs](./enums-structs.md) - Custom type definitions

---

## `#` - Enum/Struct Prefix

### Purpose

Identifies **enum/struct types** and **values**.

### Syntax

```polyglot
#TypeName                          // Type reference
#TypeName.variant                  // Enum variant
#TypeName.constructor              // Struct constructor
```

### Examples

**Enum definition:**
```polyglot
{#} #OrderStatus
[.] .pending
[.] .processing
[.] .completed
[.] .cancelled
{x}
```

**Enum usage:**
```polyglot
[r] $status :OrderStatus << #OrderStatus.Processing
```

**Struct definition:**
```polyglot
{#} #User
[.] .name :string
[.] .email :string
[.] .age :int
{x}
```

**Struct instantiation:**
```polyglot
[r] $user :User << #User
   [.] .name << "Alice"
   [.] .email << "alice@example.com"
   [.] .age << 30
```

### Hierarchy

**Pattern:** `#Type.value` or `#Type.nested.value`

```polyglot
#OrderStatus.Processing
   ^           ^
  Type      Variant

#Result.Success.Ok
   ^      ^      ^
  Type  Variant Sub-variant
```

### Reserved Indication

**Semicolons (`;`) denote reserved segments:**

```polyglot
#;Boolean;True                     // Fully reserved
#;Boolean;False                    // Fully reserved
#OrderStatus.Processing            // User-defined
#;DT;Business.FiscalYear.Q1       // Mixed: reserved;user
```

**Syntax:**
- `;` - Reserved segment (only core devs can create)
- `.` - User-defined segment

### See Also

- [Enums & Structs](./enums-structs.md) - Complete enum/struct reference
- [Reserved Indication System](../language/advanced/reserved-indication.md) - Semicolon system

---

## `|` - Pipeline Prefix

### Purpose

Identifies **pipelines** (functions, operations, transformations).

### Syntax

```polyglot
|PipelineName
|Namespace.PipelineName
|Namespace.Sub.PipelineName
```

### Examples

**Pipeline definition:**
```polyglot
{|} |ProcessOrder
[|] <order_id :string
[|] >result :string
{x}
```

**Pipeline call:**
```polyglot
[r] $result :string << |ProcessOrder <order_id << "ORD-123"
```

**Namespace hierarchy:**
```polyglot
|Database.Users.Find               // Database > Users > Find
|Database.Orders.Create            // Database > Orders > Create
|U.Math.Add                        // Utilities > Math > Add
|U.String.Concat                   // Utilities > String > Concat
```

**Inline pipelines:**
```polyglot
[r] $timestamp :string << |DT.Now""                    // Empty (no parameters)
[r] $doubled :float << |U.Math.Double"{5.0}"           // Literal in template
[r] $uppercase :string << |String.Upper"{$name}"       // Variable in template
[r] $sum :int << |U.Math.Add"{$x}, {$y}"               // Multiple variables
```

### Inline Pipeline Syntax: Formatted String Templates

**The string after the pipeline name is a formatted string template**, not simple arguments.

**Pattern:** `|Pipeline"{template-with-{$var:format}-substitutions}"`

**Template Components:**
- `{$var}` - Variable with **default** string representation
- `{$var:format}` - Variable with **specific** format (`:hex`, `:json`, etc.)
- `literal` - Literal text preserved as-is
- Separators - Commas, hyphens, etc. are part of the template

**Examples:**

```polyglot
// Empty template (no parameters)
[r] $now << |DT.Now""

// Single variable, default format
[r] $upper << |String.Upper"{$name}"

// Multiple variables with separator
[r] $sum << |U.Math.Add"{$x}, {$y}"

// Variables with format specifiers
[r] $hex << |Display.Hex"{$value:hex}"
[r] $json << |Export.JSON"{$data:json}"

// Mixed literals and variables
[r] $url << |API.Get"users/{$id}/posts/{$post_id:hex}"

// Literal-only (no variables)
[r] $config << |LoadConfig"production"
```

### How It Works: Three-Phase Execution

**Phase 1: String Representation (Parallel)**
- Each `{$var:format}` runs a type-specific pipeline: `|U.String.Polyglot.{Type}.{Format}`
- Example: `{$count:hex}` runs `|U.String.Polyglot.Int.Hex <variable << $count`

**Phase 2: String Substitution**
- Representations substituted into template
- Example: `"{$x}, {$y}"` with `$x=5`, `$y=3` → `"5, 3"`

**Phase 3: Feed to Formatter**
- Formatted string passed to parser pipeline via `%Inline.FormattedString`
- Parser extracts actual parameters for the main pipeline
- Example: `"5, 3"` → split by comma → `x=5`, `y=3`

**See:** [Inline Pipelines Complete Specification](../advanced/inline-pipelines.md)

---

### The `%Inline.FormattedString` Special Variable

**What it is:**
`%Inline.FormattedString` is a **compiler-populated special variable** available ONLY inside formatter pipelines registered with `%Inline` metadata.

**When it's populated:**
- During Phase 3 of inline pipeline execution
- Contains the result of Phase 2 (string substitution)
- Automatically provided by the compiler—you don't declare it

**How formatter pipelines use it:**

```polyglot
{|} |U.Math.Add.FormattedString.For.Add
[|] <formatted_string :pg.string << %Inline.FormattedString  // ← Compiler populates this

   // Parse the formatted string:
   [r] $parts :pg.array.pg.string << |U.String.Split"{%Inline.FormattedString}, {\", \"}"
   [r] $a :pg.int << |U.String.ToInt"{$parts[0]}"
   [r] $b :pg.int << |U.String.ToInt"{$parts[1]}"

   // Wire to main pipeline:
   [|] >a >> <a    // Output 'a' wired to main pipeline's <a input
   [|] >b >> <b    // Output 'b' wired to main pipeline's <b input
{x}
```

**Example execution:**
1. User writes: `|U.Math.Add"{$x}, {$y}"` where `$x=5`, `$y=3`
2. Phase 1 & 2: Template becomes `"5, 3"`
3. Phase 3: Formatter pipeline receives `%Inline.FormattedString = "5, 3"`
4. Formatter parses and outputs `a=5`, `b=3` to main pipeline

**Key Rules:**
- ✅ ONLY available in formatter pipelines (those registered with `%Inline`)
- ✅ Type is always `:pg.string`
- ✅ Read-only (compiler-populated)
- ❌ NOT available in main pipelines
- ❌ NOT available in regular pipelines
- ❌ Cannot be declared or assigned by user code

**See:**
- [Inline Pipelines - Formatter Pipelines](../advanced/inline-pipelines.md#formatter-pipelines) - Complete formatter pipeline specification
- [Pipeline Structure - %Inline Metadata](../control-flow/pipeline-structure.md#7-inline-metadata---making-pipelines-inline-callable) - How to register formatters

---

### Common Patterns

**Pattern 1: Comma-separated (most common)**
```polyglot
[r] $result << |U.Math.Add"{$x}, {$y}"
[r] $replaced << |U.String.Replace"{$text}, {$old}, {$new}"
```

**Pattern 2: Path-style**
```polyglot
[r] $user << |API.Get"users/{$id}"
```

**Pattern 3: Format specifiers**
```polyglot
[r] $hex_id << |Display.Value"{$id:hex}"
```

**Pattern 4: Literal-only**
```polyglot
[r] $env << |GetConfig"production"
```

### Hierarchy

**Pattern:** `|Namespace.Category.Pipeline`

```polyglot
|Database.Users.Find
   ^        ^     ^
   NS      Cat  Pipeline
```

**Standard library namespaces:**

| Namespace | Purpose | Example |
|-----------|---------|---------|
| `\|U.*` | Utilities | `\|U.Math.Add`, `\|U.String.Concat` |
| `\|W.*` | Wrappers | `\|W.Polyglot.Scope`, `\|W.RT.Python3.12` |
| `\|T.*` | Triggers | `\|T.Call`, `\|T.Schedule` |
| `\|Q.*` | Queue types | `\|Q.Serial`, `\|Q.Parallel` |

**User pipelines:**
```polyglot
|MyApp.Users.Authenticate
|MyApp.Orders.Process
|MyApp.Payments.Charge
```

### See Also

- [Pipeline Structure](./pipeline-structure.md) - Pipeline execution
- [Standard Library](../stdlib/index.md) - All standard pipelines

---

## `!` - Error Prefix

### Purpose

Identifies **error types** and **error instances**.

### Syntax

```polyglot
!ErrorType
!Category.Domain.ErrorType
```

### Examples

**Error definition:**
```polyglot
{!} !InvalidOrder
[.] .message :string
[.] .code :int
{x}
```

**Error usage:**
```polyglot
[y] $order_id =? ""
   [|] >error << !InvalidOrder
      [.] .message << "Order ID cannot be empty"
      [.] .code << 400
```

**Error handling:**
```polyglot
[z] $data :string << |FetchFromAPI
[z][!] !Network.HTTP.Timeout ? "Request timed out"
[z][!] !Network.HTTP.* ? "Network error"
[z][!] *! ? "Unknown error"
```

### Hierarchy

**Pattern:** `!Category.Domain.ErrorType`

```polyglot
!Network.HTTP.Timeout
   ^       ^      ^
  Cat    Domain  Error

!IO.File.NotFound
 ^   ^      ^
Cat Dom   Error
```

**Common categories:**

| Category | Domain Examples | Error Examples |
|----------|-----------------|----------------|
| `!etwork.*` | `HTTP`, `TCP`, `DNS` | `Timeout`, `ConnectionRefused` |
| `!O.*` | `File`, `Stream` | `NotFound`, `PermissionDenied` |
| `!arse.*` | `JSON`, `YAML`, `XML` | `Invalid`, `Unexpected` |
| `!atabase.*` | `SQL`, `NoSQL` | `ConnectionFailed`, `QueryError` |

### See Also

- [Error Handling](../features/error-handling/error-handling.md) - Complete error handling guide
- [Markers Reference](./markers.md#control-flow-markers) - `[z]` try block

---

## `@` - Registry Prefix

### Purpose

Identifies **registries** (packages, modules).

### Syntax

```polyglot
@Registry::PackageName:Version
```

**Parts:**
- `Registry` - Where package is hosted (`Local`, `Public`, `GitHub`)
- `PackageName` - Unique package name
- `Version` - Semantic version

### Examples

**Registry definition:**
```polyglot
{@} @Local::OrderProcessing:1.0.0.0
[A] @OrderProc                     // Alias
[<] @PgTypes                       // Import Polyglot types
[<] @Local::DatabaseOps:1.0.0.0    // Import another package
{x}
```

**Registry reference:**
```polyglot
[r] |MyPipeline << @OrderProc::ProcessOrder
```

### Hierarchy

**Pattern:** `@Registry::Package:MajorMinor.Patch.Build`

```polyglot
@Local::MyApp:1.2.3.456
   ^      ^    ^ ^ ^ ^
  Reg    Name  | | | Build
               | | Patch
               | Minor
               Major
```

**Registry types:**

| Registry | Purpose | Example |
|----------|---------|---------|
| `@ocal::*` | Local packages | `@ocal::MyApp:1.0.0.0` |
| `@ublic::*` | Public registry | `@ublic::Utils:2.1.0.0` |
| `@itHub::*` | GitHub packages | `@itHub::user/repo:1.0.0.0` |

### Aliases

```polyglot
{@} @Local::VeryLongPackageName:1.0.0.0
[A] @Short                         // Create alias

// Use alias:
[<] @Short
```

### See Also

- [Core Syntax - Registry](./core-syntax/registry.md) - (To be created)

---

## `%` - Metadata Prefix

### Purpose

Identifies **metadata properties** (documentation, annotations).

### Syntax

```polyglot
%PropertyName
```

### Examples

**Pipeline metadata:**
```polyglot
{|} |ProcessOrder
[%] %Doc << "Processes customer orders with validation"
[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "1.0.0"
[%] %Deprecated
   [.] .since << "2.0.0"
   [.] .replacement << "|ProcessOrderV2"

[|] <order_id :string
   [%] %description << "Unique order identifier"
   [%] %example << "ORD-123"

{x}
```

**Struct metadata:**
```polyglot
{#} #User
[%] %Doc << "Represents a user account"

[.] .name :string
   [%] %description << "Full name"
[.] .email :string
   [%] %description << "Email address"
   [%] %format << "email"
{x}
```

### Common Metadata Properties

| Property | Purpose | Example |
|----------|---------|---------|
| `%Doc` | Documentation | `%Doc << "Description"` |
| `%Author` | Author info | `%Author` with `.name`, `.since` |
| `%Deprecated` | Deprecation | `%Deprecated` with `.since`, `.replacement` |
| `%Example` | Usage example | `%Example << "ORD-123"` |
| `%Reserved` | Reserved indication | `%Reserved << "true"` (privileged!) |
| `%Description` | Short description | `%Description << "Brief text"` |

### Reserved Metadata (Privileged)

**Only core developers can set:**
```polyglot
{#} #;Boolean;True
[%] %Reserved << "true"            // ⚠️ Privileged operation
{x}
```

### See Also

- [Metadata System](../language/advanced/metadata-system.md) - Complete metadata guide

---

## `.` - Subfield Operator

### Purpose

The **subfield operator** (`.`) serves dual purposes:
1. **Field access** - Navigate hierarchical structures
2. **Field definition** - Define subfields in enums, structs, and serials

**Note:** Unlike the prefixes above, `.` is an **operator**, not a prefix. It works **with** prefixes to create hierarchical paths.

### Syntax

**Field Access:**
```polyglot
$variable.field.nested.path
#EnumType.variant.subfield
:namespace.type.collection
```

**Field Definition (with `[.]` marker):**
```polyglot
[.] .field_name :type
[.] .nested
   [.] .subfield :type
```

### Examples

**Variable field access:**
```polyglot
[r] $user :#User << #User
   [.] .profile :#Profile << #Profile
      [.] .name << "Alice"
      [.] .email << "alice@example.com"

[r] $name :pg.string << $user.profile.name
[r] $email :pg.string << $user.profile.email
```

**Enum subfield definition:**
```polyglot
{#} #Status
[.] .Pending                        // Variant subfield (no type)
[.] .Active                         // Variant subfield
   [.] .started_at :pg.datetime     // Data subfield (with type)
   [.] .progress :pg.int             // Data subfield
[.] .Completed                      // Variant subfield
   [.] .finished_at :pg.datetime    // Data subfield
{x}
```

**Serial field access:**
```polyglot
[r] $config :pg.serial << #Serial
   database << #Serial
      host << "localhost"
      port << 5432

[r] $db_host :pg.string << $config."database.host"
[r] $db_port :pg.int << $config."database.port"
```

**Type hierarchy:**
```polyglot
:pg.array.string              // pg namespace → array collection → string element
:custom.data.user             // custom namespace → data module → user type
```

**Pipeline hierarchy:**
```polyglot
|U.Math.Add                   // Utilities → Math → Add
|Database.Users.Find          // Database → Users → Find
```

### Hierarchy Navigation

The `.` operator **always** means "subfield" or "child in hierarchy":

```
parent.child.grandchild
   ^      ^
   |      |
  dot    dot
  (subfield access)
```

**Examples across all constructs:**

| Context | Example | Meaning |
|---------|---------|---------|
| Variable | `$user.profile.name` | Variable `$user` → field `profile` → field `name` |
| Type | `:pg.array.int` | Namespace `pg` → collection `array` → element `int` |
| Enum | `#tatus.Active.progress` | Type `#tatus` → variant `Active` → field `progress` |
| Pipeline | `\|Database.Users.Find` | Namespace `Database` → category `Users` → pipeline `Find` |
| Error | `!etwork.HTTP.Timeout` | Category `Network` → domain `HTTP` → error `Timeout` |
| Metadata | `%Author.name` | Property `Author` → field `name` |

### Subfield Marker: `[.]`

When **defining** subfields (not accessing), use the `[.]` marker:

**In enum/struct definitions:**
```polyglot
{#} #Address
[.] .street :pg.string          // [.] marks this as a subfield definition
[.] .city :pg.string            // [.] marker required
[.] .zip :pg.string             // [.] marker required
{x}
```

**In serial construction:**
```polyglot
[r] $data :pg.serial << #Serial
   [.] .name << "Alice"          // [.] marks subfield assignment
   [.] .age << 30
   [.] .contact << #Serial
      [.] .email << "alice@example.com"
      [.] .phone << "555-1234"
```

### `.` vs `[.]`

| Symbol | Context | Purpose | Example |
|--------|---------|---------|---------|
| `.` | Access/navigation | Read field value | `$user.name` |
| `[.]` | Definition/assignment | Define or assign field | `[.] .name << "Alice"` |

**Think of it as:**
- `.` = "dot operator" (navigation)
- `[.]` = "subfield marker" (definition/construction)

### Universal Subfield Concept

**Key Insight:** Subfields are **universal** in Polyglot, not specific to any one feature.

- Used in **variables** for field access
- Used in **enums** for variant and data subfields
- Used in **types** for namespace navigation
- Used in **pipelines** for hierarchy organization
- Used in **errors** for categorical organization
- Used in **serials** for structured data

**The `.` operator is the universal mechanism for hierarchy in Polyglot.**

### See Also

- [Enums & Serial Data](./enums-serial.md) - Subfields in enums
- [Variables & Lifecycle](./variables-lifecycle.md) - Variable field access
- [Types System](./types.md) - Type hierarchy navigation
- [Markers Reference](./markers.md) - `[.]` subfield marker

---

## Universal Hierarchy Pattern

**All prefixes follow the same hierarchy pattern:**

```
PREFIX.namespace.identifier.field
```

### Examples Across All Prefixes

| Prefix | Example | Pattern |
|--------|---------|---------|
| `$` | `$user.profile.name` | Variable + fields |
| `:` | `:pg.array.int` | Namespace + type |
| `#` | `#rderStatus.Processing` | Type + variant |
| `\|` | `\|Database.Users.Find` | Namespace + pipeline |
| `!` | `!etwork.HTTP.Timeout` | Category + error |
| `@` | `@ocal::MyApp:1.0.0.0` | Registry::name:version |
| `%` | `%Author.name` | Property + field |

**Consistency:** Once you learn the pattern for one prefix, you know it for all.

---

## Prefix Usage Summary

### Quick Reference

**Declaration & Definition:**
```polyglot
{@} @Local::MyApp:1.0.0.0          // Registry definition
{|} |PipelineName                  // Pipeline definition
{#} #EnumName                      // Enum definition
{!} !ErrorName                     // Error definition

[%] %MetadataProperty              // Metadata
[r] $variable :TypeName            // Variable with type
```

**Usage & Reference:**
```polyglot
$variable                          // Variable access
:pg.string                         // Type reference
#EnumName.Variant                  // Enum value
|PipelineName                      // Pipeline call
!ErrorName                         // Error instance
@Registry::Package:Version         // Registry reference
%PropertyName                      // Metadata property
```

### When to Use Each Prefix

| If you want to... | Use prefix... | Example |
|-------------------|---------------|---------|
| Store/reference a value | `$` | `$user`, `$config.database.host` |
| Declare a type | `:` | `:string`, `:array.int` |
| Define/use enum/struct | `#` | `#rderStatus.Processing` |
| Call a pipeline | `\|` | `\|Database.Users.Find` |
| Handle an error | `!` | `!etwork.HTTP.Timeout` |
| Import a package | `@` | `@ocal::MyApp:1.0.0.0` |
| Annotate with metadata | `%` | `%Doc`, `%Author` |

---

## Common Patterns

### Pattern 1: Type + Variable + Pipeline

```polyglot
[r] $user :User << |Database.Users.Find
//        ^           ^
//       Type      Pipeline

[r] $name :string << $user.name
//        ^           ^
//       Type      Variable field
```

### Pattern 2: Enum Value + Error

```polyglot
[r] $status :OrderStatus << #OrderStatus.Processing
//          ^                 ^
//         Type            Enum value

[y] $status =? #OrderStatus.Failed
   [|] >error << !OrderProcessing.Failed
//                  ^
//                Error
```

### Pattern 3: Pipeline with Metadata

```polyglot
{|} |ProcessOrder
//  ^
// Pipeline

[%] %Doc << "Processes orders"
//  ^
// Metadata

[|] <order_id :string
//            ^
//           Type

{x}
```

### Pattern 4: Complete Ecosystem

```polyglot
{@} @Local::OrderProcessing:1.0.0.0    // Registry
[%] %Doc << "Order processing module"  // Metadata

{#} #OrderResult                       // Enum
[.] .success
   [.] .order_id :string               // Type
[.] .error
   [.] .code :int                      // Type
{x}

{|} |ProcessOrder                      // Pipeline
[|] <order_id :string                  // Type
[|] >result :OrderResult               // Type (custom)

[t] |T.Call                            // Pipeline (trigger)
[W] |W.Polyglot.Scope                  // Pipeline (wrapper)

   [r] $order << |Database.Orders.Find // Pipeline call, variable
   [|] <order_id << $order_id          // Variable

   [y] $order.valid                    // Variable field
      [|] >result << #OrderResult.success  // Enum value
         [.] .order_id << $order_id    // Variable
   [y] *?
      [|] >result << #OrderResult.error    // Enum value
         [.] .code << 404
         [|] >error << !OrderNotFound      // Error

{x}
{@}
```

**This example uses all 7 prefixes:**
- `@` - Registry
- `%` - Metadata
- `#` - Enum
- `:` - Type
- `|` - Pipeline
- `$` - Variable
- `!` - Error

---

## Summary

### The 7 Prefixes

1. **`$`** - Variables/parameters
2. **`:`** - Types
3. **`#`** - Enums/structs
4. **`|`** - Pipelines
5. **`!`** - Errors
6. **`@`** - Registries
7. **`%`** - Metadata

### Universal Pattern

```
PREFIX.namespace.identifier.field
```

### Why Prefixes?

- **Namespace separation** - Each prefix has its own namespace
- **Visual clarity** - Instantly identify construct type
- **Greppable** - Search for specific construct types
- **Consistency** - Same hierarchy pattern everywhere
- **No conflicts** - Variables, types, pipelines can have same names

---

## Related Documentation

- [Core Principles](./core-principles.md) - Universal hierarchy principle
- [Markers Reference](./markers.md) - Markers vs operators distinction
- [Variables & Lifecycle](./variables-lifecycle.md) - `$` prefix usage
- [Types System](./types.md) - `:` prefix usage
- [Enums & Structs](./enums-structs.md) - `#` prefix usage
- [Pipeline Structure](./pipeline-structure.md) - `|` prefix usage
- [Error Handling](../features/error-handling/error-handling.md) - `!` prefix usage
- [Metadata System](../language/advanced/metadata-system.md) - `%` prefix usage

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
