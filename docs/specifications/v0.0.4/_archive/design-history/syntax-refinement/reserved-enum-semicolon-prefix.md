<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

# Reserved Enum Indication - Semicolon Prefix Proposal

**Date:** 2025-12-11
**Status:** 💡 USER PROPOSAL - Semicolon for Reserved Segments

---

## The Proposal

**Use `;` (semicolon) prefix to mark reserved segments in enum hierarchy**

### Key Insight

Instead of marking the entire enum as reserved, mark **individual segments** as reserved or user-defined.

---

## Syntax Examples

### Example 1: Fully Reserved Enum

```polyglot
#;Boolean;True                         // Both segments reserved
#;Boolean;False                        // Both segments reserved

// Breakdown:
// # - enum prefix
// ;Boolean - reserved namespace
// ;True - reserved field
```

### Example 2: Mixed Reserved/User Hierarchy

```polyglot
#;DT;Business.MyCompanyWeek;WorkingDays

// Breakdown:
// # - enum prefix
// ;DT - reserved namespace (datetime)
// ;Business - reserved sub-namespace
// .MyCompanyWeek - USER-DEFINED namespace
// ;WorkingDays - reserved field

// Meaning:
// - DT and Business are language-level reserved
// - MyCompanyWeek is user's custom calendar
// - WorkingDays is reserved enum field
```

### Example 3: User Extension of Reserved Namespace

```polyglot
// Standard library defines:
#;HTTP;Status;Ok                       // Fully reserved

// User extends with custom statuses:
#;HTTP;Status.Custom;RateLimited       // Mixed: HTTP/Status reserved, Custom/RateLimited user-defined

// Breakdown:
// ;HTTP - reserved (network namespace)
// ;Status - reserved (status codes)
// .Custom - USER-DEFINED (custom extensions)
// ;RateLimited - reserved field (or could be .RateLimited if user-defined)
```

---

## Visual Distinction

### Reserved vs User Segments

```polyglot
// All reserved
#;Boolean;True                         // ; ; (both reserved)

// Mixed
#;DT.MyCalendar;WorkingDays           // ; . ; (reserved, user, reserved)

// User-defined
#OrderStatus.Processing                // . . (both user-defined)
```

**Pattern:**
- `;` = Reserved segment (language/stdlib)
- `.` = User-defined segment

---

## Benefits

### 1. Granular Control

**Can extend reserved namespaces with user types:**

```polyglot
// Standard library:
#;Violation;Action;Raise
#;Violation;Action;Clip
#;Violation;Action;Transform

// User extends with custom actions:
#;Violation;Action.Custom;LogAndContinue
//                 ^^^^^^^ user-defined extension
```

### 2. Clear at Use Site

**IDE can immediately identify reserved vs user segments:**

```polyglot
[y] $status =? #;Boolean;True          // Syntax highlighting: both reserved

[y] $priority =? #OrderStatus.High     // Syntax highlighting: both user-defined

[y] $day =? #;DT.MyCompany;Monday     // Syntax highlighting: mixed
//             ^^           ^^^^^^
//         reserved     user   reserved
```

### 3. Namespace Organization

**Reserved namespaces can contain user extensions:**

```polyglot
#;HTTP;Status;Ok                       // 200
#;HTTP;Status;NotFound                 // 404
#;HTTP;Status.Custom;RateLimited       // User's custom status
#;HTTP;Status.Custom;MaintenanceMode   // User's custom status
```

### 4. Prevents Conflicts

**User cannot redefine reserved segments:**

```polyglot
// User tries to define:
{#} #;Boolean                          // ❌ COMPILE ERROR: ;Boolean is reserved
[.] .True
{x}

// But can extend:
{#} #;Boolean.Custom                   // ✅ OK: .Custom is user namespace
[.] .Maybe
{x}
```

### 5. IDE Autocomplete Intelligence

**IDE knows which completions to offer:**

```polyglot
#;DT;                                  // IDE offers: ;Business, ;Calendar, etc. (reserved)
#;DT;Business.                         // IDE offers: user's custom calendars
#;DT;Business.MyCompany;              // IDE offers: ;WorkingDays, ;Weekend, etc. (reserved)
```

---

## Standard Reserved Hierarchies

### Core Primitives

```polyglot
#;Boolean;True
#;Boolean;False

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

// User can extend:
#;Violation;Action.Custom;LogAndRetry
```

### I/O Streams

```polyglot
#;IO;Stream;SinglePush                // <
#;IO;Stream;SinglePull                // >
#;IO;Stream;DoublePush                // <<
#;IO;Stream;DoublePull                // >>
#;IO;Stream;TriplePush                // <<<
#;IO;Stream;TriplePull                // >>>

#;IO;Stream;Keys;Numeric              // Array indexing
#;IO;Stream;Keys;Named                // Map keys

// User can extend:
#;IO;Stream.Custom;QuadruplePush      // <<<<
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
// ... etc

// User extends with custom calendars:
#;DT;Business.FiscalYear;Q1
#;DT;Business.FiscalYear;Q2
```

### HTTP

```polyglot
#;HTTP;Method;GET
#;HTTP;Method;POST
#;HTTP;Method;PUT
#;HTTP;Method;DELETE

#;HTTP;Status;Ok                      // 200
#;HTTP;Status;Created                 // 201
#;HTTP;Status;BadRequest              // 400
#;HTTP;Status;NotFound                // 404
#;HTTP;Status;ServerError             // 500

// User custom status codes:
#;HTTP;Status.Custom;RateLimited
```

### Comparison/Ordering

```polyglot
#;Ordering;Less
#;Ordering;Equal
#;Ordering;Greater
```

---

## Grammar Rules

### Parser Rules

1. **Enum path starts with `#`**
2. **Segments separated by `;` or `.`**
3. **`;` marks reserved segment**
4. **`.` marks user-defined segment**
5. **Cannot redefine reserved segments**

### Valid Patterns

```polyglot
#;Reserved;Path                        // ✅ Fully reserved
#UserDefined.Path                      // ✅ Fully user-defined
#;Reserved.User;Reserved               // ✅ Mixed
#;Reserved;Path.User.Extension         // ✅ Mixed
```

### Invalid Patterns

```polyglot
#.;Invalid                             // ❌ Can't start with .
#;;DoubleReserved                      // ❌ Double semicolon
```

---

## IDE Configuration

### Syntax Highlighting Rules

```yaml
syntax_highlighting:
  enum:
    prefix: '#'

    reserved_segment:
      pattern: ';[A-Z][A-Za-z0-9]*'
      style:
        color: keyword_blue
        font_weight: bold

    user_segment:
      pattern: '\.[A-Z][A-Za-z0-9]*'
      style:
        color: type_green
        font_weight: normal

autocomplete:
  enum_path:
    after_semicolon:
      source: language_server_reserved
      priority: high

    after_dot:
      source: user_definitions
      priority: medium
```

### Visual Example in IDE

```polyglot
#;Boolean;True
 ^^^^^^^^ ^^^^
 blue/bold blue/bold (both reserved)

#OrderStatus.Processing
 ^^^^^^^^^^^ ^^^^^^^^^^
 green/normal green/normal (both user-defined)

#;DT;Business.MyCompanyWeek;WorkingDays
 ^^^ ^^^^^^^^ ^^^^^^^^^^^^^ ^^^^^^^^^^^
 blue green    green         blue
 bold normal   normal        bold
```

---

## Compiler Behavior

### Reserved Segment Protection

```polyglot
// User cannot define reserved segment
{#} #;Boolean                          // ❌ ERROR: ;Boolean is reserved
[.] .Maybe
{x}

// Compiler error:
// Error: Cannot redefine reserved enum ';Boolean'
// Reserved by: @Polyglot::Core:0.0.4.0
// Suggestion: Use user namespace: #Boolean.Custom
```

### Allowed User Extensions

```polyglot
// User CAN extend with user namespace
{#} #;Boolean.Extended                 // ✅ OK: .Extended is user-defined
[.] .Maybe
[.] .Unknown
{x}

// Usage:
[y] $value =? #;Boolean.Extended.Maybe
```

### Mixed Path Validation

```polyglot
{#} #;DT;Business.MyCompany           // ✅ OK: extends reserved with user namespace
[.] .FiscalQuarters
   [.] .Q1
   [.] .Q2
   [.] .Q3
   [.] .Q4
{x}

// Usage:
[y] $quarter =? #;DT;Business.MyCompany.FiscalQuarters.Q1
//                  ^^^ ^^^^^^^^ ^^^^^^^^^ ^^^^^^^^^^^^^^
//                  res reserved user      user
```

---

## Standard Library Organization

### @Polyglot::Core Reserved Enums

```
@Polyglot::Core:0.0.4.0
├── enums/
│   ├── boolean.pg             (#;Boolean)
│   ├── optional.pg            (#;Optional)
│   ├── result.pg              (#;Result)
│   ├── violation.pg           (#;Violation)
│   ├── io.pg                  (#;IO)
│   ├── ordering.pg            (#;Ordering)
│   ├── datetime.pg            (#;DT)
│   └── http.pg                (#;HTTP)
```

### Reserved Namespace Registry

```polyglot
// File: @Polyglot::Core/enums/boolean.pg
{@} @Polyglot::Core:0.0.4.0
{x}

{#} #;Boolean
[%] %Reserved
   [.] .namespace << ";Boolean"
   [.] .version << "0.0.4.0"

[.] ;True
[.] ;False
{x}
```

**Note:** Within the definition file, can use shorthand `;True` because `#;Boolean` prefix is implied.

---

## Comparison with `pg.*` Namespace Proposal

| Aspect | `pg.*` Namespace | `;` Prefix |
|--------|------------------|------------|
| Visual distinction | Moderate | **Excellent** |
| Granularity | Namespace-level only | **Segment-level** |
| User extensions | Must create separate namespace | **Can extend reserved** |
| Greppability | `grep "#pg\\."` | `grep "#;.*;"` |
| IDE support | Simple patterns | **Richer semantics** |
| Syntax change | None | **New separator** |
| Conflicts | No | **No** |

---

## Examples in Practice

### Example 1: DateTime with Custom Calendar

```polyglot
{@} @Local::MyCompany:1.0.0.0

// Define company-specific calendar extending reserved DT
{#} #;DT;Business.FiscalYear
[.] .Q1
   [.] .months:array.int << {10, 11, 12}
[.] .Q2
   [.] .months:array.int << {1, 2, 3}
[.] .Q3
   [.] .months:array.int << {4, 5, 6}
[.] .Q4
   [.] .months:array.int << {7, 8, 9}
{x}

{|} |GetFiscalQuarter
[<] i<month:int

[t] |T.Call
[W] |W.Polyglot.Scope

// Match against custom fiscal calendar
[m] $quarter << $month
   [?] ?[10, 12] ? #;DT;Business.FiscalYear.Q1
   [?] ?[1, 3] ? #;DT;Business.FiscalYear.Q2
   [?] ?[4, 6] ? #;DT;Business.FiscalYear.Q3
   [?] ?[7, 9] ? #;DT;Business.FiscalYear.Q4
   [?] * ? #;DT;Business.FiscalYear.Q1

[>] o>quarter << $quarter
{x}
```

### Example 2: HTTP with Custom Status Codes

```polyglot
{@} @Local::WebAPI:1.0.0.0

// Extend HTTP status with custom codes
{#} #;HTTP;Status.Custom
[.] .RateLimited
   [.] .code:int << 429
   [.] .message:string << "Too Many Requests"
[.] .MaintenanceMode
   [.] .code:int << 503
   [.] .message:string << "Service in Maintenance"
{x}

{|} |HandleRequest
[t] |T.Call
[W] |W.Polyglot.Scope

// Mix standard and custom statuses
[y] $rate_limited
   [>] o>status << #;HTTP;Status.Custom.RateLimited
[y] $maintenance
   [>] o>status << #;HTTP;Status.Custom.MaintenanceMode
[y] *?
   [>] o>status << #;HTTP;Status;Ok              // Standard status
{x}
```

### Example 3: Violation Actions with Custom Handlers

```polyglot
{@} @Local::Validation:1.0.0.0

// Extend violation actions
{#} #;Violation;Action.Custom
[.] .LogAndRetry
   [.] .max_retries:int << 3
   [.] .backoff:string << "exponential"
[.] .EmailAdmin
   [.] .email:string << "admin@company.com"
{x}

{:} :data.critical_value
[<] i<value:int

[%] %Constraint
   [.] .min << 0
   [.] .max << 100
   [.] .violation:#pg.violation
      [.] .error << !Critical.OutOfBounds
      [.] .action << #;Violation;Action.Custom.EmailAdmin
         [.] .pipeline << |NotifyAdmin
            <error << "Critical value out of bounds"
            <value << :data.critical_value<value
{x}
```

---

## Migration from `pg.*` Namespace

### If Already Using `pg.*`

```polyglot
// Old style (pg.* namespace):
#pg.Boolean.True                       // Still works (via alias)

// New style (semicolon):
#;Boolean;True                         // Native syntax

// Alias for compatibility:
{A}
[A] #;Boolean;True >> #pg.Boolean.True
{x}
```

### Migration Path

**Phase 1:** Both syntaxes work
- `#pg.Boolean.True` (aliased to `#;Boolean;True`)
- `#;Boolean;True` (native)

**Phase 2:** Deprecate `pg.*` namespace
- Warning on `#pg.*` usage
- Suggest migration to `;` syntax

**Phase 3:** Remove `pg.*` namespace
- Only `;` syntax for reserved enums

---

## Grammar Specification

### EBNF

```ebnf
enum_path ::= '#' enum_segments

enum_segments ::= enum_segment ('.' enum_segment | ';' enum_segment)*

enum_segment ::= [';'] identifier

identifier ::= [A-Z][A-Za-z0-9]*

reserved_segment ::= ';' identifier

user_segment ::= '.' identifier | identifier

(* Examples *)
(* #;Boolean;True - fully reserved *)
(* #User.Defined - fully user *)
(* #;DT.MyCalendar;Monday - mixed *)
```

### Parsing Rules

1. **Parse `#` prefix** - Start of enum path
2. **Parse first segment**
   - If starts with `;` → reserved
   - Otherwise → user-defined
3. **Parse remaining segments**
   - `.identifier` → user-defined
   - `;identifier` → reserved
4. **Validate reserved segments against registry**
   - Check if segment is in reserved list
   - Error if user tries to define reserved segment

---

## Implementation Notes

### Compiler Tasks

1. **Load reserved enum registry** from standard library
2. **Parse enum paths** with `;` and `.` separators
3. **Validate** user cannot define reserved segments
4. **Generate code** with same semantics (internal representation)
5. **Provide metadata** for IDE/tooling

### IDE/LSP Tasks

1. **Syntax highlighting** - Different colors for `;` vs `.`
2. **Autocomplete** - Offer reserved after `;`, user after `.`
3. **Hover info** - Show if segment is reserved
4. **Go to definition** - Jump to reserved enum source
5. **Rename refactoring** - Prevent renaming reserved segments

---

## Final Recommendation

**✅ Use `;` (semicolon) prefix for reserved enum segments**

### Why This is Better

1. **Granular control** - Mark individual segments, not entire paths
2. **User extensions** - Can extend reserved namespaces safely
3. **Visual clarity** - Immediate distinction at use site
4. **IDE intelligence** - Rich autocomplete and validation
5. **Backward compatible** - Can alias old `pg.*` syntax
6. **Consistent** - Same pattern for types (`:;reserved;path`?)

### Open Question

**Should we use `;` for reserved types too?**

```polyglot
// Current:
:pg.int                                // Namespace prefix

// With semicolon:
:;int                                  // Reserved type
:;array;int                            // Reserved collection of reserved type
:;array.int                            // Reserved collection of user type (?)

// Or keep types as-is:
:pg.int                                // Keep pg.* for types
#;Boolean;True                         // Use ; for enums only
```

---

**Status:** ✅ Excellent proposal - Recommended for adoption

**Key Achievement:** Granular reserved/user distinction enables safe extension of standard library while maintaining clear visual semantics.
