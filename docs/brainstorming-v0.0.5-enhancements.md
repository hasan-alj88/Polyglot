# v0.0.5 Enhancement Proposals

**Date:** 2026-01-01
**Status:** Brainstorming - Ideas from training sessions
**Purpose:** Track proposed enhancements discovered during v0.0.5 development

---

## 1. Code Block Marker `[c]` with Variable Substitution

### Current: Line Continuation with `[+]`

```polyglot
[r] |DB.Query
 |  <db-DB-Connection << $dbConn
 |  <query:string << |SQL""
[+] +"SELECT id, name, email, language, segment"
[+] +"FROM customers"
[+] +"WHERE segment IN ({$CampaignConfig.customer_segments})"
[+] +"AND email_verified = true"
 |  >results:array#Customer.Record >> $customers
```

**Issues:**
- Verbose - every line needs `[+]` marker and `+` prefix
- String concatenation syntax `+"..."` is repetitive
- Hard to read multi-line SQL/code

### Proposed: Code Block Marker `[c]`

```polyglot
[r] |DB.Query
 |  <db-DB-Connection << $dbConn
 |  <query:string << |SQL""
[c] SELECT id, name, email, language, segment
[c] FROM customers
[c] WHERE segment IN ({$CampaignConfig.customer_segments})
[c] AND email_verified = true
 |  >results:array#Customer.Record >> $customers
```

**Benefits:**
- Cleaner syntax - no `+"..."` wrapping
- Natural multi-line code appearance
- Variable substitution: `{$variable}` works seamlessly
- Indentation preserved from column 1 after `[c]`

### Use Cases

**SQL Queries:**
```polyglot
[r] |DB.Query
 |  <query:string << |SQL""
[c] SELECT
[c]   users.id,
[c]   users.name,
[c]   orders.total
[c] FROM users
[c] INNER JOIN orders ON orders.user_id = users.id
[c] WHERE users.created_at > '{$start_date}'
[c] ORDER BY orders.total DESC
[c] LIMIT {$limit}
```

**Shell Commands:**
```polyglot
[r] |Shell.Execute
 |  <command:string << |Bash""
[c] #!/bin/bash
[c] export APP_ENV={$environment}
[c] cd {$project_dir}
[c] npm install
[c] npm run build
[c] pm2 restart {$app_name}
```

**Templates:**
```polyglot
[r] |Template.RenderString
 |  <template:string << ""
[c] <html>
[c]   <body>
[c]     <h1>Welcome {$user_name}!</h1>
[c]     <p>Your order #{$order_id} has been confirmed.</p>
[c]   </body>
[c] </html>
```

### Implementation Notes

1. **Indentation:** Content preserved from position 1 space after `[c]`
2. **Substitution:** `{$variable}` and `{$enum.field}` evaluated
3. **Escaping:** `\{` for literal braces
4. **Termination:** Code block ends at next non-`[c]` marker
5. **String Building:** Lines joined with newlines automatically

### Grammar

```ebnf
code_block ::= code_line+
code_line ::= INDENT "[c]" " " code_content NEWLINE
code_content ::= (TEXT | substitution)*
substitution ::= "{" variable_ref "}"
```

---

## 2. Reserved Enum Schema for Database Settings

### Current: Individual DB Parameters

```polyglot
[w] |W.DB.Postgresql
 |  <db-host:string << #CampaignConfig.db_host
 |  <db-port:int << #CampaignConfig.db_port
 |  <db-user:string << #CampaignConfig.db_user
 |  <db-password:string << #CampaignConfig.db_password
 |  <db-name:string << #CampaignConfig.db_name
```

**Issues:**
- Verbose - 5+ parameters
- No compile-time validation of required fields
- Easy to forget parameters

### Proposed: `-DB-Settings` Reserved Schema

**Schema Definition (stdlib):**
```yaml
name: -DB-Settings
type: schema
required_fields:
  - host:string
  - port:uint
  - username:string
  - password:string
  - database:string
optional_fields:
  - ssl_enabled:bool
  - connection_timeout:uint
  - pool_size:uint
```

**Usage:**
```polyglot
%% Define app-specific DB config implementing schema
{#} #MyApp.Database-DB-Settings
[A] #MyAppDB

[s] << |TOML.Load"\\FileDir\\config\database.toml"
   [.] .host:string << .connection.host
   [.] .port:uint << .connection.port
   [.] .username:string << .credentials.user
   [.] .password:string << .credentials.pass
   [.] .database:string << .database_name

[s][!] !*
{x}

%% Use in wrapper - shorthand syntax
[w] |W.DB.Postgresql
 |  <settings-DB-Settings#MyAppDB
 |  >db-DB-Connection >> $dbConn
```

**Shorthand Syntax:**
```polyglot
<settings-DB-Settings#MyAppDB
```

Is equivalent to:
```polyglot
<settings-DB-Settings << -DB-Settings#MyAppDB
```

All field values come from `#MyAppDB` enum definition.

**Benefits:**
- Type safety - compiler validates schema implementation
- Less verbose - single parameter
- Self-documenting - schema defines requirements
- Reusable - same enum for multiple connections

### Other Reserved Schemas to Define

**-HTTP-Client-Settings:**
```yaml
required_fields:
  - base_url:string
  - timeout_ms:uint
optional_fields:
  - retry_count:uint
  - headers:serial
```

**-Email-Settings:**
```yaml
required_fields:
  - smtp_host:string
  - smtp_port:uint
  - from_address:string
optional_fields:
  - use_tls:bool
  - auth_user:string
  - auth_password:string
```

**-Cache-Settings:**
```yaml
required_fields:
  - host:string
  - port:uint
optional_fields:
  - ttl_seconds:uint
  - max_memory_mb:uint
```

---

## 3. Field Naming Convention Clarification

### Issue: Ambiguity with Dashes

```polyglot
[.] .total-customers:uint
```

Could be interpreted as:
- Field name `total-customers` (compound word)
- Reference to reserved enum `-customers`

### Proposed Convention

**Use underscores for compound field names:**
```polyglot
[.] .total_customers:uint
[.] .emails_sent:uint
[.] .created_at:dt
[.] .user_name:string
```

**Reserve dashes for reserved enum references:**
```polyglot
[.] .status-Status << -Campaign-Status-Success
[.] .db-DB-Connection << $dbConn
```

**Grammar Rule:**
- `field_name` → underscore for compound words (user-defined)
- `field-ReservedType` → dash before reserved enum type
- `-ReservedEnum-Value` → dashes for reserved enum paths

---

## 4. Exhaustive Conditional Checking

### Current Behavior

Compiler requires exhaustive handling like switch statements:

```polyglot
[f] $value ?= 1
   [r] $result << "one"

[f] $value ?= 2
   [r] $result << "two"

%% COMPILE ERROR: Missing [f] *? catch-all
```

### Proposed Enhancement: Exhaustive Enum Matching

For enum-typed variables, compiler validates exhaustiveness:

```polyglot
[r] $status#Campaign.Status <~ #Campaign.Status.Success

[f] $status ?= #Campaign.Status.Success
   [r] $message << "All succeeded"

[f] $status ?= #Campaign.Status.PartialFailure
   [r] $message << "Some failed"

[f] $status ?= #Campaign.Status.Failed
   [r] $message << "All failed"

%% Compiler validates: all enum values covered, no [f] *? needed
```

**Benefits:**
- Safer than `[f] *?` catch-all
- Refactoring-safe - adding enum value causes compile error
- Self-documenting - explicitly handles each case

---

## Implementation Priority

1. **High Priority:**
   - Reserved DB schema `-DB-Settings` (needed for wrappers)
   - Field naming convention documentation (avoid ambiguity)

2. **Medium Priority:**
   - Code block marker `[c]` (improves readability significantly)
   - Exhaustive enum matching (safety improvement)

3. **Future:**
   - Additional reserved schemas (HTTP, Email, Cache)
   - IDE integration for schemas

---

## Related Documentation

- [Variable Lifecycle](./v0.0.5/language/variable-lifecycle.md)
- [Error Handling](./v0.0.5/language/error-handling.md)
- [Reserved Enums](./v0.0.5/stdlib/reserved-enums.yaml)
- [Stdlib Audit](./v0.0.5/stdlib/stdlib-audit-serial-outputs.md)

---

**Contributors:** Claude (Polly) during training sessions
**Last Updated:** 2026-01-01
