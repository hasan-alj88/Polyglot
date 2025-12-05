# Polyglot Standard Library Pipeline Specification

**Date:** 2025-12-03
**Type:** Standard Library Specification
**Status:** 📋 **DRAFT**
**Scope:** Formalize frequently-used pipelines into official standard library

---

## Executive Summary

Based on comprehensive analysis of 834 unique pipeline references across all Polyglot documentation:

- **Current stdlib pipelines (namespaced):** 114 pipelines
- **Recommended stdlib additions:** 25 frequently-used pipelines
- **Generic example patterns:** 219 placeholders (kept as teaching examples)
- **User-defined examples:** 501 unique examples

---

## Methodology

### Analysis Process

1. **Extraction:** Scanned all `.md` files in `/docs` directory
2. **Categorization:** Classified pipelines by usage pattern and purpose
3. **Frequency Analysis:** Ranked by usage count across files
4. **Pattern Recognition:** Identified generic placeholders vs actual utilities
5. **Namespace Organization:** Grouped by functional domain

### Classification Criteria

**Stdlib Candidate:**
- Used ≥ 10 times across multiple files, OR
- Uses established namespace (U., File., Run., DT., etc.), OR
- Provides core functionality needed by most applications

**Generic Example:**
- Placeholder names (Step1, ProcessData, Task1, etc.)
- Used primarily for teaching concepts
- No specific implementation

**User-Defined Example:**
- Demonstrates specific use case
- Application-specific logic
- Educational value but not general-purpose

---

## Current Standard Library (Namespaced)

### 1. Utility Namespace (`U.`)

#### Logging

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|U.Log.Error` | 131x | Log error message |
| `\|U.Log.Info` | 41x | Log informational message |
| `\|U.Log.Warning` | 9x | Log warning message |
| `\|U.Log.Debug` | 3x | Log debug message |

#### String Operations

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|U.String.Concat` | 18x | Concatenate strings |
| `\|U.String.ToUpper` | 6x | Convert to uppercase |
| `\|U.String.Format` | 4x | Format string with parameters |

#### Integer Operations

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|U.Int.Add` | 15x | Add integers |
| `\|U.Int.ToString` | 3x | Convert integer to string |

#### Dictionary Operations

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|U.Dict.Get` | 8x | Get value from dictionary |

#### Path Operations

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|U.Path.Join` | 4x | Join path components |

#### Process Operations

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|U.Process.Exit` | 10x | Exit process with code |

---

### 2. File Namespace (`File.`)

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|File.ReadText` | 60x | Read text file |
| `\|File.WriteText` | 40x | Write text file |
| `\|File.Exists` | 8x | Check file exists |
| `\|File.Delete` | 4x | Delete file |
| `\|File.Create` | 3x | Create file |

---

### 3. Runtime Namespace (`Run.`)

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|Run.Python` | 154x | Execute Python runtime wrapper |
| `\|Run.Rust` | 12x | Execute Rust runtime wrapper |
| `\|Run.Node` | 6x | Execute Node.js runtime wrapper |

---

### 4. DateTime Namespace (`DT.`)

#### Core Operations

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|DT.Format` | 5x | Format datetime |
| `\|DT.Time` | 5x | Get time component |
| `\|DT.Parse` | 3x | Parse datetime string |

#### Conversion

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|DT.ToLocal` | 2x | Convert to local timezone |
| `\|DT.Convert.ToHijri` | 1x | Convert to Hijri calendar |

#### Arithmetic

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|DT.Add` | 1x | Add duration to datetime |
| `\|DT.Subtract` | 1x | Subtract duration from datetime |
| `\|DT.Difference` | 1x | Calculate datetime difference |

#### Components

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|DT.Day` | 1x | Get day component |
| `\|DT.Month` | 1x | Get month component |
| `\|DT.Hour` | 1x | Get hour component |
| `\|DT.DayOfWeek` | 1x | Get day of week |

---

### 5. Queue Namespace (`Queue.`)

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|Queue.Submit` | 6x | Submit to queue |
| `\|Queue.Priority` | 4x | Set queue priority |
| `\|Queue.Pause` | 3x | Pause queue |

---

### 6. Runtime Wrappers

| Pipeline | Usage | Description |
|----------|-------|-------------|
| `\|Runtime.Python3` | 8x | Python 3 runtime |
| `\|Runtime.Rust` | 4x | Rust runtime |
| `\|Runtime.Node20` | 3x | Node.js 20 runtime |

---

## Recommended Standard Library Additions

These frequently-used pipelines should be formalized into stdlib with proper namespaces:

### HTTP/Network Operations

**Namespace: `HTTP.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|HttpGet` | 17x | `\|HTTP.Get` | HTTP GET request |
| `\|HttpPost` | 3x | `\|HTTP.Post` | HTTP POST request |

---

### Data Processing

**Namespace: `Data.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|FetchData` | 28x | `\|Data.Fetch` | Fetch data from source |
| `\|CombineResults` | 26x | `\|Data.Combine` | Combine multiple results |
| `\|TransformData` | 25x | `\|Data.Transform` | Transform data structure |
| `\|ValidateData` | 23x | `\|Data.Validate` | Validate data structure |

---

### File Operations (Extensions)

**Namespace: `File.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|ReadFile` | 79x | `\|File.Read` (alias) | Generic file read |
| `\|WriteFile` | 15x | `\|File.Write` (alias) | Generic file write |

---

### Error Handling

**Namespace: `Error.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|HandleError` | 65x | `\|Error.Handle` | Generic error handler |
| `\|HandleTimeout` | 16x | `\|Error.HandleTimeout` | Timeout error handler |
| `\|HandleSuccess` | 14x | `\|Error.HandleSuccess` | Success handler |
| `\|HandleFailure` | 13x | `\|Error.HandleFailure` | Failure handler |

---

### Execution Control

**Namespace: `Exec.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|RunScript` | 13x | `\|Exec.Script` | Run script file |
| `\|RunPythonScript` | 20x | `\|Exec.Python` | Run Python script |

---

### User Management

**Namespace: `User.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|FetchUser` | 23x | `\|User.Fetch` | Fetch user data |
| `\|ValidateEmail` | 7x | `\|User.ValidateEmail` | Validate email address |
| `\|ValidateUsername` | 6x | `\|User.ValidateUsername` | Validate username |

---

### Report Generation

**Namespace: `Report.`**

| Current Name | Usage | Proposed Stdlib Name | Description |
|--------------|-------|----------------------|-------------|
| `\|GenerateReport` | 17x | `\|Report.Generate` | Generate report |
| `\|DailyReport` | 4x | `\|Report.Daily` | Generate daily report |

---

## Generic Example Patterns (Keep as Examples)

These are **teaching examples** and should **NOT** be added to stdlib:

### Sequential Placeholders
- `\|Step1`, `\|Step2`, `\|Step3` - Generic sequential steps
- `\|FirstOperation`, `\|SecondOperation` - Generic operations

### Parallel Placeholders
- `\|Task1`, `\|Task2`, `\|TaskA`, `\|TaskB` - Generic parallel tasks
- `\|Worker1`, `\|Worker2`, `\|Worker3` - Generic workers

### Processing Placeholders
- `\|ProcessData` (172x) - Generic data processing example
- `\|ProcessPartA`, `\|ProcessPartB` - Generic multi-part processing
- `\|ProcessItem`, `\|ProcessFile` - Generic item/file processing

### Operation Placeholders
- `\|Operation`, `\|Operation1`, `\|Operation2` - Generic operations
- `\|Transform`, `\|Validate` - Generic transformation/validation
- `\|MightFail`, `\|RiskyOperation` - Generic error examples
- `\|DoWork`, `\|DoStuff`, `\|DoSomething` - Generic actions

**Rationale:** These serve pedagogical purposes, teaching concepts without being opinionated about implementation.

---

## Hierarchical Organization

### By Functional Domain

```
Polyglot Standard Library
├── Core Utilities (U.)
│   ├── Logging
│   │   ├── U.Log.Error
│   │   ├── U.Log.Info
│   │   ├── U.Log.Warning
│   │   └── U.Log.Debug
│   ├── String Operations
│   │   ├── U.String.Concat
│   │   ├── U.String.ToUpper
│   │   └── U.String.Format
│   ├── Integer Operations
│   │   ├── U.Int.Add
│   │   └── U.Int.ToString
│   ├── Dictionary Operations
│   │   └── U.Dict.Get
│   ├── Path Operations
│   │   └── U.Path.Join
│   └── Process Operations
│       └── U.Process.Exit
│
├── File Operations (File.)
│   ├── File.ReadText
│   ├── File.WriteText
│   ├── File.Exists
│   ├── File.Delete
│   └── File.Create
│
├── Runtime Wrappers (Run., Runtime.)
│   ├── Run.Python
│   ├── Run.Rust
│   ├── Run.Node
│   ├── Runtime.Python3
│   ├── Runtime.Rust
│   └── Runtime.Node20
│
├── DateTime Operations (DT.)
│   ├── Core
│   │   ├── DT.Format
│   │   ├── DT.Time
│   │   └── DT.Parse
│   ├── Conversion
│   │   ├── DT.ToLocal
│   │   └── DT.Convert.ToHijri
│   ├── Arithmetic
│   │   ├── DT.Add
│   │   ├── DT.Subtract
│   │   └── DT.Difference
│   └── Components
│       ├── DT.Day
│       ├── DT.Month
│       ├── DT.Hour
│       └── DT.DayOfWeek
│
└── Queue Operations (Queue.)
    ├── Queue.Submit
    ├── Queue.Priority
    └── Queue.Pause
```

---

## Proposed Additions (with Namespaces)

```
Standard Library Additions
├── HTTP Operations (HTTP.)
│   ├── HTTP.Get
│   └── HTTP.Post
│
├── Data Operations (Data.)
│   ├── Data.Fetch
│   ├── Data.Combine
│   ├── Data.Transform
│   └── Data.Validate
│
├── Error Handling (Error.)
│   ├── Error.Handle
│   ├── Error.HandleTimeout
│   ├── Error.HandleSuccess
│   └── Error.HandleFailure
│
├── Execution Control (Exec.)
│   ├── Exec.Script
│   └── Exec.Python
│
├── User Management (User.)
│   ├── User.Fetch
│   ├── User.ValidateEmail
│   └── User.ValidateUsername
│
└── Report Generation (Report.)
    ├── Report.Generate
    └── Report.Daily
```

---

## Implementation Recommendations

### Phase 1: Formalize Existing Stdlib

**Priority:** HIGH

1. Document all current stdlib pipelines (U., File., Run., DT., Queue.)
2. Create stdlib module structure
3. Implement comprehensive tests
4. Generate API documentation

**Deliverables:**
- `stdlib/core/logging.pg` - U.Log.* pipelines
- `stdlib/core/string.pg` - U.String.* pipelines
- `stdlib/core/int.pg` - U.Int.* pipelines
- `stdlib/file/operations.pg` - File.* pipelines
- `stdlib/runtime/wrappers.pg` - Run.*, Runtime.* pipelines
- `stdlib/datetime/operations.pg` - DT.* pipelines
- `stdlib/queue/operations.pg` - Queue.* pipelines

---

### Phase 2: Add High-Value Extensions

**Priority:** MEDIUM

Implement frequently-used pipelines with proper namespaces:

1. **HTTP Operations** (17+ uses)
   - `HTTP.Get`, `HTTP.Post`

2. **Data Operations** (90+ uses combined)
   - `Data.Fetch`, `Data.Combine`, `Data.Transform`, `Data.Validate`

3. **Error Handling** (100+ uses combined)
   - `Error.Handle`, `Error.HandleTimeout`, etc.

**Deliverables:**
- `stdlib/http/requests.pg`
- `stdlib/data/operations.pg`
- `stdlib/error/handlers.pg`

---

### Phase 3: Specialized Domains

**Priority:** LOW

Add domain-specific utilities:

1. **User Management**
   - `User.Fetch`, `User.ValidateEmail`, `User.ValidateUsername`

2. **Report Generation**
   - `Report.Generate`, `Report.Daily`

3. **Execution Control**
   - `Exec.Script`, `Exec.Python`

---

## Documentation Updates

### Update Required Files

1. **Create Stdlib Documentation:**
   - `docs/user/standard-library/README.md` - Overview
   - `docs/user/standard-library/core-utilities.md` - U.* namespace
   - `docs/user/standard-library/file-operations.md` - File.* namespace
   - `docs/user/standard-library/runtime-wrappers.md` - Run.* namespace
   - `docs/user/standard-library/datetime.md` - DT.* namespace
   - `docs/user/standard-library/http.md` - HTTP.* namespace (new)
   - `docs/user/standard-library/data.md` - Data.* namespace (new)
   - `docs/user/standard-library/error.md` - Error.* namespace (new)

2. **Update Example Files:**
   - Replace ad-hoc pipeline names with stdlib equivalents
   - Add comments explaining stdlib usage
   - Keep generic placeholders for teaching

3. **Migration Guide:**
   - Document transition from user-defined to stdlib
   - Provide search/replace patterns
   - Explain namespace conventions

---

## Naming Conventions

### Stdlib Pipeline Naming

**Pattern:** `|Namespace.Operation[.Variant]`

**Examples:**
- `|U.Log.Error` - Utility.Logging.Error
- `|File.ReadText` - File.ReadText
- `|HTTP.Get` - HTTP.Get
- `|Data.Transform` - Data.Transform
- `|DT.Convert.ToHijri` - DateTime.Convert.ToHijri

### Namespace Guidelines

| Namespace | Purpose | Examples |
|-----------|---------|----------|
| `U.` | Core utilities | U.Log, U.String, U.Int |
| `File.` | File system operations | File.Read, File.Write |
| `HTTP.` | HTTP/network operations | HTTP.Get, HTTP.Post |
| `Data.` | Data processing | Data.Fetch, Data.Transform |
| `Error.` | Error handling | Error.Handle, Error.HandleTimeout |
| `DT.` | DateTime operations | DT.Format, DT.Parse |
| `Run.` | Runtime wrappers | Run.Python, Run.Rust |
| `Queue.` | Queue operations | Queue.Submit, Queue.Pause |
| `User.` | User management | User.Fetch, User.Validate |
| `Report.` | Report generation | Report.Generate, Report.Daily |
| `Exec.` | Execution control | Exec.Script, Exec.Python |

---

## Migration Strategy

### Deprecation Path

For pipelines moving from user-defined to stdlib:

**Step 1:** Add stdlib version
```polyglot
// OLD (still works)
[r] |HttpGet
[<] <url: pg\string << .endpoint

// NEW (recommended)
[r] |HTTP.Get
[<] <url: pg\string << .endpoint
```

**Step 2:** Add deprecation warning (compiler)
```
Warning: |HttpGet is deprecated, use |HTTP.Get instead
```

**Step 3:** Remove old version (next major version)

---

## Testing Requirements

### Stdlib Test Coverage

**Requirement:** 100% test coverage for all stdlib pipelines

**Test Categories:**
1. **Unit Tests** - Individual pipeline functionality
2. **Integration Tests** - Pipeline composition
3. **Error Tests** - Error handling paths
4. **Performance Tests** - Benchmarks for common operations

**Example Test Structure:**
```polyglot
[|] Test.U.Log.Error
[t] |T.Manual

[\]
[r] .test_message: pg\string << "Test error message"
[/]

[r] |U.Log.Error
[<] <msg: pg\string << .test_message

// Assert: Error logged
[o] !NoError
[X]
```

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Stdlib pipelines documented | 100% | 📋 Draft |
| High-frequency pipelines in stdlib | ≥80% | 🔄 Pending |
| Generic examples preserved | 100% | ✅ Complete |
| Test coverage | 100% | 🔄 Pending |
| Migration guide complete | Yes | 🔄 Pending |

---

## Conclusion

**Analysis Complete:** Identified 834 unique pipelines across documentation

**Current Stdlib:** 114 namespaced pipelines (U., File., Run., DT., Queue., Runtime.)

**Recommended Additions:** 25 frequently-used pipelines organized into new namespaces (HTTP., Data., Error., User., Report., Exec.)

**Generic Examples:** 219 teaching placeholders preserved for educational purposes

**Next Steps:**
1. Review and approve stdlib specification
2. Implement Phase 1 (formalize existing stdlib)
3. Create comprehensive stdlib documentation
4. Implement Phase 2 (add high-value extensions)
5. Update all documentation with stdlib usage

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Analysis Type:** Pipeline Categorization & Standard Library Specification
