# Enum Syntax Guide (v0.0.4)

**Version:** v0.0.4
**Category:** Language Reference
**Audience:** All users

---

## Overview

Polyglot v0.0.4 introduces simplified enum syntax for reserved types while maintaining extensibility for custom schemas.

**Key change from v0.0.3:**
- ❌ Old: `#;Boolean;True` (semicolon prefix)
- ✅ New: `#Boolean.True` (dot syntax)

---

## Reserved Enum Syntax

### Full Form

```polyglot
#EnumType.Value
```

**Pattern:**
- `#` - Enum prefix
- Dot (`.`) separator - **not semicolon!**
- No prefix semicolon like v0.0.3

**Examples:**
```polyglot
#Boolean.True
#Boolean.False
#Status.Success
#Status.Failure
#DT.Business.Week
```

### Alias Form (Shorthand)

Common reserved enums have short aliases:

```polyglot
#True       // Alias for #Boolean.True
#False      // Alias for #Boolean.False
```

**Full list of aliases:**
- `#True` → `#Boolean.True`
- `#False` → `#Boolean.False`

---

## Custom Schema Extensions

### Syntax

```polyglot
#BaseType;CustomSchema.Field
          └────┬───┘
          Semicolon separates
          base from extension
```

**Pattern:**
- Start with reserved base type
- Semicolon (`;`) separates base from custom extension
- Custom schema follows

**Example:**
```polyglot
#DT.Business.Week;MyCompanyWeek.RestDays
└──────┬────────┘ └──────────┬──────────┘
  Base type           Custom extension
  (reserved)          (your schema)
```

**Breakdown:**
- `#DT.Business.Week` - Reserved base type
- `;` - Separator
- `MyCompanyWeek.RestDays` - Your custom schema

---

## Usage Examples

### Boolean Values

```polyglot
// Full form
[|] <active :pg.bool << #Boolean.True
[|] <enabled :pg.bool << #Boolean.False

// Alias (preferred for brevity)
[|] <active :pg.bool << #True
[|] <enabled :pg.bool << #False
```

### Comparisons

```polyglot
[f] $status =? #True
   [r] // Handle true case
{x}

[f] $status =? #False
   [r] // Handle false case
{x}
```

### Custom Extensions

```polyglot
// Company-specific work schedule
[r] $schedule :pg.serial << #DT.Business.Week;AcmeCorp.Schedule.RestDays

// Custom status codes
[r] $result :pg.serial << #Status;MyApp.CustomError.NetworkTimeout
```

---

## Migration from v0.0.3

### Old Syntax (v0.0.3)

```polyglot
// Reserved enums (semicolon prefix)
#;Boolean;True
#;Boolean;False

// Custom schemas (dot prefix)
#.Custom.Type
```

### New Syntax (v0.0.4)

```polyglot
// Reserved enums (dot syntax, no prefix)
#Boolean.True
#Boolean.False
#True           // Alias
#False          // Alias

// Custom extensions (semicolon separator)
#BaseType;Custom.Extension
```

### Migration Checklist

- [ ] Replace `#;Boolean;True` with `#Boolean.True` or `#True`
- [ ] Replace `#;Boolean;False` with `#Boolean.False` or `#False`
- [ ] Update custom schemas to use semicolon separator
- [ ] Test all enum comparisons
- [ ] Update documentation references

---

## Style Guide

### When to Use Full Form

Use full form in:
- ✅ Type definitions
- ✅ Formal specifications
- ✅ Documentation
- ✅ Public APIs

**Example:**
```polyglot
[|] <input :pg.bool <~ #Boolean.True
```

### When to Use Aliases

Use aliases in:
- ✅ Code examples
- ✅ Inline expressions
- ✅ Quick scripts
- ✅ Tests

**Example:**
```polyglot
[f] $success =? #True
   [r] // ...
{x}
```

---

## Common Patterns

### Default Boolean Parameters

```polyglot
[|] <verbose :pg.bool <~ #False
[|] <enabled :pg.bool <~ #True
```

### Boolean State Checks

```polyglot
[f] $flag =? #True
   [r] // When true
{x}

[f] $flag =? #False
   [r] // When false
{x}

[f] *?
   [r] // Exhaustiveness
{x}
```

### Custom Enum Pattern

```polyglot
// Define base type (reserved)
// #Status.Success
// #Status.Failure

// Extend with custom statuses
[r] $result :pg.serial << #Status;MyApp.PendingApproval
```

---

## Type System Integration

### Boolean Type

```polyglot
:pg.bool

// Values:
#Boolean.True   or   #True
#Boolean.False  or   #False
```

### Serial Type (for complex enums)

```polyglot
:pg.serial

// Custom schemas stored as serial
[r] $data :pg.serial << #MyEnum;CustomSchema.Field
```

---

## Frequently Asked Questions

### Q: Can I create my own aliases?

**A:** No, aliases are language-reserved shortcuts. Use full form for custom enums.

### Q: When should I use semicolon?

**A:** Only when extending a reserved base type with custom schema:
```polyglot
#ReservedBase;YourCustomExtension.Field
```

### Q: Are `#True` and `#Boolean.True` exactly the same?

**A:** Yes! `#True` is an alias that resolves to `#Boolean.True`.

### Q: Can I use v0.0.3 syntax?

**A:** No, v0.0.3 syntax (`#;Boolean;True`) is deprecated and will cause errors.

---

## Complete Examples

### Before (v0.0.3)

```polyglot
[|] <flag :pg.bool << #;Boolean;True

[f] $status =? #;Boolean;False
   [r] // handle
{x}
```

### After (v0.0.4)

```polyglot
[|] <flag :pg.bool << #True

[f] $status =? #False
   [r] // handle
{x}
```

---

## Reserved Enum Types (Partial List)

```polyglot
// Boolean
#Boolean.True
#Boolean.False

// Status (common pattern)
#Status.Success
#Status.Failure
#Status.Pending

// DateTime
#DT.Business.Week
#DT.Calendar.Gregorian

// More to be documented...
```

---

## See Also

- [Type System](./type-system.md) - Complete type reference
- [Operators Reference](./operators-reference.md) - Assignment operators
- [Fork Patterns](./fork-patterns.md) - Using enums in conditions
- [Variable Lifecycle](./variable-lifecycle.md) - State management

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-26
**Migration Note:** This guide reflects v0.0.4 syntax. For v0.0.3 migration, see [Migration Guide](./v0.0.3-to-v0.0.4-migration.md).
