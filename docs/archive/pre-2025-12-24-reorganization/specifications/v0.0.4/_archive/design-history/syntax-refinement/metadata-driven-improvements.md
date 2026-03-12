<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

# Metadata-Driven Syntax Improvements

**Date:** 2025-12-11
**Status:** 💡 BRAINSTORMING - Unlocking Metadata Power

---

## Philosophy

> **"Metadata enables powerful features without syntax bloat."**

Instead of adding keywords or special syntax, use metadata to unlock:
- Performance optimizations
- Safety guarantees
- Code generation
- Type constraints
- Async/concurrency
- Resource management

**Core principle:** Keep syntax minimal, express intent through metadata.

---

## Category 1: Type Constraints & Validation (HIGH IMPACT)

### Problem
Type annotations alone don't express constraints:
```polyglot
[<] i<age:int                          // Any int, but we want 0-120
[<] i<email:string                     // Any string, but we want valid email
```

### Solution: Constraint Metadata
```polyglot
{|} |RegisterUser
[<] i<age:int
   [%] %Constraint
      [.] .min << 0
      [.] .max << 120
      [.] .message << "Age must be 0-120"

[<] i<email:string
   [%] %Constraint
      [.] .pattern << "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
      [.] .message << "Invalid email format"

[<] i<password:string
   [%] %Constraint
      [.] .min_length << 8
      [.] .max_length << 128
      [.] .require << {"uppercase", "lowercase", "digit", "special"}
{x}
```

**Compiler generates validation code automatically!**

### Benefits
- ✅ Self-documenting constraints
- ✅ Compile-time validation generation
- ✅ IDE can show constraints
- ✅ No runtime library needed (compiler generates validators)

---

## Category 2: Immutability & Constants (HIGH IMPACT)

### Problem
No way to express immutability:
```polyglot
[r] $config << |LoadConfig""          // Can be modified later
```

### Solution: Immutability Metadata
```polyglot
{|} |Initialize
[t] |T.Call
[W] |W.Polyglot.Scope

[r] $config << |LoadConfig""
   [%] %Immutable                      // Cannot be reassigned after init

[r] $constant << 42
   [%] %Const                          // Compile-time constant

[r] $user_data << |FetchUser""
   [%] %Frozen                         // Can assign once, then immutable
```

**Compiler enforces:**
```polyglot
[r] $config << |LoadConfig""
   [%] %Immutable

[r] $config << |LoadAgain""            ❌ ERROR: Cannot reassign immutable variable
```

### Advanced: Field-level Immutability
```polyglot
{#} #User
[.] .id:string
   [%] %Immutable                      // ID never changes
[.] .email:string
   [%] %Mutable                        // Email can change
[.] .created:dt
   [%] %Immutable                      // Timestamp never changes
{x}
```

---

## Category 3: Performance Hints (MEDIUM IMPACT)

### Problem
No way to express performance intent.

### Solution: Performance Metadata
```polyglot
{|} |ProcessLargeDataset
[<] i<data:array.serial

[t] |T.Call
[W] |W.Polyglot.Scope

// Hint: This loop can run in parallel
[p] ~ForEach
   <array << $data
   >item >> $item
   [%] %Parallel                       // Safe to parallelize
   [%] %ThreadSafe                     // No shared state

   [r] $processed << |TransformItem <item << $item
   [>] >result << $processed

// Cache expensive computation
[r] $result << |ExpensiveComputation
   <input << $data
   [%] %Cache
      [.] .ttl << 300                  // Cache for 5 minutes
      [.] .key << "computation_{$data.hash}"
```

**Compiler can:**
- Generate parallel code for `%Parallel` loops
- Insert caching layer for `%Cache` operations
- Skip optimizations when unsafe

### Additional Performance Hints
```polyglot
[r] $small_value << |InlineMe""
   [%] %Inline                         // Inline expansion hint

[r] $hot_path << |CriticalOperation""
   [%] %Optimize << "speed"            // Optimize for speed, not size

[r] $rarely_used << |EdgeCase""
   [%] %Optimize << "size"             // Optimize for code size
```

---

## Category 4: Async/Await (HIGH IMPACT)

### Problem
No async/concurrency primitives in current syntax.

### Solution: Async Metadata
```polyglot
{|} |FetchMultipleAPIs
[<] i<urls:array.url

[t] |T.Call
[W] |W.Polyglot.Scope

// Mark pipeline as async
[%] %Async

// Concurrent fetch operations
[p] ~ForEach
   <array << $urls
   >item >> $url
   [%] %Concurrent                     // Run iterations concurrently
   [%] %MaxConcurrency << 10           // Limit to 10 at once

   [r] $response << |HTTP.Get
      <url << $url
      [%] %Await                       // Await async operation
      [%] %Timeout << 5000             // 5 second timeout

   [>] >result << $response

[>] o>results << $all_responses:array.serial
{x}
```

**Compiler generates:**
- Async runtime setup
- Concurrent execution
- Timeout handling
- Result aggregation

### Await with Timeout & Retry
```polyglot
[r] $data << |UnreliableAPI
   <endpoint << $url
   [%] %Await
   [%] %Timeout << 5000
   [%] %Retry
      [.] .max_attempts << 3
      [.] .backoff << "exponential"
      [.] .initial_delay << 100
```

---

## Category 5: Error Handling Strategy (MEDIUM IMPACT)

### Problem
No declarative error handling strategy.

### Solution: Error Handling Metadata
```polyglot
{|} |RobustOperation
[<] i<input:string

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $result << |MayFailOperation
   <input << $input
   [%] %OnError
      [.] .retry << 3
      [.] .fallback << |DefaultValue""
      [.] .log << #Boolean.True

// Alternative: Panic on error
[r] $critical << |MustSucceed
   <data << $input
   [%] %OnError
      [.] .strategy << "panic"
      [.] .message << "Critical operation failed"

// Alternative: Ignore errors
[r] $optional << |BestEffort
   <data << $input
   [%] %OnError
      [.] .strategy << "ignore"
      [.] .default << :optional.None
```

---

## Category 6: Testing Metadata (MEDIUM IMPACT)

### Problem
No built-in testing infrastructure.

### Solution: Test Metadata
```polyglot
{|} |Add
[<] i<a:int
[<] i<b:int
[t] |T.Call
[W] |W.Polyglot.Scope
[r] $result << $a + $b
[>] o>result << $result:int
{x}

// Test cases defined via metadata
{|} |Add
[%] %Test
   [.] .name << "Add positive numbers"
   [.] .input
      [.] .a << 2
      [.] .b << 3
   [.] .expect
      [.] .result << 5

[%] %Test
   [.] .name << "Add negative numbers"
   [.] .input
      [.] .a << -2
      [.] .b << -3
   [.] .expect
      [.] .result << -5

[%] %Test
   [.] .name << "Add zero"
   [.] .input
      [.] .a << 0
      [.] .b << 5
   [.] .expect
      [.] .result << 5
{x}

// Benchmark metadata
{|} |ExpensiveOperation
[%] %Benchmark
   [.] .iterations << 1000
   [.] .warmup << 100
   [.] .measure << "throughput"
{x}
```

**Compiler can:**
- Generate test runner
- Execute tests during compilation
- Generate benchmarks

---

## Category 7: Ownership & Borrowing (ADVANCED)

### Problem
No memory safety guarantees like Rust.

### Solution: Ownership Metadata
```polyglot
{|} |ProcessData
[<] i<data:serial
   [%] %Ownership << "move"            // Takes ownership
   // data consumed, can't use in caller after

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $processed << |Transform
   <data << $data
   [%] %Ownership << "borrow"          // Borrows data (read-only)

[r] $modified << |Mutate
   <data << $data
   [%] %Ownership << "borrow_mut"      // Mutable borrow

[>] o>result << $modified:serial
{x}
```

**Compiler can:**
- Track ownership transfers
- Prevent use-after-move
- Ensure no simultaneous mutable borrows
- Generate lifetime annotations

---

## Category 8: Code Generation (HIGH IMPACT)

### Problem
Repetitive boilerplate for common patterns.

### Solution: Derive/Generate Metadata
```polyglot
{#} #User
[%] %Derive << "Serialize"             // Auto-generate serialization
[%] %Derive << "Deserialize"           // Auto-generate deserialization
[%] %Derive << "Debug"                 // Auto-generate debug output
[%] %Derive << "Clone"                 // Auto-generate clone method

[.] .id:string
[.] .email:string
[.] .created:dt
{x}

// Compiler generates:
// - |User.Serialize
// - |User.Deserialize
// - |User.Debug
// - |User.Clone
```

### Database Mapping
```polyglot
{#} #User
[%] %Table << "users"                  // Maps to database table

[.] .id:string
   [%] %Column << "user_id"
   [%] %PrimaryKey
   [%] %AutoGenerate

[.] .email:string
   [%] %Column << "email_address"
   [%] %Unique
   [%] %Index

[.] .created:dt
   [%] %Column << "created_at"
   [%] %Default << "CURRENT_TIMESTAMP"
{x}

// Compiler generates:
// - SQL schema
// - CRUD operations
// - Migrations
```

---

## Category 9: Conditional Compilation (MEDIUM IMPACT)

### Problem
No feature flags or platform-specific code.

### Solution: Conditional Metadata
```polyglot
{|} |Initialize
[t] |T.Call
[W] |W.Polyglot.Scope

// Only in debug builds
[r] $debug_info << |GatherDebugInfo""
   [%] %If << @BuildConfig.Debug

// Platform-specific
[r] $path_separator << "/"
   [%] %If << @Platform.Unix

[r] $path_separator << "\\"
   [%] %If << @Platform.Windows

// Feature flag
[r] $experimental << |NewFeature""
   [%] %If << @Features.Experimental
{x}
```

---

## Category 10: Security/Privacy (HIGH IMPACT)

### Problem
No way to mark sensitive data or audit requirements.

### Solution: Security Metadata
```polyglot
{#} #User
[.] .username:string
[.] .email:string
[.] .password_hash:string
   [%] %Sensitive                      // Never log or expose
   [%] %Encrypted                      // Encrypted at rest
[.] .ssn:string
   [%] %Sensitive
   [%] %PII                            // Personally Identifiable Information
   [%] %Audit                          // Audit all access
{x}

{|} |AccessUserData
[<] i<user_id:string

[t] |T.Call
[W] |W.Polyglot.Scope

[%] %Requires << "authentication"      // Must be authenticated
[%] %Capability << "user_data_read"    // Requires capability
[%] %Audit << #Boolean.True            // Audit this operation

[r] $user << |Database.GetUser <id << $user_id
{x}
```

**Compiler can:**
- Prevent logging sensitive fields
- Generate audit logs
- Enforce access control
- Ensure encryption

---

## Category 11: Resource Management (MEDIUM IMPACT)

### Problem
No automatic resource cleanup.

### Solution: Resource Metadata
```polyglot
{|} |ProcessFile
[<] i<filename:string

[t] |T.Call
[W] |W.Polyglot.Scope

[r] $file << |File.Open <path << $filename
   [%] %Resource << "file_handle"
   [%] %Cleanup << |File.Close         // Auto-cleanup on exit

[r] $db << |Database.Connect""
   [%] %Resource << "database_connection"
   [%] %Transaction                    // Run in transaction
   [%] %Cleanup << |Database.Disconnect

// Compiler ensures cleanup even on error
{x}
```

---

## Top 10 Recommendations

### Tier 1: Essential (Immediate Value)
1. **%Constraint** - Type constraints and validation
2. **%Immutable** - Immutability guarantees
3. **%Async / %Await** - Async/concurrency primitives
4. **%Derive** - Code generation (Serialize, etc.)
5. **%Test** - Built-in testing framework

### Tier 2: High Value (Next Phase)
6. **%OnError** - Error handling strategies
7. **%Parallel** - Parallelization hints
8. **%Cache** - Caching layer
9. **%If** - Conditional compilation
10. **%Sensitive** - Security/privacy markers

### Tier 3: Advanced (Future)
11. **%Ownership** - Rust-like ownership
12. **%Resource** - Resource management
13. **%Benchmark** - Performance testing
14. **%Table** - Database mapping
15. **%FFI** - Foreign function interface

---

## Example: All Together

```polyglot
{@} @MyApp::UserService:1.0.0.0
[A] @UserSvc
{x}

{#} #User
[%] %Derive << "Serialize"
[%] %Derive << "Deserialize"
[%] %Table << "users"

[.] .id:string
   [%] %PrimaryKey
   [%] %Immutable
[.] .email:string
   [%] %Constraint
      [.] .pattern << "email"
   [%] %Unique
[.] .password_hash:string
   [%] %Sensitive
   [%] %Encrypted
[.] .age:int
   [%] %Constraint
      [.] .min << 0
      [.] .max << 120
{x}

{|} |RegisterUser
[%] %Async
[%] %Requires << "unauthenticated"
[%] %Audit

[<] i<email:string
   [%] %Constraint
      [.] .pattern << "email"
[<] i<password:string
   [%] %Sensitive
   [%] %Constraint
      [.] .min_length << 8

[t] |T.Call
[W] |W.Polyglot.Scope

// Check email availability
[r] $exists << |Database.UserExists
   <email << $email
   [%] %Await
   [%] %Timeout << 3000
   [%] %Cache
      [.] .ttl << 60

[f] $exists =? #Boolean.True
   [>] o>error << !UserExists"Email already registered"

// Hash password
[r] $password_hash << |Bcrypt.Hash
   <password << $password
   [%] %Await

// Create user
[r] $user << #User
   [.] .id << |UUID.New""
   [.] .email
   [.] .password_hash
   [.] .age << 0

// Save to database
[r] |Database.SaveUser
   <user << $user
   [%] %Await
   [%] %Transaction
   [%] %OnError
      [.] .rollback << #Boolean.True
      [.] .log << #Boolean.True

[>] o>user << $user:User
   [%] %Redact                         // Don't include password_hash in response

// Tests
[%] %Test
   [.] .name << "Register valid user"
   [.] .input
      [.] .email << "test@example.com"
      [.] .password << "SecurePass123!"
   [.] .expect
      [.] .success << #Boolean.True

[%] %Test
   [.] .name << "Reject invalid email"
   [.] .input
      [.] .email << "invalid-email"
      [.] .password << "SecurePass123!"
   [.] .expect
      [.] .error << !InvalidInput
{x}
```

**This single pipeline demonstrates:**
- Type constraints
- Immutability
- Async/await
- Caching
- Transactions
- Security (sensitive data)
- Audit logging
- Code generation (Serialize/Deserialize)
- Built-in tests
- Error handling
- Timeouts

**All through metadata - zero syntax bloat!**

---

## Implementation Strategy

### Phase 1: Foundation
1. %Constraint (validation)
2. %Immutable (safety)
3. %Test (testing)

### Phase 2: Performance
4. %Async / %Await
5. %Parallel
6. %Cache

### Phase 3: Generation
7. %Derive
8. %Table (DB mapping)

### Phase 4: Advanced
9. %Ownership
10. %Resource management

---

**Status:** 💡 Ready for discussion - Which metadata features resonate most?
