---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/safety-mechanisms.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Safety Mechanisms: Explicit Intent

Polyglot enforces **explicit intent** for common omissions to prevent accidental errors. These safety mechanisms require developers to explicitly declare when certain blocks are intentionally omitted.

---

## Safety Mechanism 1: `[W] |W.Polyglot.Scope` - Explicit Wrapper Intent

**Rule:** When omitting `[\]` and `[/]` step blocks, you MUST explicitly declare `[W] |W.Polyglot.Scope` to show it's intentional.

**Why:** `|W.Polyglot.Scope` is **always implicit** (always runs automatically, but declaring it explicitly prevents accidental omission of required step blocks.

```polyglot
// ❌ COMPILE ERROR: Missing [W] or {[\], [/] - was this intentional?
[|] |MyPipeline
[t] |T.Manual
[r] .x:pg.int << 10
[X]

// ✅ CORRECT: Explicit intent - no step blocks needed
[|] |MyPipeline
[t] |T.Manual
[W] |W.Polyglot.Scope  // "I intentionally omitted [\] and [/]"
[r] .x:pg.int << 10
[X]

// ✅ CORRECT: Step blocks present
[|] |MyPipeline
[t] |T.Manual
[\]
[r] .x:pg.int << 10
[/]
[X]

// ✅ CORRECT: Another wrapper makes |W.Polyglot.Scope implicit
[|] |MyPipeline
[t] |T.Manual
[W] |W.RT.Python3.12
[\]
[r] .x:pg.int << py\calculate(
[/]
[X]
```

---

## Safety Mechanism 2: `[o] !NoError` - Explicit No-Output Intent

**Rule:** When a pipeline intentionally produces no outputs, you MUST declare `[o] !NoError`.

**Why:** All successful pipelines yield `!NoError` automatically, but declaring it explicitly prevents accidental omission of output declarations.

```polyglot
// ❌ COMPILE ERROR: No outputs declared - was this intentional?
[|] |LogMessage
[i] .message:pg.string
[t] |T.Call
[r] U.Log.Info"{.message"
[X]

// ✅ CORRECT: Explicit intent - no outputs needed
[|] |LogMessage
[i] .message:pg.string
[o] !NoError  // "I intentionally have no outputs"
[t] |T.Call
[r] U.Log.Info"{.message"
[X]

// ✅ CORRECT: Has output
[|] |ProcessMessage
[i] .message:pg.string
[o] .result:pg.string
[t] |T.Call
[r] .result:pg.string << U.String.ToUpper"{.message"
[X]
```

---

## Safety Mechanism 3: `[i]` Input Usage - Input Safety Mechanism

**Rule:** All `[i]` inputs must be declared before `[t]` trigger and used in pipeline body.

**Why:** Prevents unused input declarations and ensures all declared inputs serve a purpose.

```polyglot
// ❌ COMPILE ERROR: Input .count declared but never used
[|] |ProcessData
[i] .data:pg.string
[i] .count:pg.int     // Declared but unused
...
[o] .result:pg.string
[t] |T.Call
[r] .result:pg.string << U.String.ToUpper"{.data"
[X]

// ✅ CORRECT: No unused inputs
[|] |ProcessData
[i] .data:pg.string
[o] .result:pg.string
[t] |T.Call
[r] .result:pg.string << U.String.ToUpper"{.data"
[X]
```

---

## Safety Mechanism 4: `[s][!] *` - Explicit Serial Error Handling

**Rule:** When using `[s]` blocks to load serial files, you MUST explicitly declare error handling with `[s][!] *` for default handling or provide custom error handling.

**Why:** `[s]` is an execution block that loads serial files (YAML, JSON, TOML in parallel with shared error handling. Forgetting error handling could cause silent failures.

**Key Characteristics:**
- `[s]` blocks load serial data files in parallel
- All `[s]` blocks in an enumeration share error handling
- Must explicitly declare how errors should be handled

```polyglot
// ❌ COMPILE ERROR: No error handling declared - was this intentional?
[#] #Config
[<] .api_key:pg.string
[<] .timeout:pg.int <~ 30
[s] "config.yaml"  // Error: Missing error handling
[X]

// ✅ CORRECT: Explicit default error handling
[#] #Config
[<] .api_key:pg.string
[<] .timeout:pg.int <~ 30
[s] "config.yaml"
[s][!] *  // "Use default error handling for all [s] blocks"
[X]

// ✅ CORRECT: Custom error handling
[#] #Config
[<] .api_key:pg.string
[<] .timeout:pg.int <~ 30
[s] "config.yaml"
[s][!]
[r] U.Log.Error"Failed to load config: {!.message"
[r] U.Process.Exit"1"
[X]

// ✅ CORRECT: Multiple serial files with shared error handling
[#] #AppConfig
[<] .database_url:pg.url
[<] .api_key:pg.string
[<] .feature_flags:pg.dict
[s] "database.yaml"    // Loaded in parallel
[s] "secrets.yaml"     // Loaded in parallel
[s] "features.json"    // Loaded in parallel
[s][!] *  // Default error handling for all serial loads
[X]
```

**Parallel Loading:**

All `[s]` blocks within an enumeration load their files **in parallel** for performance:

```polyglot
[#] #Config
[<] .db:pg.url
[<] .cache:pg.url
[<] .queue:pg.url
[s] "database.yaml"    // Loads in parallel
[s] "cache.yaml"       // Loads in parallel
[s] "queue.yaml"       // Loads in parallel
[s][!] *               // Shared error handling
[X]
```

If any file fails to load, the error handler is triggered with details about which file(s failed.

---

## Safety Mechanism 5: `[~][s] <~ .field:type` - Serial Schema Enforcement

**Rule:** When using `[s]` blocks to load serial files into enumeration fields, you MUST explicitly declare all expected fields with their types using `[~][s] <~ .field:type` syntax, or use wildcard `[~][s] <~ *` to allow flexible schema.

**Why:** Prevents runtime surprises from unexpected fields in data files, enables compile-time type validation, makes data contracts explicit, and catches typos in field names at compile time.

**Key Characteristics:**
- Schema declarations specify which fields can be loaded from files
- Undeclared fields in files are silently ignored (unless `*` wildcard is used)
- Type validation ensures file data matches expected types
- Applies ONLY to enumerations, NOT to pipeline `[s]` blocks

```polyglot
// ❌ COMPILE ERROR: No schema declared - which fields are expected?
[#] #Config
[s] |YAML.Load"config.yaml"
[s][!] *
[X]

// ✅ CORRECT: Explicit schema declaration
[#] #Config
[~][s] <~ .timeout:pg\int
[~][s] <~ .api_key:pg\string
[~][s] <~ .debug:pg\bool
[s] |YAML.Load"config.yaml"
[s][!] *
[X]

// ✅ CORRECT: Wildcard allows flexible schema
[#] #FlexibleConfig
[~][s] <~ .timeout:pg\int       // Must declare this field
[~][s] <~ .api_key:pg\string    // Must declare this field
[~][s] <~ *                     // Allow any other fields with type inference
[s] |YAML.Load"config.yaml"
[s][!] *
[X]

// ✅ CORRECT: Multiple files with same schema
[#] #MultiConfig
[~][s] <~ .database_url:pg\url
[~][s] <~ .timeout:pg\int
[s] |YAML.Load"prod.yaml"       // Uses declared schema
[s] |YAML.Load"staging.yaml"    // Uses same schema
[s][!] *
[X]
```

**Schema Enforcement Example:**

Given this YAML file:
```yaml
# config.yaml
timeout: 30
api_key: "secret123"
debug: true
unknown_field: "this will be ignored"
another_field: 42
```

And this enumeration:
```polyglot
[#] #Config
[~][s] <~ .timeout:pg\int
[~][s] <~ .api_key:pg\string
[~][s] <~ .debug:pg\bool
[s] |YAML.Load"config.yaml"
[s][!] *
[X]
```

**Result:**
- `timeout`, `api_key`, and `debug` are loaded and type-checked
- `unknown_field` and `another_field` are silently ignored
- If `timeout` had wrong type (e.g., string instead of int), compile error

**When to Use Wildcard `[~][s] <~ *`:**

Use wildcard when:
- Working with dynamic configuration where new fields may be added
- Prototyping and field structure is not finalized
- Deliberately allowing flexible schema

**Trade-off:** Wildcard disables compile-time validation for unlisted fields.

**Why Schema Enforcement ONLY for Enumerations:**

This safety mechanism applies ONLY to enumeration definitions, NOT pipeline `[s]` blocks.

In pipelines, variables have explicit assignments which make the schema clear:

```polyglot
// Pipeline [s] - No schema required (explicit variable assignments)
[|] LoadData
[i] .json_file:pg\path
[t] |T.Call
[W] |W.Polyglot.Scope

[s] |JSON.Load.Files
[<] <files:pg\array.pg\path << .json_files
[>] >json_data:pg\serial >> .data    // Explicit variable assignment

[s][!] *

[o] .data:pg\serial
[X]
```

**Difference:**
- **Enumerations:** Fields are ambiguous without schema (could be from file, could be typo)
- **Pipelines:** Variables are explicitly declared with types and bindings

---

## Design Philosophy

### Why Safety Mechanisms?

**Problem:** Developers make common mistakes:
1. Forgetting step blocks in manual trigger pipelines
2. Forgetting to declare outputs
3. Declaring inputs they never use
4. Forgetting error handling for serial file loads
5. Loading data files without declaring expected schema

**Solution:** Force explicit acknowledgment of intent:
1. Can't omit `[\]` and `[/]` without declaring `[W] |W.Polyglot.Scope`
2. Can't omit outputs without declaring `[o] !NoError`
3. Can't declare unused inputs without compiler error
4. Can't use `[s]` blocks without declaring `[s][!] *` or custom error handling
5. Can't load serial files into enumerations without declaring schema with `[~][s] <~ .field:type` or wildcard `[~][s] <~ *`

**Benefits:**
- **Prevents accidental errors** - Catches mistakes at compile time
- **Enforces clarity** - Code explicitly shows intent
- **Self-documenting** - Safety declarations explain why things are omitted
- **Reduces bugs** - Forces developers to think about structure

---

## Safety Mechanism Comparison

| Mechanism | What It Protects | Error Prevented | Explicit Declaration |
|-----------|------------------|-----------------|---------------------|
| `[W] |W.Polyglot.Scope` | Step blocks | Forgetting `[\]` and `[/]` | Required when omitting blocks |
| `[o] !NoError` | Output declarations | Forgetting outputs | Required when no outputs |
| `[i]` usage check | Input usage | Unused inputs | Automatic validation |
| `[s][!] *` | Serial file error handling | Silent load failures | Required for all `[s]` blocks |
| `[~][s] <~ .field:type` | Serial schema | Unexpected file fields, type mismatches | Required for enumeration serial loads |

---

## Similar Patterns in Other Languages

### Rust: `#[allow(dead_code]`
```rust
#[allow(dead_code]  // Explicit: "I know this is unused"
fn helper( {
```

### Go: `//nolint`
```go
//nolint:unused  // Explicit: "I know this looks unused"
func internal( {
```

### Polyglot: Safety Mechanisms
```polyglot
[W] |W.Polyglot.Scope       // Explicit: "I know I omitted [\] and [/]"
[o] !NoError                // Explicit: "I know I have no outputs"
[s][!] *                    // Explicit: "I know I need error handling for serial loads"
[~][s] <~ .field:type       // Explicit: "These are the fields I expect from serial files"
```

**Common Theme:** Require developers to explicitly acknowledge intentional deviations from standard patterns or explicitly handle critical operations like error handling.

---

## Teaching Points

### For New Polyglot Developers

**1. Safety mechanisms are your friends:**
- They prevent mistakes you don't know you're making
- They make your code clearer and more explicit
- They catch errors at compile time, not runtime

**2. Don't fight the mechanisms:**
- If compiler asks for `[W] |W.Polyglot.Scope`, ask yourself: "Should I have step blocks?"
- If compiler asks for `[o] !NoError`, ask yourself: "Should I have outputs?"
- If compiler complains about unused input, ask yourself: "Why did I declare this?"
- If compiler asks for `[s][!]`, ask yourself: "How should I handle serial file load errors?"
- If compiler asks for `[~][s] <~`, ask yourself: "What fields do I expect from these files?"

**3. Use mechanisms to document intent:**
- `[W] |W.Polyglot.Scope` says "I thought about step blocks and don't need them"
- `[o] !NoError` says "I thought about outputs and don't need them"
- Clean input list says "These are all the inputs I actually use"
- `[s][!] *` says "I want default error handling for serial file loads"
- `[~][s] <~ .field:type` says "These are exactly the fields I expect from serial files"

---

## See Also

- [Block Markers](block-markers.md - Complete block marker reference
- [Pipeline Structure](../getting-started.md#pipeline-structure - Pipeline anatomy
- [Wrapper System](wrappers.md - Wrapper documentation
