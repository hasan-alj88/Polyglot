---
# BMAD Agent Context Block
# Schema: bmad-context-v1
id: correct-patterns-v004
type: reference
status: draft-for-review
updated: 2025-12-18
version: 0.0.4
---

# Polyglot v0.0.4 Correct Syntax Patterns

**For Review and Approval**
**Based on**: User corrections + v0.0.4-feature-showcase-corrected.pg + loop-system.md

---

## 1. File Structure Pattern

```polyglot
// REQUIRED: Package definition (exactly one, must be first)
{@} @Registry::Package:Version
[A] @Alias
[#] 1 // 1st file for this pakage (optional)
[@] @DependencyAlias << @Dependency.Full.name
... other dependices ...
{x}

// OPTIONAL: Any number of enumerations
{#} #Enumeration.Name
[.] .variant1
[.] .variant2
{x}

// OPTIONAL: Any number of error definitions
{!} !Error.Name
[.] .variant1
[.] .variant2
{x}

// OPTIONAL: Any number of pipeline definitions
{|} |Pipeline.Name
[|] <input1 :type
[|] <input2 :type
[t] |T.Trigger
[W] |W.Wrapper
[r] // ... logic ...
[|] >output << $destination
{x}
```

**Rules:**
- `{@}` package definition MUST be first
- Any number of `{#}`, `{!}`, `{|}` definitions can follow
- Order after `{@}` is flexible

---

## 2. Source Patterns (Value Patterns)

Sources are where values originate before being pushed to destinations.

```polyglot
// Source as literal value
[r] $num :pg.int << 5

// Source as variable
[p] $num2 :pg.int << $num

// Source as inline pipeline (formatted string IS the input)
[r] $data :pg.serial << |YAML.Load"{$file}"

// Source as enum data field
[r] $value :pg.int << #SomeEnum.variant.data_field

// Source as pipeline call result
[r] |ProcessData
[|] <input << $raw_data
[|] >output >> $result  // $result becomes source for next operation
```

**Pattern:**
```
(marker) $destination (optional:type) << (source)
```

Type is needed if it cannot be inferred. otherwise its optional.

---

## 3. Enumeration Definition Pattern

```polyglot
{#} #Enumeration.Full.Name.Including;Reserved;Fields
[A] #Alias
   [%] %Scope << #Scope.File  // Optional: file scope only
[.] .enum_field1                 // Variant field (no type = variant)
[.] .enum_field2                 // Variant field
   [.] .data_field :data.type    // Data field (type = data, Pending state)
   [.] .data_final :data.type << (Source)  // Data field with value (Final state)
{x}
```

**Rules:**
- Siblings MUST be either ALL variant fields OR ALL data fields (no mixing at same level)
- Variant fields CAN have nested fields (3-space indent)
- Data fields CANNOT have nested fields
- Same pattern applies to `{!}` error definitions

**Example:**
```polyglot
{#} #ApiResponse
[.] .success
   [.] .data :serial           // Data field under .success variant
   [.] .status_code :int       // Data field under .success variant
[.] .error
   [.] .message :string        // Data field under .error variant
   [.] .error_code :int        // Data field under .error variant
{x}
```

---

## 4. IO Pattern (Pipeline Invocation)

```polyglot
(execution_marker) |PipelineName
(operator_marker) (input_identifier) (datatype) << (Source)
... as many input params ...
(operator_marker) (output_identifier) (datatype) >> $destination
... as many output params ...
```

**Example:**
```polyglot
[r] |MyPipeline
[|] <input1 << |DT.Now""
[|] <input2 << 1
[|] <input3 << $num
[|] >output >> $out
```

**Another Example (Loop):**
```polyglot
[p] ~ForEach.Array
[~] <array << $array_var
[~] >item >> $item_var
   [r] // ... iteration code ...
   [v] *Collect.Array
   [*] <item << $processed_item
   [*] >array >> $processed_array
```

**Pattern Components:**
- Definition context: `i<` for inputs, `o>` for outputs
- Invocation context: `<` for inputs, `>` for outputs
- `[|]` marker for pipeline I/O
- `[~]` marker for unpack I/O
- `[*]` marker for pack I/O

---

## 5. Conditional Execution Pattern (Fork)

**Marker Change:** v0.0.3 `[?]` → v0.0.4 `[f]` (visualizes fork/branch)

**Single Condition:**
```polyglot
[f] $value >? 10
   [r] $log << "Greater than 10"
[f] *?                           // MANDATORY catch-all
   [r] $log << "Less than or equal to 10"
```

**Multiple Conditions:**
```polyglot
[f] $status =? #Active
   [r] $log << "Active"
[f] $status =? #Inactive
   [r] $log << "Inactive"
[f] *?                           // MANDATORY catch-all
   [r] $log << "Other status"
```

**Nested Conditions (Exhaustive at all levels):**
```polyglot
[f] $value >? 10
   [f] $value <? 20
      [r] $log << "Between 10 and 20"
   [f] *?
      [r] |U.Do.Nothing         // Empty operation
[f] *?
   [r] |U.Do.Nothing
```

**Pattern:**
```
[f] (comparison_expression)
   (indented execution markers - 3 spaces)
[f] *?                          // MANDATORY: exhaustive catch-all
   (default branch)
```

**Rules:**
- Conditionals MUST be exhaustive
- MUST include `[f] *?` catch-all
- All `[f]` branches at same indentation level are siblings

---

## 6. Match Expression Pattern

**Pattern:** Assign destination based on source value matching

```polyglot
[m] $destination << $source
   [?] (source_possible_value) ? (destination_value)
   [?] (source_possible_value) ? (destination_value)
   [?] * ? (default_destination_value)  // Wildcard for exhaustiveness
```

**Example:**
```polyglot
[m] $routing << $priority
   [?] "high" ? "express"
   [?] "medium" ? "standard"
   [?] "low" ? "economy"
   [?] * ? "standard"            // REQUIRED: exhaustive
```

**Complex Example with Enum Construction:**
```polyglot
[m] $response << $response_type
   [?] "success" ? #ApiResponse.success
      [.] .data << $data
      [.] .status_code << 200
   [?] "error" ? #ApiResponse.error
      [.] .message << "Error occurred"
      [.] .error_code << 500
   [?] * ? #ApiResponse.error
      [.] .message << "Unknown type"
      [.] .error_code << 400
```

**Pattern:**
```
[m] $variable << $source
   [?] source_value ? destination_value
   [?] * ? default  // MANDATORY: exhaustive
```

**Rules:**
- Source possible values MUST be exhaustive
- MUST include `[?] * ?` wildcard catch-all
- Pattern matches: source → destination

---

## 7. Error Handling Pattern

**Try Block with Error Catching:**
```polyglot
[r] |MightFail
[|] <input << $data
[|] >result >> $result
   [!] !Network.Timeout
      [r] $retry << true
   [!] !Network.*               // Wildcard: all Network errors
      [r] $log << "Network Error"
   [!] !IO.FileNotFound
      [r] $log << "File missing"
   [!] !*                       // Catch-all: any unhanded error
      [r] $log << "Unknown error"
[r] $log << "Completed with result: {$result}"
```

**Pattern:**
```
[p] |PipelineThatMightFail
(IO bindings)
   [!] !SpecificError.Type
      (error handling actions - 3 spaces indent)
   [!] !Error.Category.*       // Wildcard for category
      (category error handling)
   [!] !*                      // Catch-all for unhandled
      (default error handling)
(code after error block executes regardless)
```

**Rules:**
- Error cases indented 3 spaces under `[!]` pipeline marker
- `!*` catches all unhandled errors
- Can use wildcards: `!Error.Category.*`
- Error handlers are siblings at same indentation level
- Code after error block always executes

---

## 8. Boolean Logic Patterns

**AND Logic:**
```polyglot
[f] $a >? 5
[&] $b <? 10
   [r] $result << "Both true"
```

**OR Logic:**
```polyglot
[f] $a >? 5
[|] $b <? 10
   [r] $result << "At least one true"
```

**XOR Logic:**
```polyglot
[f] $a >? 5
[^] $b <? 10
   [r] $result << "Exactly one true"
```

**Grouped Boolean:**
```polyglot
[f] $a >? 5
   [^] $b <? 10
[|] $c >? 5
[^] $d >? 100
   [r] $result << "(c1 ^ c2) | (c3 ^ c4)"
```

**Pattern:**
```
[marker with boolean expression] (boolean expression)
[boolean_marker] (boolean expression)
   [boolean_marker] (boolean expression)
   [boolean_marker] (boolean expression)
   (indented action when logic satisfied)
```

**Rules:**
- `[|]` means OR **if and only if** it's under:
  - Boolean marker context, OR
  - `[t]` trigger context, OR
  - `[f]` conditional context
- Otherwise `[|]` means pipeline I/O
- Boolean markers chain: `[f] → [&] → [|] → [^]`

---

## 9. Loop Collection Pattern (Unpack/Pack)

**Array Iteration:**
```polyglot
[p] ~ForEach.Array
[~] <array << $source_array
[~] >item >> $current_item
[~] >index >> $current_index
   [r] $processed << |Process
   [|] <data << $current_item
   [|] >result >> $processed_item
   [v] *Collect.Array
   [*] <item << $processed_item
   [*] >array >> $result_array
```

**Set Iteration:**
```polyglot
[p] ~ForEach.Set
[~] <set << $source_set
[~] >item >> $current_item
   (iteration logic)
   [v] *Into.Set
   [*] <item << $processed_item
   [*] >set >> $result_set
```

**Map Iteration:**
```polyglot
[p] ~ForEach.Serial
[~] <map << $source_map
[~] >key >> $current_key
[~] >value >> $current_value
   (iteration logic)
   [v] *Into.Serial
   [*] <key << $current_key
   [*] <value << $processed_value
   [*] >map >> $result_map
```

**Pattern:**
```
[execution_marker] ~UnpackOperator.Type
[~] <collection << $source
[~] >item >> $iteration_var
(optional: [~] >index >> $index_var)
   (iteration execution markers - 3 spaces indent)
   [v] *PackOperator.Type
   [*] <item << $processed
   [*] >collection >> $destination
```

**Rules:**
- `[~]` unpack starts iteration scope
- Iteration variables only exist in nested scope (3-space indent)
- `[*]` pack collects results back to main scope
- Unpack type must match pack type
- The unpack outputs (`[~] >item`) are inputs to mini-pipeline
- Pack operator outputs (`[*] >collection`) go to main pipeline (or next iteration if chained)
- `[v]` join marker required before pack operator

---

## 10. Join Operation Pattern

Join operators to to join variables from parallel pipelines into the main pipeline.

```polyglot
[v] *Join.All
[*] $var_from_parallel1
[*] $var_from_parallel2
[*] $var_from_parallel3
```

similar to IO pattern in 4

---

## 11. Serial Load Block Pattern

**Loading Multiple Files in Parallel:**
```polyglot
[s] $load_group
   [r] $config << |YAML.Load"{$config_file}"
   [r] $data << |JSON.Load"{$data_file}"
   [r] $schema << |XML.Load"{$schema_file}"
```

**Pattern:**
```
[s] $serial_group_name
   [r] $var1 << |LoadOperation1
   [r] $var2 << |LoadOperation2
   (all execute in parallel, block until ALL complete)
```

**Rules:**
- All lines inside `[s]` block execute in parallel
- Execution blocks until ALL operations complete
- Results available in main scope after `[s]` block
- Each line must be independent (no dependencies between them)

---

## 12. Variable Life-cycle Patterns

**Pending State (Declaration only):**
```polyglot
[r] $value :pg.int
```

**Default State (With default value):**
```polyglot
[r] $value :pg.int <~ 10
```

**Final State (Assigned value):**
```polyglot
[r] $value :pg.int << 5
```

**Faulted State (Error occurred):**
```polyglot
[!] |MightFail
[|] >result >> $value  // If pipeline fails, $value is Faulted
```

**Released State (Out of scope):**
```polyglot
...
[r] $temp << 5     
[|] <out << $temp
{x}
// $temp released after this block
```

**Lifecycle Flow:**
```
Pending → Default → Final
      ↓    ↓         ↓
     Faulted   →  Released
```

**Pattern:**
```
[r] $variable :type                 // Pending
[r] $variable :type <~ (default)    // Default
[r] $variable :type << (source)     // Final
```

---

## 13. Metadata Annotation Pattern
**Meta-data schema**
%
├─ Package {@}
 |       ├─ Doc
 |       ├─ Deprecated
 |       └─ Author
 |                ├─ 0
 |                 |     ├─ Name
 |                 |     └─ Since
 |                ├─ 1
 |                 |     ├─ Name
 |                 |     └─ Since
 |                 ...
├─ Pipeline {|}
 |       ├─ Doc
 |       ├─ Deprecated
 |       ├─ Author
 |        |       ├─ 0
 |        |        |     ├─ Name
 |        |        |     └─ Since
 |        |       ├─ 1
 |        |        |     ├─ Name
 |        |        |     └─ Since
 |        |        ...
 |       └─ Inline
├─ Enum {#}
 |       ├─ Doc
 |       ├─ Deprecated
 |       ├─ Reserved
 |       └─ Author
 |                ├─ 0
 |                 |     ├─ Name
 |                 |     └─ Since
 |                ├─ 1
 |                 |     ├─ Name
 |                 |     └─ Since
 |                 ...
├─ Error {!}
 |       ├─ Doc
 |       ├─ Deprecated
 |       ├─ Reserved
 |       └─ Author
 |                ├─ 0
 |                 |     ├─ Name
 |                 |     └─ Since
 |                ├─ 1
 |                 |     ├─ Name
 |                 |     └─ Since
 |                 ...
└─ Enum field `[.]`
     └─ Reserved

**Single Metadata:**
```polyglot
[r] $value << 5
   [%] %Doc << "This is documentation"
```

**Multiple Metadata:**
```polyglot
{|} |MyPipeline
[%] %Doc << "Pipeline description"
[%] %Author << "John Doe"
[%] %Version << "1.0.0"
[<] i<input :pg.int
[>] o>output :pg.int >> $result
{x}
```

**Multi-line Metadata:**
```polyglot
[%] %Doc <<
[+] +"This is multi-line documentation"
[+] +"with multiple lines"
```

**Pattern:**
```
(any marker or definition)
[%] %MetadataKey << "value"
[%] %AnotherKey << "value"
```

**Common Metadata Keys:**
- `%Doc` - Documentation
- `%Author` - Author information
- `%Version` - Version number
- `%Deprecated` - Deprecation notice
- `%Since` - Version introduced
- `%Scope` - Scope specification (#Scope.File, #Scope.Package)
- `%InStream` - Variadic input configuration
- `%ItemType` - Type specification

---

## 14. Pipeline Composition Pattern

**Chaining Pipelines:**
```polyglot
[r] |FetchData
   <url << $url
   |> |ParseJSON
   |> |ValidateSchema
   |> |TransformData
   >result >> $final_data
   >errors >> $processing_errors
```

**Pattern:**
```
[r] |Pipeline1
   <input << $source
   |> |Pipeline2
   |> |Pipeline3
   >output >> $destination
```

**Rules:**
- `|>` operator chains pipeline output to next pipeline input
- Left-to-right evaluation
- Highest precedence operator (precedence level 1)
- Output of one pipeline becomes input of next

---

## 15. Variadic Push Pattern

**Push Multiple Values:**
```polyglot
[r] $array << {1, 2, 3}
[r] $array <<< 4              // Push single value (append)
[r] $array <<< {5, 6, 7}      // Push multiple values (append collection)
```

**Pull Multiple Values:**
```polyglot
[r] $values >>> $array        // Pull from array into $values
```

**Variadic Pipeline Inputs:**
```polyglot
[r] |Set.Union
   <<< $set1
   <<< $set2
   <<< $set3
   >>> $all_numbers
```

**Pattern:**
```
[r] $collection <<< (value_or_collection)  // Variadic push
[r] $destination >>> $collection            // Variadic pull
[r] |Pipeline
   <<< $input1
   <<< $input2
   >>> $output
```

**Rules:**
- `<<<` pushes to collection without replacing (append/extend)
- `>>>` pulls from collection
- Can push/pull single values or collections
- Used with variadic pipelines (%InStream metadata)

---

## 16. Type Annotation Pattern

**Variable with Type:**
```polyglot
[r] $value :pg.int << 5
```

**Complex Collection Types:**
```polyglot
[r] $list :pg.array.pg.string << {"a", "b", "c"}
[r] $map :pg.map.pg.string.pg.int << {"key": 1}
[r] $set :pg.set.pg.int << {1, 2, 3}
```

**Pipeline Input/Output Types:**
```polyglot
{|} |TypedPipeline
[<] i<input :pg.string
[<] i<count :pg.int
[>] o>result :pg.array.pg.string >> $output
{x}
```

**Pattern:**
```
$variable :type.path.segments
i<input :type.path.segments
o>output :type.path.segments
```

**Type Segments:**
- Primitive: `:pg.int`, `:pg.string`, `:pg.float`, `:pg.bool`, `:pg.dt`
- Collection: `:pg.array.T`, `:pg.set.T`, `:pg.map.K.V`
- Serial: `:pg.serial` (polymorphic data)
- Custom: `:CustomType.SubType`
- Reserved: `:pg;Type;Reserved` (semicolon for reserved segments)

---

## 17. Collection Literal Pattern

**Array Literal:**
```polyglot
[r] $array :pg.array.pg.int << {1, 2, 3, 4, 5}
```

**Set Literal:**
```polyglot
[r] $set :pg.set.pg.string << {"a", "b", "c"}
```

**Map Literal:**
```polyglot
[r] $map :pg.map.pg.string.pg.int << {"key1": 1, "key2": 2}
```

**Empty Collections:**
```polyglot
[r] $empty_array << {}
[r] $empty_set :pg.set.pg.int << {}
[r] $empty_map :pg.map.pg.string.pg.int << {}
```

**Pattern:**
```
{value1, value2, value3}           // Array or Set
{key1: value1, key2: value2}       // Map
{}                                  // Empty (type inferred or annotated)
```

**Rules:**
- Curly braces `{}` for collection literals
- Type determines if array vs set (set requires unique values)
- Map uses `:` separator between key and value
- Empty `{}` requires type annotation for disambiguation

---

## 18. Range Operator Pattern

**Numeric Range:**
```polyglot
[r] $range << 1..10         // 1 to 10 inclusive
[r] $range << 1..<10        // 1 to 9 (exclusive end)
[r] $range << 1..=10        // 1 to 10 (explicit inclusive)
```

**Range in Conditionals:**
```polyglot
[f] $age ?[18, 65]          // Inclusive both: [18, 65]
[f] $age ?(18, 65)          // Exclusive both: (18, 65)
[f] $age ?(18, 65]          // Exclusive left: (18, 65]
[f] $age ?[18, 65)          // Exclusive right: [18, 65)
[f] $age !?[18, 65]         // NOT in range
```

**Iterating Range:**
```polyglot
[p] ~ForEach.Range"1..100"
[~] >value >> $i
   [r] $log << "Iteration {$i}"
   [v] *Into.Array
   [*] <item << $i
   [*] >array >> $indices
```

**Pattern:**
```
// Range literals
(start)..(end)      // Inclusive range
(start)..<(end)     // Exclusive end
(start)..=(end)     // Explicit inclusive

// Range conditionals
?[(start), (end)]   // Inclusive both
?((start), (end))   // Exclusive both
?((start), (end)]   // Exclusive left, inclusive right
?[(start), (end))   // Inclusive left, exclusive right
!?[...]             // Negated range
```

---

## 19. String Formatting Pattern

**Formatted String Variable:**
```polyglot
[r] $name << "World"
[r] $message << "Hello, {$name}!"
```

**Inline Pipeline with Format:**
```polyglot
[r] $formatted << |String.Format"{$first} {$last}"
```

**Multi-line String with [+] Marker:**
```polyglot
[r] $query << |SQL""
[+] +"SELECT * FROM users"
[+] +"WHERE id = "
[+] +|SQL"{$user_id}"
```

**Pattern:**
```
"text {$variable} more text"        // Single-line with interpolation
|Pipeline"{$arg1} {$arg2}"          // Inline pipeline (NO SPACE!)
[+] +"line1"                         // Multi-line concatenation
[+] +"line2"
```

**Rules:**
- `{$variable}` for interpolation
- Inline pipelines: NO SPACE between pipeline identifier and string
- `[+]` marker for string concatenation/multi-line
- Can nest inline pipelines: `+|SQL"{$var}"`

---

## 20. Comment Patterns

**Single Line Comment:**
```polyglot
// This is a comment
[r] $value << 5  // Inline comment
```

**Block Comment:**
```polyglot
/*
 * Multi-line comment
 * Can span multiple lines
 */
[r] $value << 5
```

**Documentation via Metadata:**
```polyglot
{|} |DocumentedPipeline
[%] %Doc <<
[+] +"This pipeline processes data."
[+] +"Parameters:"
[+] +"  - input: The data to process"
[+] +"  - output: The processed result"
[<] i<input :pg.string
[>] o>output :pg.string >> $result
{x}
```

**Pattern:**
```
// (single line comment)
/* (multi-line comment) */
[%] %Doc << "documentation"         // Single-line doc
[%] %Doc <<                          // Multi-line doc
[+] +"line 1"
[+] +"line 2"
```

---

## 21. Inline Pipeline Pattern

**Critical:** Inline pipeline = Pipeline identifier + Formatted string (NO SPACE)

```polyglot
// Inline pipeline with arguments
[r] $result << |FormatName"{$first} {$last}"

// Inline pipeline with no arguments (empty string)
[r] $timestamp << |DT.Now""

// Inline pipeline in multi-line context
[r] $query << |SQL""
[+] +"SELECT * FROM users WHERE id = "
[+] +|SQL"{$user_id}"
```

**Pattern:**
```
|PipelineIdentifier"formatted string with {$interpolation}"
```

**Rules:**
- NO SPACE between `|Identifier` and `"`
- Formatted string IS the ONLY input to the pipeline
- Pipeline extracts arguments from `{$var}` interpolations
- Empty string `""` means no arguments
- Used for transformation/formatting pipelines

---

## 22. Pipeline Definition Pattern

**Complete Pipeline Structure:**
```polyglot
{|} |Pipeline.Full.Name;Reserved;Segments
[A] |Alias
[%] %Doc << "Documentation"

// Input parameters (definition)
[<] i<input1 :pg.string
[<] i<input2 :pg.int <~ 10          // With default

// Trigger (MANDATORY)
[t] |T.TriggerType

// Wrapper (MANDATORY - can be |W.Polyglot.Scope for none)
[W] |W.WrapperType

// Logic
[r] $result << |SomeOperation
[|] <input << $input1
[|] >output >> $intermediate

// Output parameters (definition)
[>] o>output1 :pg.string >> $result
[>] o>output2 :pg.int >> $count
{x}
```

**Minimal Pipeline:**
```polyglot
{|} |MinimalPipeline
[t] |T.Call
[W] |W.Polyglot.Scope
[r] $result << "Hello"
[>] o>output :pg.string >> $result
{x}
```

**Pattern:**
```
{|} |PipelineName
[A] |Alias                           // Optional
[%] %Metadata << "value"             // Optional
[<] i<input :type                    // Input params (optional)
[<] i<input :type <~ default         // With default
[t] |T.Trigger                       // MANDATORY
[W] |W.Wrapper                       // MANDATORY
// ... logic markers ...
[>] o>output :type >> $destination   // Output params (optional)
{x}
```

**Rules:**
- `[<] i<` for input parameter definition
- `[>] o>` for output parameter definition
- `[t]` trigger is MANDATORY
- `[W]` wrapper is MANDATORY (use |W.Polyglot.Scope for none)
- Logic section can contain any execution markers

---

## 23. Package Definition Pattern

```polyglot
{@} @Registry::Package:Version.Patch.Build.Revision
[A] @Alias
[<] @Dependency1
[<] @Dependency2
[%] %Doc << "Package documentation"
[%] %Author << "Author name"
{x}
```

**Pattern:**
```
{@} @Registry::PackageName:Version
[A] @Alias                           // Optional
[<] @Dependency                      // Optional, multiple allowed
[%] %Metadata                        // Optional
{x}
```

**Rules:**
- MUST be first in file
- Format: `@Registry::Package:Version`
- Version format: `Major.Minor.Patch.Build` (all integers)
- Dependencies: `[<] @OtherPackage` (full registry path)
- Aliases: `[A] @ShortName` for this package

---

## Summary of Key Pattern Rules

1. **Indentation:** 3 spaces = 1 nesting level
2. **Exhaustiveness:** `[f]` conditionals MUST include `[f] *?` catch-all
3. **Match:** `[m]` expressions MUST include `[?] * ?` wildcard
4. **Errors:** `[!]` blocks should include `[!] !*` catch-all
5. **Definition vs Invocation:** `i<` / `o>` in definitions, `<` / `>` in invocations
6. **Variable Prefix:** `$` for all variables
7. **Inline Pipelines:** NO SPACE between identifier and string
8. **Boolean Context:** `[|]` means OR only under boolean/trigger/conditional context
9. **Loop Structure:** `[~]` unpack → iteration logic → `[v]` join → `[*]` pack
10. **Marker Order in Pipeline:** `[<]` inputs → `[t]` trigger → `[W]` wrapper → logic → `[>]` outputs

---

**Status:** Ready for review and approval
**Next Step:** Validate against actual parser implementation
