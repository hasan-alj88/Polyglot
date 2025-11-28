# Carson's Brainstorming Session: Macro System Specification

**Session ID:** carson-2025-11-18-macro-system
**Date:** 2025-11-18
**Facilitator:** Carson (Elite Brainstorming Specialist)
**Participant:** hhj (Project Owner)
**Observer/Notes:** Mai (Secretary)
**Duration:** ~60 minutes
**Related Ticket:** Brainstorming Backlog Item #2

---

## 🎯 **Session Purpose**

Define complete specification for Polyglot's macro system:
1. **`[M]` Macro Definition Block** - Reusable code templates
2. **`[W]` Macro Unwrap** - Inline code insertion
3. **`[{]` Scope Input** - Variables flowing INTO macro
4. **`[}]` Scope Output** - Variables flowing OUT of macro
5. **Macro Include Declaration** - Block type manifest

**Impact:** Enable DRY (Don't Repeat Yourself) for setup/cleanup patterns, resolves Brainstorming Item #2

---

## 📋 **Session Summary**

### **MAJOR DECISIONS**
✅ **Macros are compile-time inline code templates**
✅ **Blocks insert by TYPE, not position**
✅ **Scope flow with `[{]` and `[}]`**
✅ **Multiple macros unwrap with FIFO/LIFO ordering**

### **KEY OUTCOMES**
1. ✅ Complete macro definition syntax specified
2. ✅ Scope input/output mechanism designed (`[{]`, `[}]`)
3. ✅ Macro include declaration finalized
4. ✅ Unwrapping syntax and semantics defined
5. ✅ Multiple macro ordering rules established (FIFO setup, LIFO cleanup)
6. ✅ Type safety and validation rules documented
7. ✅ Constant input syntax defined (`[=]` replaces `Fixed`)
8. ✅ Ready for documentation and lexer implementation

---

## 🎨 **COMPLETE DESIGN SPECIFICATION**

### **1. What Are Macros in Polyglot?**

**Mental Model:** Macros are **compile-time inline code templates** that define repeated operations.

**Key Characteristic:** When you unwrap a macro, it's **equivalent to copying those block lines** directly into the pipeline.

**Most Common Use Case:** Setup and cleanup operations (database connections, file handles, logging, etc.)

---

### **2. Macro Definition: `[M]` Block**

#### **Syntax:**

```polyglot
[M] MacroName
[<] Macro.include"<block_types+"
[{] .input_var: type                // Optional: scope inputs
[}] .output_var: type               // Optional: scope outputs
[i] .parameter: type                // Optional: macro parameters
[=] .constant: type << value        // Optional: constant inputs
// ... block definitions by type ...
[X]
```

---

#### **Example: Database Connection Macro**

```polyglot
[M] DatabaseConnection
[<] Macro.include"{\/"             // Has scope input/output, setup, cleanup
[{] .db_host: pg\string            // Input from caller
[}] .db_conn: pg\db                // Output to caller

[\] |U.DB.Connect
[<] .host: pg\string << .db_host
[>] .connection: pg\db >> .db_conn

[/] |U.DB.Disconnect
[<] .conn: pg\db << .db_conn
[X]
```

---

### **3. Macro Include Declaration**

**Purpose:** Explicitly declare which block types the macro contains

**Syntax:**
```polyglot
[<] Macro.include"<characters+"
```

**Each character represents a block type:**

| Character | Block Type | Description |
|-----------|------------|-------------|
| `{` | Scope input `[{]` | Variables flowing INTO macro |
| `}` | Scope output `[}]` | Variables flowing OUT of macro |
| `i` | Parameters `[i]` | Macro parameters |
| `=` | Constants `[=]` | Constant inputs (immutable) |
| `t` | Triggers `[t]` | Trigger definitions |
| `o` | Outputs `[o]` | Pipeline outputs |
| `Q` | Queue control `[Q]` | Queue operations |
| `\` | Setup `[\]` | Setup operations |
| `r` | Run/ops `[r]` | Sequential operations |
| `p` | Parallel `[p]` | Parallel operations |
| `b` | Background `[b]` | Fire-and-forget async |
| `/` | Cleanup `[/]` | Cleanup operations |

---

#### **Include String Rules:**

**1. Order doesn't matter** (blocks have implicit hierarchy)
```polyglot
[<] Macro.include"\/"    // Same as
[<] Macro.include"/\"    // Same as
[<] Macro.include"\/\/"  // Duplicates ignored
```

**2. Compiler validation is STRICT**
```polyglot
// ✅ Valid
[M] Valid
[<] Macro.include"\/"
[\] |Setup
[/] |Cleanup
[X]

// ❌ Compile Error - 'r' not declared
[M] Invalid
[<] Macro.include"\/"
[\] |Setup
[r] |Operation           // ERROR: 'r' not in Macro.include!
[/] |Cleanup
[X]
```

**3. Empty string is invalid**
```polyglot
[M] Invalid
[<] Macro.include""      // ❌ Compile Error
[X]
```

---

### **4. Scope Flow: `[{]` Input and `[}]` Output**

#### **A. Scope Input `[{]` - Bring INTO Macro**

**Purpose:** Declare variables that flow FROM caller INTO macro scope

**Syntax:**
```polyglot
[{] .variable_name: type
```

**Example:**
```polyglot
[M] LogToFile
[<] Macro.include"{\/"
[{] .log_file_path: pg\path     // REQUIRES from caller
[}] .log_handle: pg\file        // PROVIDES to caller

[\] |U.File.Text.Open
[<] .file: pg\path << .log_file_path    // Use input
[>] .handler: pg\file >> .log_handle    // Create output
[X]
```

---

#### **B. Scope Output `[}]` - Send OUT of Macro**

**Purpose:** Declare variables that flow FROM macro TO caller scope

**Syntax:**
```polyglot
[}] .variable_name: type
```

**Key Point:** Variables declared with `[}]` become available in the caller's pipeline scope after unwrapping.

---

#### **C. Complete Scope Flow Example**

```polyglot
[M] CacheSetup
[<] Macro.include"={}\/"
[=] .cache_size: pg\int << 1024        // Constant (immutable)
[{] .cache_name: pg\string             // Input from caller
[}] .cache_handle: pg\cache            // Output to caller

[\] |U.Cache.Initialize
[<] .name: pg\string << .cache_name
[<] .size: pg\int << .cache_size
[>] .handle: pg\cache >> .cache_handle

[/] |U.Cache.Destroy
[<] .handle: pg\cache << .cache_handle
[X]
```

---

### **5. Macro Unwrapping: `[W]` Block**

#### **Syntax:**

```polyglot
[W] |MacroName
[<] .input_var << source_value      // Wire scope inputs [{]
[>] .output_var >> target_variable  // Wire scope outputs [}]
```

**Key:** Uses pipeline call syntax `|MacroName`

---

#### **Complete Example:**

```polyglot
[M] OpenLogFile
[<] Macro.include"{\/"
[{] .log_file: pg\path
[}] .log_handle: pg\file

[\] |U.File.Text.Open
[<] .file: pg\path << .log_file
[>] .file_handler: pg\file >> .log_handle

[/] |U.File.Text.Close
[<] .file_handler: pg\file << .log_handle
[X]

// Usage in pipeline:
[|] SomePipeline
[=] .a_log_file: pg\path << \\FileDir\\log.txt

[W] |OpenLogFile                    // Unwrap macro
[<] .log_file << .a_log_file        // Wire input (feeds [{])
[>] .log_handle >> .log             // Wire output (receives [}])

// .log is now available!
[r] |File.Text.AddLine
[<] .handle: pg\file << .log
[<] .line: pg\string << "handle used"
[X]
```

---

### **6. Block Insertion by Type**

**Critical Rule:** When a macro is unwrapped, blocks are **inserted by TYPE**, not by position.

**Block Hierarchy Order:**
```
1. [t] Triggers
2. [i] Parameters
3. [=] Constants
4. [Q] Queue control
5. [\] Setup
6. [r] Operations
7. [p] Parallel
8. [b] Background
9. [/] Cleanup
10. [o] Outputs
```

---

#### **Example: Block Merging**

**Macro:**
```polyglot
[M] LoggingSetup
[<] Macro.include"\/"
[\] |U.Log.Start
[<] .level: pg\string << "INFO"
[/] |U.Log.Stop
[X]
```

**Pipeline:**
```polyglot
[|] ProcessData
[i] .input: pg\string         // Pipeline input
[W] |LoggingSetup             // Unwrap macro
[r] |ProcessOperation         // Pipeline operation
[o] .result: pg\string        // Pipeline output
[X]
```

**After Unwrapping (equivalent to):**
```polyglot
[|] ProcessData
[i] .input: pg\string         // Original pipeline input
[\] |U.Log.Start              // FROM MACRO - inserted with setup
[<] .level: pg\string << "INFO"
[r] |ProcessOperation         // Original pipeline operation
[/] |U.Log.Stop               // FROM MACRO - inserted with cleanup
[o] .result: pg\string        // Original pipeline output
[X]
```

**Blocks merged by type:** All `[\]` together, all `[r]` together, all `[/]` together, etc.

---

### **7. Multiple Macro Unwrapping - Smart Ordering**

#### **Setup Blocks: FIFO (First In, First Out)**

Setup blocks execute in the **same order** as macros are unwrapped:

```polyglot
[|] Pipeline
[W] |Macro1      →  [\] Macro1_Setup    (executes FIRST)
[W] |Macro2      →  [\] Macro2_Setup    (executes SECOND)
[W] |Macro3      →  [\] Macro3_Setup    (executes THIRD)
// ... operations ...
[X]
```

**Mental Model:** Build up resources in order

---

#### **Cleanup Blocks: LIFO (Last In, First Out - REVERSE!)**

Cleanup blocks execute in **reverse order** of setup:

```polyglot
[|] Pipeline
[W] |Macro1
[W] |Macro2
[W] |Macro3
// ... operations ...
                 [/] Macro3_Cleanup  (cleans FIRST - last setup)
                 [/] Macro2_Cleanup  (cleans SECOND)
                 [/] Macro1_Cleanup  (cleans LAST - first setup)
[X]
```

**Mental Model:** Like RAII or try-finally - last resource acquired, first released

---

#### **Complete Example: Multiple Macros**

**Define Macros:**
```polyglot
[M] DatabaseConnection
[<] Macro.include"{\/"
[{] .db_host: pg\string
[}] .db_conn: pg\db
[\] |U.DB.Connect
[<] .host: pg\string << .db_host
[>] .conn: pg\db >> .db_conn
[/] |U.DB.Disconnect
[<] .conn: pg\db << .db_conn
[X]

[M] LoggingSetup
[<] Macro.include"\/"
[\] |U.Log.Start
[<] .level: pg\string << "INFO"
[/] |U.Log.Stop
[X]

[M] CacheSetup
[<] Macro.include"}\/"
[}] .cache: pg\cache
[\] |U.Cache.Init
[>] .cache: pg\cache >> .cache
[/] |U.Cache.Clear
[<] .cache: pg\cache << .cache
[X]
```

**Use Macros:**
```polyglot
[|] ProcessUsers
[i] .users: pg\array{pg\string}

[W] |DatabaseConnection        // 1st macro
[<] .db_host << "localhost"
[>] .db_conn >> .db

[W] |LoggingSetup              // 2nd macro

[W] |CacheSetup                // 3rd macro
[>] .cache >> .app_cache

[r] |QueryUsers
[<] .conn: pg\db << .db
[<] .cache: pg\cache << .app_cache

[o] .results: pg\array{pg\user}
[X]
```

**After Unwrapping:**
```polyglot
[|] ProcessUsers
[i] .users: pg\array{pg\string}

// Setup (FIFO - in order)
[\] |U.DB.Connect         // 1st macro
[\] |U.Log.Start          // 2nd macro
[\] |U.Cache.Init         // 3rd macro

[r] |QueryUsers

// Cleanup (LIFO - reverse order!)
[/] |U.Cache.Clear        // 3rd macro (cleans FIRST!)
[/] |U.Log.Stop           // 2nd macro
[/] |U.DB.Disconnect      // 1st macro (cleans LAST!)

[o] .results: pg\array{pg\user}
[X]
```

---

### **8. Constant Inputs: `[=]` Block**

**Purpose:** Define immutable constant inputs (replaces `Fixed` keyword)

**Syntax:**
```polyglot
[=] .constant_name: type << value
```

**Example:**
```polyglot
[M] APIClient
[<] Macro.include"=\/"
[=] .api_key: pg\string << "secret-key-123"    // Immutable
[=] .timeout: pg\int << 30
[\] |U.HTTP.Connect
[<] .key: pg\string << .api_key
[<] .timeout_ms: pg\int << .timeout
[X]
```

**Difference from `[i]`:**
- `[i]` - Caller CAN provide value
- `[=]` - Caller CANNOT override (constant)

---

### **9. Type Safety and Validation**

#### **Type Checking at Unwrap Site:**

```polyglot
[M] TypedMacro
[<] Macro.include"{\/"
[{] .input: pg\string         // Expects pg\string
[}] .output: pg\int           // Provides pg\int
[X]

[|] Pipeline
[r] .wrong_type: pg\int << 42

[W] |TypedMacro
[<] .input << .wrong_type     // ❌ Compile Error: Type mismatch!
                              //    Expected pg\string, got pg\int
[>] .output >> .result        // ✅ Type correct
[X]
```

---

#### **Missing Wiring Detection:**

```polyglot
[M] RequiresInput
[<] Macro.include"{\/"
[{] .required_input: pg\string
[X]

[|] Pipeline
[W] |RequiresInput
// ❌ Compile Error: Missing wiring for [{] .required_input
[X]
```

---

### **10. Variable Renaming at Unwrap**

**Flexible naming:** Macro variables can be renamed when unwrapped

```polyglot
[M] GenericSetup
[<] Macro.include"{\/"
[{] .resource_name: pg\string
[}] .resource_handle: pg\handle
[\] |AcquireResource
[<] .name: pg\string << .resource_name
[>] .handle: pg\handle >> .resource_handle
[X]

// Pipeline 1: Use as database
[|] Pipeline1
[W] |GenericSetup
[<] .resource_name << "database"
[>] .resource_handle >> .db_handle
// Use .db_handle
[X]

// Pipeline 2: Use as cache
[|] Pipeline2
[W] |GenericSetup
[<] .resource_name << "cache"
[>] .resource_handle >> .cache_handle
// Use .cache_handle
[X]
```

**Same macro, different names in each pipeline!**

---

### **11. TBD (To Be Implemented) Syntax**

**For work-in-progress macros:**

```polyglot
[M] WorkInProgress
[<] Macro.include"\/"
[\] |U.DB.Connect
[<] .host: pg\string << "localhost"
[/] |Code.ToBeImplemented        // ← Placeholder
[X]
```

**`|Code.ToBeImplemented`** is a standard library pipeline that serves as a placeholder.

---

## 🎯 **Design Strengths**

### **1. DRY Principle**
✨ No more copying setup/cleanup code
✨ Define once, use everywhere
✨ Consistency across pipelines

### **2. Explicit Scope Flow**
✨ `[{]` and `[}]` make data flow visible
✨ No hidden magic
✨ Clear contract at macro definition

### **3. Type Safety**
✨ Compiler validates types at unwrap site
✨ No runtime type errors
✨ Catches mistakes early

### **4. Smart Ordering**
✨ FIFO setup (build up)
✨ LIFO cleanup (tear down)
✨ RAII-like resource management

### **5. Flexibility**
✨ Variable renaming at unwrap
✨ Same macro, multiple use cases
✨ Composable macros

### **6. No Keywords**
✨ Uses existing syntax patterns
✨ `[=]` instead of `Fixed`
✨ Consistent with Polyglot philosophy

---

## 📝 **Complete Operator Reference**

### **Macro Definition:**
```
[M]     Macro definition
[X]     End macro
```

### **Macro Include:**
```
[<] Macro.include"<characters+"
```

### **Scope Flow:**
```
[{]     Scope input (bring IN from caller)
[}]     Scope output (send OUT to caller)
```

### **Macro Unwrap:**
```
[W]     Unwrap macro (inline insertion)
[<]     Wire scope inputs (at [W] site)
[>]     Wire scope outputs (at [W] site)
```

### **Input Types:**
```
[i]     Parameter (caller can provide)
[=]     Constant (caller cannot override)
```

---

## 📊 **Lexer Token Requirements**

**For Amelia (Story 1.2):**

### **New Block Markers:**
```rust
// Macro System
MacroDefine,       // [M] - Macro definition
MacroUnwrap,       // [W] - Unwrap macro
ScopeInput,        // [{] - Scope input
ScopeOutput,       // [}] - Scope output
ConstantInput,     // [=] - Constant input (replaces Fixed)
```

### **Deprecated:**
```rust
// REMOVE from lexer:
// Fixed keyword - replaced by [=]
```

---

## 📚 **Documentation Requirements**

**For Paige (Technical Writer):**

### **Files to Update:**

1. **`docs/user/language/06-block-markers.md`**
   - Add `[M]` Macro definition
   - Add `[W]` Macro unwrap
   - Add `[{]` Scope input
   - Add `[}]` Scope output
   - Add `[=]` Constant input
   - Remove `Fixed` keyword references

2. **`docs/user/language/XX-macros.md`** (NEW)
   - Complete macro system documentation
   - Scope flow explanation
   - Multiple macro ordering
   - Type safety rules
   - Comprehensive examples

3. **`docs/user/language/01-syntax-complete.md`**
   - Update quick reference with macro syntax
   - Add macro examples

4. **`docs/user/audit/quick-language-reference.md`**
   - Add macro syntax to grammar
   - Update block marker list

### **New Examples to Create:**

- Database connection macro
- File handling macro
- Logging setup macro
- Cache initialization macro
- Multiple macro composition
- Error handling with macros

---

## 🔧 **Design Decisions Log**

### **Decision 1: Macros are Compile-Time Inline Templates**
- **Decision:** Macros insert code at compile-time by block type
- **Rationale:** True code reuse, zero runtime overhead
- **Alternative Rejected:** Runtime macro expansion (too slow)
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 2: Blocks Insert by Type, Not Position**
- **Decision:** Macro blocks merge with pipeline blocks by type
- **Rationale:** Maintains logical block hierarchy automatically
- **Example:** All `[\]` setup together, all `[/]` cleanup together
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 3: Explicit Include Declaration**
- **Decision:** Use `[<] Macro.include"<chars+"` to declare block types
- **Rationale:** Explicit over implicit, compiler validation, prevents mistakes
- **Alternative Rejected:** Implicit omission (too error-prone)
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 4: Scope Flow with `[{]` and `[}]`**
- **Decision:** `[{]` brings variables IN, `[}]` sends variables OUT
- **Rationale:** Symmetric braces, clear directionality, explicit contract
- **Alternatives Considered:** `[o]` outputs, `^` prefix, `[@]` reference
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 5: Multiple Macro Ordering (FIFO/LIFO)**
- **Decision:** Setup = FIFO (in order), Cleanup = LIFO (reverse)
- **Rationale:** Resource management pattern (RAII), last acquired first released
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 6: `[=]` Replaces `Fixed` Keyword**
- **Decision:** Use `[=]` block for constant inputs instead of `Fixed` keyword
- **Rationale:** No keywords policy, `=` symbolizes assignment/constant
- **Migration:** `[i] Fixed .x << 3` → `[=] .x: pg\int << 3`
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 7: Macro Unwrap Uses Pipeline Syntax**
- **Decision:** `[W] |MacroName` (like calling a pipeline)
- **Rationale:** Familiar syntax, consistent with pipeline calls
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 8: Type Safety at Unwrap Site**
- **Decision:** Compiler validates types when wiring `[<]` and `[>]`
- **Rationale:** Catch errors at compile-time, not runtime
- **Approved By:** hhj
- **Date:** 2025-11-18

### **Decision 9: TBD Placeholder**
- **Decision:** Use `|Code.ToBeImplemented` for work-in-progress
- **Rationale:** Standard library approach, no new syntax
- **Approved By:** hhj
- **Date:** 2025-11-18

---

## 💡 **Key Insights from Session**

1. **Macros ≠ Functions:** Inline code insertion, not function calls
2. **Scope Flow is Explicit:** `[{]` and `[}]` make data flow visible
3. **Block Types Matter:** Insertion by type maintains logical structure
4. **FIFO/LIFO Pattern:** Natural resource management
5. **No Keywords Victory:** `[=]` replaces `Fixed`, maintains philosophy
6. **Type Safety Wins:** Compiler catches mistakes early
7. **Braces are Perfect:** `{` open brings in, `}` close sends out

---

## 🎯 **Next Steps**

### **Immediate Actions:**

1. **Mai:** Update brainstorming-backlog.md item #2 as "Completed"
2. **Paige:** Create `language/XX-macros.md` documentation
3. **Paige:** Update `language/06-block-markers.md` with new blocks
4. **Paige:** Remove `Fixed` keyword references
5. **Amelia:** Add macro tokens to lexer (Story 1.2)

### **Follow-up:**

6. **Paige:** Create comprehensive macro examples
7. **Winston:** Consider macro scope in architecture
8. **Murat:** Define test cases for macro system

---

## 📎 **Related Artifacts**

- **Brainstorming Item:** [Item #2](../brainstorming-backlog.md#2)
- **Related Sessions:**
  - [Comparison Operators](carson-2025-11-18-comparison-operators-design.md)
  - [Line Continuation](carson-2025-11-18-line-continuation-spec.md)

---

## ✍️ **Session Metadata**

**Facilitated By:** Carson (Elite Brainstorming Specialist)
**Notes Maintained By:** Mai (Secretary)
**Session Type:** Syntax Specification
**Outcome:** ✅ COMPLETE
**Blocking Status:** Brainstorming Item #2 RESOLVED

---

**Session Status:** ✅ COMPLETE
**Next Steps:**
1. Mai: Update brainstorming backlog
2. Paige: Document macro system
3. Amelia: Add tokens to lexer

---

🧠 *Brainstorming session complete - Carson*
