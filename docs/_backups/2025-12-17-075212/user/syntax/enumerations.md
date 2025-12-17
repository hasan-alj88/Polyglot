# Polyglot Enumerations

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Enumeration Definition

Use `[#]` marker with `#` prefix:

```polyglot
[#] #UserRole
[<] .Admin                         // Enum field (no type
[<] .User                          // Enum field (no type
[<] .Guest                         // Enum field (no type
[X]
```

---

## Serial Fields vs Enum Fields

**Cannot mix as siblings!**

### ✗ INVALID (Siblings

```polyglot
[#] #BadMixing
[<] .EnumField                     // Enum (no type
[<] <serial_field:pg.int         // Serial (has type - ERROR!
[X]
```

### ✓ VALID (Uncles - Different Hierarchy

```polyglot
[#] #GoodMixing
[<] .category.Option1              // Enum nested under .category
[<] .category.Option2              // Enum nested under .category
[<] <metadata:pg.string          // Serial at top level (uncle
[X]
```

---

## Extendable Enumerations

Use `.*` pattern for user-extendable enums:

```polyglot
[#] #DT.Hijri.*                    // Extendable reserved enum
[<] .SaudiArabia                   // Built-in
[<] .UKMoonSighting                // Built-in
// Users can add: .MyCustomProfile
```

---

## Alias Definitions

Create shortcuts with `[A]`:

```polyglot
[#] #Boolean.True
[A] True                           // Creates #True alias
[<] Boolean.False
[A] False                          // Creates #False alias
[X]

// Usage:
.flag:pg.bool << #True
.flag:pg.bool << #Boolean.True   // Same thing
```

---

## Reserved Enumerations

### `#Boolean`

```polyglot
#Boolean.True
#Boolean.False
```

### `#PgVar.States`

```polyglot
#PgVar.States.Declared
#PgVar.States.DefaultReady
#PgVar.States.Pending
#PgVar.States.Ready
#PgVar.States.Faulted
```

### `!No.Input`

Used when pipeline has no inputs (error marker, not enumeration:

```polyglot
[i] !No.Input
```

**Note:** This is an error marker (`!`, not an enumeration marker (`#`.

---

## Loading Enumeration Values from Files

Use `[s]` blocks to load enumeration field values from serial files (YAML, JSON, TOML. Multiple files load **in parallel** with shared error handling.

**⚠️ Safety Mechanism:** Must declare error handling with `[s][!] *` (default or `[s][!]` (custom.

### Basic Configuration Loading

```polyglot
[#] #Config
[<] .api_key:pg.string
[<] .timeout:pg.int <~ 30           // Default value
[<] .max_retries:pg.int <~ 3        // Default value
[s] "config.yaml"                    // Load values from file
[s][!] *                             // Default error handling (required
[X]
```

**File: `config.yaml`**
```yaml
api_key: "sk_live_abc123..."
timeout: 5000                        # Overrides default 30
max_retries: 5                       # Overrides default 3
```

### Multiple Files (Parallel Loading

```polyglot
[#] #AppConfig
[<] .database_url:pg.url
[<] .cache_url:pg.url
[<] .api_key:pg.string
[s] "database.yaml"                  // Loads in parallel
[s] "cache.yaml"                     // Loads in parallel
[s] "secrets.yaml"                   // Loads in parallel
[s][!] *                             // Shared error handling for all
[X]
```

All three files load simultaneously. If any file fails, the error handler is triggered.

### Custom Error Handling

```polyglot
[#] #Secrets
[<] .api_key:pg.string
[<] .jwt_secret:pg.string
[s] ".env.secrets"
[s][!]                               // Custom error handler
[r] |U.Log.Error"CRITICAL: Secrets file missing"
[r] |U.Process.Exit"1"
[X]
```

### Optional Files with Defaults

```polyglot
[#] #FeatureFlags
[<] .new_ui:pg.bool <~ #Boolean.False
[<] .beta:pg.bool <~ #Boolean.False
[s] "features.yaml"
[s][!]                               // Continue with defaults if file missing
[r] |U.Log.Warn"Feature flags not found, using defaults"
[X]
```

**See:** [Safety Mechanisms](safety-mechanisms.md#safety-mechanism-4-s---explicit-serial-error-handling

---

## Enum Usage

```polyglot
[?] .role =? #UserRole.Admin
[~][r] |GrantAdminAccess
[~]

[?] .role =? #UserRole.User
[~][r] |GrantUserAccess
[~]

[?] *?
[~][o] !UnauthorizedError
[~]
```

---

**Next:** [Error Handling →](error-handling.md
