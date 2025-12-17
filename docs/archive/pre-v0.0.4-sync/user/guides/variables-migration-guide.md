---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/guides/variables-migration-guide.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Variables Migration Guide

**Migrating to the State-Aware Variable Model**

**Last Updated:** 2025-11-24

---

## Overview

This guide helps you migrate from older Polyglot variable concepts to the new state-aware model.

### What Changed?

**Old Mental Model:** "Variables are immutable"
**New Mental Model:** "Variables transition through states in an async-centric language"

**Why It Matters:** The new model better reflects how Polyglot actually works and provides better error handling and debugging.

---

## Key Conceptual Shifts

### 1. From "Immutable" to "State Transitions"

**Before (Old Thinking):**
> "Variables can't change because Polyglot is immutable"

**After (New Understanding):**
> "Variables transition Declared → Pending → Ready. Once Ready, they're immutable as a consequence of async coordination, not a design choice."

**Impact:** You can now **query variable states** for advanced control flow:
```polyglot
[?] .var.state =? #Variables.States.Ready
[~][r] |ProcessData
```

---

### 2. From "Undefined Defaults" to "DefaultReady State"

**Before (Workaround):**
```polyglot
# Had to check if field was populated
[#] Config
[<] .timeout:pg.int
[X]

[r] |CheckIfSet
[<] .value:pg.int << .config.timeout  # Might fail if not set
```

**After (Native Support):**
```polyglot
# Use default assignment operator
[#] Config
[<] .timeout:pg.int <~ 30  # Default is 30, can override once
[X]

[i] .config: #Config << #Config
# .timeout is Ready with value 30 (or overridden value)
```

**Impact:** Cleaner config objects with built-in defaults.

---

### 3. From "Manual Error Checks" to ".errors Field"

**Before (Inconsistent):**
```polyglot
[r] |RiskyOperation
[>] .result >> .data
# No standard way to check errors
```

**After (Standardized):**
```polyglot
[r] |RiskyOperation
[>] .result:pg.string >> .data
[>] .errors:pg.array{!} >> .operation_errors

[?] .data.state =? #Variables.States.Faulted
[~][r] |HandleError
[~][<] .errors:pg.array{!} << .operation_errors
```

**Impact:** Consistent error handling across all pipelines.

---

## Migration Checklist

### Step 1: Update Enumeration Definitions

**Identify patterns:**

❌ **Old (Implicit Behavior):**
```polyglot
[#] Config
[<] .timeout:pg.int          # Unclear if this has a default
[<] .retries:pg.int
[X]
```

✅ **New (Explicit):**
```polyglot
[#] Config
[<] .timeout:pg.int <~ 30    # Explicit default
[<] .retries:pg.int <~ 3     # Explicit default
[X]
```

**Action Items:**
- Review all enumerations
- Add `<~` operator for fields that should have defaults
- Use schema-only (no operator) for fields populated by pipelines

---

### Step 2: Add Error Handling

**Identify patterns:**

❌ **Old (No Error Output):**
```polyglot
[r] |FetchData
[>] .result:pg.string >> .data
```

✅ **New (Explicit Error Handling):**
```polyglot
[r] |FetchData
[>] .result:pg.string >> .data
[>] .errors:pg.array{!} >> .fetch_errors

[?] .data.state =? #Variables.States.Faulted
[~][r] |U.Log.Error
[~][<] .errors:pg.array{!} << .fetch_errors
```

**Action Items:**
- Add `.errors` output to all pipelines that can fail
- Add state checking for Faulted variables
- Use error blocks `[!]` for specific error types

---

### Step 3: Clarify `[i]` Block Expectations

**Understand:**

✅ **`[i]` blocks expect Ready variables:**
```polyglot
[|] MyPipeline
[i] .required_param:pg.string       # Must be Ready
[i] .config: #Config << #Config       # Ready (defaults applied)
[t] |T.Call
```

**Action Items:**
- Ensure `[i]` variables are Ready before pipeline triggers
- Use defaults (`<~`) for optional parameters
- Document which parameters are required vs optional

---

### Step 4: Replace Manual Waiting with Automatic

**Identify patterns:**

❌ **Old (If you were manually checking):**
```polyglot
# Some custom waiting logic (anti-pattern)
[r] |FetchData
[>] .result >> .data

[r] |WaitForData  # Not needed!
[r] |ProcessData
[<] .input << .data
```

✅ **New (Automatic Waiting):**
```polyglot
[r] |FetchData
[>] .result >> .data

# Polyglot automatically waits for .data to be Ready
[r] |ProcessData
[<] .input << .data  # Waits automatically
```

**Action Items:**
- Remove manual waiting code
- Trust automatic waiting behavior
- Use state checks only for error handling

---

## Common Migration Scenarios

### Scenario 1: Config Objects

**Before:**
```polyglot
[#] ServerConfig
[<] .host:pg.string
[<] .port:pg.int
[<] .timeout:pg.int
[X]

[|] StartServer
[i] .host:pg.string
[i] .port:pg.int
[i] .timeout:pg.int
[t] |T.Call
# Had to pass all params manually
```

**After:**
```polyglot
[#] ServerConfig
[<] .host:pg.string <~ "localhost"
[<] .port:pg.int <~ 8080
[<] .timeout:pg.int <~ 30
[X]

[|] StartServer
[i] .config: #ServerConfig << #ServerConfig  # Defaults applied
[t] |T.Call
# Cleaner - just pass config object
```

---

### Scenario 2: API Calls with Retries

**Before:**
```polyglot
[r] |CallAPI
[>] .response >> .api_response
# Manual retry logic needed
```

**After:**
```polyglot
[r] |CallAPI
[>] .response:pg.string >> .api_response
[>] .errors:pg.array{!} >> .api_errors
[~]
[~][!] !pg.Network.Timeout
[~][~]# Automatic retry can be configured
[~][~]# Or manual fallback:
[~][~][r] |GetCachedData
[~][~][>] .cached >> .fallback_data
[~][~][o] .fallback_data
```

---

### Scenario 3: Parallel Operations

**Before:**
```polyglot
[p] |Task1
[>] .result >> .data1

[p] |Task2
[>] .result >> .data2

# Manual Join without error handling
[Y] |Y.Join
[<] .data1
[<] .data2
```

**After:**
```polyglot
[p] |Task1
[>] .result >> .data1
[>] .errors:pg.array{!} >> .errors1

[p] |Task2
[>] .result >> .data2
[>] .errors:pg.array{!} >> .errors2

[Y] |Y.Join
[<] .data1
[<] .data2

# Check if all succeeded
[?] .data1.state =? #Variables.States.Ready
[~][?] .data2.state =? #Variables.States.Ready
[~][~]# All ready
[~][~][r] |ProcessBoth

[?] *?
[~]# At least one failed
[~][r] |HandlePartialFailure
```

---

## Breaking Changes

### None (Backwards Compatible)

**Good news:** The new model is **backwards compatible** with existing code.

**What works as-is:**
- Existing variable declarations
- Existing assignments
- Existing pipeline flows

**What's new (opt-in):**
- Default operator `<~` / `~>`
- `.errors` field (always available, but optional to use)
- `.state` field (always available, optional to query)

---

## Recommended Migration Path

### Phase 1: Understanding (Week 1)
- Read [Variables User Guide](../language/variables-user-guide.md)
- Review brainstorming session results
- Understand three operators and state lifecycle

### Phase 2: New Code (Week 2-4)
- Use new patterns in new code
- Add default operators to new enumerations
- Add `.errors` outputs to new pipelines

### Phase 3: Gradual Update (Month 2-3)
- Update high-traffic pipelines with error handling
- Refactor config objects to use defaults
- Add state checking to critical paths

### Phase 4: Complete (Month 4+)
- Update remaining code as time permits
- No rush - old code still works

---

## FAQs

### Q: Do I HAVE to update my existing code?

**A:** No. Existing code continues to work. Update when:
- Adding new features
- Improving error handling
- Simplifying config management

---

### Q: Will old code break?

**A:** No. The new model is fully backwards compatible.

---

### Q: Should I use state checking everywhere?

**A:** No. Use it only for:
- Error handling (Faulted state)
- Advanced control flow
- Debugging

Most code doesn't need explicit state checks.

---

### Q: What about performance?

**A:** The new model has **no performance impact**:
- State tracking is lightweight
- Automatic waiting is efficient (non-busy)
- Error handling is zero-cost when no errors

---

### Q: Can I mix old and new patterns?

**A:** Yes. You can:
- Use defaults in some enumerations, not others
- Add `.errors` to some pipelines, not all
- Gradually adopt state checking

---

## Resources

- **User Guide:** [Variables User Guide](../language/variables-user-guide.md)
- **Technical Spec:** [Variable States Specification](../../technical/variable-states-specification.md)
- **Cheatsheet:** [Quick Reference](../quick-reference/variables-cheatsheet.md)
- **Examples:** [Practical Examples](../examples/variables-examples.md)

---

## Support

**Questions?**
- Check [Variables FAQ](../language/variables-user-guide.md#faq)
- Community Forums: [forum.polyglot.dev](https://forum.polyglot.dev)
- Discord: [discord.gg/polyglot](https://discord.gg/polyglot)

**Found a bug?**
- GitHub Issues: [github.com/polyglot/polyglot/issues](https://github.com/polyglot/polyglot/issues)

---

**Happy migrating! The new state-aware model makes Polyglot even more powerful for async automation.**
