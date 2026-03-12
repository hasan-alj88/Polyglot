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

# Variable Reassignment & Pack/Unpack Operators

**Date:** 2025-12-11
**Status:** 🔧 DESIGN PROBLEM - Needs Solution

---

## The Problem

**Current syntax fails for common transformation pattern:**

```polyglot
[r] $result << "initial"               // $result is Ready

[r] $transformer
   <input << $result                   // Read $result (stays Ready)
   >output >> $result                  // ❌ COMPILE ERROR: $result already exists!
```

**The conflict:**
- `$result` already has a value (state: Ready)
- Cannot push to existing variable
- Need to "update" or "reassign" the variable

---

## Common Use Cases

### Use Case 1: Iterative Transformation
```polyglot
[r] $text << "  HELLO WORLD  "

// Want to apply multiple transformations
[r] |String.Trim
   <input << $text
   >output >> $text                    // ❌ Can't overwrite!

[r] |String.ToLowerCase
   <input << $text
   >output >> $text                    // ❌ Can't overwrite!
```

### Use Case 2: Accumulator Pattern
```polyglot
[r] $sum << 0

[p] ~ForEach
   <array << $numbers
   >item >> $num

   [r] |Add
      <a << $sum
      <b << $num
      >result >> $sum                  // ❌ Can't update accumulator!
```

### Use Case 3: Mutable Update
```polyglot
[r] $record << #Record
   [.] .count << 0
   [.] .status << "pending"

// Later, want to update count
[r] $record << $record
   [.] .count << $record.count + 1     // ❌ $record already exists!
```

---

## Solution Options

### Option 1: Unpack Operator `~<<` (Read for Reassignment)

**Idea:** Explicitly unpack and allow reassignment

```polyglot
[r] $result << "initial"

[r] |Transform
   <input ~<< $result                  // Unpack: read AND mark for reassignment
   >output >> $result                  // ✅ OK: was unpacked

// Alternative syntax:
[r] |Transform
   <input << ~$result                  // Prefix on variable
   >output >> $result
```

**State transition:**
- `$result` state: Ready → Unpacked
- `$result` can now be reassigned

**Problems:**
- Unclear semantics: does `~<<` consume the variable?
- What if you want to read without reassigning?
- Asymmetric: input side affects output side behavior

### Option 2: Reassignment Operator `<<~` (Forced Overwrite)

**Idea:** Explicitly overwrite existing variable

```polyglot
[r] $result << "initial"               // Initial assignment

[r] |Transform
   <input << $result                   // Normal read
   >output >> $result <<~              // ✅ Explicit reassignment

// Or on the marker:
[r] |Transform
   <input << $result
   >output >>~ $result                 // ✅ Reassignment operator

// Or on the variable side:
[r] |Transform
   <input << $result
   >output >> ~$result                 // ✅ Mark variable for overwrite
```

**Benefits:**
- Explicit at assignment point
- Clear intent to overwrite
- Variable remains Ready, just gets new value

**Questions:**
- Which side: `>>~` or `>> ~$var` or `$var <<~`?
- Does this create a new variable or mutate?

### Option 3: Update Operator `<<=` (In-place Update)

**Idea:** Dedicated operator for updating existing variables

```polyglot
[r] $result << "initial"               // Initial assignment: <<

[r] |Transform
   <input << $result
   >output >> $result                  // Still fails...

// Need update syntax on the result marker:
[r] $result <<= |Transform             // Update operator
   <input << $result

// Or:
[r] |Transform
   <input << $result
   >output >>= $result                 // Update operator
```

**Benefits:**
- Clear update semantics
- Familiar from C-style languages (+=, -=, etc.)

**Problems:**
- `<<=` looks like bit shift
- Asymmetric syntax

### Option 4: Shadowing with Explicit Marker `[r!]`

**Idea:** Create new variable that shadows the old one

```polyglot
[r] $result << "initial"               // First version

[r!] $result << |Transform             // ✅ Shadow/reassign: [r!]
   <input << $result                   // Reads old version
   >output >> $result                  // Creates new version

// Or just allow shadowing without special marker:
[r] $result << "initial"
[r] $result << |Transform              // ✅ Automatically shadows
   <input << $result                   // Reads old version
```

**Benefits:**
- Clean syntax
- Functional programming style (immutability)
- Old value accessible during computation

**Problems:**
- Might be confusing: which `$result` is being read?
- Memory implications: keeps both versions

### Option 5: Mutable Variable Declaration `[m]`

**Idea:** Variables are immutable by default, use `[m]` for mutable

```polyglot
[r] $immutable << 42                   // Immutable variable
[r] $immutable << 99                   // ❌ ERROR: cannot reassign

[m] $mutable << 42                     // Mutable variable
[m] $mutable << 99                     // ✅ OK: reassignment allowed

[m] $result << "initial"
[r] |Transform
   <input << $result
   >output >> $result                  // ✅ OK: $result is mutable
```

**Benefits:**
- Clear declaration of intent
- Compiler can optimize immutables
- Familiar from other languages (let vs var)

**Problems:**
- `[m]` already used for match expressions!
- Would need different marker

### Option 6: Unpack Prefix `~$` (User's Suggestion)

**Idea:** Use `~` prefix on variables to indicate pack/unpack

```polyglot
[r] $result << "initial"               // Pack value into variable

[r] |Transform
   <input << ~$result                  // Unpack: extract value
   >output >> ~$result                 // Repack: update variable

// Semantic:
// ~$result on input side: read the value
// ~$result on output side: allow overwrite
```

**Benefits:**
- Symmetric on both sides
- Clear visual indicator
- Single character prefix

**Problems:**
- `~` already used for default operators (`<~`, `~>`)
- Potential confusion with defaults
- Is `~$result` unpacking or setting default?

---

## User's Question: Is `~` the Best Prefix?

**Current uses of `~`:**
- `<~` - Default input value
- `~>` - Default output capture
- `~ForEach` - Pattern/procedure markers

**If we use `~` for pack/unpack:**
- `~$var` - Unpack/repack variable
- But conflicts with pattern markers (`~ForEach`)

**Alternative prefixes:**

### `&` (Reference/Mutable)
```polyglot
[r] |Transform
   <input << &$result                  // Mutable reference
   >output >> &$result                 // Update in place
```
- Familiar from Rust (`&` = borrow, `&mut` = mutable borrow)
- But do we need `&` vs `&mut`?

### `*` (Pointer/Dereference)
```polyglot
[r] |Transform
   <input << *$result                  // Dereference
   >output >> *$result                 // Update through pointer
```
- Familiar from C/C++
- But suggests pointer semantics

### `!` (Force/Override)
```polyglot
[r] |Transform
   <input << $result
   >output >> !$result                 // Force overwrite
```
- Clear intent: "force this"
- But `!` used for errors

### `=` (Update/Mutate)
```polyglot
[r] |Transform
   <input << $result
   >output >> =$result                 // Update assignment
```
- Suggests mutation
- Visually distinct

---

## Recommended Solution: Mutable Marker with New Syntax

**Proposal: Use `[v]` for mutable variables**

- `[r]` - Read-only variable (immutable, single assignment)
- `[v]` - Variable (mutable, can reassign)

**Why `[v]`:**
- `v` = "variable" (truly variable)
- `r` = "result" or "read-only" (immutable)
- Clear distinction
- No marker conflicts

**Syntax:**

```polyglot
{|} |IterativeTransform
[<] i<text:string

[t] |T.Call
[W] |W.Polyglot.Scope

[v] $result << $text                   // Mutable variable

[r] |String.Trim
   <input << $result
   >output >> $result                  // ✅ OK: $result is [v]

[r] |String.ToLowerCase
   <input << $result
   >output >> $result                  // ✅ OK: $result is [v]

[r] |String.RemoveSpaces
   <input << $result
   >output >> $result                  // ✅ OK: $result is [v]

[>] o>result << $result:string
{x}
```

**Accumulator pattern:**

```polyglot
{|} |Sum
[<] i<numbers:array.int

[t] |T.Call
[W] |W.Polyglot.Scope

[v] $sum << 0                          // Mutable accumulator

[p] ~ForEach
   <array << $numbers
   >item >> $num

   [v] $sum << $sum + $num             // ✅ Reassignment allowed

[>] o>sum << $sum:int
{x}
```

**Immutable by default:**

```polyglot
[r] $config << |LoadConfig""          // Immutable
[r] $config << |LoadAgain""            // ❌ ERROR: $config is [r]

[v] $counter << 0                      // Mutable
[v] $counter << $counter + 1           // ✅ OK
```

**With metadata for immutability:**

```polyglot
[r] $config << |LoadConfig""
   [%] %Immutable                      // Redundant (default for [r])

[v] $state << "initial"
   [%] %Mutable                        // Redundant (default for [v])
```

---

## Alternative: Allow Reassignment on Same Line

**Idea:** Within single pipeline call, allow input→output on same variable

```polyglot
[r] $result << "initial"

// Special case: same pipeline call, input feeds output
[r] |Transform
   <input << $result                   // Read
   >output >> $result                  // ✅ OK: same call, clear data flow

// Still error for separate assignments:
[r] $result << "new value"             // ❌ ERROR: separate assignment
```

**Benefits:**
- Handles common case
- No new syntax
- Clear data flow within single operation

**Problems:**
- What about indirect updates?
```polyglot
[r] |Pipeline1
   <input << $result
   >temp >> $temp

[r] |Pipeline2
   <input << $temp
   >output >> $result                  // ❌ Still error?
```

---

## Comparison Table

| Solution | Syntax | Pros | Cons | Recommended |
|----------|--------|------|------|-------------|
| Unpack `~<<` | `<input ~<< $var` | Explicit unpack | Unclear semantics, asymmetric | ❌ |
| Reassign `>>~` | `>output >>~ $var` | Clear intent | Multiple syntax options | ⚠️ |
| Update `<<=` | `$var <<= value` | Familiar | Looks like bit shift | ❌ |
| Shadow `[r!]` | `[r!] $var` | Functional style | Confusing, memory cost | ❌ |
| Mutable `[v]` | `[v] $var << value` | Clear, no conflicts | New marker | ⭐ **YES** |
| Unpack `~$` | `~$var` | Symmetric | Conflicts with patterns | ❌ |

---

## Final Recommendation: `[v]` Mutable Variables

**Summary:**

1. **`[r]`** - Result/Read-only variable (immutable, default)
   - Single assignment only
   - Compiler can optimize
   - Safe by default

2. **`[v]`** - Variable (mutable)
   - Can reassign multiple times
   - Explicit opt-in to mutability
   - Clear at declaration site

**Benefits:**
- No new operators or prefixes
- Clear at variable declaration
- Immutable by default (safer)
- Mutable when needed (practical)
- No conflicts with existing syntax
- Compiler can optimize based on marker

**Migration:**
- All existing `[r]` stays immutable (no breaking change)
- Add `[v]` for mutable variables when needed

**Example combining both:**

```polyglot
{|} |ProcessData
[<] i<items:array.serial

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $config << |LoadConfig""          // Immutable config
[v] $count << 0                        // Mutable counter
[v] $results:array.serial << {}       // Mutable accumulator

[p] ~ForEach
   <array << $items
   >item >> $item

   [r] |ProcessItem
      <item << $item
      <config << $config               // ✅ Read immutable
      >processed >> $processed

   [v] $results << |Array.Append       // ✅ Update mutable
      <array << $results
      <item << $processed

   [v] $count << $count + 1            // ✅ Update mutable

[>] o>results << $results:array.serial
[>] o>count << $count:int
{x}
```

---

**Status:** 🔧 Proposed solution - `[v]` for mutable variables

**Answer to your question:** Instead of pack/unpack with `~`, use `[v]` marker for mutable variables.
