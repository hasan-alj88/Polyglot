---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/line-continuation.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Line Continuation

## Overview

Polyglot supports multiple ways to split long expressions across lines for better readability. This guide covers all line continuation rules and best practices.

## Backslash Continuation

Use `\` at the end of a line to continue to the next line:

```polyglot
# Long expression split across lines
result = some_long_function_name(arg1, arg2, arg3 \
  |> transform \
  |> validate \
  |> save

# Long string concatenation
message = "This is a very long message " \
  + "that spans multiple lines " \
  + "for better readability"

# Long arithmetic
total = base_price \
  + tax \
  + shipping \
  - discount
```

## Natural Continuation

Certain contexts allow natural line continuation without backslash:

### Function Arguments

```polyglot
# Arguments naturally continue
result = complex_function(
  argument1,
  argument2,
  argument3,
  argument4


# No backslash needed in parentheses
value = calculate(
  base * multiplier,
  offset + adjustment,
  config.parameter

```

### List Literals

```polyglot
# Lists can span multiple lines
numbers = [
  1,
  2,
  3,
  4,
  5
]

# Complex list items
users = [
  User{id: 1, name: "Alice",
  User{id: 2, name: "Bob",
  User{id: 3, name: "Charlie"
]
```

### Map Literals

```polyglot
# Maps naturally span lines
config = {
  "host": "localhost",
  "port": 8080,
  "timeout": 30,
  "retries": 3


# Nested structures
settings = {
  "database": {
    "host": "db.example.com",
    "port": 5432
  ,
  "cache": {
    "host": "cache.example.com",
    "port": 6379
  

```

### Struct Initialization

```polyglot
# Struct fields can span lines
user = User{
  id: 1,
  name: "Alice",
  email: "alice@example.com",
  created_at: now(,
  updated_at: now(

```

## Pipeline Continuation

Pipeline operators naturally continue across lines:

```polyglot
# Clean pipeline formatting
result = fetch_data(
  |> parse
  |> validate
  |> transform
  |> aggregate
  |> format

# With backslash for additional clarity
report = load_data( \
  |> clean_data \
  |> apply_business_rules \
  |> generate_report \
  |> send_to_recipients
```

## Method Chaining

Method chains can continue naturally:

```polyglot
# Method chaining across lines
result = data
  .filter(is_valid
  .map(transform
  .sort(
  .take(10

# With intermediate results
processed = input
  .clean(
  .validate(
  .transform(
  .aggregate(
```

## Continuation in Control Flow

### If Expressions

```polyglot
# Long conditions
if very_long_condition_one &&
   very_long_condition_two &&
   very_long_condition_three {
  # Body


# With backslash for clarity
if complex_check(data \
   && another_check(config \
   && final_validation(state {
  # Body


# Ternary-style if
result = if long_condition_check(x, y, z
  { value_when_true 
  else { value_when_false 
```

### Match Expressions

```polyglot
# Match arms naturally span lines
match result {
  Result.Ok(value => {
    process(value
    log_success(
  ,
  Result.Err(e => {
    log_error(e
    handle_failure(
  


# Long patterns
match complex_enum {
  ComplexType.VariantOne(a, b, c =>
    process_variant_one(a, b, c,
  ComplexType.VariantTwo(x, y =>
    process_variant_two(x, y,
  _ =>
    handle_default(

```

## Block Continuation

Block bodies are naturally indented:

```polyglot
&#124;Pipeline ProcessData
  input: LargeDataStructure
  output: Result<
    ProcessedData,
    ProcessingError
  >

  # Pipeline body naturally continues
  step1 = extract(input
  step2 = transform(step1
  step3 = validate(step2

  return Result.Ok(
    ProcessedData{step3
  
!
```

## String Continuation

### Multi-line Strings

```polyglot
# Triple-quoted strings preserve newlines
text = """
  This is a multi-line string.
  Each line is preserved.
  Indentation is maintained.
  """

# Single-line strings with backslash
message = "This is a long message " \
  "that is split across " \
  "multiple lines but " \
  "remains a single string"
```

### String Interpolation

```polyglot
# Multi-line interpolated strings
greeting = """
  Hello, {user.name!

  Your account status: {user.status
  Last login: {user.last_login
  """

# Long interpolation expressions
info = "User {user.id " \
  + "({user.name " \
  + "registered on {user.created_at"
```

## Expression Continuation

### Logical Expressions

```polyglot
# AND conditions
valid = is_authenticated(user &&
  has_permission(user, resource &&
  is_within_quota(user &&
  is_enabled(feature

# OR conditions
should_notify = is_critical(event ||
  is_high_priority(event ||
  user_opted_in(user

# Complex combinations
result = (
  condition_a &&
  condition_b
 || (
  condition_c &&
  condition_d

```

### Arithmetic Expressions

```polyglot
# Long calculations
total_cost = base_price +
  (tax_rate * base_price +
  shipping_cost +
  handling_fee -
  discount_amount

# Complex formulas
result = (
  numerator_part_a +
  numerator_part_b
 / (
  denominator_part_a -
  denominator_part_b

```

## Indentation Rules

### Consistent Indentation

```polyglot
# Good: Consistent indentation
result = function(
  arg1,
  arg2,
  arg3


# Good: Aligned continuation
value = some_value +
  another_value +
  third_value

# Less clear: Inconsistent indentation
result = function(
arg1,
    arg2,
  arg3

```

### Nested Continuation

```polyglot
# Nested structures maintain indentation
config = {
  "database": {
    "primary": {
      "host": "db1.example.com",
      "port": 5432
    ,
    "replica": {
      "host": "db2.example.com",
      "port": 5432
    
  ,
  "cache": {
    "host": "cache.example.com",
    "port": 6379
  

```

## Best Practices

### 1. Use Natural Continuation When Possible

```polyglot
# Good: Natural continuation
users = [
  user1,
  user2,
  user3
]

# Unnecessary: Backslash in natural context
users = [ \
  user1, \
  user2, \
  user3 \
]
```

### 2. Align Related Items

```polyglot
# Good: Aligned parameters
result = calculate(
  base_amount,
  tax_rate,
  discount_percent


# Good: Aligned pipeline
data = input
  |> step1
  |> step2
  |> step3
```

### 3. Break at Logical Points

```polyglot
# Good: Break at operators
total = subtotal +
  tax +
  shipping -
  discount

# Less clear: Break in middle of terms
total = subtotal + tax +
  shipping - discount
```

### 4. Keep Related Code Together

```polyglot
# Good: Grouped by relationship
user = User{
  # Identity
  id: generate_id(,
  username: input.username,

  # Profile
  name: input.name,
  email: input.email,

  # Metadata
  created_at: now(,
  updated_at: now(

```

### 5. Avoid Excessive Line Length

```polyglot
# Good: Reasonable line length
result = transform(
  very_long_parameter_name,
  another_long_parameter,
  configuration_object


# Less readable: Too long
result = transform(very_long_parameter_name, another_long_parameter, configuration_object
```

## Common Patterns

### Long Function Signatures

```polyglot
fn process_complex_data(
  input_data: LargeDataStructure,
  configuration: ProcessingConfig,
  options: ProcessingOptions,
  callback: (Result -> (
 -> Result<ProcessedData, Error> {
  # Implementation

```

### Chained Transformations

```polyglot
report = raw_data
  .filter(is_valid
  .map(normalize
  .group_by(get_category
  .aggregate(sum
  .sort_by(get_value
  .take(10
  .format(
```

### Complex Conditionals

```polyglot
if (user.is_authenticated( &&
    user.has_permission(resource &&
    resource.is_available( ||
   (user.is_admin( &&
    emergency_mode {
  grant_access(

```

### Pipeline with Error Handling

```polyglot
result = fetch_data( \
  |> parse \
  |> validate \
  |> transform \
  |> save

match result {
  Result.Ok(data =>
    log_success(data,
  Result.Err(e =>
    handle_error(e

```

## Quick Reference

| Context | Continuation Style | Example |
|---------|-------------------|---------|
| Explicit | Backslash `\` | `a + b \` |
| Function args | Natural (in `(` | `fn(arg1,` |
| Lists | Natural (in `[]` | `[item1,` |
| Maps | Natural (in `{` | `{"key": value,` |
| Pipelines | Natural after `&#124;>` | `data &#124;> fn` |
| Methods | Natural after `.` | `obj.method(` |
| Operators | Natural after operator | `a +` |

## Continuation Rules Summary

1. **Backslash (`\`**: Explicit continuation anywhere
2. **Parentheses**: Natural continuation in `(...`
3. **Brackets**: Natural continuation in `[...]` and `{...`
4. **Operators**: Natural continuation after binary operators
5. **Pipeline**: Natural continuation after `|>`
6. **Method chains**: Natural continuation after `.`

## Next Steps

- **Style Guide**: Learn about [code formatting standards](../style-guide.md
- **Syntax Overview**: Review [general syntax rules](overview.md
- **Examples**: See [well-formatted code examples](../examples/

---

**See also**: [Syntax Overview](overview.md | [Operators](operators.md | [Comments](comments.md
