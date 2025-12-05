# Polyglot Hierarchy Trees - Complete Reference

**Version:** 0.0.2
**Last Updated:** 2025-11-30
**Purpose:** Comprehensive tree diagrams for all Polyglot hierarchies

---

## Table of Contents

1. [Variable Hierarchies](#variable-hierarchies)
2. [Pipeline Hierarchies](#pipeline-hierarchies)
3. [Enumeration Hierarchies](#enumeration-hierarchies)
4. [Error Hierarchies](#error-hierarchies)
5. [Type System Hierarchies](#type-system-hierarchies)
6. [Reserved Namespace Hierarchies](#reserved-namespace-hierarchies)
7. [Block Element Hierarchies](#block-element-hierarchies)
8. [Operator Prefix Map](#operator-prefix-map)

---

## Variable Hierarchies

### Variable Namespace Structure

```
.variable: pg\serial                // Root variable with type
│
├─ .variable.field1: pg\string      // Direct field with type
│
├─ .variable.field2: pg\serial      // Nested serial namespace
│   ├─ .variable.field2.subfield1: pg\int    // Nested field with type
│   └─ .variable.field2.subfield2: pg\bool   // Nested field with type
│
└─ .variable.pgvar.*                // Reserved metadata (ALWAYS Ready)
    ├─ .variable.pgvar.state: #PgVar.States.*        // Current state (enum)
    ├─ .variable.pgvar.errors: pg\array{!}           // Error list
    └─ .variable.pgvar.history.*    // State transition timestamps
        ├─ .variable.pgvar.history.Declared.at: pg\dt
        ├─ .variable.pgvar.history.DefaultReady.at: pg\dt
        ├─ .variable.pgvar.history.Pending.at: pg\dt
        ├─ .variable.pgvar.history.Ready.at: pg\dt
        └─ .variable.pgvar.history.Faulted.at: pg\dt
```

### Example: User Profile Variable

```
.user: pg\serial                    // User profile data
│
├─ .user.id: pg\int                 // User ID
├─ .user.email: pg\string           // Email address
├─ .user.verified: pg\bool          // Email verified flag
│
├─ .user.name: pg\serial            // Name components (nested serial)
│   ├─ .user.name.first: pg\string
│   ├─ .user.name.middle: pg\string
│   └─ .user.name.last: pg\string
│
├─ .user.address: pg\serial         // Address (nested serial)
│   ├─ .user.address.street: pg\string
│   ├─ .user.address.city: pg\string
│   ├─ .user.address.state: pg\string
│   └─ .user.address.zip: pg\string
│
├─ .user.roles: pg\array{#Role}     // Array of Role enumeration
├─ .user.created_at: pg\dt          // Creation timestamp
├─ .user.updated_at: pg\dt          // Last update timestamp
│
└─ .user.pgvar.*                    // Reserved metadata (ALWAYS Ready)
    ├─ .user.pgvar.state: #PgVar.States.Ready
    ├─ .user.pgvar.errors: pg\array{!}   // Empty if Ready
    └─ .user.pgvar.history.*
        ├─ .user.pgvar.history.Declared.at: pg\dt
        └─ .user.pgvar.history.Ready.at: pg\dt
```

---

## Pipeline Hierarchies

### Complete Pipeline Namespace Map

```
|Pipelines                          // All pipeline namespaces (| prefix)
│
├─ |DT.*                            // DateTime pipelines (see DT tree below)
│
├─ |U.*                             // Utility pipelines
│   ├─ |U.String.*                  // String utilities
│   │   ├─ |U.String                // Default string formatter → pg\string
│   │   └─ |U.String.{Lang}.*       // Language-specific
│   │       └─ |U.String.{Lang}.{Type}.*
│   │           └─ |U.String.{Lang}.{Type}.{Format}  // → pg\string
│   │
│   ├─ |U.YAML.*                    // YAML utilities
│   │   ├─ |U.YAML.Load             // Load YAML file → pg\yaml
│   │   ├─ |U.YAML.Parse            // Parse YAML string → pg\yaml
│   │   └─ |U.YAML.Stringify        // Convert to YAML → pg\string
│   │
│   └─ |U.JSON.*                    // JSON utilities
│       ├─ |U.JSON.Parse            // → pg\serial
│       └─ |U.JSON.Stringify        // → pg\string
│
├─ |T.*                             // Trigger pipelines
│   ├─ |T.Call                      // Manual call trigger
│   ├─ |T.Cron                      // Cron-based trigger
│   ├─ |T.FileWatch                 // File system trigger
│   ├─ |T.HTTP                      // HTTP endpoint trigger
│   ├─ |T.Daily                     // Daily recurrence trigger
│   ├─ |T.Weekly                    // Weekly recurrence trigger
│   └─ |T.DT.*                      // DateTime triggers (mirrors |DT.*)
│
├─ |W.*                             // Wrapper pipelines
│   ├─ |W.Polyglot.*                // Polyglot wrappers
│   │   └─ |W.Polyglot.Scope        // RAII scope placeholder
│   │
│   └─ |RT.*                        // Runtime wrappers
│       ├─ |RT.Python               // Python runtime wrapper
│       ├─ |RT.Rust                 // Rust runtime wrapper
│       ├─ |RT.Node                 // Node.js runtime wrapper
│       └─ |RT.Go                   // Go runtime wrapper
│
└─ |Q.*                             // Queue control pipelines
    └─ |Q.Priority                  // Priority queue
```

### DT.* DateTime Pipeline Tree (Complete)

```
DT.*                                // DateTime namespace root
│
├─ DT.Now                           // Current instant (pg\dt)
│   └─ DT.Now.Local                 // Current instant (local timezone)
│
├─ DT.Time                          // Time only (param: "15:00", "3:00PM")
├─ DT.Date                          // Date only (param: "2025-11-30")
├─ DT.Day                           // Day of week (param: "Monday")
│
├─ DT.Ago                           // Past time (param: "2h", "3d")
├─ DT.From                          // Future time (param: "2h", "30m")
├─ DT.ToNow                         // Duration to now (param: "{.start}")
├─ DT.Between                       // Duration between instants
│
├─ DT.Seconds                       // Duration in seconds (param: "30")
├─ DT.Minutes                       // Duration in minutes (param: "5")
├─ DT.Hours                         // Duration in hours (param: "2")
├─ DT.Days                          // Duration in days (param: "7")
├─ DT.Weeks                         // Duration in weeks (param: "2")
├─ DT.Months                        // Duration in months (param: "3")
├─ DT.Years                         // Duration in years (param: "1")
│
├─ DT.Daily                         // Daily recurrence (param: "9:00AM")
├─ DT.Weekly                        // Weekly recurrence (param: "Monday 9:00AM")
├─ DT.Monthly                       // Monthly recurrence (param: "15")
├─ DT.Yearly                        // Yearly recurrence (param: "12-25")
│
├─ DT.Every.*                       // Generic recurrence
│   ├─ DT.Every.Second              // Every second
│   ├─ DT.Every.Minute              // Every minute
│   ├─ DT.Every.Hour                // Every hour
│   ├─ DT.Every.Day                 // Every day
│   ├─ DT.Every.Week                // Every week
│   ├─ DT.Every.Month               // Every month
│   └─ DT.Every.Year                // Every year
│
├─ DT.Add                           // Add duration to instant
├─ DT.Subtract                      // Subtract duration from instant
│
├─ DT.Year                          // Extract year component
├─ DT.Month                         // Extract month component
├─ DT.Day                           // Extract day component
├─ DT.Hour                          // Extract hour component
├─ DT.Minute                        // Extract minute component
├─ DT.Second                        // Extract second component
├─ DT.DayOfWeek                     // Extract day of week
│
├─ DT.IsBefore                      // Comparison: before
├─ DT.IsAfter                       // Comparison: after
├─ DT.IsBetween                     // Comparison: between range
├─ DT.Contains                      // Duration contains instant
│
├─ DT.Overlaps                      // Duration overlap check
├─ DT.Intersection                  // Duration intersection
├─ DT.Union                         // Duration union
├─ DT.Gap                           // Gap between durations
│
├─ DT.ToUTC                         // Convert to UTC
├─ DT.ToLocal                       // Convert to local timezone
│
├─ DT.Gregorian.*                   // Gregorian calendar
│   ├─ DT.Gregorian                 // Base Gregorian (param: "2025-11-30")
│   ├─ DT.Gregorian.Yearly          // Annual recurrence
│   └─ DT.Gregorian.{Month}.*       // Month namespace
│       └─ DT.Gregorian.{Month}.{Occurrence}.*
│           └─ DT.Gregorian.{Month}.{Occurrence}.{DayOfWeek}
│                                   // Example: DT.Gregorian.November.Fourth.Thursday
│
├─ DT.Julian.*                      // Julian calendar
│   └─ DT.Julian                    // Base Julian
│
├─ DT.Assyrian.*                    // Assyrian/Syriac calendar
│   └─ DT.Assyrian                  // Base Assyrian
│
├─ DT.Hijri.*                       // Islamic calendar (extendable)
│   ├─ DT.Hijri                     // Algorithmic Islamic
│   ├─ DT.Hijri.SaudiArabia*        // Saudi Arabia profile
│   ├─ DT.Hijri.UKMoonSighting*     // UK profile
│   ├─ DT.Hijri.Turkey*             // Turkey profile
│   └─ DT.Hijri.{UserDefined}*      // User-defined profiles
│
├─ DT.Hebrew.*                      // Hebrew calendar (extendable)
│   ├─ DT.Hebrew                    // Algorithmic Hebrew
│   ├─ DT.Hebrew.Sephardic*         // Sephardic tradition
│   ├─ DT.Hebrew.Ashkenazi*         // Ashkenazi tradition
│   └─ DT.Hebrew.{UserDefined}*     // User-defined profiles
│
├─ DT.Chinese.*                     // Chinese calendar (extendable)
│   ├─ DT.Chinese                   // Algorithmic Chinese
│   ├─ DT.Chinese.Mainland*         // PRC observance
│   ├─ DT.Chinese.Taiwan*           // Taiwan observance
│   └─ DT.Chinese.{UserDefined}*    // User-defined profiles
│
├─ DT.TimeZone.*                    // Timezone namespace (extendable)
│   ├─ DT.TimeZone.US.*             // US timezones
│   │   ├─ DT.TimeZone.US.Eastern
│   │   ├─ DT.TimeZone.US.Central
│   │   ├─ DT.TimeZone.US.Mountain
│   │   └─ DT.TimeZone.US.Pacific
│   │
│   ├─ DT.TimeZone.Europe.*         // European timezones
│   │   ├─ DT.TimeZone.Europe.London
│   │   ├─ DT.TimeZone.Europe.Paris
│   │   └─ DT.TimeZone.Europe.Berlin
│   │
│   ├─ DT.TimeZone.Asia.*           // Asian timezones
│   │   ├─ DT.TimeZone.Asia.Tokyo
│   │   ├─ DT.TimeZone.Asia.Shanghai
│   │   └─ DT.TimeZone.Asia.Dubai
│   │
│   └─ DT.TimeZone.{UserDefined}*   // User-defined timezones
│
├─ DT.Alias.*                       // DateTime aliases (user-extendable)
│   └─ DT.Alias.{UserDefined}*      // User-defined shortcuts
│
├─ DT.Holiday.*                     // Holiday definitions (user-extendable)
│   └─ DT.Holiday.{UserDefined}*    // User-defined holidays
│
├─ DT.Format.*                      // Format macros (user-extendable)
│   └─ DT.Format.{UserDefined}*     // User-defined formats
│
└─ DT.Convert.*                     // Calendar conversions
    ├─ DT.Convert.Gregorian
    ├─ DT.Convert.Hijri
    ├─ DT.Convert.Hebrew
    └─ DT.Convert.Chinese
```

---

## Unpack Operator Hierarchies

### Unpack Operators (~)

```
~ForEach                            // Iterate over collection items
~Enumerate                          // Iterate with index (item, index)
~Zip                                // Zip multiple collections together
~Flatten                            // Flatten nested collections
~Filter                             // Filter collection by predicate
~Map                                // Transform each item in collection
```

**Usage Example:**
```polyglot
[p] ~ForEach                        // Unpack operator
[<] .items: pg\array{pg\int}        // Input collection
[>] .item: pg\int >> .result        // Each item output
```

---

## Join Operator Hierarchies

### Join/Pack Operators (~Y)

```
~Y.IntoArray                        // Pack parallel results → pg\array{T}
~Y.IntoSet                          // Pack parallel results → pg\set{T}
~Y.IntoSerial                       // Pack parallel results → pg\serial
```

**Usage Example:**
```polyglot
[p] ~ForEach                        // Parallel unpack
[<] .items: pg\array{pg\int}
[Y] ~Y.IntoArray                    // Join results into array
[>] .results: pg\array{pg\int}
```

---

## Enumeration Hierarchies

### Reserved Enumerations

```
#PgVar.*                            // Polyglot runtime enumerations
│
├─ #PgVar.States.*                  // Variable states
│   ├─ #PgVar.States.Declared       // Type defined, no value
│   ├─ #PgVar.States.DefaultReady   // Has default value
│   ├─ #PgVar.States.Pending        // Async operation in progress
│   ├─ #PgVar.States.Ready          // Final immutable value
│   ├─ #PgVar.States.Faulted        // Operation failed
│   ├─ #PgVar.States.Retrying       // Post-MVP: Retry in progress
│   ├─ #PgVar.States.Paused         // Post-MVP: Execution paused
│   ├─ #PgVar.States.Cached         // Post-MVP: Cached value
│   └─ #PgVar.States.Dirty          // Post-MVP: Cache invalidated
│
└─ #Boolean.*                       // Boolean enumeration
    ├─ #Boolean.True
    └─ #Boolean.False

#Pipeline.*                         // Pipeline markers
└─ !No.Input                // No input marker

#None                               // Unit type (no fields)
```

### User-Defined Enumeration Example

```
#UserProfile                        // User profile enumeration
│
├─ #UserProfile.role.*              // Role variants (enum namespace)
│   ├─ #UserProfile.role.Admin      // Admin role (enum field - no type)
│   ├─ #UserProfile.role.Moderator  // Moderator role (enum field - no type)
│   ├─ #UserProfile.role.User       // Regular user (enum field - no type)
│   └─ #UserProfile.role.Guest      // Guest role (enum field - no type)
│
├─ #UserProfile.status.*            // Status variants (enum namespace)
│   ├─ #UserProfile.status.Active   // Active status (enum field - no type)
│   ├─ #UserProfile.status.Suspended// Suspended (enum field - no type)
│   └─ #UserProfile.status.Deleted  // Deleted (enum field - no type)
│
├─ #UserProfile.id: pg\int          // Serial field (HAS type)
├─ #UserProfile.email: pg\string    // Serial field (HAS type)
├─ #UserProfile.username: pg\string // Serial field (HAS type)
│
├─ #UserProfile.preferences: pg\serial  // Nested serial (HAS type)
│   ├─ #UserProfile.preferences.theme: pg\string
│   ├─ #UserProfile.preferences.language: pg\string
│   └─ #UserProfile.preferences.notifications: pg\bool
│
├─ #UserProfile.created_at: pg\dt   // Serial field (HAS type)
└─ #UserProfile.last_login: pg\dt   // Serial field (HAS type)
```

### Enum vs Serial Mixing Rules

```
#ValidMixing                        // ✅ VALID: Enum and serial as uncles
│
├─ #ValidMixing.category.*          // Enum namespace
│   ├─ #ValidMixing.category.Option1  // Enum field (no type)
│   └─ #ValidMixing.category.Option2  // Enum field (no type)
│
└─ #ValidMixing.metadata: pg\string // Serial field (HAS type, different level)

#InvalidMixing                      // ❌ INVALID: Enum and serial as siblings
├─ #InvalidMixing.EnumField         // ❌ Enum field (no type)
└─ #InvalidMixing.serial_field: pg\string  // ❌ Serial field (has type)
                                    // Cannot mix enum/serial as siblings!
```

---

## Error Hierarchies

### Error Namespace Structure

```
!Error.*                            // Error namespace root
│
├─ !No.Output                         // No error marker (reserved)
│
├─ !Network.*                       // Network errors
│   ├─ !Network.Timeout.*
│   │   ├─ !Network.Timeout.Connection
│   │   ├─ !Network.Timeout.Read
│   │   └─ !Network.Timeout.Write
│   │
│   ├─ !Network.DNS.*
│   │   ├─ !Network.DNS.NotFound
│   │   ├─ !Network.DNS.Unreachable
│   │   └─ !Network.DNS.InvalidResponse
│   │
│   └─ !Network.HTTP.*
│       ├─ !Network.HTTP.4xx.*      // Client errors
│       │   ├─ !Network.HTTP.4xx.BadRequest          // 400
│       │   ├─ !Network.HTTP.4xx.Unauthorized        // 401
│       │   ├─ !Network.HTTP.4xx.Forbidden           // 403
│       │   ├─ !Network.HTTP.4xx.NotFound            // 404
│       │   └─ !Network.HTTP.4xx.TooManyRequests     // 429
│       │
│       └─ !Network.HTTP.5xx.*      // Server errors
│           ├─ !Network.HTTP.5xx.InternalError       // 500
│           ├─ !Network.HTTP.5xx.BadGateway          // 502
│           └─ !Network.HTTP.5xx.ServiceUnavailable  // 503
│
├─ !Database.*                      // Database errors
│   ├─ !Database.Connection.*
│   │   ├─ !Database.Connection.Refused
│   │   └─ !Database.Connection.Timeout
│   │
│   ├─ !Database.Query.*
│   │   ├─ !Database.Query.SyntaxError
│   │   └─ !Database.Query.ConstraintViolation
│   │
│   └─ !Database.Transaction.*
│       ├─ !Database.Transaction.Deadlock
│       └─ !Database.Transaction.Rollback
│
├─ !FileSystem.*                    // File system errors
│   ├─ !FileSystem.NotFound
│   ├─ !FileSystem.PermissionDenied
│   ├─ !FileSystem.AlreadyExists
│   └─ !FileSystem.DiskFull
│
├─ !Validation.*                    // Validation errors
│   ├─ !Validation.Required
│   ├─ !Validation.InvalidFormat
│   ├─ !Validation.OutOfRange
│   └─ !Validation.TypeMismatch
│
└─ !Runtime.*                       // Runtime errors
    ├─ !Runtime.PushCountExceeded
    ├─ !Runtime.InvalidStateTransition
    ├─ !Runtime.TypeConversionFailed
    └─ !Runtime.PipelineNotFound
```

---

## Type System Hierarchies

### Primitive Types

```
pg\*                                // Polyglot primitive types
│
├─ pg\string                        // String type
├─ pg\int                           // Integer type (64-bit signed)
├─ pg\float                         // Floating point (64-bit)
├─ pg\bool                          // Boolean type
├─ pg\dt                            // DateTime type (unified date/time/duration)
├─ pg\path                          // File system path type
│
├─ pg\serial                        // Serial (struct) type
│
├─ pg\array{T}                      // Array collection (ordered)
│   ├─ pg\array{pg\int}             // Integer array
│   ├─ pg\array{pg\string}          // String array
│   ├─ pg\array{#Enum}              // Enumeration array
│   └─ pg\array{pg\serial}          // Serial array
│
└─ pg\set{T}                        // Set collection (unordered, unique)
    ├─ pg\set{pg\int}               // Integer set
    ├─ pg\set{pg\string}            // String set
    └─ pg\set{#Enum}                // Enumeration set
```

### Foreign Types

```
Foreign Types                       // External runtime types
│
├─ py\*                             // Python types
│   ├─ py\str
│   ├─ py\int
│   ├─ py\float
│   ├─ py\bool
│   ├─ py\list
│   ├─ py\dict
│   └─ py\tuple
│
├─ rs\*                             // Rust types
│   ├─ rs\String
│   ├─ rs\i32
│   ├─ rs\i64
│   ├─ rs\f64
│   ├─ rs\bool
│   ├─ rs\Vec
│   └─ rs\HashMap
│
├─ js\*                             // JavaScript types
│   ├─ js\string
│   ├─ js\number
│   ├─ js\boolean
│   ├─ js\Array
│   └─ js\Object
│
├─ node\*                           // Node.js types
│   └─ node\Buffer
│
└─ go\*                             // Go types
    ├─ go\string
    ├─ go\int
    ├─ go\float64
    ├─ go\bool
    └─ go\slice
```

---

## Reserved Namespace Hierarchies

### Complete Reserved Namespace Map

```
Reserved Namespaces                 // All reserved hierarchies
│
├─ .*.pgvar.*                       // Variable metadata (ALWAYS Ready)
│   ├─ .*.pgvar.state               // #PgVar.States.*
│   ├─ .*.pgvar.errors              // pg\array{!}
│   └─ .*.pgvar.history.*           // State transition history
│       └─ .*.pgvar.history.{State}.at  // pg\dt (timestamp)
│
├─ #PgVar.*                         // Runtime enumerations
│   ├─ #PgVar.States.*
│   └─ #Boolean.*
│
├─ #Pipeline.*                      // Pipeline markers
│   └─ !No.Input
│
├─ !No.Output                         // No error marker
│
├─ DT.*                             // DateTime namespace (100+ pipelines)
│
├─ U.*                              // Utility namespace
│   ├─ U.String.*
│   ├─ U.YAML.*
│   └─ U.JSON.*
│
├─ T.*                              // Trigger namespace
│   ├─ T.Call
│   ├─ T.Cron
│   ├─ T.FileWatch
│   ├─ T.HTTP
│   ├─ T.Daily
│   ├─ T.Weekly
│   └─ T.DT.*
│
├─ W.*                              // Wrapper namespace
│   ├─ W.Polyglot.Scope
│   └─ RT.*
│       ├─ RT.Python
│       ├─ RT.Rust
│       ├─ RT.Node
│       └─ RT.Go
│
└─ Q.*                              // Queue namespace
    └─ Q.Priority
```

---

## Block Element Hierarchies

### Pipeline Block Structure Tree

```
[|] PipelineName                    // Pipeline declaration
│
├─ [i] Inputs                       // Input declarations (MANDATORY)
│   ├─ [i] .input1: Type            // Schema-only input
│   ├─ [i] .input2: Type <~ default // Default input
│   ├─ [i] .input3: Type << value   // Constant input
│   └─ [i] !No.Input        // No input marker
│
├─ [t] Triggers                     // Trigger declarations (MANDATORY)
│   ├─ [t] |T.Call                  // Manual call
│   ├─ [t] |T.Cron                  // Cron trigger
│   │   └─ [<] Config bindings
│   ├─ [t] |T.FileWatch
│   │   └─ [<] Config bindings
│   └─ [t] |T.HTTP
│       └─ [<] Config bindings
│
├─ [Q] Queue Config                 // Queue configuration (optional)
│   └─ [<] Queue bindings
│
├─ [W] Wrapper                      // Wrapper (MANDATORY: option A)
│   ├─ [W] |W.Polyglot.Scope        // Scope placeholder
│   ├─ [W] RT.Python"script.py"     // Runtime wrapper
│   └─ [<] Wrapper config bindings
│
├─ [\] Setup                        // Setup (MANDATORY: option B if no [W])
│   └─ Setup blocks
│
├─ Execution Blocks                 // Body (optional but common)
│   │
│   ├─ [r] Sequential               // Sequential execution
│   │   ├─ [r] .variable: Type << value
│   │   └─ [r] |Pipeline            // Pipeline call
│   │       ├─ [<] Input bindings
│   │       └─ [>] Output bindings
│   │
│   ├─ [?] Conditional              // Conditional execution
│   │   ├─ [?] condition1
│   │   │   └─ [~] Body
│   │   │       └─ Nested blocks
│   │   │   [~]
│   │   │
│   │   ├─ [?] condition2
│   │   │   └─ [~] Body
│   │   │   [~]
│   │   │
│   │   └─ [?] *?                   // Catch-all (MANDATORY)
│   │       └─ [~] Body
│   │       [~]
│   │
│   ├─ {[p] Parallel}               // Parallel execution
│   │   ├─ [p] Task 1
│   │   ├─ [p] Task 2
│   │   └─ [Y] Join point           // Join results
│   │
│   ├─ [b] Background               // Background execution
│   │
│   └─ [s] Streaming                // Streaming execution
│
├─ [/] Cleanup                      // Cleanup (if [\] used)
│   └─ Cleanup blocks
│
├─ [o] Outputs                      // Output declarations (MANDATORY)
│   ├─ [o] .output1: Type
│   ├─ [o] .output2: Type
│   └─ [o] !No.Output                 // No error marker
│
└─ [X]                              // Pipeline end (MANDATORY)
```

### Nested Conditional Example

```
[?] .status =? #Status.Active       // Top-level condition
├─ [~]                              // Body start
│   ├─ [r] .priority: pg\int << 1   // Sequential block
│   │
│   └─ [?] .user.role =? #Role.Admin  // Nested condition
│       ├─ [~]
│       │   └─ [r] .priority << 0     // Higher priority for admin
│       └─ [~]
│       │
│       [?] *?                        // Nested catch-all
│       └─ [~]
│           └─ [r] .priority << 2     // Default priority
│       [~]
└─ [~]                              // Body end
│
[?] *?                              // Top-level catch-all (MANDATORY)
└─ [~]
    └─ [r] .priority << 10          // Inactive users
[~]
```

---

## Operator Prefix Map

### Complete Operator Hierarchy

```
Polyglot Operators                  // All operator prefixes
│
├─ . (dot)                          // Variables (with types)
│   └─ .variable.field.subfield: lang\type   // Unlimited nesting
│
├─ # (hash)                         // Enumerations
│   ├─ #Enum.variant.*              // Enum fields (NO type)
│   └─ #Enum.field: lang\type       // Serial fields (HAS type)
│
├─ | (pipe)                         // Pipelines, Triggers, Wrappers
│   ├─ |Pipeline.Namespace.Name.*   // Pipeline namespace
│   ├─ |T.Trigger.*                 // Trigger namespace
│   └─ |W.Wrapper.*                 // Wrapper namespace
│
├─ ~ (tilde)                        // Unpack operators
│   ├─ ~ForEach                     // Iterate collection
│   ├─ ~Enumerate                   // Iterate with index
│   └─ ~Zip                         // Zip collections
│
├─ ~Y (tilde-Y)                     // Join/Pack operators
│   ├─ ~Y.IntoArray                 // Pack into array
│   ├─ ~Y.IntoSet                   // Pack into set
│   └─ ~Y.IntoSerial                // Pack into serial
│
└─ ! (bang)                         // Errors
    └─ !Error.Category.Specific.*   // Unlimited nesting
```

---

## Quick Reference Summary

### Hierarchy Applies To:

1. **Variables:** `.variable.field.subfield: pg\type`
2. **Pipelines:** `|DT.Gregorian.November.Fourth.Thursday`
3. **Enumerations:** `#UserProfile.role.Admin` (enum - no type) or `#UserProfile.email: pg\string` (serial - has type)
4. **Errors:** `!Network.HTTP.4xx.NotFound`
5. **Unpack Operators:** `~ForEach`, `~Enumerate`, `~Zip`
6. **Join Operators:** `~Y.IntoArray`, `~Y.IntoSet`, `~Y.IntoSerial`
7. **Reserved Namespaces:** `.*.pgvar.history.Ready.at: pg\dt`
8. **Block Elements:** `[|] → [i] → [t] → [W] → [r] → [o] → [X]`

### Tree Notation Key:

- `│` = Vertical continuation
- `├─` = Branch (has siblings below)
- `└─` = Last branch (no siblings)
- `*` = Extendable namespace (users can add)

---

**See Also:**
- [Hierarchy Tree Notation Standard](/docs/technical/hierarchy-tree-notation.md)
- [DT Pipeline Tree](/docs/technical/dt-pipeline-tree.md)
- [Type System](/docs/user/language/type-system.md)
- [Variables](/docs/user/language/variables.md)
- [Enumerations](/docs/user/language/enumerations.md)

---

**Last Updated:** 2025-11-30
**Version:** 0.0.2
**Status:** Canonical reference for all Polyglot hierarchies
