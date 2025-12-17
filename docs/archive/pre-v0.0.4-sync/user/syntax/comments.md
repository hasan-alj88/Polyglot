---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/comments.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Comments

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Single-Line Comments

Use `//` for single-line comments:

```polyglot
// This is a comment
[r] .x:pg.int << 42              // Inline comment
```

---

## Multi-Line Comments

Use `/* */` for multi-line comments:

```polyglot
/*
 * This is a multi-line comment
 * spanning several lines
 */
[|] Pipeline
```

---

## Comment Style (PFG v1.0

### Block Comments

```polyglot
// This pipeline processes user data
// and returns validation results
[|] ValidateUser
```

### Inline Comments

Separated by at least 2 spaces:

```polyglot
[r] .timeout:pg.int << 30        // Default timeout in seconds
```

### Complete Sentences

Comments should be complete sentences with proper capitalization:

```polyglot
// Process the input file and generate report.
[|] ProcessFile
```

---

## Documentation Comments

Use comments above definitions for auto-doc generation:

```polyglot
// Processes user registration data.
// Validates email, password strength, and user age.
// Returns UserProfile on success or ValidationError on failure.
[|] RegisterUser
[i] .email:pg.string
[i] .password:pg.string
[i] .age:pg.int
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .profile: #UserProfile
[X]
```

---

## Comment Placement

```polyglot
// File-level comment
[@] @Local::MyProject:1.0.0.0
[X]


// Pipeline-level comment
[|] ProcessData
[i] .input:pg.string             // Input-level comment
[t] |T.Call
[W] |W.Polyglot.Scope

// Execution block comment
[r] .result:pg.string << ""

[o] .result:pg.string
[X]
```

---

**End of Syntax Reference**

**See Also:**
- [Syntax Overview](overview.md
- [Advanced Features](/docs/user/advanced/
- [Examples](/docs/user/examples/
