# Migration Guide: v0.0.3 → v0.0.4 Enum Syntax

**Author:** Polly (Polyglot Language Expert)
**Date:** 2025-12-30
**Target:** Parser validation tests in `polyglot-parser/src/validation.rs`
**Status:** Implementation Guide

---

## Overview of Changes

This guide documents the complete enum syntax migration from v0.0.3 to v0.0.4, focusing on updating failing parser validation tests.

### Block Markers
```diff
- [#] #EnumName          (v0.0.3)
+ {#} #EnumName          (v0.0.4)

- [X]                    (v0.0.3)
+ {x}                    (v0.0.4)
```

### Field Declaration
```diff
- [<] .field:type << value     (v0.0.3 - field with default)
+ [.] .field:type << .source   (v0.0.4 - field mapping)
```

### Serial Field Input
```diff
- [~][s] <~ .field:type        (v0.0.3 - field input marker)
+ [s] |Format.Load"path"       (v0.0.4 - serial load)
+    [.] .field:type << .path  (v0.0.4 - indented field mapping)
```

---

## Test-by-Test Migration

### Test 1: `test_validate_serial_enum_with_default_handler_passes`

**v0.0.3 Syntax (Current - FAILING):**
```polyglot
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #Config
[<] .timeout: pg.int << 30
[<] .api_key: pg.string << "default"
[~][s] <~ .timeout: pg.int
[~][s] <~ .api_key: pg.string
[s] |YAML.Load"config.yaml"
[s][!] *
[X]
```

**v0.0.4 Syntax (Corrected):**
```polyglot
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;Default
[A] #Config

[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .timeout:pg.int << .timeout
   [.] .api_key:pg.string << .api_key

[s][!] !*
{x}
```

**Key Changes:**
1. ✅ `[#]` → `{#}`, `[X]` → `{x}`
2. ✅ Removed `[<]` default value declarations
3. ✅ Removed `[~][s]` markers
4. ✅ Indented `[.]` field mappings under `[s]`
5. ✅ Added extension name `;Default` and alias `[A] #Config`
6. ✅ Used `\\FileDir\\` path syntax

---

### Test 2: `test_validate_serial_enum_with_custom_handler_passes`

**v0.0.3 Syntax (Current - FAILING):**
```polyglot
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #Secrets
[<] .api_key: pg.string << ""
[<] .jwt_secret: pg.string << ""
[~][s] <~ .api_key: pg.string
[~][s] <~ .jwt_secret: pg.string
[s] |YAML.Load".env.secrets"
[s][!]
[r] .error_count: pg.int << 1
[X]
```

**v0.0.4 Syntax (Corrected):**
```polyglot
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Secrets;EnvSecrets
[A] #Secrets

[s] |YAML.Load"\\FileDir\\.env.secrets"
   [.] .api_key:pg.string << .api_key
   [.] .jwt_secret:pg.string << .jwt_secret

[s][!]
   [r] $error_count :pg.int << 1
{x}
```

**Key Changes:**
1. ✅ Block markers updated
2. ✅ Removed default values (`[<]`)
3. ✅ Indented field mappings
4. ✅ Custom error handler body indented under `[s][!]`
5. ✅ Changed `.error_count` to `$error_count` (variable, not field)

---

### Test 3: `test_validate_serial_enum_missing_handler_fails`

**v0.0.3 Syntax (Current - FAILING):**
```polyglot
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #Config
[<] .timeout: pg.int << 30
[<] .api_key: pg.string << "default"
[~][s] <~ .timeout: pg.int
[~][s] <~ .api_key: pg.string
[s] |YAML.Load"config.yaml"
[X]
```

**v0.0.4 Syntax (Corrected - Should Still Fail):**
```polyglot
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;MissingHandler
[A] #Config

[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .timeout:pg.int << .timeout
   [.] .api_key:pg.string << .api_key

# Missing [s][!] handler - intentionally omitted for test
{x}
```

**Expected:** Should still fail validation (missing `[s][!]`)

---

### Test 4: `test_validate_serial_enum_multiple_serial_blocks_with_handler_passes`

**v0.0.3 Syntax (Current - FAILING):**
```polyglot
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #SystemConfig
[<] .database_url: pg.string << "postgres://localhost"
[<] .cache_url: pg.string << "redis://localhost"
[<] .queue_url: pg.string << "amqp://localhost"
[~][s] <~ *
[s] |YAML.Load"database.yaml"
[s] |YAML.Load"cache.yaml"
[s] |YAML.Load"queue.yaml"
[s][!] *
[X]
```

**v0.0.4 Syntax (Corrected):**
```polyglot
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #SystemConfig;MultiSource
[A] #SystemConfig

[s] |YAML.Load"\\FileDir\\database.yaml"
   [.] .database_url:pg.string << .url

[s] |YAML.Load"\\FileDir\\cache.yaml"
   [.] .cache_url:pg.string << .url

[s] |YAML.Load"\\FileDir\\queue.yaml"
   [.] .queue_url:pg.string << .url

[s][!] !*
{x}
```

**Key Changes:**
1. ✅ Multiple `[s]` blocks, each with indented field mappings
2. ✅ Each file maps to different field
3. ✅ Single `[s][!] !*` handler for all three loads
4. ✅ Removed `[~][s] <~ *` (v0.0.3 wildcard input)

---

### Test 5: `test_validate_serial_enum_multiple_serial_blocks_missing_handler_fails`

**v0.0.3 Syntax (Current - FAILING):**
```polyglot
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #Config
[<] .db: pg.string << "postgres://localhost"
[<] .cache: pg.string << "redis://localhost"
[~][s] <~ *
[s] |YAML.Load"database.yaml"
[s] |YAML.Load"cache.yaml"
[X]
```

**v0.0.4 Syntax (Corrected - Should Still Fail):**
```polyglot
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;MultiMissingHandler
[A] #Config

[s] |YAML.Load"\\FileDir\\database.yaml"
   [.] .db:pg.string << .connection

[s] |YAML.Load"\\FileDir\\cache.yaml"
   [.] .cache:pg.string << .connection

# Missing [s][!] handler - intentionally omitted for test
{x}
```

**Expected:** Should still fail validation (missing `[s][!]`)

---

### Test 6: `test_validate_serial_multiple_enumerations_mixed`

**v0.0.3 Syntax (Current - FAILING):**
```polyglot
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #Config
[<] .timeout: pg.int << 30
[~][s] <~ .timeout: pg.int
[s] |YAML.Load"config.yaml"
[s][!] *
[X]

[#] #Secrets
[<] .api_key: pg.string << ""
[~][s] <~ .api_key: pg.string
[s] |YAML.Load"secrets.yaml"
[s][!] *
[X]
```

**v0.0.4 Syntax (Corrected):**
```polyglot
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;AppConfig
[A] #Config

[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .timeout:pg.int << .timeout

[s][!] !*
{x}

{#} #Secrets;AppSecrets
[A] #Secrets

[s] |YAML.Load"\\FileDir\\secrets.yaml"
   [.] .api_key:pg.string << .api_key

[s][!] !*
{x}
```

---

## Migration Checklist

### Step 1: Update Block Markers
- [ ] Replace `[#]` with `{#}`
- [ ] Replace `[X]` with `{x}`
- [ ] Replace `[@]` with `{@}` (if using package blocks)

### Step 2: Update Enum Declaration
- [ ] Add extension name: `{#} #BaseType;ExtensionName`
- [ ] Add alias: `[A] #AliasName`

### Step 3: Remove v0.0.3 Field Declarations
- [ ] Remove all `[<] .field:type << default_value` lines

### Step 4: Remove v0.0.3 Field Input Markers
- [ ] Remove all `[~][s] <~ .field:type` lines
- [ ] Remove wildcard `[~][s] <~ *` lines

### Step 5: Update Serial Load Blocks
- [ ] Ensure `[s] |Format.Load"path"` uses `\\FileDir\\` syntax
- [ ] Indent field mappings under `[s]`:
  ```polyglot
  [s] |YAML.Load"\\FileDir\\config.yaml"
     [.] .field:pg.type << .source.path
     [.] .nested:pg.serial << .source.nested
        [.] .subfield:pg.string << .value
  ```

### Step 6: Update Error Handlers
- [ ] Change `[s][!] *` to `[s][!] !*`
- [ ] If custom handler, indent body under `[s][!]`

### Step 7: Update Path Syntax
- [ ] Replace relative paths with `\\FileDir\\` prefix
- [ ] Use double backslashes: `\\FileDir\\config.yaml`

---

## Code Pattern Summary

**Complete v0.0.4 Pattern:**
```polyglot
{@} @Registry:Package:Version
{x}

{#} #BaseType;ExtensionName
[A] #AliasName

[s] |Format.Load"\\FileDir\\file.ext"
   [.] .field1:pg.type << .source.path1
   [.] .field2:pg.type << .source.path2
   [.] .nested:pg.serial << .source.object
      [.] .subfield1:pg.type << .sub1
      [.] .subfield2:pg.type << .sub2

[s][!] !*
{x}
```

---

## Indentation Rules

### Serial Load Field Mapping
- Field mappings (`[.]`) must be **indented** under their parent `[s]` block
- Subfields must be **indented** under their parent field
- Use consistent indentation (typically 3 spaces to align with content after marker)

**Example:**
```polyglot
[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .database:pg.serial << .database      # ← 3 spaces
      [.] .host:pg.string << .host           # ← 6 spaces (nested)
      [.] .port:pg.int << .port              # ← 6 spaces (nested)
   [.] .api_key:pg.string << .api_key        # ← 3 spaces
```

### Custom Error Handlers
- Error handler body must be **indented** under `[s][!]`

**Example:**
```polyglot
[s][!]
   [r] $error_count :pg.int << 1             # ← Indented under [s][!]
   [r] $error_msg :pg.string << "Load failed"
```

---

## Implementation Status

**Files Affected:**
- ⚠️ `polyglot-parser/src/validation.rs` (tests use v0.0.3 syntax)
- ✅ `docs/User/language/types/enum-definitions.md` (v0.0.4 reference)
- ✅ `docs/User/language/advanced/serial-load-block.md` (v0.0.4 reference)

**Parser Implementation:**
- ✅ Lexer supports `{#}`, `{x}`, `{@}` markers (Story 13.1 complete)
- ⚠️ Parser rejects indentation in enum blocks (Story 13.2 in progress)
- ⚠️ Validation tests use v0.0.3 syntax

**Next Steps:**
1. Update all 6 failing validation tests to v0.0.4 syntax
2. Verify parser correctly handles indented field mappings in enum blocks
3. Update parser architecture documentation with complete indentation rules

---

## Related Documentation

### User Documentation
- [Enum Definitions Guide](../../../User/language/types/enum-definitions.md) - Complete v0.0.4 user guide
- [Enums Syntax Guide](../../../User/language/types/enums.md) - Basic enum usage patterns
- [Error Handling Basics](../../../User/language/error-handling/basics.md) - Error handling patterns

### Technical Documentation
- [Parser Architecture - Enum Indentation Parsing](../../../Agile/architecture/parser-architecture-v0.0.4.md#enum-block-indentation-parsing) - Parser implementation algorithm
- [EBNF Grammar - Serial Load Blocks](../../../User/reference/grammar.md#serial-load-blocks) - Formal grammar specification
- [Canonical Enum Example](./canonical-enum-example-v0.0.4.md) - Reference implementation with all features
- [v0.0.4 Parsing Rules Assessment](./v0.0.4-parsing-rules-assessment.md) - Documentation completeness analysis

---

**Generated by:** Polly (Polyglot Language Expert)
**Confidence:** 🟢 Confident (based on v0.0.4 documentation)
**Last Updated:** 2025-12-30
