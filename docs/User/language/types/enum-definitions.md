# Enum Definitions Guide

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate users
**Prerequisites:** Basic syntax, enums, types

---

## Overview

Enum definition blocks (`{#}...{x}`) allow you to create **custom enumerations** with fields loaded from **serial data files** (YAML, JSON, TOML). This enables configuration-driven enums where field values come from external files rather than being hardcoded.

**Key Concepts:**
- `{#}...{x}` - Enum definition block
- `[A]` - Alias name declaration
- `[s]` - Serial load block
- `[.]` - Field accessor
- `[s][!]` - Scope-wide error handling

---

## Basic Syntax

### Enum Definition Block Structure

```polyglot
{#} #BaseType;CustomName
[A] #AliasName
[s] |Format.Load"path"
   [.] .field_name:type << .source.path
[s][!] !*
{x}
```

**Pattern:**
1. `{#}` starts enum definition block
2. Declare base type and custom extension name
3. `[A]` declares alias for convenient access
4. `[s]` loads serial data file(s)
5. `[.]` maps loaded fields to enum fields
6. `[s][!]` handles errors from ALL `[s]` blocks
7. `{x}` ends the block

---

## Simple Example

### Loading Configuration from YAML

**Config File:** `llm.yaml`
```yaml
api:
  key: "sk-abc123"
  username: "user@example.com"
model:
  name: "gpt-4"
```

**Polyglot Enum Definition:**
```polyglot
{#} #LLM.Config;MyLLM
[A] #MyLLM
[s] |YAML.Load"\\FileDir\\llm.yaml"
   [.] .api_key:pg.string << .api.key
   [.] .username:pg.string << .api.username
   [.] .model:pg.string << .model.name
[s][!] !*
{x}
```

**What happens:**
1. `{#} #LLM.Config;MyLLM` - Creates custom enum extending `#LLM.Config`
2. `[A] #MyLLM` - Creates alias `#MyLLM` for easy reference
3. `[s] |YAML.Load...` - Loads YAML file
4. `[.]` lines map YAML paths to enum fields
5. `[s][!] !*` - Handles any load errors
6. Can now use `#MyLLM.api_key` in code

---

## Enum Definition Block `{#}...{x}`

### Syntax

```polyglot
{#} #BaseType;ExtensionName
```

**Components:**
- `#BaseType` - Reserved enum being extended (e.g., `#LLM.Config`, `#Folders.Rolling`)
- `;` - Separator (semicolon separates base from extension)
- `ExtensionName` - Your custom extension name

**Examples:**
```polyglot
{#} #LLM.Config;MyLLM
{#} #Folders.Rolling;LogSummary
{#} #Database.Config;Production
{#} #App.Settings;Development
```

---

## Alias Declaration `[A]`

### Syntax

```polyglot
[A] #AliasName
```

**Purpose:** Creates a short, convenient alias for the full enum path.

**Example:**
```polyglot
{#} #LLM.Config;MyLLMConfiguration
[A] #MyLLM  // Short alias instead of long name
```

**Usage:**
```polyglot
// Without alias:
[r] $key :pg.string << #LLM.Config;MyLLMConfiguration.api_key

// With alias:
[r] $key :pg.string << #MyLLM.api_key
```

**Benefits:**
- Shorter, more readable code
- Easier to change underlying config
- Conventional naming

---

## Serial Load Blocks `[s]`

### Syntax

```polyglot
[s] |Format.Load"path"
   [.] .field:type << .source.path
```

**Supported Formats:**
- `|YAML.Load` - YAML files
- `|JSON.Load` - JSON files
- `|TOML.Load` - TOML files
- `|XML.Load` - XML files (if supported)

### Basic Example

```polyglot
[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .database:pg.string << .db.connection
   [.] .port:pg.int << .server.port
   [.] .timeout:pg.int << .network.timeout
```

### Multiple Serial Loads

**UNIQUE FEATURE:** All `[s]` blocks in same scope share ONE error handler!

```polyglot
{#} #App.Config;Full
[A] #FullConfig
[s] |YAML.Load"\\FileDir\\database.yaml"
   [.] .db_host:pg.string << .host
   [.] .db_port:pg.int << .port
[s] |JSON.Load"\\FileDir\\features.json"
   [.] .logging:pg.bool << .features.logging
   [.] .cache:pg.bool << .features.cache
[s] |TOML.Load"\\FileDir\\server.toml"
   [.] .workers:pg.int << .server.workers
[s][!] !*  // Handles errors from ALL THREE files
{x}
```

**Behavior:**
- All three files load **in parallel**
- ONE error handler for all
- Simplifies error management
- More efficient

---

## Field Accessors `[.]`

### Syntax

```polyglot
[.] .destination_field:type << .source.field.path
```

**Left Side (Destination):**
- `.destination_field:type` - Field name in your enum
- Must include type annotation

**Right Side (Source):**
- `.source.field.path` - Path in loaded data
- Uses dot notation for nesting

### Field Mapping Examples

**YAML File:** `config.yaml`
```yaml
api:
  credentials:
    key: "abc123"
    secret: "xyz789"
network:
  settings:
    timeout: 30
    retries: 3
features:
  logging:
    enabled: true
    level: "info"
```

**Field Accessors:**
```polyglot
[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .api_key:pg.string << .api.credentials.key
   [.] .api_secret:pg.string << .api.credentials.secret
   [.] .timeout:pg.int << .network.settings.timeout
   [.] .retries:pg.int << .network.settings.retries
   [.] .log_enabled:pg.bool << .features.logging.enabled
   [.] .log_level:pg.string << .features.logging.level
```

**Key Points:**
- Destination uses single dot: `.api_key`
- Source uses dot-separated path: `.api.credentials.key`
- Type must match data type in file
- Deep nesting supported

---

## Scope-Wide Error Handling `[s][!]`

### Syntax

```polyglot
[s][!] !*
```

**CRITICAL:** All `[s]` blocks in the same enum definition share ONE error handler.

### How It Works

```polyglot
{#} #Config;App
[A] #AppConfig
[s] |YAML.Load"\\FileDir\\db.yaml"      // Load 1
   [.] .database:pg.string << .connection
[s] |JSON.Load"\\FileDir\\cache.json"   // Load 2
   [.] .cache_ttl:pg.int << .ttl
[s] |TOML.Load"\\FileDir\\server.toml"  // Load 3
   [.] .port:pg.int << .port
[s][!] !*  // Handles errors from Load 1, 2, AND 3
{x}
```

**Error Handler Scope:**
- Handles ALL `[s]` blocks in this `{#}...{x}` block
- Even parallel loads
- ONE place for error logic

### Default Error Types

`!*` (wildcard) catches:
- `!File.NotFound` - File doesn't exist
- `!YAML.Field.NotFound` - Field missing in YAML
- `!JSON.Parse.Error` - Invalid JSON syntax
- `!TOML.Parse.Error` - Invalid TOML syntax
- `!Permission.Denied` - Cannot read file
- Any other file/parse errors

**Usage:**
```polyglot
[s][!] !*  // Default: catch all errors, use defaults or fail gracefully
```

---

## Complete Example: Application Configuration

### Config Files

**`database.yaml`:**
```yaml
host: "localhost"
port: 5432
username: "admin"
password: "secret123"
database: "production"
```

**`cache.json`:**
```json
{
  "redis": {
    "host": "localhost",
    "port": 6379,
    "ttl": 3600
  }
}
```

**`server.toml`:**
```toml
[server]
workers = 4
timeout = 30

[features]
logging = true
metrics = true
```

### Enum Definition

```polyglot
{@} @Local:Config.Loader:0.0.0.1
{x}


{#} #App.Config;Production
[A] #ProdConfig

[s] |YAML.Load"\\FileDir\\config\\database.yaml"
   [.] .db_host:pg.string << .host
   [.] .db_port:pg.int << .port
   [.] .db_user:pg.string << .username
   [.] .db_pass:pg.string << .password
   [.] .db_name:pg.string << .database

[s] |JSON.Load"\\FileDir\\config\\cache.json"
   [.] .cache_host:pg.string << .redis.host
   [.] .cache_port:pg.int << .redis.port
   [.] .cache_ttl:pg.int << .redis.ttl

[s] |TOML.Load"\\FileDir\\config\\server.toml"
   [.] .workers:pg.int << .server.workers
   [.] .timeout:pg.int << .server.timeout
   [.] .logging:pg.bool << .features.logging
   [.] .metrics:pg.bool << .features.metrics

[s][!] !*
{x}


{|} |StartApplication
[%] %Doc << "Start application with loaded configuration"

[r] |Server.Start
(|) <database_host << #ProdConfig.db_host
(|) <database_port << #ProdConfig.db_port
(|) <cache_host << #ProdConfig.cache_host
(|) <cache_port << #ProdConfig.cache_port
(|) <workers << #ProdConfig.workers
(|) <enable_logging << #ProdConfig.logging
{x}
```

**Data Flow:**
1. Three config files loaded in parallel
2. Fields mapped to enum
3. Single error handler for all loads
4. Enum fields accessed with `#ProdConfig.field_name`
5. Values passed to server startup

---

## Enum Fields in Pipelines

### Using Loaded Configuration

```polyglot
{#} #Database.Config;Dev
[A] #DbConfig
[s] |YAML.Load"\\FileDir\\db.yaml"
   [.] .host:pg.string << .host
   [.] .port:pg.int << .port
   [.] .database:pg.string << .database
[s][!] !*
{x}


{|} |ConnectToDatabase
[r] $connection :pg.string << |DB.Connect
(|) <host << #DbConfig.host
(|) <port << #DbConfig.port
(|) <database << #DbConfig.database
{x}
```

### Conditional Configuration

```polyglot
{#} #Features.Config;Current
[A] #Features
[s] |JSON.Load"\\FileDir\\features.json"
   [.] .api_enabled:pg.bool << .api.enabled
   [.] .cache_enabled:pg.bool << .cache.enabled
[s][!] !*
{x}


{|} |Initialize
[f] #Features.cache_enabled =? #True
   [r] |Cache.Initialize""
{x}

[f] #Features.api_enabled =? #True
   [r] |API.Start""
{x}
{x}
```

---

## Advanced Patterns

### Pattern 1: Environment-Specific Configs

```polyglot
{#} #App.Config;Development
[A] #DevConfig
[s] |YAML.Load"\\FileDir\\config\\dev.yaml"
   [.] .debug:pg.bool << .debug
   [.] .log_level:pg.string << .logging.level
[s][!] !*
{x}

{#} #App.Config;Production
[A] #ProdConfig
[s] |YAML.Load"\\FileDir\\config\\prod.yaml"
   [.] .debug:pg.bool << .debug
   [.] .log_level:pg.string << .logging.level
[s][!] !*
{x}


{|} |GetConfig
[|] >config <~ #App.Config

[f] $environment =? "development"
   [r] #DevConfig >> >config
{x}

[f] $environment =? "production"
   [r] #ProdConfig >> >config
{x}
{x}
```

### Pattern 2: Fallback Configuration

```polyglot
{#} #Config.Defaults;Base
[A] #DefaultConfig
[s] |YAML.Load"\\FileDir\\defaults.yaml"
   [.] .timeout:pg.int << .timeout
   [.] .retries:pg.int << .retries
[s][!] !*
{x}

{#} #Config.Custom;Override
[A] #CustomConfig
[s] |YAML.Load"\\FileDir\\custom.yaml"
   [.] .timeout:pg.int << .timeout
   [.] .retries:pg.int << .retries
[s][!] !*
{x}


{|} |GetTimeout
[|] >timeout <~ :pg.int

[r] $has_custom :pg.bool << |File.Exists"\\FileDir\\custom.yaml"

[f] $has_custom =? #True
   [r] #CustomConfig.timeout >> >timeout
{x}

[f] *?
   [r] #DefaultConfig.timeout >> >timeout
{x}
{x}
```

### Pattern 3: Multi-Format Loading

```polyglot
{#} #Config.All;Multi
[A] #AllConfig
[s] |YAML.Load"\\FileDir\\base.yaml"
   [.] .base_setting:pg.string << .setting
[s] |JSON.Load"\\FileDir\\overrides.json"
   [.] .override:pg.string << .override
[s] |TOML.Load"\\FileDir\\advanced.toml"
   [.] .advanced:pg.int << .advanced.value
[s][!] !*
{x}
```

---

## Best Practices

### ✅ 1. Always Use Aliases

```polyglot
// ✅ GOOD: Short, readable alias
{#} #LLM.Config;MyLLMConfiguration
[A] #MyLLM

// Usage:
[r] $key << #MyLLM.api_key
```

```polyglot
// ❌ AVOID: No alias, long references
{#} #LLM.Config;MyLLMConfiguration

// Usage:
[r] $key << #LLM.Config;MyLLMConfiguration.api_key
```

### ✅ 2. Group Related Fields

```polyglot
// ✅ GOOD: Logical grouping
[s] |YAML.Load"\\FileDir\\config.yaml"
   // Database fields
   [.] .db_host:pg.string << .database.host
   [.] .db_port:pg.int << .database.port

   // Cache fields
   [.] .cache_host:pg.string << .cache.host
   [.] .cache_port:pg.int << .cache.port
```

### ✅ 3. Use Type-Appropriate Formats

```polyglot
// ✅ GOOD: YAML for configs (readable, comments)
[s] |YAML.Load"\\FileDir\\config.yaml"

// ✅ GOOD: JSON for data exchange
[s] |JSON.Load"\\FileDir\\data.json"

// ✅ GOOD: TOML for structured configs
[s] |TOML.Load"\\FileDir\\server.toml"
```

### ✅ 4. Handle Errors at Scope Level

```polyglot
// ✅ GOOD: ONE error handler for all loads
{#} #Config;App
[A] #AppConfig
[s] |YAML.Load"\\FileDir\\file1.yaml"
   [.] .field1:pg.string << .field1
[s] |JSON.Load"\\FileDir\\file2.json"
   [.] .field2:pg.int << .field2
[s][!] !*  // Handles both
{x}
```

---

## Troubleshooting

### Issue 1: Field Not Found in Loaded Data

**Error:** `!YAML.Field.NotFound`

```polyglot
// ❌ WRONG: Field path doesn't match YAML structure
[.] .api_key:pg.string << .api_key

// YAML file has:
// api:
//   key: "value"
```

**Solution:** Match the actual YAML structure:

```polyglot
// ✅ RIGHT: Correct nested path
[.] .api_key:pg.string << .api.key
```

### Issue 2: Type Mismatch

**Error:** Type conversion error

```polyglot
// ❌ WRONG: YAML has number, expecting string
[.] .port:pg.string << .port

// YAML file:
// port: 8080
```

**Solution:** Use correct type:

```polyglot
// ✅ RIGHT: Match data type
[.] .port:pg.int << .port
```

### Issue 3: File Not Found

**Error:** `!File.NotFound`

```polyglot
// ❌ WRONG: Incorrect path
[s] |YAML.Load"config.yaml"  // Missing full path
```

**Solution:** Use full path with `\\Path\\` syntax:

```polyglot
// ✅ RIGHT: Full path literal
[s] |YAML.Load"\\FileDir\\config.yaml"
```

### Issue 4: Missing Error Handler

**Compilation Error:** No error handler for serial loads

```polyglot
// ❌ WRONG: No [s][!] handler
{#} #Config;App
[A] #AppConfig
[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .field:pg.string << .field
{x}  // Missing [s][!] !*
```

**Solution:** Always include error handler:

```polyglot
// ✅ RIGHT: Include error handler
{#} #Config;App
[A] #AppConfig
[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .field:pg.string << .field
[s][!] !*
{x}
```

---

## Quick Reference

```
┌─────────────────────────────────────────────┐
│ ENUM DEFINITIONS                            │
├─────────────────────────────────────────────┤
│                                             │
│  BLOCK STRUCTURE                            │
│  {#} #BaseType;ExtensionName                │
│  [A] #Alias                                 │
│  [s] |Format.Load"path"                     │
│     [.] .field:type << .source.path         │
│  [s][!] !*                                  │
│  {x}                                        │
│                                             │
│  MARKERS                                    │
│  {#}...{x} - Enum definition block          │
│  [A] - Alias declaration                    │
│  [s] - Serial load block                    │
│  [.] - Field accessor                       │
│  [s][!] - Scope-wide error handler          │
│                                             │
│  FORMATS                                    │
│  |YAML.Load - YAML files                    │
│  |JSON.Load - JSON files                    │
│  |TOML.Load - TOML files                    │
│                                             │
│  FIELD MAPPING                              │
│  [.] .dest_field:type << .source.path       │
│                                             │
│  ERROR HANDLING                             │
│  [s][!] !* - Catches all serial errors      │
│  All [s] in scope share ONE handler         │
│                                             │
└─────────────────────────────────────────────┘
```

---

## See Also

### User Documentation
- [Enums Syntax Guide](../types/enums.md) - Basic enum usage
- [Error Handling Basics](../error-handling/basics.md) - Error patterns
- [Type System](../types/type-system.md) - Type annotations
- [I/O Operators](../syntax/io-operators.md) - Path handling and I/O

### Technical Documentation
- [Parser Architecture - Enum Indentation Parsing](../../../Agile/architecture/parser-architecture-v0.0.4.md#enum-block-indentation-parsing) - Parser implementation details
- [EBNF Grammar - Serial Load Blocks](../../reference/grammar.md#serial-load-blocks) - Formal grammar specification
- [Canonical Enum Example](../../../Tech/implementation/parser/canonical-enum-example-v0.0.4.md) - Complete reference implementation
- [v0.0.3 → v0.0.4 Migration Guide](../../../Tech/implementation/parser/enum-syntax-migration-v0.0.3-to-v0.0.4.md) - Syntax migration reference

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-28
**Confidence:** ✅ Verified - All patterns from session-2025-12-27-trigger-io-advanced
