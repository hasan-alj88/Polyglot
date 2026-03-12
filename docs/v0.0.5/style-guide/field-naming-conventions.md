# Field Naming Conventions - v0.0.5

**Version:** 0.0.5
**Status:** Official Style Guide
**Last Updated:** 2026-01-02

---

## TL;DR

- **Compound field names:** Use `underscores` (e.g., `.total_customers`)
- **Reserved enum references:** Use `dashes` (e.g., `.status-Campaign-Status`)
- **Simple names:** lowercase (e.g., `.name`, `.email`, `.host`)
- **Avoid ambiguity:** Never mix patterns

---

## The Ambiguity Problem

### Issue

Dashes (`-`) in field names create visual ambiguity with reserved enum syntax:

```polyglot
[.] .total-customers:uint
```

**Question:** Is this:
1. A field named `total-customers` (compound word)?
2. A reference to reserved enum `-customers`?

Without context, it's impossible to tell.

### Solution

Use **different separators** for different purposes:
- **Underscores (`_`)** for compound field names
- **Dashes (`-`)** for reserved enum references

---

## Rules

### Rule 1: Compound Field Names Use Underscores

✅ **Correct:**
```polyglot
{#} #Campaign.Report
[.] .total_customers:uint
[.] .emails_sent:uint
[.] .emails_failed:uint
[.] .sent_at:dt
[.] .processing_time:uint
{x}
```

❌ **Incorrect (ambiguous):**
```polyglot
{#} #Campaign.Report
[.] .total-customers:uint      %% Looks like -customers enum!
[.] .emails-sent:uint          %% Looks like -sent enum!
[.] .sent-at:dt                %% Looks like -at enum!
{x}
```

### Rule 2: Reserved Enum References Use Dashes

✅ **Correct:**
```polyglot
{#} #Database.Config
[.] .connection-DB-Connection << $dbConn
[.] .status-Connection-Status << -Connection-Status-Active
{x}
```

❌ **Incorrect:**
```polyglot
{#} #Database.Config
[.] .connection_DB_Connection << $dbConn  %% Wrong separator
{x}
```

### Rule 3: Simple Names Are Lowercase

✅ **Correct:**
```polyglot
{#} #User
[.] .name:string
[.] .email:string
[.] .age:uint
[.] .active:bool
{x}
```

❌ **Avoid:**
```polyglot
{#} #User
[.] .Name:string      %% PascalCase - reserved for types
[.] .Email:string     %% PascalCase - reserved for types
[.] .userName:string  %% camelCase - not Polyglot style
{x}
```

### Rule 4: Nested Fields Follow Same Rules

✅ **Correct:**
```polyglot
{#} #Database.Config
[s] << |TOML.Load"config.toml"
   [.] .connection:serial << .database
      [.] .host:string << .host
      [.] .port:uint << .port
      [.] .max_connections:uint << .pool.max
      [.] .idle_timeout:uint << .pool.timeout
{x}
```

---

## Pattern Reference

### Pattern: Simple Field

**Use:** Single-word field names
**Format:** `lowercase`

```polyglot
[.] .name:string
[.] .email:string
[.] .port:uint
[.] .active:bool
```

### Pattern: Compound Field

**Use:** Multi-word field names
**Format:** `lowercase_with_underscores`

```polyglot
[.] .first_name:string
[.] .created_at:dt
[.] .max_retries:uint
[.] .is_verified:bool
[.] .total_count:uint
```

### Pattern: Reserved Enum Reference

**Use:** Field typed as reserved enum
**Format:** `field-ReservedType`

```polyglot
[.] .connection-DB-Connection << $dbConn
[.] .status-Email-Status << -Email-Status-Success
[.] .calendar-Calendar << -DT-Calendar-Hijri-Standard
```

### Pattern: User Enum Reference

**Use:** Field typed as user enum
**Format:** `field#UserType`

```polyglot
[.] .config#App.Config
[.] .status#Campaign.Status
[.] .report#Processing.Report
```

---

## Common Naming Patterns

### Timestamps

✅ **Recommended:**
```polyglot
[.] .created_at:dt
[.] .updated_at:dt
[.] .deleted_at:dt
[.] .processed_at:dt
[.] .sent_at:dt
```

❌ **Avoid:**
```polyglot
[.] .creation_time:dt      %% Inconsistent with _at pattern
[.] .update_timestamp:dt   %% Too verbose
```

### Counts and Totals

✅ **Recommended:**
```polyglot
[.] .total_count:uint
[.] .error_count:uint
[.] .success_count:uint
[.] .retry_count:uint
```

✅ **Also acceptable:**
```polyglot
[.] .total_customers:uint
[.] .active_users:uint
[.] .failed_attempts:uint
```

### Boolean Fields

✅ **Recommended:**
```polyglot
[.] .is_active:bool
[.] .is_verified:bool
[.] .has_errors:bool
[.] .can_retry:bool
[.] .should_notify:bool
```

❌ **Avoid:**
```polyglot
[.] .active:bool           %% Ambiguous - could be status
[.] .verified:bool         %% Better as is_verified
[.] .enabled:bool          %% Ambiguous
```

### Configuration Fields

✅ **Recommended:**
```polyglot
[.] .max_connections:uint
[.] .min_pool_size:uint
[.] .connection_timeout:uint
[.] .retry_interval:uint
[.] .batch_size:uint
```

### Status and State Fields

✅ **Recommended:**
```polyglot
%% User enum
[.] .status#Campaign.Status
[.] .state#Workflow.State

%% Reserved enum
[.] .connection_status-DB-Status
[.] .email_status-Email-Status
```

---

## Anti-Patterns

### ❌ Anti-Pattern 1: Mixed Separators

```polyglot
{#} #MixedConfig
[.] .db_host:string          %% Underscore
[.] .db-port:uint            %% Dash - INCONSISTENT!
[.] .dbPassword:string       %% camelCase - WRONG!
{x}
```

**Fix:**
```polyglot
{#} #ConsistentConfig
[.] .db_host:string
[.] .db_port:uint
[.] .db_password:string
{x}
```

### ❌ Anti-Pattern 2: Abbreviations

```polyglot
{#} #AbbreviatedFields
[.] .usr_nm:string           %% Too abbreviated
[.] .pwd:string              %% Unclear
[.] .cnt:uint                %% What count?
{x}
```

**Fix:**
```polyglot
{#} #ClearFields
[.] .user_name:string
[.] .password:string
[.] .total_count:uint
{x}
```

### ❌ Anti-Pattern 3: Redundant Prefixes

```polyglot
{#} #User.Profile
[.] .user_name:string        %% Redundant - already in User context
[.] .user_email:string       %% Redundant
[.] .user_age:uint           %% Redundant
{x}
```

**Fix:**
```polyglot
{#} #User.Profile
[.] .name:string
[.] .email:string
[.] .age:uint
{x}
```

### ❌ Anti-Pattern 4: Type Suffixes (Usually)

```polyglot
{#} #Config
[.] .host_string:string      %% Type already in annotation
[.] .port_number:uint        %% Redundant
[.] .created_datetime:dt     %% Redundant
{x}
```

**Fix:**
```polyglot
{#} #Config
[.] .host:string
[.] .port:uint
[.] .created_at:dt
{x}
```

**Exception:** When disambiguation needed:
```polyglot
{#} #Report
[.] .user_count:uint         %% Number of users
[.] .user_names:array.string %% List of names - suffix OK
{x}
```

---

## Special Cases

### Database Column Mapping

When mapping from databases with different conventions:

**SQL (snake_case):**
```sql
SELECT user_id, first_name, created_at FROM users
```

**Polyglot:**
```polyglot
{#} #User.Record
[s] << |DB.Query
   [.] .user_id:uint << .user_id
   [.] .first_name:string << .first_name
   [.] .created_at:dt << .created_at
{x}
```

**Keep the same convention** to avoid confusion during mapping.

### API Response Mapping

When mapping from external APIs:

**JSON (camelCase):**
```json
{
  "userId": 123,
  "firstName": "John",
  "createdAt": "2024-01-01"
}
```

**Polyglot (convert to snake_case):**
```polyglot
{#} #User.DTO
[s] << |JSON.Parse
   [.] .user_id:uint << .userId
   [.] .first_name:string << .firstName
   [.] .created_at:dt << .createdAt
{x}
```

**Rationale:** Internal consistency more important than matching external format.

---

## Migration from v0.0.4

### Breaking Changes

v0.0.4 allowed dashes in field names:
```polyglot
%% v0.0.4
[.] .total-customers:uint
[.] .sent-at:dt
```

v0.0.5 requires underscores:
```polyglot
%% v0.0.5
[.] .total_customers:uint
[.] .sent_at:dt
```

### Migration Steps

1. Search for field definitions: `\[.\] \.\w+-\w+`
2. Replace dashes with underscores
3. Verify no reserved enum conflicts
4. Update all references

**Example:**
```bash
# Find all problematic fields
grep -r '\[.\] \.\w\+-\w\+' src/

# Manual review and fix
```

---

## Tooling Support

### Linter Rules

Recommended linter configuration:

```yaml
field_naming:
  compound_separator: underscore
  simple_case: lowercase
  reserved_enum_reference: dash_prefix
  error_on_dash_in_compound: true
  error_on_camelCase: true
  error_on_PascalCase: true
```

### IDE Suggestions

Configure IDE to suggest:
- `total_customers` not `total-customers`
- `created_at` not `createdAt`
- `is_active` not `isActive`

---

## Summary Table

| Use Case | Format | Example |
|----------|--------|---------|
| Simple field | `lowercase` | `.name`, `.email` |
| Compound field | `lowercase_underscore` | `.created_at`, `.total_count` |
| Reserved enum ref | `field-Reserved-Type` | `.status-Email-Status` |
| User enum ref | `field#User.Type` | `.config#App.Config` |
| Boolean field | `is_*`, `has_*`, `can_*` | `.is_active`, `.has_errors` |
| Timestamp | `*_at` | `.created_at`, `.updated_at` |
| Count | `*_count`, `total_*` | `.error_count`, `.total_users` |

---

## References

- [Variable Lifecycle](../language/variable-lifecycle.md)
- [Error Handling](../language/error-handling.md)
- [Reserved Enums](../stdlib/reserved-enums.yaml)
- [Training Session 001](../training-sessions/session-001-2026-01-02.md)

---

**Status:** ✅ Official Style Guide
**Enforcement:** Recommended (linter warnings)
**Version:** 0.0.5
**Last Updated:** 2026-01-02
