---
last-redoc-date: 2025-12-18
---

# Complete Syntax Patterns Catalog for Parser Implementation

**Version**: v0.0.4
**Purpose**: Exhaustive list of all valid Polyglot syntax patterns
**Rule**: Every valid line matches: `[Optional Indentation] + [Marker(s)] + [One Expression]`

---

## Table of Contents

1. [Single-Line Patterns](#single-line-patterns)
2. [Multi-Line Patterns](#multi-line-patterns)
3. [Definition Patterns](#definition-patterns)
4. [Expression Patterns](#expression-patterns)
5. [Complete Pattern Grammar](#complete-pattern-grammar)

---

## Single-Line Patterns

### Pattern Group 1: Variable Assignment

```polyglot
# Basic assignment
[r] $variable << expression

# With type annotation
[r] $variable :type << expression

# Multiple push operators
[r] $a << $b << $c              # Right-associative: $a << ($b << $c)

# Default assignment
[r] $variable <~ expression     # Only assigns if $variable is pending

# Variadic assignment
[r] $array <<< $collection      # Unpacks collection elements
```

**Grammar**:
```
assignment ::= [execution_marker] variable [type_annotation]? push_operator expression
execution_marker ::= [r] | [p] | [b]
push_operator ::= << | >> | <<< | >>> | <~ | ~>
```

### Pattern Group 2: Pipeline Invocation

```polyglot
# Basic pipeline call
[r] |PipelineName

# With input binding
[r] |PipelineName
[|] <input1 << expression
[|] <input2 << expression

# With output binding
[r] |PipelineName
[|] >output1 >> $variable

# Complete I/O binding
[r] |PipelineName
[|] <input << expression
[|] >output >> $target

# Inline pipeline (formatted string)
[r] $result << |PipelineName"{$arg1} text {$arg2:format}"
```

**Grammar**:
```
pipeline_call ::= [execution_marker] pipeline_identifier
pipeline_binding ::= [|] io_parameter push_operator expression
io_parameter ::= input_argument | output_argument
```

### Pattern Group 3: Conditional Execution

```polyglot
# Simple conditional
[?] $variable >? value

# Conditional with expression
[?] expression comparison_operator expression

# Boolean logic markers
[+] $condition1             # OR
[&] $condition2             # AND
[-] $condition3             # NOT
[^] $condition4             # XOR

# Wildcard (catch-all)
[?] *
```

**Grammar**:
```
conditional ::= [?] (expression | wildcard)
boolean_logic ::= ([+] | [&] | [-] | [^]) expression
comparison ::= expression comparison_op expression
comparison_op ::= =? | >? | <? | >=? | <=? | (and negated variants)
```

### Pattern Group 4: Loop Constructs

```polyglot
# Unpack operator (iteration start)
[r] ~UnpackOperator
[~] <input << collection
[~] >item >> $iterationVariable

# Pack operator (iteration result)
[r] *PackOperator
[*] <item << $processedItem
[*] >collection >> $result
```

**Grammar**:
```
unpack ::= [execution_marker] unpack_identifier
         ([~] io_binding)*
pack ::= [execution_marker] pack_identifier
       ([*] io_binding)*
```

### Pattern Group 5: Error Handling

```polyglot
# Catch specific error
[!] !ErrorType ? |ErrorHandler

# Catch all errors
[!] * ? |ErrorHandler

# Multiple error handlers
[!] !Network.Timeout ? |RetryHandler
[!] !Database.Error ? |DbErrorHandler
[!] * ? |GenericHandler
```

**Grammar**:
```
error_catch ::= [!] (error_identifier | wildcard) ? pipeline_identifier
```

### Pattern Group 6: Metadata Annotations

```polyglot
# Single metadata
%Doc "This is documentation"

# Multiple metadata
%Doc "Description"
%Author "Developer Name"
%Deprecated "Use NewFunction instead"

# Before definitions
%Doc "Pipeline documentation"
{|} |PipelineName
```

**Grammar**:
```
metadata ::= metadata_identifier [string_literal]?
```

### Pattern Group 7: Match/Select Expressions

```polyglot
# Match expression
[m] $variable
   [?] pattern1 ? result1
   [?] pattern2 ? result2
   [?] * ? defaultResult
```

**Grammar**:
```
match ::= [m] expression
        (INDENT [?] (pattern | wildcard) ? expression)*
```

---

## Multi-Line Patterns

### Pattern Group 8: Indented Blocks

```polyglot
# One level of indentation (3 spaces)
[r] $x << 5
   [?] $x >? 10
   [r] $log << "Greater"

# Two levels of indentation (6 spaces)
[r] $status << #Active
   [?] $status =? #Active
      [r] $log << "Status is active"
      [r] $processed << #Boolean.True

# Three levels of indentation (9 spaces)
[m] $value
   [?] 1 ? #Small
      [r] $category << "tiny"
         [r] $log << "Very small"
   [?] 10 ? #Medium
   [?] * ? #Large
```

**Grammar**:
```
indented_block ::= statement
                 (NEWLINE INDENT{3} statement)*
INDENT{n} ::= n spaces (must be multiple of 3)
```

**Indentation Rules**:
- Each indentation level = exactly 3 spaces
- Indentation creates sub-marker relationship
- Child statements execute within parent context
- Max nesting depth: unlimited (practical limit ~10 levels)

### Pattern Group 9: Pipeline Definition

```polyglot
# Complete pipeline definition
{|} |PipelineName
[|] <input1 :pg.string
[|] <input2 :pg.int
[|] >output :pg.string << $result
{x}

# Pipeline with trigger
{|} |TriggeredPipeline
[t] TG.HTTP
[|] <endpoint :pg.string
[|] >response :pg.string << $data
{x}

# Pipeline with queue
{|} |QueuedPipeline
[Q] #QueueType.Serial
[|] <data :pg.any
{x}
```

**Grammar**:
```
pipeline_def ::= {|} pipeline_identifier
                [metadata]*
                [trigger_block]?
                [queue_block]?
                input_params*
                code_body
                output_params*
                {x}

input_params ::= [|] input_argument type_annotation
output_params ::= [|] output_argument type_annotation push_operator expression
```

### Pattern Group 10: Enum Definition

```polyglot
# Simple enum
{#} #StatusType
[#] Active
[#] Inactive
[#] Pending
{x}

# Enum with fields (struct-like)
{#} #UserRecord
[.] name :pg.string
[.] age :pg.int
[.] email :pg.string
{x}
```

**Grammar**:
```
enum_def ::= {#} enum_identifier
           ([#] identifier | [.] field_def)*
           {x}

field_def ::= identifier type_annotation
```

### Pattern Group 11: Error Definition

```polyglot
# Error definition
{!} !NetworkError
[!] Timeout
[!] ConnectionFailed
[!] InvalidResponse
{x}
```

**Grammar**:
```
error_def ::= {!} error_identifier
            ([!] identifier)*
            {x}
```

### Pattern Group 12: Package Definition

```polyglot
# Package definition
{@} @Local::MyApp:1.0.0
[@] description "My application"
[@] author "Developer"
{x}
```

**Grammar**:
```
package_def ::= {@} package_spec_identifier
              ([@] metadata_field)*
              {x}
```

---

## Definition Patterns

### Pattern Group 13: Dual-Context Markers

**Context 1: Inside Definition Block**

```polyglot
{|} |MyPipeline              # Define pipeline
[|] <input1 :pg.string       # [|] declares parameter of |MyPipeline
[|] <input2 :pg.int          # [|] declares parameter of |MyPipeline
   [r] $result << |Process   # Code body (indented)
[|] >output :pg.int << $result  # [|] declares output of |MyPipeline
{x}                          # End definition
```

**Interpretation**: `[|]` means "this is a component OF the pipeline being defined"

**Context 2: After Invocation**

```polyglot
[r] |MyPipeline              # Invoke pipeline
[|] <input1 << "Alice"       # [|] binds value TO |MyPipeline's <input1
[|] <input2 << 25            # [|] binds value TO |MyPipeline's <input2
[|] >output >> $userAge      # [|] binds output FROM |MyPipeline TO $userAge
```

**Interpretation**: `[|]` means "bind value to/from the pipeline being invoked"

**Parser State Machine**:
```
NORMAL state:
  - See {X} → Enter DEFINITION_BLOCK state (track X type)
  - See [exec] Identifier → Enter INVOCATION state

DEFINITION_BLOCK state:
  - [X] markers declare components
  - Exit when {x} encountered → Return to NORMAL

INVOCATION state:
  - [X] markers bind arguments
  - Exit when non-[X] marker or blank line → Return to NORMAL
```

### Pattern Group 14: Generalized Dual-Context Pattern

**This pattern applies to ALL identifiers**:

```polyglot
# Unpack operator
[r] ~ForEach.Array           # Invoke ~ForEach.Array
[~] <array << $items         # [~] binds TO ~ForEach's <array parameter
[~] >item >> $current        # [~] binds FROM ~ForEach's >item parameter

# Pack operator
[r] *Collect.Array           # Invoke *Collect.Array
[*] <item << $processed      # [*] binds TO *Collect's <item
[*] >array >> $results       # [*] binds FROM *Collect's >array

# Enum reference (if definable)
{#} #Status
[#] Active                   # [#] declares value OF #Status
[#] Inactive                 # [#] declares value OF #Status
{x}
```

**Rule**: Marker `[X]` means:
- Inside `{X}...{x}`: "Declare component of X"
- After `[exec] X`: "Bind argument to/from X"

---

## Expression Patterns

### Pattern Group 15: Literal Expressions

```polyglot
# Integer literals
42
-10
0

# Float literals
3.14
-0.5
0.0

# String literals (plain)
"Hello, world!"
"Multi-word string"

# String literals (interpolated)
"Hello, {$userName}"
"Count: {$count:Hex}"
"Status: {$status} at {DT.Now:DateTime}"

# Collection literals
{1, 2, 3, 4, 5}
{"Alice", "Bob", "Charlie"}
{$var1, $var2, $var3}

# Datetime literals
DT"2024-01-15T14:30:00Z"

# Duration literals
DT.Minutes"5"
DT.Hours"2"
DT.Days"7"
```

**Grammar**:
```
literal ::= integer | float | string | collection | datetime | duration
string ::= " (string_content | interpolation)* "
interpolation ::= { expression [: format_identifier]? }
collection ::= { expression (, expression)* }
```

### Pattern Group 16: Variable References

```polyglot
# Simple variable
$userName
$count
$isActive

# Hierarchical variable (if supported)
$user.name
$config.database.host
```

**Grammar**:
```
variable_ref ::= $ identifier (. identifier)*
```

### Pattern Group 17: Enum References

```polyglot
# Custom enum (dots)
#Status.Active
#OrderType.Retail.Online

# Reserved enum (semicolons)
#Boolean.True
#PgVar.States;Ready

# Mixed (dots and semicolons)
#DT.Business;Week.CustomWeek;RestDays
```

**Grammar**:
```
enum_ref ::= # (identifier (. identifier)* | reserved_path | mixed_path)
reserved_path ::= ; identifier (; identifier)*
mixed_path ::= (. identifier | ; identifier)+
```

### Pattern Group 18: Binary Expressions

```polyglot
# Comparison operators
$x >? 10
$name =? "Alice"
$age >=? 18

# Negated comparisons
$status =!? #Inactive
$count >!? 0

# Pattern matching
$text re? "pattern.*"
$item in? {1, 2, 3, 4}

# String concatenation
$fullName << $firstName +" " +" $lastName

# Push operators (assignment)
$result << |Pipeline
$target >> |Source
```

**Grammar**:
```
binary_expr ::= expression operator expression
operator ::= comparison_op | pattern_op | push_op | compose_op
```

### Pattern Group 19: Pipeline Composition

```polyglot
# Compose two pipelines
$result << |Pipeline1 |> |Pipeline2

# Compose multiple pipelines (conceptual - see note below)
$final << |Step1 |> |Step2 |> |Step3 |> |Step4

# Composition with arguments
$result << (|Transform |> |Validate |> |Save)
```

> **⚠️ IMPORTANT:** While the composition operator `|>` allows chaining, **multi-pipeline chains MUST be split across multiple lines** in actual code to comply with the "one line = one marker + one expression" principle. The examples above show the conceptual composition syntax.
>
> **Correct multi-line format:**
> ```polyglot
> [r] |Step1 |> |Step2                  // Chain Step1 → Step2
> [|] <input << $value                 // Input to Step1
> [|] >output1 >> <input2              // Step1 output → Step2 input
> [|] |> |Step3                         // Chain Step2 → Step3
> [|] >output2 >> <input3              // Step2 output → Step3 input
> [|] |>                                // End chain
> [|] >final >> $result                // Capture final output
> ```
>
> **See:** [Core Principles - Pipeline Chaining](../User/getting-started/core-principles.md#critical-pipeline-chaining) for complete explanation.

**Grammar**:
```
composition ::= pipeline_ref (|> pipeline_ref)+
```

### Pattern Group 20: Range Expressions

```polyglot
# Closed range (both inclusive)
$x ?[ 1, 10 ]

# Open range (both exclusive)
$x ?( 0, 100 )

# Half-right (left exclusive, right inclusive)
$x ?] 0, 10 ]

# Half-left (left inclusive, right exclusive)
$x ?) 1, 11 )
```

**Grammar**:
```
range ::= value range_op expression, expression ]
range_op ::= ?[ | ?( | ?] | ?)
```

---

## Complete Pattern Grammar

### Top-Level Structure

```ebnf
program ::= statement*

statement ::= definition
           | execution
           | metadata
           | comment
           | NEWLINE

definition ::= pipeline_definition
            | enum_definition
            | error_definition
            | package_definition
            | macro_definition
            | alias_definition

execution ::= assignment
           | pipeline_call
           | conditional
           | loop
           | match_expression
           | error_handling
```

### Execution Statements

```ebnf
assignment ::= [execution_marker] variable [type_annotation]? push_operator expression

pipeline_call ::= [execution_marker] pipeline_identifier
                ([|] io_binding)*

conditional ::= [?] (comparison_expression | wildcard)

loop ::= unpack_statement | pack_statement

unpack_statement ::= [execution_marker] unpack_identifier
                    ([~] io_binding)*
                    (NEWLINE INDENT statement)*

pack_statement ::= [execution_marker] pack_identifier
                  ([*] io_binding)*

match_expression ::= [m] expression
                    (NEWLINE INDENT [?] (pattern | wildcard) ? expression)*

error_handling ::= [!] (error_identifier | wildcard) ? pipeline_identifier
```

### Definition Blocks

```ebnf
pipeline_definition ::= {|} pipeline_identifier
                       [metadata]*
                       [trigger_block]?
                       [queue_block]?
                       ([|] parameter_declaration)*
                       code_body
                       ([|] output_declaration)*
                       {x}

enum_definition ::= {#} enum_identifier
                   ([#] identifier | [.] field_declaration)*
                   {x}

error_definition ::= {!} error_identifier
                    ([!] identifier)*
                    {x}

package_definition ::= {@} package_spec_identifier
                      ([@] metadata_field)*
                      {x}
```

### Expressions

```ebnf
expression ::= literal
            | variable_reference
            | enum_reference
            | pipeline_reference
            | binary_expression
            | unary_expression
            | collection_literal
            | inline_pipeline
            | range_expression
            | grouped_expression

binary_expression ::= expression binary_operator expression

binary_operator ::= push_operator
                 | comparison_operator
                 | pattern_operator
                 | pipeline_compose_operator
                 | string_concat_operator

grouped_expression ::= ( expression )
```

### Identifiers

```ebnf
variable_reference ::= $ identifier hierarchy?
enum_reference ::= # (identifier hierarchy | reserved_hierarchy | mixed_hierarchy)
pipeline_reference ::= | identifier hierarchy?
error_reference ::= ! identifier hierarchy?
unpack_reference ::= ~ identifier hierarchy?
pack_reference ::= * identifier hierarchy?
metadata_reference ::= % identifier hierarchy?
type_reference ::= : identifier hierarchy
package_spec ::= @ identifier :: identifier : version

hierarchy ::= (. identifier)*
reserved_hierarchy ::= (; identifier)+
mixed_hierarchy ::= ((. identifier) | (; identifier))+
```

### Literals

```ebnf
literal ::= integer_literal
         | float_literal
         | string_literal
         | collection_literal
         | datetime_literal
         | duration_literal

string_literal ::= " (string_content | interpolation)* "
interpolation ::= { expression [: format_identifier]? }
collection_literal ::= { expression (, expression)* }
datetime_literal ::= DT " iso8601_datetime "
duration_literal ::= DT. duration_unit " number "
```

### Operators

```ebnf
push_operator ::= << | >> | <<< | >>> | <~ | ~>
comparison_operator ::= =? | =!? | >? | >!? | <? | <!? | >=? | >=!? | <=? | <=!?
pattern_operator ::= re? | re!? | in? | in!? | *?
pipeline_compose_operator ::= |>
string_concat_operator ::= +"
range_operator ::= ?[ | ?( | ?] | ?)
```

### Special Constructs

```ebnf
inline_pipeline ::= pipeline_identifier string_literal

io_binding ::= (input_argument | output_argument) push_operator expression

parameter_declaration ::= (input_argument | output_argument) type_annotation

output_declaration ::= output_argument type_annotation push_operator expression

type_annotation ::= : type_path
```

---

## Pattern Validation Checklist

**For Parser Implementation**, verify you can parse:

### Single-Line Patterns (18 types)
- ✅ Basic variable assignment: `[r] $x << 5`
- ✅ Typed variable assignment: `[r] $x :pg.int << 5`
- ✅ Pipeline call: `[r] |Pipeline`
- ✅ Pipeline with I/O: `[|] <input << $value`
- ✅ Conditional: `[?] $x >? 5`
- ✅ Boolean logic: `[+] $condition`
- ✅ Error catch: `[!] * ? |Handler`
- ✅ Metadata: `%Doc "description"`
- ✅ Inline pipeline: `$result << |Pipe"{$arg}"`
- ✅ Comparison: `$a =? $b`
- ✅ Pattern match: `$text re? "pattern"`
- ✅ Collection membership: `$item in? $collection`
- ✅ Pipeline composition: `$result << |A |> |B`
- ✅ Range: `$x ?[ 1, 10 ]`
- ✅ String concat: `$s << $a +" $b`
- ✅ Variadic push: `$arr <<< $collection`
- ✅ Default push: `$var <~ $defaultValue`
- ✅ Enum reference: `#Status.Active` or `#Boolean.True`

### Multi-Line Patterns (12 types)
- ✅ Indented blocks (1-3+ levels)
- ✅ Pipeline definition: `{|} |Name ... {x}`
- ✅ Enum definition: `{#} #Name ... {x}`
- ✅ Error definition: `{!} !Name ... {x}`
- ✅ Package definition: `{@} @Spec ... {x}`
- ✅ Match expression: `[m] $var ...`
- ✅ Unpack loop: `[r] ~ForEach ...`
- ✅ Pack operation: `[r] *Collect ...`
- ✅ Nested conditionals
- ✅ Pipeline with trigger/queue blocks
- ✅ Multi-handler error catch
- ✅ Nested pipeline calls

### Special Patterns (8 types)
- ✅ Reserved indication: `#Reserved.Path` (dots for reserved)
- ✅ Mixed hierarchy: `#Reserved.Path;Custom;More` (dots=reserved, semicolons=custom)
- ✅ Inline pipeline: `|Pipe"{$arg}"`
- ✅ Definition vs invocation context
- ✅ Dual-context markers `[|]`, `[~]`, `[*]`
- ✅ String interpolation: `"{$var:format}"`
- ✅ Collection literals: `{1, 2, 3}`
- ✅ Datetime/Duration literals: `DT"..."`

---

## Pattern Priority for Implementation

### Phase 1: Core Patterns (MVP)
1. Variable assignment
2. Literals (int, float, string)
3. Basic expressions (binary operators)
4. Simple conditionals
5. Pipeline calls (basic, no I/O yet)

### Phase 2: Control Flow
6. Indentation-based nesting
7. Match expressions
8. Error handling
9. Boolean logic markers

### Phase 3: Advanced Features
10. Pipeline definitions
11. I/O binding (input/output parameters)
12. Enum/error definitions
13. Loop constructs (unpack/pack)

### Phase 4: Complex Patterns
14. Reserved indication (`;` hierarchies)
15. Inline pipelines
16. Dual-context markers
17. Pipeline composition
18. Type annotations and inference

---

**Last Updated**: 2025-12-18
**Companion Documents**:
- [Token Patterns](./token-patterns.md) - Token-level patterns
- [Operator Precedence](./README.md#operator-precedence-table) - Expression parsing
- [Syntax Guide](../User/language/syntax/README.md) - Detailed syntax explanations
