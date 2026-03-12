---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: operators
shard: false

# --- Classification ---
type: reference
topic: Operators Reference
summary: Reference for Operators Reference
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
# Operators Reference

**What You'll Learn:**
- Complete reference of all Polyglot operators
- Operator categories and usage
- Conditional operators with negation
- Range operators with inclusivity/exclusivity
- Collection and pattern matching operators
- Common patterns and examples

---

## Operator Categories

Polyglot operators fall into these categories:

1. **Assignment & Flow** - `<<`, `>>`, `<~`, `~>`, `<<<`, `>>>`
2. **Conditional** - `=?`, `=!?`, `>?`, `<?`, `>=?`, `<=?` (and negations)
3. **Range** - `?[min, max]`, `?(min, max]`, `?[min, max)`, `?(min, max)`
4. **Collection** - `in?`, `in!?`, `re?`, `re!?`
5. **Composition** - `|>`
6. **Hierarchy** - `.`
7. **Match** - `?` (separator)

---

## Assignment & Flow Operators

### `<<` - Push From Right (Assignment)

**Purpose:** Push value from right side into left side (assignment operation)

**Usage:**
```polyglot
[r] $variable :type << value
[|] <input << $value
[|] >output << $value
```

**Examples:**
```polyglot
[r] $name :string << "Alice"
[r] $age :int << 30
[r] $result :float << |Calculate <x << 5.0
```

### `>>` - Pull From Left (CAPTURE →)

**Purpose:** **PULL/CAPTURE** value from left parameter **into** right variable (data flows left to right →)

**Direction:** `>parameter` → `>>` → `$variable`

**Usage:**
```polyglot
[|] >output >> $variable
```

**Examples:**
```polyglot
[r] |MyPipeline
[|] >result >> $output          // PULL >result → $output
[|] >status >> $status_code     // PULL >status → $status_code
```

### `<~` - Default From Right

**Purpose:** Provide default value for input parameter (pushes parameter to Default state)

**Direction:** `[source:value]` → `<~` → `[dest:param]`
- **Source (pulled):** default value on the right
- **Destination (pushed):** parameter enters Default state

**Usage:**
```polyglot
[|] <parameter :type <~ default_value
```

**Examples:**
```polyglot
{|} |ProcessOrder
[|] <priority :string <~ "medium"    // [source:"medium"] → <~ → [dest:<priority Default]
[|] <timeout :int <~ 30              // [source:30] → <~ → [dest:<timeout Default]
{x}
```

**Semantics:** Value is pulled from right and pushed into parameter, setting it to Default state.

### `~>` - Default From Right

**Purpose:** Provide default value for output parameter (pushes parameter to Default state)

**Direction:** `[source:value]` → `~>` → `[dest:param]`
- **Source (pulled):** default value on the right
- **Destination (pushed):** parameter enters Default state

**Usage:**
```polyglot
[|] >parameter :type ~> default_value
```

**Examples:**
```polyglot
{|} |FetchData
[|] >value :string ~> ""             // [source:""] → ~> → [dest:>value Default]
[|] >found :bool ~> #False  // [source:False] → ~> → [dest:>found Default]
{x}
```

**Semantics:** Value is pulled from right and pushed into parameter, setting it to Default state.

### `<<<` - Variadic Input

**Purpose:** Pass multiple values to variadic parameter

**Usage:**
```polyglot
[|] <parameter <<< {value1, value2, value3}
```

**Note:** Requires parameter to have `%variadic` metadata

**Examples:**
```polyglot
{|} |ConcatStrings
[|] <strings :array.string
   [%] %variadic << "true"
{x}

[r] |ConcatStrings
[|] <strings <<< {"Hello", " ", "World"}
```

### `>>>` - Variadic Output

**Purpose:** Capture multiple values from variadic output

**Usage:**
```polyglot
[|] >parameter >>> $variable
```

**Examples:**
```polyglot
[r] |SplitString
[|] <input << "a,b,c"
[|] >parts >>> $array              // Captures multiple values
```

---

## Conditional Operators

### Equality

#### `=?` - Equal To

**Purpose:** Check if two values are equal

**Usage:**
```polyglot
[f] $variable =? value
```

**Examples:**
```polyglot
[f] $status =? "active"
   [r] $message << "User is active"

[f] $count =? 0
   [r] $empty << #True
```

#### `=!?` - Not Equal To (Negation)

**Purpose:** Check if two values are NOT equal

**Usage:**
```polyglot
[f] $variable =!? value
```

**Examples:**
```polyglot
[f] $status =!? "active"
   [r] $message << "User is not active"

[f] $count =!? 0
   [r] $has_items << #True
```

**Pattern:** `!` prefix negates the operator

---

### Comparison

#### `>?` - Greater Than

**Purpose:** Check if left value is greater than right

**Usage:**
```polyglot
[f] $variable >? value
```

**Examples:**
```polyglot
[f] $age >? 18
   [r] $adult << #True

[f] $price >? 100.0
   [r] $expensive << #True
```

#### `>!?` - Not Greater Than (Less Than or Equal)

**Purpose:** Check if left value is NOT greater than right (≤)

**Usage:**
```polyglot
[f] $variable >!? value            // Same as $variable <=? value
```

**Examples:**
```polyglot
[f] $age >!? 18                    // age <= 18
   [r] $minor << #True
```

---

#### `<?` - Less Than

**Purpose:** Check if left value is less than right

**Usage:**
```polyglot
[f] $variable <? value
```

**Examples:**
```polyglot
[f] $temperature <? 0
   [r] $freezing << #True

[f] $score <? 50
   [r] $failing << #True
```

#### `<!?` - Not Less Than (Greater Than or Equal)

**Purpose:** Check if left value is NOT less than right (≥)

**Usage:**
```polyglot
[f] $variable <!? value            // Same as $variable >=? value
```

**Examples:**
```polyglot
[f] $age <!? 21                    // age >= 21
   [r] $can_drink << #True
```

---

#### `>=?` - Greater Than or Equal

**Purpose:** Check if left value is greater than or equal to right

**Usage:**
```polyglot
[f] $variable >=? value
```

**Examples:**
```polyglot
[f] $age >=? 18
   [r] $adult << #True

[f] $score >=? 90
   [r] $grade << "A"
```

#### `>=!?` - Not Greater Than or Equal (Less Than)

**Purpose:** Check if left value is NOT greater than or equal to right (<)

**Usage:**
```polyglot
[f] $variable >=!? value           // Same as $variable <? value
```

**Examples:**
```polyglot
[f] $score >=!? 60                 // score < 60
   [r] $failing << #True
```

---

#### `<=?` - Less Than or Equal

**Purpose:** Check if left value is less than or equal to right

**Usage:**
```polyglot
[f] $variable <=? value
```

**Examples:**
```polyglot
[f] $temperature <=? 0
   [r] $freezing_or_below << #True

[f] $age <=? 12
   [r] $child << #True
```

#### `<=!?` - Not Less Than or Equal (Greater Than)

**Purpose:** Check if left value is NOT less than or equal to right (>)

**Usage:**
```polyglot
[f] $variable <=!? value           // Same as $variable >? value
```

**Examples:**
```polyglot
[f] $age <=!? 17                   // age > 17
   [r] $adult << #True
```

---

### Conditional Operator Summary

| Operator | Meaning | Negation | Equivalent |
|----------|---------|----------|------------|
| `=?` | Equal | `=!?` | Not equal |
| `>?` | Greater than | `>!?` | Less than or equal (`<=?`) |
| `<?` | Less than | `<!?` | Greater than or equal (`>=?`) |
| `>=?` | Greater or equal | `>=!?` | Less than (`<?`) |
| `<=?` | Less or equal | `<=!?` | Greater than (`>?`) |

**Negation Pattern:** Prefix `!` to any operator to negate it

---

## Wildcard Operator

### `*?` - Wildcard Condition (Else)

**Purpose:** Match/test against any value (catch-all / else case)

**Important:** `*?` is a **compound operator** (single token), not two separate operators.

**Usage in Conditionals:**
```polyglot
[f] *?                             // Wildcard condition (else)
```

**Usage in Match Expressions:**
```polyglot
[?] * ? result                     // Wildcard case (may have space for readability)
```

**Examples:**

**Conditional else:**
```polyglot
[f] $age >=? 18
   [r] $status << "adult"
[f] *?                             // Else case (any other age)
   [r] $status << "minor"
```

**Match wildcard:**
```polyglot
[m] $grade << $score
   [?] $score >=? 90 ? "A"
   [?] $score >=? 80 ? "B"
   [?] $score >=? 70 ? "C"
   [?] * ? "F"                     // Wildcard - any other score
```

**Multiple else branches:**
```polyglot
[f] $role =? "admin"
   [r] $access << "full"
[f] $role =? "moderator"
   [r] $access << "elevated"
[f] *?                             // Everything else
   [r] $access << "basic"
```

### Wildcard Semantics

**In conditionals (`[f] *?`):**
- Acts as "else" branch
- Matches when no prior conditions matched
- Must come after all specific conditions
- Evaluates to true for any value

**In match expressions (`[?] * ?`):**
- Catch-all pattern
- Matches any value not matched by prior cases
- Should typically be last case
- Optional space between `*` and `?` for readability

### When to Use Wildcard

**Use `[f] *?` when:**
- You want an explicit else branch
- You need to handle "all other cases"
- Exhaustive matching is required but no specific pattern fits

**Use `[?] * ?` when:**
- Match expression needs a default case
- You want to catch unmatched enum variants
- Providing fallback value for unknown inputs

**Example with both:**
```polyglot
// Conditional with wildcard
[f] $status =? #Status.Active
   [r] $message << "Running"
[f] *?
   [r] $message << "Not active"

// Match with wildcard
[m] $category << $status
   [?] #Status.Active ? "online"
   [?] #Status.Paused ? "offline"
   [?] * ? "unknown"               // Wildcard case
```

### Wildcard vs Explicit Conditions

**Explicit conditions (preferred when possible):**
```polyglot
[f] $age <? 18
   [r] $category << "minor"
[f] $age >=? 18
   [r] $category << "adult"        // Explicit condition
```

**Wildcard (when explicit is verbose):**
```polyglot
[f] $status =? #Status.Error
   [r] $handle << |HandleError
[f] *?                             // All non-error statuses
   [r] $handle << |ProcessNormal
```

---

## Range Operators

### Inclusive Both Ends

#### `?[min, max]` - Inclusive Range

**Purpose:** Check if value is within range, including both boundaries

**Usage:**
```polyglot
[f] $variable ?[min, max]
```

**Math notation:** `min ≤ value ≤ max`

**Examples:**
```polyglot
[f] $age ?[18, 65]                 // 18 <= age <= 65
   [r] $working_age << #True

[f] $score ?[0, 100]               // 0 <= score <= 100
   [r] $valid_score << #True
```

### Exclusive Minimum

#### `?(min, max]` - Exclusive Min, Inclusive Max

**Purpose:** Check if value is within range, excluding minimum, including maximum

**Usage:**
```polyglot
[f] $variable ?(min, max]
```

**Math notation:** `min < value ≤ max`

**Examples:**
```polyglot
[f] $percentage ?(0, 100]          // 0 < percentage <= 100
   [r] $valid << #True
```

### Exclusive Maximum

#### `?[min, max)` - Inclusive Min, Exclusive Max

**Purpose:** Check if value is within range, including minimum, excluding maximum

**Usage:**
```polyglot
[f] $variable ?[min, max)
```

**Math notation:** `min ≤ value < max`

**Examples:**
```polyglot
[f] $index ?[0, 10)                // 0 <= index < 10
   [r] $valid_index << #True
```

### Exclusive Both Ends

#### `?(min, max)` - Exclusive Range

**Purpose:** Check if value is within range, excluding both boundaries

**Usage:**
```polyglot
[f] $variable ?(min, max)
```

**Math notation:** `min < value < max`

**Examples:**
```polyglot
[f] $temperature ?(0, 100)         // 0 < temperature < 100
   [r] $liquid_water << #True
```

---

### Range Operator Summary

| Operator | Min | Max | Math Notation | Example |
|----------|-----|-----|---------------|---------|
| `?[min, max]` | Inclusive | Inclusive | `min ≤ x ≤ max` | `?[0, 10]` → 0,1,...,10 |
| `?(min, max]` | Exclusive | Inclusive | `min < x ≤ max` | `?(0, 10]` → 1,2,...,10 |
| `?[min, max)` | Inclusive | Exclusive | `min ≤ x < max` | `?[0, 10)` → 0,1,...,9 |
| `?(min, max)` | Exclusive | Exclusive | `min < x < max` | `?(0, 10)` → 1,2,...,9 |

**Mnemonic:**
- `[` / `]` = **inclusive** (bracket "holds" the value)
- `(` / `)` = **exclusive** (parenthesis "lets go" of the value)

---

## Collection Operators

### `in?` - In Collection

**Purpose:** Check if value exists in collection

**Usage:**
```polyglot
[f] $variable in? $collection
[f] $variable in? {literal_collection}
```

**Examples:**

**With literal:**
```polyglot
[f] $priority in? {"low", "medium", "high"}
   [r] $valid_priority << #True

[f] $day in? {"Monday", "Tuesday", "Wednesday"}
   [r] $weekday << #True
```

**With variable:**
```polyglot
[r] $valid_statuses :array.string << {"active", "pending", "completed"}

[f] $status in? $valid_statuses
   [r] $is_valid << #True
```

### `in!?` - Not In Collection

**Purpose:** Check if value does NOT exist in collection

**Usage:**
```polyglot
[f] $variable in!? $collection
```

**Examples:**
```polyglot
[f] $status in!? {"deleted", "banned", "suspended"}
   [r] $is_active << #True

[f] $char in!? {"!", "@", "#", "$"}
   [r] $is_alphanumeric << #True
```

---

## Pattern Matching Operators

### `re?` - Regex Match

**Purpose:** Check if value matches regex pattern

**Usage:**
```polyglot
[f] $variable re? "regex_pattern"
[f] $variable re? $regex_variable
```

**Examples:**
```polyglot
[f] $email re? "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
   [r] $valid_email << #True

[f] $phone re? "^\\d{3}-\\d{3}-\\d{4}$"
   [r] $valid_phone << #True

[r] $pattern :string << "[0-9]+"
[f] $input re? $pattern
   [r] $is_numeric << #True
```

### `re!?` - Not Regex Match

**Purpose:** Check if value does NOT match regex pattern

**Usage:**
```polyglot
[f] $variable re!? "regex_pattern"
```

**Examples:**
```polyglot
[f] $username re!? "\\s"           // No whitespace
   [r] $valid_username << #True

[f] $password re!? "^[a-z]+$"      // Not only lowercase
   [r] $has_uppercase_or_other << #True
```

---

## Composition Operator

### `|>` - Pipeline Composition

**Purpose:** Compose pipelines (chain operations)

**Usage:**
```polyglot
[r] |Pipeline1 |> |Pipeline2               // Chain Pipeline1 → Pipeline2
[|] <input:datatype << $value             // Input to Pipeline1
[|] >output1:datatype >> <input2          // Pipeline1 output → Pipeline2 input
[|] |> |Pipeline3                          // Chain Pipeline2 → Pipeline3
[|] >output2:datatype >> <input3          // Pipeline2 output → Pipeline3 input
[|] |>                                     // End chain
[|] >final:datatype >> $result            // Capture Pipeline3 output
```

**Critical:** Each `|>` must be on its own line (one marker + one expression rule).

**Examples:**

**Simple composition:**
```polyglot
[r] |String.Trim |> |String.Lower             // Chain Trim → Lower
[|] <input:pg.string << $raw_input           // Input to Trim
[|] >trimmed:pg.string >> <input             // Trim output → Lower input
[|] |> |String.Validate                       // Chain Lower → Validate
[|] >lowered:pg.string >> <input             // Lower output → Validate input
[|] |>                                        // End chain
[|] >validated:pg.string >> $result          // Capture Validate output
```

**With additional parameters:**
```polyglot
[r] |Parse.Int |> |Math.Double                // Chain Parse.Int → Math.Double
[|] <input:pg.string << $data                // Input to Parse.Int
[|] >value:pg.int >> <input                  // Parse.Int output → Math.Double input
[|] |> |Math.Add                              // Chain Math.Double → Math.Add
[|] >doubled:pg.int >> <x                    // Math.Double output → Math.Add input x
[|] <y:pg.int << 10                          // Additional input y to Math.Add
[|] |>                                        // End chain
[|] >result:pg.int >> $final                 // Capture Math.Add output
```

**Multi-step composition:**
```polyglot
[r] |Clean.RemoveWhitespace |> |Transform.Normalize    // Chain RemoveWhitespace → Normalize
[|] <input:pg.string << $raw_data                     // Input to RemoveWhitespace
[|] >cleaned:pg.string >> <input                      // RemoveWhitespace output → Normalize input
[|] |> |Validate.Schema                                // Chain Normalize → Validate.Schema
[|] >normalized:pg.string >> <input                   // Normalize output → Validate.Schema input
[|] <schema:pg.serial << $my_schema                   // Schema input to Validate.Schema
[|] |> |Store.Save                                     // Chain Validate.Schema → Store.Save
[|] >validated:pg.serial >> <data                     // Validate.Schema output → Store.Save input
[|] <table:pg.string << "processed_data"              // Table name input to Store.Save
[|] |>                                                 // End chain
[|] >stored:pg.bool >> $success                       // Capture Store.Save output
```

**See:** [Pipeline Composition](../features/pipeline-features/pipeline-composition.md)

---

## Hierarchy Operator

### `.` - Dot (Hierarchy Separator)

**Purpose:** Universal hierarchy separator

**Usage:**
```polyglot
$variable.field.nested
:namespace.type
#Type.variant
|Namespace.Pipeline
!Category.Domain.Error
```

**Examples:**

**Variable field access:**
```polyglot
[r] $user_name :string << $user.profile.name
[r] $db_host :string << $config.database.host
```

**Type hierarchy:**
```polyglot
[r] $names :array.string
[r] $matrix :array.array.int
[r] $data :pg.serial
```

**Enum hierarchy:**
```polyglot
[r] $status << #OrderStatus.Processing
[r] $result << #Result.Success.Ok
```

**Pipeline hierarchy:**
```polyglot
[r] $user << |Database.Users.Find
[r] $json << |File.JSON.Load
```

**Error hierarchy:**
```polyglot
[z][!] !Network.HTTP.Timeout ? "Timeout"
[z][!] !IO.File.NotFound ? "File not found"
```

---

## Match Operator

### `?` - Match Separator

**Purpose:** Separate pattern from result in match expressions

**Usage:**
```polyglot
[?] pattern ? result
```

**Examples:**

**Match expression:**
```polyglot
[m] $result << $value
   [?] "case1" ? "result1"
   [?] "case2" ? "result2"
   [?] * ? "default"               // Wildcard case
```

**Enum matching:**
```polyglot
[m] $message << $status
   [?] #OrderStatus.Pending ? "Waiting to process"
   [?] #OrderStatus.Processing ? "Currently processing"
   [?] #OrderStatus.Completed ? "Finished"
   [?] * ? "Unknown status"
```

**Range matching:**
```polyglot
[m] $grade << $score
   [?] ?[90, 100] ? "A"
   [?] ?[80, 90) ? "B"
   [?] ?[70, 80) ? "C"
   [?] ?[60, 70) ? "D"
   [?] * ? "F"
```

**See:** [Match Expressions](../features/control-flow/match-expressions.md)

---

## Operator Precedence

**Polyglot doesn't have traditional operator precedence** because:
1. No binary operations (no `$a + $b * $c`)
2. One expression per line
3. Pipeline calls are explicit

**Guidelines:**
- Use parentheses for clarity in complex conditions
- Break complex expressions into multiple lines
- Use temporary variables for readability

---

## Common Patterns

### Pattern 1: Validation Chain

```polyglot
[f] $age >=? 18
[&] $has_license =? #True
[&] $vision_test_passed =? #True
   [r] $can_drive << #True
```

### Pattern 2: Range with Else

```polyglot
[f] $age ?[0, 12]
   [r] $category << "child"
[f] $age ?[13, 17]
   [r] $category << "teen"
[f] $age >=? 18
   [r] $category << "adult"
```

### Pattern 3: Collection Membership

```polyglot
[r] $admin_roles :array.string << {"admin", "superadmin", "owner"}

[f] $user_role in? $admin_roles
   [r] $has_admin_access << #True
```

### Pattern 4: Regex Validation

```polyglot
[f] $email re? "^[\\w.%+-]+@[\\w.-]+\\.[A-Za-z]{2,}$"
[&] $email in!? $blocked_emails
   [r] $valid_email << #True
```

### Pattern 5: Pipeline Composition

```polyglot
[r] |Trim |> |Lower                              // Chain Trim → Lower
[|] <input:pg.string << $raw_input              // Input to Trim
[|] >trimmed:pg.string >> <input                // Trim output → Lower input
[|] |> |RemoveSpecialChars                       // Chain Lower → RemoveSpecialChars
[|] >lowered:pg.string >> <input                // Lower output → RemoveSpecialChars input
[|] |> |Validate                                 // Chain RemoveSpecialChars → Validate
[|] >cleaned:pg.string >> <input                // RemoveSpecialChars output → Validate input
[|] |>                                           // End chain
[|] >validated:pg.string >> $clean_data         // Capture Validate output
```

---

## Operator Negation Table

| Operator | Negation | Equivalent |
|----------|----------|------------|
| `=?` | `=!?` | Not equal |
| `>?` | `>!?` | `<=?` |
| `<?` | `<!?` | `>=?` |
| `>=?` | `>=!?` | `<?` |
| `<=?` | `<=!?` | `>?` |
| `in?` | `in!?` | Not in collection |
| `re?` | `re!?` | Does not match regex |

**Rule:** Prefix `!` to any operator to negate it.

---

## Summary

### All Operators by Category

**Assignment & Flow:**
- `<<`, `>>`, `<~`, `~>`, `<<<`, `>>>`

**Conditional:**
- `=?`, `=!?`, `>?`, `>!?`, `<?`, `<!?`, `>=?`, `>=!?`, `<=?`, `<=!?`

**Range:**
- `?[min, max]`, `?(min, max]`, `?[min, max)`, `?(min, max)`

**Collection:**
- `in?`, `in!?`

**Pattern:**
- `re?`, `re!?`

**Composition:**
- `|>`

**Hierarchy:**
- `.`

**Match:**
- `?` (separator)

### Key Concepts

1. **Negation with `!`** - Prefix any conditional operator
2. **Range inclusivity** - `[` = inclusive, `(` = exclusive
3. **Question mark suffix** - Denotes conditional/query operators
4. **Direction arrows** - `<<` and `>>` show data flow
5. **Universal hierarchy** - `.` separates all hierarchies

---

## Related Documentation

- [I/O Operators](./io-operators.md) - `<<`, `>>`, `<~`, `~>` in detail
- [Markers Reference](./markers.md) - `[f]`, `[m]`, `[?]` markers
- [Match Expressions](../features/control-flow/match-expressions.md) - Match operator usage
- [Pipeline Composition](../features/pipeline-features/pipeline-composition.md) - `|>` operator
- [Range Operators](../features/operators/range-operators.md) - Range operator details

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
