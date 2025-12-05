# Configuration Best Practices - Use `[#]` Not `[i]`

**Date:** 2025-12-03
**Type:** Best Practice / Architecture Guideline
**Status:** ✅ **COMPLETE**
**Scope:** Distinguish between pipeline inputs (`[i]`) and configuration (`[#]`)

---

## Summary

Established critical architectural guideline: **Application configuration and settings should use `[#]` enumerations with `[s]` serial file loading, NOT `[i]` input defaults.**

`[i]` defaults are for **pipeline input parameters** with reasonable defaults, not for application configuration.

---

## The Distinction

| Purpose | Use | Example |
|---------|-----|---------|
| **Pipeline Input Parameters** | `[i]` with `<~` | Batch size, retry flags, optional behaviors |
| **Application Configuration** | `[#]` with `[s]` | API keys, database URLs, timeouts, settings |

---

## ❌ WRONG: Using `[i]` for Configuration

```polyglot
// ❌ BAD: Don't use [i] for application configuration
[|] |MyPipeline
[i] .max_retries: pg\int <~ 3        // ❌ This is configuration!
[i] .timeout_ms: pg\int <~ 5000      // ❌ This is configuration!
[i] .cache_enabled: pg\bool <~ #Boolean.True  // ❌ This is configuration!
[i] .log_level: pg\string <~ "INFO"  // ❌ This is configuration!
[i] .api_key: pg\string              // ❌ Sensitive data as input!
[t] |T.Call
```

**Problems:**
1. ❌ Configuration mixed with pipeline logic
2. ❌ No centralized configuration management
3. ❌ Can't load configuration from secure files
4. ❌ Hard to manage sensitive data (API keys, credentials)
5. ❌ Every pipeline needs same configuration inputs
6. ❌ Changes require updating every pipeline call

---

## ✅ CORRECT: Use `[#]` Enumeration for Configuration

```polyglot
// ✅ GOOD: Centralized configuration with [#]
[#] #Config
[<] .max_retries: pg\int <~ 3
[<] .timeout_ms: pg\int <~ 5000
[<] .cache_enabled: pg\bool <~ #Boolean.True
[<] .log_level: pg\string <~ "INFO"
[<] .api_key: pg\string           // Sensitive - loaded from file
[<] .database_url: pg\url         // Sensitive - loaded from file
[s] "config.yaml"                 // Load from serial file
[s][!] *                          // Default error handling
[X]
```

**Benefits:**
1. ✅ Centralized configuration in one place
2. ✅ Load from files using `[s]` serial loader
3. ✅ Secure handling of sensitive data (not in code)
4. ✅ Type-safe with defaults
5. ✅ Easy to manage across environments (dev/staging/prod)
6. ✅ Separated from pipeline logic

---

## ✅ CORRECT: Use `[i]` for Pipeline Input Parameters

**`[i]` defaults are for pipeline-specific input parameters:**

```polyglot
[|] |ProcessBatch
[i] .items: pg\array              // Required input
[i] .batch_size: pg\int <~ 100    // Optional: default batch size
[i] .parallel: pg\bool <~ #Boolean.False  // Optional: parallel processing
[t] |T.Call

// Process items in batches
// Uses .batch_size and .parallel parameters
[o] .results: pg\array
[X]
```

**Use cases for `[i]` defaults:**
- Optional processing modes
- Default batch/chunk sizes
- Retry behavior flags
- Filtering criteria
- Sorting parameters

**Key distinction:** These are **per-call parameters**, not global configuration.

---

## Configuration with `[s]` Serial File Loader

### Basic Configuration Loading

```polyglot
[#] #AppConfig
[<] .port: pg\uint <~ 8080
[<] .host: pg\string <~ "localhost"
[<] .debug_mode: pg\bool <~ #Boolean.False
[s] "app-config.yaml"             // Load values from YAML
[s][!] *                          // Default error handling
[X]
```

**File: `app-config.yaml`**
```yaml
port: 3000
host: "0.0.0.0"
debug_mode: true
```

**Result:** Enumeration fields override defaults with file values.

---

### Sensitive Data Configuration

```polyglot
[#] #Secrets
[<] .api_key: pg\string
[<] .database_password: pg\string
[<] .jwt_secret: pg\string
[s] ".env.secrets"                // Load from secure file (not in git)
[s][!] *                          // Default error handling
[X]
```

**File: `.env.secrets`** (gitignored)
```yaml
api_key: "sk_live_abc123..."
database_password: "super_secure_password"
jwt_secret: "jwt_signing_secret"
```

**Benefits:**
- ✅ Secrets not in code
- ✅ Different secrets per environment
- ✅ File can be encrypted at rest
- ✅ Easy rotation without code changes

---

### Multi-Environment Configuration

**Development:**
```polyglot
[#] #Config
[<] .api_url: pg\url
[<] .log_level: pg\string <~ "DEBUG"
[s] "config.dev.yaml"
[s][!] *
[X]
```

**Production:**
```polyglot
[#] #Config
[<] .api_url: pg\url
[<] .log_level: pg\string <~ "ERROR"
[s] "config.prod.yaml"
[s][!] *
[X]
```

---

## Comparison: `[i]` vs `[#]` for Defaults

### Pipeline Input Parameter (`[i]`)

```polyglot
[|] |FetchData
[i] .url: pg\string               // Required
[i] .retry: pg\bool <~ #Boolean.True  // Optional parameter
[t] |T.Call

// Caller can override
[r] |FetchData
[<] <url: pg\string << "https://api.example.com"
[<] <retry: pg\bool << #Boolean.False  // Override default
```

**Characteristics:**
- Per-call parameter
- Caller can override
- Logic-specific
- Not global configuration

---

### Application Configuration (`[#]`)

```polyglot
[#] #Config
[<] .default_retry: pg\bool <~ #Boolean.True
[s] "config.yaml"
[s][!] *
[X]

[|] |FetchData
[i] .url: pg\string
[t] |T.Call

// Use global config
[r] .retry: pg\bool << #Config.default_retry

[r] |FetchData
[<] <url: pg\string << "https://api.example.com"
```

**Characteristics:**
- Global setting
- Loaded from file
- Environment-specific
- Centralized management

---

## Best Practices Summary

### Use `[i]` Defaults For:
- ✅ Optional pipeline parameters
- ✅ Per-call behavior flags
- ✅ Processing options (batch size, parallel mode)
- ✅ Filtering/sorting criteria
- ✅ Caller-specific overrides

### Use `[#]` with `[s]` For:
- ✅ Application configuration
- ✅ Environment settings (dev/prod)
- ✅ Sensitive data (API keys, passwords)
- ✅ Connection strings
- ✅ Timeouts and limits
- ✅ Feature flags
- ✅ Logging configuration

---

## Real-World Example

### ❌ BEFORE (Poor Architecture)

```polyglot
// Every pipeline has configuration as inputs
[|] |FetchUser
[i] .id: pg\int
[i] .api_url: pg\url              // ❌ Configuration
[i] .api_key: pg\string           // ❌ Sensitive data!
[i] .timeout: pg\int <~ 30        // ❌ Configuration
[t] |T.Call

// Every caller must provide configuration
[r] |FetchUser
[<] <id: pg\int << 123
[<] <api_url: pg\url << "https://api.example.com"
[<] <api_key: pg\string << "sk_live_abc123"  // Hardcoded secret!
[<] <timeout: pg\int << 30
```

**Problems:**
- Repeated configuration in every call
- Hardcoded secrets in code
- No centralized management
- Hard to change across environments

---

### ✅ AFTER (Good Architecture)

**Configuration:**
```polyglot
[#] #Config
[<] .api_url: pg\url
[<] .api_key: pg\string
[<] .timeout: pg\int <~ 30
[s] "config.yaml"                 // Load from file
[s][!] *                          // Default error handling
[X]
```

**Pipeline:**
```polyglot
[|] |FetchUser
[i] .id: pg\int                   // Only business logic input
[t] |T.Call

// Use centralized config
[r] |HttpGet
[<] <url: pg\string << U.String.Concat"{#Config.api_url, '/users/', U.Int.ToString"{.id}"}"
[<] <headers: pg\dict << {"Authorization": #Config.api_key}
[<] <timeout: pg\int << #Config.timeout
[>] >user: pg\dict >> .result

[o] .result: pg\dict
[X]
```

**Usage:**
```polyglot
// Clean calls - no configuration clutter
[r] |FetchUser
[<] <id: pg\int << 123
[>] >result: pg\dict >> .user
```

**Benefits:**
- ✅ Clean separation of concerns
- ✅ Secrets in files, not code
- ✅ Easy environment management
- ✅ DRY (Don't Repeat Yourself)

---

## Files Updated

1. **`docs/user/variable-state-system.md`**
   - Replaced "Configuration Defaults" section
   - Added proper `[i]` use cases (pipeline parameters)
   - Added warning about using `[#]` for configuration
   - Provided complete `[#]` with `[s]` example

---

## Migration Guide

### If You Have This:
```polyglot
[|] |MyPipeline
[i] .max_retries: pg\int <~ 3
[i] .timeout: pg\int <~ 30
```

### Refactor To:

**1. Create configuration enumeration:**
```polyglot
[#] #Config
[<] .max_retries: pg\int <~ 3
[<] .timeout: pg\int <~ 30
[s] "config.yaml"
[s][!] *
[X]
```

**2. Update pipeline to use config:**
```polyglot
[|] |MyPipeline
[i] .input: pg\string             // Keep actual inputs

// Use config internally
[r] .retries: pg\int << #Config.max_retries
[r] .timeout: pg\int << #Config.timeout
```

**3. Update callers (simpler now):**
```polyglot
// Before: Had to pass config
[r] |MyPipeline
[<] <input: pg\string << .data
[<] <max_retries: pg\int << 3    // No longer needed!
[<] <timeout: pg\int << 30       // No longer needed!

// After: Just pass business data
[r] |MyPipeline
[<] <input: pg\string << .data
```

---

## Conclusion

**Status:** ✅ **COMPLETE**

Successfully established architectural guideline distinguishing pipeline inputs from configuration:

**Key Guidelines:**
- Use `[i]` for pipeline input parameters (per-call, optional)
- Use `[#]` with `[s]` for application configuration (global, file-based)
- Keep sensitive data in files using `[s]` loader
- Centralize configuration, don't repeat in every pipeline

**Benefits:**
- ✅ Better security (secrets not in code)
- ✅ Easier environment management
- ✅ Cleaner pipeline signatures
- ✅ DRY principle
- ✅ Separation of concerns

This architectural pattern is especially critical for applications with sensitive data, multiple environments, or frequent configuration changes.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Type:** Architecture Best Practice - Configuration Management
