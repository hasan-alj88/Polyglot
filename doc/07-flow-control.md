# Flow Control (Switch Statements)

[← Back to README](../README.md)

## Table of Contents
- [Overview](#overview)
- [Mode 1: Equality Comparison](#mode-1-equality-comparison)
- [Mode 2: Boolean Pipeline Evaluation](#mode-2-boolean-pipeline-evaluation)
- [Switch Semantics](#switch-semantics)
- [Standard Library Boolean Utilities](#standard-library-boolean-utilities)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)

## Overview

The `[?]` switch element provides conditional branching within pipelines. Unlike traditional if-else statements, Polyglot switches are **non-exclusive** by default—multiple switch branches can execute for the same condition check.

**Two Modes:**
1. **Equality Comparison (`?>`)** - Compare variable against hashable values
2. **Boolean Pipeline Evaluation** - Execute any pipeline returning `pg\bool`

## Mode 1: Equality Comparison

The `?>` operator compares a variable against hashable values and executes the branch if they match.

### Basic String Matching

```polyglot
[|] ProcessByStatus
[i] status: pg\string
[t] |T.Call

[r] |U.String.Normalize << status >> normalized_status

[?] normalized_status ?> "success"
[~][r] |U.Log << #LogLevel.Info << "Operation successful"
[~][r] |SendSuccessNotification

[?] normalized_status ?> "failure"
[~][r] |U.Log << #LogLevel.Error << "Operation failed"
[~][r] |SendFailureAlert

[?] normalized_status ?> "pending"
[~][r] |U.Log << #LogLevel.Info << "Operation pending"
[~][r] |RequeueForRetry

[o] >> normalized_status
[x]
```

### Enum Matching

```polyglot
[#] DataQuality
[D] Excellent
[D] Good
[D] Acceptable
[D] Poor
[D] Invalid
[x]

[|] ProcessByQuality
[i] quality: pg\Enum
[t] |T.Call

[?] quality ?> #DataQuality.Excellent
[~][r] |U.Log << "Premium processing"
[~][r] |PremiumAnalytics >> results

[?] quality ?> #DataQuality.Good
[~][r] |U.Log << "Standard processing"
[~][r] |StandardAnalytics >> results

[?] quality ?> #DataQuality.Acceptable
[~][r] |U.Log << "Basic processing"
[~][r] |BasicAnalytics >> results

[?] quality ?> #DataQuality.Poor
[~][r] |U.Log.Warning << "Low quality data"
[~][r] |MinimalAnalytics >> results

[?] quality ?> #DataQuality.Invalid
[~][r] |U.Log.Error << "Invalid data, rejecting"
[~][x] |Exit << 400

[o] >> results
[x]
```

### Numeric Matching

```polyglot
[|] ProcessHttpStatus
[i] status_code: pg\int
[t] |T.Call

[?] status_code ?> 200
[~][r] |U.Log << "OK"
[~][r] |HandleSuccess

[?] status_code ?> 201
[~][r] |U.Log << "Created"
[~][r] |HandleCreated

[?] status_code ?> 404
[~][r] |U.Log << "Not Found"
[~][r] |HandleNotFound

[?] status_code ?> 500
[~][r] |U.Log.Error << "Server Error"
[~][r] |AlertOps

[o] >> status_code
[x]
```

### Multiple Value Handling

```polyglot
[|] ProcessPriority
[i] priority: pg\int
[t] |T.Call

\\ Handle high priority (both 0 and 1 are high)
[?] priority ?> 0
[~][r] |U.Log << "Critical priority"
[~][r] |HandleCritical

[?] priority ?> 1
[~][r] |U.Log << "High priority"
[~][r] |HandleHigh

\\ Handle medium priority
[?] priority ?> 2
[~][r] |U.Log << "Medium priority"
[~][r] |HandleMedium

\\ Handle low priority  
[?] priority ?> 3
[~][r] |U.Log << "Low priority"
[~][r] |HandleLow

[o] >> priority
[x]
```

## Mode 2: Boolean Pipeline Evaluation

The `[?]` element can evaluate any pipeline that returns `pg\bool`, enabling complex conditional logic.

### Basic Boolean Check

```polyglot
[|] ConditionalProcessing
[i] data: py\dict
[t] |T.Call

[r] |ValidateData << data >> is_valid: pg\bool

[?] is_valid ?> True
[~][r] |U.Log << "Data is valid, proceeding"
[~][r] |ProcessValidData << data >> result

[?] is_valid ?> False
[~][r] |U.Log.Error << "Data validation failed"
[~][x] |Exit << 422

[o] >> result
[x]
```

### Using Boolean Pipeline Directly

```polyglot
[|] AdaptiveProcessing
[i] dataset: py\dict
[t] |T.Call

\\ Pipeline returns boolean based on condition
[?] |U.System.Resource.Cpu.Below << 50.0 >> cpu_available: pg\bool
[~][r] |U.Log << "CPU available, using intensive algorithm"
[~][r] |IntensiveAlgorithm << dataset >> result

\\ Check opposite condition
[?] |U.System.Resource.Cpu.Above << 50.0 >> cpu_busy: pg\bool
[~][r] |U.Log << "CPU busy, using lightweight algorithm"
[~][r] |LightweightAlgorithm << dataset >> result

[o] >> result
[x]
```

### Complex Boolean Logic

```polyglot
[|] SmartCaching
[i] query: pg\string
[t] |T.Call

\\ Use custom pipeline for complex decision
[?] |ShouldUseCache << query >> use_cache: pg\bool
[~][r] |U.Log << "Cache hit"
[~][r] |FetchFromCache << query >> data

\\ Opposite logic using negation
[?] |U.Boolean.Not << use_cache >> should_fetch: pg\bool
[~][r] |U.Log << "Cache miss, fetching from source"
[~][r] |FetchFromSource << query >> data
[~][r] |UpdateCache << query << data

[o] >> data
[x]

\\ Helper pipeline with complex logic
[|] ShouldUseCache
[i] query: pg\string
[t] |T.Call

[r] |CheckCacheExists << query >> exists: pg\bool
[r] |CheckCacheFresh << query >> is_fresh: pg\bool
[r] |U.Boolean.And << exists << is_fresh >> result: pg\bool

[o] >> result
[x]
```

### File System Conditional

```polyglot
[|] ConditionalFileProcessing
[i] file_path: pg\string
[t] |T.Call

[?] |T.Check.File.Exists << file_path >> exists: pg\bool
[~][r] |U.Log << "File exists, processing"
[~][r] |ProcessFile << file_path >> result

[?] |U.Boolean.Not << exists >> missing: pg\bool
[~][r] |U.Log.Error << "File not found"
[~][x] |Exit << 404

[o] >> result
[x]
```

### Resource-Based Decisions

```polyglot
[|] ResourceAwareProcessing
[i] workload: py\dict
[t] |T.Call

\\ Check multiple resource conditions
[?] |CheckResourcesAvailable >> has_resources: pg\bool
[~][r] |U.Log << "Resources available"
[~][r] |FullProcessing << workload >> result

[?] |U.Boolean.Not << has_resources >> limited: pg\bool
[~][r] |U.Log.Warning << "Limited resources"
[~][r] |ReducedProcessing << workload >> result

[o] >> result
[x]

[|] CheckResourcesAvailable
[t] |T.Call

[r] |U.System.Resource.Cpu.Below << 70.0 >> cpu_ok: pg\bool
[r] |U.System.Resource.Memory.Below << 80.0 >> mem_ok: pg\bool
[r] |U.Boolean.And << cpu_ok << mem_ok >> result: pg\bool

[o] >> result
[x]
```

## Switch Semantics

### Non-Exclusive Execution

Multiple switch branches can execute:

```polyglot
[|] MultipleActions
[i] value: pg\int
[t] |T.Call

\\ Both branches execute if value is 1
[?] value ?> 1
[~][r] |U.Log << "Action 1"
[~][r] |DoAction1

[?] value ?> 1
[~][r] |U.Log << "Action 2"  
[~][r] |DoAction2

[o] >> value
[x]
```

### Early Exit Pattern (Mutually Exclusive)

Use `[x] |Exit` to create mutually exclusive behavior:

```polyglot
[|] ExclusiveBranches
[i] status: pg\string
[t] |T.Call

[?] status ?> "error"
[~][r] |HandleError
[~][x] |Exit << 1  \\ Exit prevents further execution

[?] status ?> "success"
[~][r] |HandleSuccess
[~][x] |Exit << 0

\\ Only runs if neither condition matched
[r] |DefaultHandler
[x]
```

### Nested Switches

```polyglot
[|] NestedDecisions
[i] category: pg\string
[i] region: pg\string
[t] |T.Call

[?] category ?> "premium"
[~][r] |U.Log << "Premium customer"
[~][?] region ?> "US"
[~][~][r] |UsSpecialProcessing
[~][?] region ?> "EU"
[~][~][r] |EuSpecialProcessing
[~][?] region ?> "APAC"
[~][~][r] |ApacSpecialProcessing

[?] category ?> "standard"
[~][r] |U.Log << "Standard customer"
[~][r] |StandardProcessing

[x]
```

### Switch with Default Case

```polyglot
[|] WithDefault
[i] action: pg\string
[t] |T.Call

[r] |U.Boolean.False >> handled: pg\bool

[?] action ?> "start"
[~][r] |HandleStart
[~][r] |U.Boolean.True >> handled

[?] action ?> "stop"
[~][r] |HandleStop
[~][r] |U.Boolean.True >> handled

[?] action ?> "restart"
[~][r] |HandleRestart
[~][r] |U.Boolean.True >> handled

\\ Default case
[?] handled ?> False
[~][r] |U.Log << "Unknown action: {action}"
[~][r] |HandleDefault

[x]
```

## Standard Library Boolean Utilities

### Logical Operations

```polyglot
\\ Boolean AND
|U.Boolean.And << a: pg\bool << b: pg\bool >> result: pg\bool

\\ Boolean OR
|U.Boolean.Or << a: pg\bool << b: pg\bool >> result: pg\bool

\\ Boolean NOT
|U.Boolean.Not << a: pg\bool >> result: pg\bool

\\ Boolean XOR
|U.Boolean.Xor << a: pg\bool << b: pg\bool >> result: pg\bool
```

### Comparison Operations

```polyglot
\\ Equality
|U.Compare.Equal << a: any << b: any >> result: pg\bool
|U.Compare.NotEqual << a: any << b: any >> result: pg\bool

\\ Numeric comparisons
|U.Compare.GreaterThan << a: number << b: number >> result: pg\bool
|U.Compare.LessThan << a: number << b: number >> result: pg\bool
|U.Compare.GreaterOrEqual << a: number << b: number >> result: pg\bool
|U.Compare.LessOrEqual << a: number << b: number >> result: pg\bool
```

### String Operations

```polyglot
|U.String.Contains << haystack: pg\string << needle: pg\string >> result: pg\bool
|U.String.StartsWith << text: pg\string << prefix: pg\string >> result: pg\bool
|U.String.EndsWith << text: pg\string << suffix: pg\string >> result: pg\bool
|U.String.IsEmpty << text: pg\string >> result: pg\bool
|U.String.Matches << text: pg\string << pattern: pg\string >> result: pg\bool
```

### Collection Operations

```polyglot
|U.Collection.Contains << collection: any << item: any >> result: pg\bool
|U.Collection.IsEmpty << collection: any >> result: pg\bool
|U.Collection.SizeEquals << collection: any << size: pg\int >> result: pg\bool
|U.Collection.SizeGreaterThan << collection: any << size: pg\int >> result: pg\bool
```

## Best Practices

### 1. Use Equality for Simple Cases

```polyglot
\\ ✅ Good: Clear and simple
[?] status ?> "active"
[~][r] |ProcessActive

\\ ❌ Overly complex
[?] |U.Compare.Equal << status << "active" >> is_active: pg\bool
[~][r] |ProcessActive
```

### 2. Use Boolean Pipelines for Complex Logic

```polyglot
\\ ✅ Good: Reusable complex logic
[?] |IsEligibleForDiscount << customer >> eligible: pg\bool
[~][r] |ApplyDiscount

\\ ❌ Bad: Inline complex logic
[?] age ?> 65
[~][?] membership ?> "gold"
[~][~][?] purchase_amount ?> 100
[~][~][~][r] |ApplyDiscount
```

### 3. Document Non-Obvious Conditions

```polyglot
\\ ✅ Good: Commented logic
\\ Check if customer qualifies for expedited shipping
\\ Requirements: premium member AND order > $50 AND item in stock
[?] |QualifiesForExpedited << order >> qualifies: pg\bool
[~][r] |ExpediteShipping
```

### 4. Avoid Deep Nesting

```polyglot
\\ ❌ Bad: Hard to read
[?] a ?> 1
[~][?] b ?> 2
[~][~][?] c ?> 3
[~][~][~][?] d ?> 4
[~][~][~][~][r] |DeepOperation

\\ ✅ Good: Extract to helper pipeline
[?] |ComplexCondition << a << b << c << d >> matches: pg\bool
[~][r] |DeepOperation
```

### 5. Use Early Exits for Error Cases

```polyglot
\\ ✅ Good: Fail fast
[?] |ValidateInput << input >> is_valid: pg\bool
[?] is_valid ?> False
[~][r] |U.Log.Error << "Invalid input"
[~][x] |Exit << 400

\\ Continue with valid input
[r] |ProcessValidInput << input
```

## Common Patterns

### Guard Clauses

```polyglot
[|] ProcessWithGuards
[i] data: py\dict
[t] |T.Call

\\ Guard: Check if data exists
[?] |U.Collection.IsEmpty << data >> is_empty: pg\bool
[~][r] |U.Log.Error << "Empty data"
[~][x] |Exit << 400

\\ Guard: Check authorization
[?] |IsAuthorized << data >> authorized: pg\bool
[?] authorized ?> False
[~][r] |U.Log.Error << "Unauthorized"
[~][x] |Exit << 403

\\ Main processing
[r] |ProcessData << data >> result
[o] >> result
[x]
```

### Strategy Pattern

```polyglot
[#] ProcessingStrategy
[D] Fast
[D] Balanced
[D] Thorough
[x]

[|] StrategyProcessor
[i] data: py\dict
[i] strategy: pg\Enum
[t] |T.Call

[?] strategy ?> #ProcessingStrategy.Fast
[~][r] |FastAlgorithm << data >> result

[?] strategy ?> #ProcessingStrategy.Balanced
[~][r] |BalancedAlgorithm << data >> result

[?] strategy ?> #ProcessingStrategy.Thorough
[~][r] |ThoroughAlgorithm << data >> result

[o] >> result
[x]
```

### Feature Flags

```polyglot
[|] FeatureFlaggedProcessing
[i] data: py\dict
[t] |T.Call

\\ Check feature flag
[?] |GetFeatureFlag << "use_new_algorithm" >> enabled: pg\bool
[~][r] |U.Log << "Using new algorithm"
[~][r] |NewAlgorithm << data >> result

[?] |U.Boolean.Not << enabled >> use_old: pg\bool
[~][r] |U.Log << "Using legacy algorithm"
[~][r] |LegacyAlgorithm << data >> result

[o] >> result
[x]
```

---

[Next: Queue System →](08-queue-system.md)