<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

---

> ⚠️ **HISTORICAL DOCUMENT - DEPRECATED SYNTAX**
>
> This document contains **v0.0.3 syntax** with significant differences from v0.0.4:
>
> **Critical Syntax Changes:**
> - `[V]` (uppercase) → `[v]` (lowercase) for join marker
> - Additional prefix system refinements
> - Reserved indication using semicolon (`;`)
>
> **For current v0.0.4 syntax, see:**
> - [Main Documentation](../../README.md)
> - [v0.0.4 Grammar](../../reference/grammar.md)
> - [Markers Reference](../../language/syntax/markers.md)

---

# Pipelines as Variables - Design Proposal

**Date:** 2025-12-11
**Status:** 💡 DESIGN PROPOSAL

---

## Philosophy

> **"Pipelines are first-class citizens. They can be stored, passed, and invoked like any other value."**

In Polyglot, pipelines should be:
- Storable in variables
- Passable as parameters
- Returnable from other pipelines
- Composable dynamically

---

## The `:pg.pipeline` Type

### Basic Pipeline Variable

```polyglot
{|} |UsePipelineVariable
[t] |T.Call
[W] |W.Polyglot.Scope

// Store pipeline reference in variable
[r] $formatter:pg.pipeline << |String.ToUpperCase

// Invoke pipeline from variable
[r] $formatter
   <input << "hello world"
   >output >> $result

[>] o>result << $result:string           // "HELLO WORLD"
{x}
```

### Pipeline Type with Signature

Specify expected inputs/outputs for type safety:

```polyglot
{:} :pipeline.string_transformer
[<] i<value:pg.pipeline

[%] %Constraint
   [.] .signature:#pg.pipeline.signature
      [.] .inputs
         [.] .input:string
      [.] .outputs
         [.] .output:string
{x}

{|} |ApplyTransform
[<] i<transformer:pipeline.string_transformer
[<] i<text:string

[t] |T.Call
[W] |W.Polyglot.Scope

// Invoke pipeline passed as parameter
[r] $transformer
   <input << $text
   >output >> $result

[>] o>result << $result:string
{x}
```

---

## Pipeline Selection - Match Expression

**Use case:** Select pipeline based on runtime conditions

```polyglot
{|} |ProcessWithStrategy
[<] i<strategy:string
[<] i<data:serial

[t] |T.Call
[W] |W.Polyglot.Scope

// Match returns pipeline reference
[m] $processor:pg.pipeline << $strategy
   [?] "fast" ? |FastProcessor
   [?] "accurate" ? |AccurateProcessor
   [?] "balanced" ? |BalancedProcessor
   [?] * ? |DefaultProcessor

// Invoke selected pipeline
[r] $processor
   <data << $data
   >result >> $processed

[>] o>result << $processed:serial
{x}
```

---

## Pipeline Collections

### Array of Pipelines

```polyglot
{|} |ChainTransformations
[<] i<input:string

[t] |T.Call
[W] |W.Polyglot.Scope

// Array of transformation pipelines
[r] $transformers:array.pg.pipeline << {
   |String.Trim,
   |String.ToLowerCase,
   |String.RemoveSpecialChars,
   |String.Normalize
}

[r] $result << $input

// Apply each transformation in sequence
[p] ~ForEach
   <array << $transformers
   >item >> $transformer

   [r] $transformer
      <input << $result
      >output >> $result

[>] o>result << $result:string
{x}
```

### Map of Named Pipelines

```polyglot
{|} |DispatchByType
[<] i<message_type:string
[<] i<payload:serial

[t] |T.Call
[W] |W.Polyglot.Scope

// Map type names to handler pipelines
[r] $handlers:map.string.pg.pipeline << #Map
   [.] .user_created << |HandleUserCreated
   [.] .order_placed << |HandleOrderPlaced
   [.] .payment_received << |HandlePaymentReceived

// Look up handler by type
[r] $handler << $handlers<$message_type>

// Invoke handler
[r] $handler
   <payload << $payload
   >result >> $result

[>] o>result << $result:serial
{x}
```

---

## Higher-Order Pipelines

### Pipeline that Returns Pipeline

```polyglot
{|} |CreateValidator
[<] i<min:int
[<] i<max:int

[t] |T.Call
[W] |W.Polyglot.Scope

// Return pipeline reference (not invoked)
[>] o>validator << |ValidateRange
   [%] %Partial                        // Partially applied
      [.] .min << $min
      [.] .max << $max
{x}

// Usage
{|} |UseCustomValidator
[t] |T.Call
[W] |W.Polyglot.Scope

// Create validator with specific bounds
[r] |CreateValidator
   <min << 0
   <max << 100
   >validator >> $age_validator

// Use the created validator
[r] $age_validator
   <value << 150
   >valid >> $is_valid
   >error >> $error

[>] o>valid << $is_valid:bool
{x}
```

### Pipeline that Takes Pipeline as Parameter

```polyglot
{|} |ApplyTwice
[<] i<operation:pg.pipeline
[<] i<value:int

[t] |T.Call
[W] |W.Polyglot.Scope

// Apply pipeline once
[r] $operation
   <input << $value
   >output >> $intermediate

// Apply pipeline again
[r] $operation
   <input << $intermediate
   >output >> $final

[>] o>result << $final:int
{x}

// Usage
{|} |DoubleValue
[<] i<input:int
[t] |T.Call
[W] |W.Polyglot.Scope
[r] $result << $input * 2
[>] o>output << $result:int
{x}

{|} |QuadrupleDemo
[t] |T.Call
[W] |W.Polyglot.Scope

// Pass pipeline as parameter
[r] |ApplyTwice
   <operation << |DoubleValue
   <value << 5
   >result >> $quadrupled             // 20

[>] o>result << $quadrupled:int
{x}
```

---

## Partial Application

**Enable partial application via metadata:**

```polyglot
{|} |HTTP.Request
[<] i<method:string
[<] i<url:string
[<] i<headers:map.string.string
[<] i<body:serial

[t] |T.Call
[W] |W.Polyglot.Scope

// ... implementation
{x}

// Create specialized versions
{|} |CreateAPIClients
[t] |T.Call
[W] |W.Polyglot.Scope

// Partially apply common parameters
[r] $get_user:pg.pipeline << |HTTP.Request
   [%] %Partial
      [.] .method << "GET"
      [.] .url << "https://api.example.com/users/"
      [.] .headers << #Map
         [.] .Authorization << "Bearer token123"

[r] $post_order:pg.pipeline << |HTTP.Request
   [%] %Partial
      [.] .method << "POST"
      [.] .url << "https://api.example.com/orders"
      [.] .headers << #Map
         [.] .Authorization << "Bearer token123"
         [.] .Content-Type << "application/json"

// Use specialized pipelines
[r] $get_user
   <url << "https://api.example.com/users/123"  // Override
   >response >> $user_data

[r] $post_order
   <body << $order_payload
   >response >> $order_result

[>] o>user << $user_data:serial
[>] o>order << $order_result:serial
{x}
```

---

## Dynamic Pipeline Composition

### Compose Multiple Pipelines at Runtime

```polyglot
{|} |BuildPipeline
[<] i<steps:array.string

[t] |T.Call
[W] |W.Polyglot.Scope

// Map step names to pipeline references
[r] $step_registry:map.string.pg.pipeline << #Map
   [.] .validate << |ValidateInput
   [.] .sanitize << |SanitizeInput
   [.] .transform << |TransformData
   [.] .enrich << |EnrichData
   [.] .save << |SaveToDatabase

// Build pipeline array from step names
[r] $pipeline_steps:array.pg.pipeline << {}

[p] ~ForEach
   <array << $steps
   >item >> $step_name

   [r] $step_pipeline << $step_registry<$step_name>
   [r] $pipeline_steps << |Array.Append
      <array << $pipeline_steps
      <item << $step_pipeline

[>] o>pipeline << $pipeline_steps:array.pg.pipeline
{x}

// Usage: Execute dynamic pipeline
{|} |ExecuteDynamicPipeline
[<] i<pipeline:array.pg.pipeline
[<] i<data:serial

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $result << $data

[p] ~ForEach
   <array << $pipeline
   >item >> $step

   [r] $step
      <input << $result
      >output >> $result

[>] o>result << $result:serial
{x}
```

---

## Pipeline Metadata Inspection

**Query pipeline signature at runtime:**

```polyglot
{|} |InspectPipeline
[<] i<pipeline:pg.pipeline

[t] |T.Call
[W] |W.Polyglot.Scope

// Get pipeline metadata
[r] $metadata << $pipeline%Metadata

// Access signature
[r] $inputs << $metadata.signature.inputs
[r] $outputs << $metadata.signature.outputs
[r] $doc << $metadata.Doc

[>] o>inputs << $inputs:array.serial
[>] o>outputs << $outputs:array.serial
[>] o>doc << $doc:string
{x}
```

---

## Callback Pattern

```polyglot
{|} |FetchDataWithCallback
[<] i<url:string
[<] i<on_success:pg.pipeline
[<] i<on_error:pg.pipeline

[t] |T.Call
[W] |W.Polyglot.Scope

[r] |HTTP.Get
   <url << $url
   >response >> $response
   >error >> $error

// Branch based on success/error
[f] $error =? :optional.None
   // Success - invoke success callback
   [r] $on_success
      <data << $response
      >result >> $final_result
[f] *?
   // Error - invoke error callback
   [r] $on_error
      <error << $error
      >result >> $final_result

[>] o>result << $final_result:serial
{x}

// Usage
{|} |UseFetchWithCallbacks
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |FetchDataWithCallback
   <url << "https://api.example.com/data"
   <on_success << |ProcessSuccessfulResponse
   <on_error << |HandleFetchError
   >result >> $result

[>] o>result << $result:serial
{x}
```

---

## Type System Integration

### Pipeline Type Constraints

```polyglot
{:} :pipeline.http_handler
[<] i<value:pg.pipeline

[%] %Constraint
   [.] .signature:#pg.pipeline.signature
      [.] .inputs
         [.] .request:http.request
      [.] .outputs
         [.] .response:http.response
         [.] .error:error
   [.] .metadata
      [.] .async << #Boolean.True
      [.] .timeout << 30000
{x}
```

### Generic Pipeline Types

```polyglot
{:} :pipeline.transformer
[<] i<value:pg.pipeline
[<] i<input_type:pg.type
[<] i<output_type:pg.type

[%] %Constraint
   [.] .signature:#pg.pipeline.signature
      [.] .inputs
         [.] .input << $input_type
      [.] .outputs
         [.] .output << $output_type
{x}

// Usage with concrete types
[r] $string_to_int:pipeline.transformer << |String.ParseInt
   <input_type << :pg.string
   <output_type << :pg.int
```

---

## Standard Library Examples

### `|Pipeline.Compose`

```polyglot
{|} |Pipeline.Compose
[%] %Doc << "Compose multiple pipelines into single pipeline"

[<] i<pipelines:array.pg.pipeline
   [%] %InStream << #IO.Stream.TriplePull

[t] |T.Call
[W] |W.Polyglot.Scope

// Return composed pipeline that chains all inputs
[>] o>composed << // Implementation returns new pipeline
{x}

// Usage
[r] |Pipeline.Compose
   <<< |String.Trim
   <<< |String.ToLowerCase
   <<< |String.RemoveSpaces
   >>> $text_normalizer
```

### `|Pipeline.Map`

```polyglot
{|} |Pipeline.Map
[%] %Doc << "Apply pipeline to each item in collection"

[<] i<collection:array.serial
[<] i<mapper:pg.pipeline

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $results:array.serial << {}

[p] ~ForEach
   <array << $collection
   >item >> $item

   [r] $mapper
      <input << $item
      >output >> $mapped

   [r] $results << |Array.Append
      <array << $results
      <item << $mapped

[>] o>results << $results:array.serial
{x}

// Usage
[r] |Pipeline.Map
   <collection << $numbers
   <mapper << |Double
   >results >> $doubled_numbers
```

### `|Pipeline.Filter`

```polyglot
{|} |Pipeline.Filter
[%] %Doc << "Filter collection using predicate pipeline"

[<] i<collection:array.serial
[<] i<predicate:pg.pipeline

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $filtered:array.serial << {}

[p] ~ForEach
   <array << $collection
   >item >> $item

   [r] $predicate
      <input << $item
      >matches >> $matches

   [f] $matches =? #Boolean.True
      [r] $filtered << |Array.Append
         <array << $filtered
         <item << $item

[>] o>filtered << $filtered:array.serial
{x}
```

---

## Benefits

1. **Flexibility**
   - Choose behavior at runtime
   - Build dynamic processing chains
   - Implement strategy pattern naturally

2. **Reusability**
   - Pass common operations as parameters
   - Create pipeline libraries
   - Compose complex from simple

3. **Type Safety**
   - Pipeline signatures enforce contracts
   - Compiler validates compatibility
   - Runtime type checking available

4. **Metaprogramming**
   - Generate pipelines from configuration
   - Inspect pipeline metadata
   - Dynamic composition

5. **Functional Programming**
   - Higher-order functions (pipelines)
   - Partial application
   - Composition operators

---

## Implementation Notes

### Parser Considerations

1. **Pipeline Reference vs Call**
   ```polyglot
   [r] $ref << |Pipeline              // Reference (no inputs)
   [r] |Pipeline <input << $x         // Call (has inputs)
   ```

2. **Type Annotation**
   ```polyglot
   [r] $handler:pg.pipeline << |ProcessData
   ```

3. **Metadata Access**
   ```polyglot
   [r] $doc << |Pipeline%Doc
   [r] $sig << |Pipeline%Signature
   ```

### Runtime Considerations

- Pipeline references are pointers/handles
- Partial application creates closure
- Metadata accessible without invocation
- Type checking on invocation

---

**Status:** ✅ Complete proposal - Ready for feedback

**Key Achievement:** First-class pipeline support enables functional programming patterns while maintaining Polyglot's explicit, greppable syntax.
