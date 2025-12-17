# AI Context Package - Corrections Applied

## Issues Found and Fixed

### Issue 1: Incorrect Block Marker Usage for Variable Declaration

**Problem:**
```polyglot
[<] .current_time: pg\dt << DT.Now
```

**Issues:**
- `[<]` is for passing variables INTO parent block (usually pipeline calls)
- Has no parent block context here
- Block elements have usage hierarchy that must be respected

**Correct:**
```polyglot
[r] .current_time: pg\dt << DT.Now""
```

**Explanation:**
- `[r]` is for variable declaration at pipeline scope
- Inline pipeline calls are string literals, so `DT.Now""`

---

### Issue 2: Inline Pipeline Syntax

**Problem:**
```polyglot
DT.Now
```

**Correct:**
```polyglot
DT.Now""
```

**Explanation:**
- All inline pipeline calls are string literals
- Empty string `""` when no parameters needed

---

### Issue 3: Misunderstanding Array Push Semantics

**Problem:**
```polyglot
[<] .channels: pg\array{#Channel} << []          // Start with empty array
```

**Issues:**
- Pushing empty array `[]` makes variable immediately **Ready state**
- Once Ready, no more pushes allowed
- Wrong syntax for empty array (should be `{}`)
- Doesn't need to "start with empty array"

**Correct:**
```polyglot
[r] .channels: pg\array{#Channel}                 // Variable declaration in Declared state
```

**Explanation:**
- Variable in **Declared state** allows values to be pushed into it
- If you directly push, variable becomes **Ready** and no more pushes allowed
- Use Declared state, then push values conditionally

---

### Issue 4: Variable State Terminology

**Critical Paradigm Shift:**
Polyglot is async-centric. Use correct terminology:

❌ **DON'T SAY:**
- "Assign" / "Assignment"
- "Mutable" / "Immutable"
- "Initialize with empty array"

✅ **DO SAY:**
- "PUSH" (to variable)
- "PULL" (from variable/pipeline)
- "Variable States" (Declared → DefaultReady → Pending → Ready/Faulted)
- "Variable in Declared state, ready for pushes"

**Key Concepts:**
- Variables transition through **states** (not just assigned values)
- **Declared state:** Schema declared, ready to accept pushes
- **DefaultReady state:** Has default, allows 1 override
- **Ready state:** Final value, no more pushes allowed
- **Pending state:** Async operation in progress

---

### Issue 5: Empty Collection Syntax

**Problem:**
```polyglot
<< []
```

**Correct:**
```polyglot
<< {}
```

**Explanation:**
- All collections use `{}` with comma separation
- `{item1, item2, item3}`
- Empty: `{}`

---

### Issue 6: Undefined Pipeline References

**Problem:**
```polyglot
[~][r] |AddToChannels    // Pipeline never defined with [|] and [t] |T.Call
```

**Issues:**
- Pipeline `|AddToChannels` used but never defined
- If from external package, must import in `[@]` scope
- Use `@ImportAlias|PipelineName`

**Correct:**
Either define the pipeline:
```polyglot
[|] AddToChannels
[t] |T.Call
[i] .channel: #Channel
[i] .list: pg\array{#Channel}
...
[X]
```

Or import from package:
```polyglot
[@] Local@MyPackage:1.0.0
[<] @stdlib << @pg.stdlib:1.0.0
[X]

// Then use:
[r] @stdlib|ArrayAppend
```

**In Example:**
Removed undefined pipeline calls, simplified to direct pushes since array was in Declared state

---

## Corrected Patterns

### Pattern 1: Variable Declaration with Inline Pipeline
```polyglot
[r] .current_time: pg\dt << DT.Now""              // Inline pipeline (string literal)
```

### Pattern 2: Variable Declaration in Declared State
```polyglot
[r] .channels: pg\array{#Channel}                 // Declared state, ready for pushes
```

### Pattern 3: Conditional Pushing to Array
```polyglot
[r] .channels: pg\array{#Channel}                 // Declared state

[?] .condition =? #Boolean.True
[~][<] .channels << #Channel.Email               // PUSH to array
[~]

[?] *?
[~][o] #None
[~]
```

**Key Points:**
- `.channels` in Declared state
- Conditionally PUSH individual items
- Each push adds to collection
- Once all pushes complete, variable becomes Ready

### Pattern 4: Variable Declaration with Immediate Value
```polyglot
[r] .success: pg\bool << #Boolean.False           // Immediately Ready state
```

**Key Points:**
- Direct push makes variable **Ready immediately**
- Push count becomes final (0 remaining)
- No more pushes allowed

---

## Updated AI Context Package Rules

### Rule: Block Marker Hierarchy

**[r] - Sequential Execution / Variable Declaration**
- At pipeline scope: Variable declaration
- In block scope: Sequential pipeline call

**[<] - Input Binding**
- ONLY used within parent block context
- Passes values INTO pipeline/operation
- Never standalone at pipeline scope

**Usage:**
```polyglot
[|] MyPipeline

[r] .my_var: pg\int << 42                        // ✓ Variable declaration

[r] |SomePipeline                                // ✓ Pipeline call
[<] .input: pg\int << .my_var                    // ✓ Input binding to pipeline

[<] .standalone: pg\int << 42                    // ❌ WRONG - no parent block
```

---

## Key Corrections Summary

1. **Use `[r]` for variable declarations** (not `[<]`)
2. **Inline pipelines are string literals** (`DT.Now""`)
3. **Empty collections use `{}`** (not `[]`)
4. **Declared state for arrays that will receive pushes** (not empty array push)
5. **Use async terminology**: PUSH/PULL, Variable States (not assign/mutable)
6. **Define pipelines before use** (or import via `[@]`)

---

## Testing Validation

The corrected example now demonstrates:
- ✅ Proper variable declarations with `[r]`
- ✅ Correct inline pipeline syntax `DT.Now""`
- ✅ Proper array push semantics (Declared → Ready)
- ✅ Correct terminology (PUSH/PULL, states)
- ✅ No undefined pipeline references

---

**Generated:** 2025-11-26
**AI Context Package Version:** v0.0.2
