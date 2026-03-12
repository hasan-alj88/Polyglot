# Polyglot Hierarchy Tree Notation Standard

**Version:** 0.0.2
**Last Updated:** 2025-11-30
**Purpose:** Standard notation for documenting Polyglot's hierarchical structures

---

## Overview

**Core Principle:** ALL Polyglot data has serial hierarchy - variables, pipelines, enumerations, errors, and block elements use dot notation to create tree structures.

**Documentation Requirement:** All hierarchical structures MUST be documented with ASCII tree diagrams.

---

## Tree Notation Standard

### Basic Symbols

```
│   Vertical line (continuation)
├─  Branch (has siblings below)
└─  Last branch (no siblings below)
*   Extendable (users can add)
```

### Tree Template

```
Root.*                              // Root description
│
├─ Root.Branch1                     // First branch description
│   ├─ Root.Branch1.Leaf1           // Nested item
│   └─ Root.Branch1.Leaf2           // Last nested item
│
├─ Root.Branch2                     // Second branch
│   └─ Root.Branch2.*               // Extendable namespace
│
└─ Root.Branch3                     // Last branch
    └─ Root.Branch3.Final           // Deeply nested
```

### Extendable Namespaces

Use `*` to indicate user-extendable namespaces:

```
#DT.Hijri.*                         // Extendable enumeration
│
├─ #DT.Hijri.SaudiArabia            // Built-in profile
├─ #DT.Hijri.UKMoonSighting         // Built-in profile
└─ #DT.Hijri.{UserDefined}*         // User can add custom profiles
```

---

## Variable Hierarchy Trees

### Basic Variable Namespace

```
.variable: pg\serial                // Root variable with type
│
├─ .variable.field: pg\string       // Nested field with type
│   ├─ .variable.field.subfield: pg\int    // Deeper nesting with type
│   └─ .variable.field.other: pg\bool     // Sibling field with type
│
└─ .variable.another: pg\dt         // Another field with type
```

### Reserved Variable Namespace (.*.pgvar.*)

```
.variable.pgvar.*                   // Reserved metadata namespace
│
├─ .variable.pgvar.state: #PgVar.States.*   // Current state (enumeration)
│
├─ .variable.pgvar.errors: pg\array{!}      // Error array
│
└─ .variable.pgvar.history.*        // State transition history
    ├─ .variable.pgvar.history.Declared.at: pg\dt
    ├─ .variable.pgvar.history.Pending.at: pg\dt
    ├─ .variable.pgvar.history.Ready.at: pg\dt
    └─ .variable.pgvar.history.Faulted.at: pg\dt
```

### Example: User Profile Variable

```
.user: pg\serial                    // User data
│
├─ .user.id: pg\int                 // User ID
├─ .user.email: pg\string           // Email address
├─ .user.name: pg\serial            // Name (nested serial)
│   ├─ .user.name.first: pg\string
│   ├─ .user.name.last: pg\string
│   └─ .user.name.middle: pg\string
│
├─ .user.roles: pg\array{#Role}     // Array of Role enumeration
│
└─ .user.pgvar.*                    // Reserved metadata (ALWAYS Ready)
    ├─ .user.pgvar.state: #PgVar.States.*
    └─ .user.pgvar.history.*
        └─ .user.pgvar.history.Ready.at: pg\dt
```

---

## Pipeline Hierarchy Trees

### Pipeline Namespace Pattern

```
|Namespace.*                        // Pipeline namespace root
│
├─ |Namespace.Operation             // Operation pipeline → pg\dt
│
├─ |Namespace.SubNamespace.*        // Nested namespace
│   ├─ |Namespace.SubNamespace.Op1  // → pg\string
│   └─ |Namespace.SubNamespace.Op2  // → pg\int
│
└─ |Namespace.{Calendar}.*          // Parameterized namespace
    └─ |Namespace.{Calendar}.{Profile}.*  // → pg\dt
```

### Example: DT.* Pipeline Tree (Excerpt)

```
|DT.*                               // DateTime namespace root
│
├─ |DT.Now                          // Current instant → pg\dt
│   └─ |DT.Now.Local                // Local timezone variant → pg\dt
│
├─ |DT.Gregorian.*                  // Gregorian calendar
│   ├─ |DT.Gregorian.November.*     // Month namespace
│   │   ├─ |DT.Gregorian.November.First.*
│   │   │   └─ |DT.Gregorian.November.First.Monday  // → pg\dt
│   │   └─ |DT.Gregorian.November.Fourth.*
│   │       └─ |DT.Gregorian.November.Fourth.Thursday  // → pg\dt
│   └─ |DT.Gregorian.Yearly         // → pg\dt
│
└─ |DT.Hijri.*                      // Islamic calendar (extendable)
    ├─ |DT.Hijri                    // Algorithmic → pg\dt
    ├─ |DT.Hijri.SaudiArabia*       // Built-in profile → pg\dt
    └─ |DT.Hijri.{UserDefined}*     // User-extendable → pg\dt
```

---

## Enumeration Hierarchy Trees

### Basic Enumeration Structure

```
#Enumeration                        // Root enumeration
│
├─ #Enumeration.Variant1            // Enum field (no type)
├─ #Enumeration.Variant2            // Enum field (no type)
│
└─ #Enumeration.field: pg\string    // Serial field (has type)
```

### Nested Enumeration Example

```
#UserProfile                        // Root enumeration
│
├─ #UserProfile.role.*              // Nested enum namespace
│   ├─ #UserProfile.role.Admin      // Enum variant (no type)
│   ├─ #UserProfile.role.User       // Enum variant (no type)
│   └─ #UserProfile.role.Guest      // Enum variant (no type)
│
├─ #UserProfile.name: pg\string     // Serial field (has type)
├─ #UserProfile.email: pg\string    // Serial field (has type)
│
└─ #UserProfile.settings: pg\serial // Nested serial (has type)
    ├─ #UserProfile.settings.theme: pg\string
    └─ #UserProfile.settings.lang: pg\string
```

### Reserved Enumeration Example

```
#PgVar.*                            // Reserved runtime namespace
│
├─ #PgVar.States.*                  // Variable states
│   ├─ #PgVar.States.Declared
│   ├─ #PgVar.States.DefaultReady
│   ├─ #PgVar.States.Pending
│   ├─ #PgVar.States.Ready
│   ├─ #PgVar.States.Faulted
│   ├─ #PgVar.States.Retrying       // Post-MVP
│   ├─ #PgVar.States.Paused         // Post-MVP
│   ├─ #PgVar.States.Cached         // Post-MVP
│   └─ #PgVar.States.Dirty          // Post-MVP
│
└─ #Boolean.*
    ├─ #Boolean.True
    └─ #Boolean.False
```

---

## Unpack Operator Hierarchy Trees

### Unpack Operators (~)

```
~ForEach                            // Iterate over collection
~Enumerate                          // Iterate with index
~Zip                                // Zip multiple collections
~Flatten                            // Flatten nested collections
~Filter                             // Filter collection
~Map                                // Transform collection
```

**Usage in Pipeline:**
```polyglot
[p] ~ForEach
[<] .items: pg\array{T}
[>] .item: T >> .result
```

---

## Join Operator Hierarchy Trees

### Join/Pack Operators (~Y)

```
~Y.IntoArray                        // Pack results into pg\array{T}
~Y.IntoSet                          // Pack results into pg\set{T}
~Y.IntoSerial                       // Pack results into pg\serial
```

**Usage in Pipeline:**
```polyglot
[p] ~ForEach
[<] .items: pg\array{T}
[Y] ~Y.IntoArray
[>] .results: pg\array{T}
```

---

## Error Hierarchy Trees

### Error Type Namespace

```
!ErrorNamespace.*                   // Error namespace root
│
├─ !ErrorNamespace.Category1.*      // Error category
│   ├─ !ErrorNamespace.Category1.Specific1
│   └─ !ErrorNamespace.Category1.Specific2
│
└─ !ErrorNamespace.Category2.*      // Another category
    └─ !ErrorNamespace.Category2.Detail
```

### Example: Network Errors

```
!Network.*                          // Network error namespace
│
├─ !Network.Timeout.*               // Timeout errors
│   ├─ !Network.Timeout.Connection
│   ├─ !Network.Timeout.Read
│   └─ !Network.Timeout.Write
│
├─ !Network.DNS.*                   // DNS errors
│   ├─ !Network.DNS.NotFound
│   └─ !Network.DNS.Unreachable
│
└─ !Network.HTTP.*                  // HTTP errors
    ├─ !Network.HTTP.4xx.*
    │   ├─ !Network.HTTP.4xx.BadRequest
    │   ├─ !Network.HTTP.4xx.Unauthorized
    │   └─ !Network.HTTP.4xx.NotFound
    └─ !Network.HTTP.5xx.*
        ├─ !Network.HTTP.5xx.InternalError
        └─ !Network.HTTP.5xx.ServiceUnavailable
```

---

## Block Element Hierarchy Trees

### Pipeline Block Structure

```
[|] PipelineName                    // Pipeline declaration
│
├─ [i] Inputs                       // Input declarations
│   ├─ [i] .input1: Type
│   └─ [i] .input2: Type <~ default
│
├─ [t] Triggers                     // Trigger declarations
│   ├─ [t] |T.Call
│   └─ [t] |T.Cron""
│
├─ [W] Wrapper                      // Wrapper/Setup
│   └─ [<] Config bindings
│
├─ Execution Blocks                 // Body
│   ├─ [r] Sequential blocks
│   │   ├─ [r] .variable: Type << value
│   │   └─ [r] |Pipeline
│   │       ├─ [<] Input bindings
│   │       └─ [>] Output bindings
│   │
│   ├─ [?] Conditional blocks
│   │   ├─ [?] condition
│   │   │   └─ [~] Body
│   │   └─ [?] *?                   // Catch-all
│   │       └─ [~] Body
│   │
│   └─ [p] Parallel blocks
│       └─ [Y] Join point
│
├─ [o] Outputs                      // Output declarations
│   ├─ [o] .output1: Type
│   └─ [o] .output2: Type
│
└─ [X]                              // Pipeline end
```

### Nested Block Example

```
[|] ComplexPipeline
│
├─ [i] .data: pg\string
│
├─ [t] |T.Call
│
├─ [W] |W.Polyglot.Scope
│
├─ [r] .result: pg\serial           // Variable declaration
│
├─ [?] .data =? "value"             // Conditional
│   ├─ [~]                          // Body start
│   │   ├─ [r] |ProcessA            // Nested pipeline call
│   │   │   ├─ [<] .input << .data  // Input binding
│   │   │   └─ [>] .output >> .temp // Output binding
│   │   │
│   │   └─ [?] .temp.pgvar.state =? #PgVar.States.Ready
│   │       ├─ [~]                  // Nested conditional body
│   │       │   └─ [r] .result << .temp
│   │       └─ [~]
│   │
│   │       [?] *?
│   │       └─ [~]
│   │           └─ [r] .result << "error"
│   │       [~]
│   └─ [~]                          // Body end
│
├─ [?] *?                           // Top-level catch-all
│   └─ [~]
│       └─ [r] .result << "default"
│   [~]
│
├─ [o] .result: pg\serial
│
└─ [X]
```

---

## Reserved Namespace Trees

### Complete Reserved Namespace Map

```
Reserved Namespaces                 // All reserved hierarchies
│
├─ .*.pgvar.*                       // Variable metadata
│   ├─ .*.pgvar.state
│   ├─ .*.pgvar.errors
│   └─ .*.pgvar.history.*
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
├─ DT.*                             // DateTime namespace
│   ├─ DT.{Calendar}.*
│   └─ DT.TimeZone.*
│
├─ U.*                              // Utility namespace
│   ├─ U.String.*
│   └─ U.YAML.*
│
├─ T.*                              // Trigger namespace
│   ├─ T.Call
│   ├─ T.Daily
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

## Type Hierarchy Trees

### Type System Namespace

```
pg\*                                // Polyglot primitive types
│
├─ pg\string                        // String type
├─ pg\int                           // Integer type
├─ pg\float                         // Float type
├─ pg\bool                          // Boolean type
├─ pg\dt                            // DateTime type
├─ pg\path                          // Path type
├─ pg\serial                        // Serial (struct) type
│
├─ pg\array{T}                      // Array collection
│   └─ pg\array{pg\int}             // Typed array example
│
└─ pg\set{T}                        // Set collection
    └─ pg\set{#Enum}                // Typed set example

Foreign Types
│
├─ py\*                             // Python types
│   ├─ py\str
│   ├─ py\int
│   └─ py\list
│
├─ rs\*                             // Rust types
│   ├─ rs\String
│   ├─ rs\i32
│   └─ rs\Vec
│
├─ js\*                             // JavaScript types
│   ├─ js\string
│   └─ js\Array
│
├─ node\*                           // Node.js types
│
└─ go\*                             // Go types
    ├─ go\string
    └─ go\slice
```

---

## Operator Prefix Trees

### Operator Namespace Map

```
Operators                           // All operator prefixes
│
├─ . (dot)                          // Variables
│   └─ .variable.field.subfield
│
├─ # (hash)                         // Enumerations
│   └─ #Enum.variant.nested
│
├─ | (pipe)                         // Pipelines
│   ├─ |Pipeline.Name
│   ├─ |T.Trigger                   // Triggers
│   └─ |W.Wrapper                   // Wrappers
│
└─ ! (bang)                         // Errors
    └─ !Error.Category.Specific
```

---

## Documentation Template

### Required Tree Sections

Every hierarchical structure documentation MUST include:

1. **Tree Diagram** - ASCII art showing structure
2. **Branch Descriptions** - Inline comments for each node
3. **Depth Indication** - Indentation shows nesting level
4. **Extendability Markers** - `*` for user-extendable namespaces
5. **Type Annotations** - Show types for serial fields
6. **Examples** - Real-world usage patterns

### Template Format

```markdown
## {Hierarchy Name}

### Tree Structure

\`\`\`
Root.*                              // Description
│
├─ Root.Branch1                     // Description
│   └─ Root.Branch1.Leaf            // Description (Type if applicable)
│
└─ Root.Branch2*                    // Extendable
    └─ Root.Branch2.{UserDefined}   // User-defined items
\`\`\`

### Usage Examples

\`\`\`polyglot
[r] .example: Type << Root.Branch1.Leaf
\`\`\`

### Extendability

Users can extend `Root.Branch2.*` by defining...
```

---

## Best Practices

### DO ✅
- Always include tree diagrams for hierarchical structures
- Use consistent indentation (4 spaces per level)
- Add inline comments for all branches
- Mark extendable namespaces with `*`
- Show type information for serial fields
- Include practical examples after tree diagrams

### DON'T ❌
- Don't omit tree diagrams for "simple" hierarchies
- Don't use inconsistent tree symbols
- Don't forget to show reserved namespaces
- Don't skip depth levels in nested structures
- Don't use prose when a tree would be clearer

---

## Tree Diagram Tools

### ASCII Tree Generation

```bash
# Use tree command for filesystem-based hierarchies
tree -L 3 --charset ascii

# Manual tree notation for conceptual hierarchies
# Use this template:
Root.*
│
├─ Branch1
│   └─ Leaf
└─ Branch2
```

### Verification Checklist

Before publishing documentation with tree diagrams:

- [ ] All hierarchical structures have tree diagrams
- [ ] Tree symbols are consistent (│ ├─ └─)
- [ ] Inline comments describe each node
- [ ] Extendable namespaces marked with `*`
- [ ] Types shown for serial fields
- [ ] Reserved namespaces clearly indicated
- [ ] Examples provided after each tree
- [ ] Tree matches actual implementation

---

## Related Documentation

- **DT Pipeline Tree:** `/docs/Tech/implementation/technical/dt-pipeline-tree.md` (reference implementation)
- **Type System:** `/docs/user/language/type-system.md`
- **Variables:** `/docs/user/language/01-variables.md`
- **Enumerations:** `/docs/user/language/enumerations.md`

---

**Standard Version:** 1.0
**Effective Date:** 2025-11-30
**Status:** REQUIRED for all hierarchical structure documentation
