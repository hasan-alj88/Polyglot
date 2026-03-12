# Canonical Enum Example - v0.0.4 Syntax

**Purpose:** Reference implementation for parser architecture documentation
**Date:** 2025-12-31
**Author:** Polly (corrected by user feedback)
**Status:** ✅ Verified canonical example

---

## Complete Database Configuration Enum

This example demonstrates ALL v0.0.4 enum features:
- Dotted field names (`.db.host` not `.db_host`)
- Enum value assignments (`#DB.RDBMS.Posgresql`)
- Multiple serial load blocks
- Nested subfields (2 levels deep)
- Dotted nested paths (`.connections.min`)
- Error handler
- Usage in complete pipeline with trigger, wrappers, forks

---

## Enum Definition

```polyglot
{#} #App.Config;DatabaseConfig
[A] #AppConfig

[s] |YAML.Load"\\FileDir\\config\\database.yaml"
   [.] .db.RDBMS:pg.string << #DB.RDBMS.Posgresql
   [.] .db.host:pg.string << .connection.host
   [.] .db.port:pg.int << .connection.port
   [.] .db.credentials:pg.serial << .connection.credentials
      [.] .username:pg.string << .username
      [.] .password:pg.string << .password
   [.] .db.pool:pg.serial << .connection.pool
      [.] .connections.min:pg.int << .min_connections
      [.] .connections.max:pg.int << .max_connections
      [.] .timeout_seconds:pg.int << .timeout_seconds

[s] |YAML.Load"\\FileDir\\config\\cache.yaml"
   [.] .cache.enabled:pg.bool << .cache.enabled
   [.] .cache.ttl_seconds:pg.int << .cache.ttl

[s] |YAML.Load"\\FileDir\\config\\logging.yaml"
   [.] .logging.level:pg.string << .logging.level
   [.] .logging.format:pg.string << .logging.format

[s][!] !*
{x}
```

---

## Usage in Pipeline

```polyglot
{|} StartApp
[t] |T.Cli

[|] !Input.None
[|] >error <~ !Success

[r] $config :pg.serial << #AppConfig

[w] |W.DB.Connect"{$config.db.host}:{$config.db.port}"
   [r] $db :pg.serial << ~output

   [f] $db.RDBMS ?= #DB.RDBMS.Posgresql
      [r] $pg_version :pg.string << |PG.Version"{$db}"
      [|]

   [f] $db.RDBMS ?= #DB.RDBMS.MySQL
      [r] $mysql_version :pg.string << |MySQL.Version"{$db}"
      [|]

   [f] *?
      [!] !DBError -> !ConfigError
      [|] >error

   [|]

[w][!]
   [!] !DBConnectionError -> !StartupError
   [|] >error

[|] >output
{x}
```

---

## Parser Architecture Requirements

### Field Accessor Parsing

The parser must handle dotted field names in enum definitions:

```rust
fn parse_field_mapping(&mut self) -> Result<FieldMapping, ParserError> {
    // [.] .db.host:pg.string << .connection.host
    self.expect(TokenKind::MarkerSubfield)?;  // [.]

    // Parse dotted field name: .db.host
    let field_name = self.parse_dotted_field_name()?;

    self.expect(TokenKind::Colon)?;
    let field_type = self.parse_type()?;

    self.expect(TokenKind::OpPullFinal)?;  // <<

    // Parse RHS - could be field accessor OR enum constant
    let source = if self.check(TokenKind::Hash) {
        // Enum constant: #DB.RDBMS.Posgresql
        self.parse_enum_constant()?
    } else {
        // Field accessor: .connection.host
        self.parse_field_accessor()?
    };

    // Check for nested subfields
    let mut subfields = Vec::new();
    if self.check(TokenKind::Indent) {
        self.advance();
        while !self.check(TokenKind::Dedent) {
            subfields.push(self.parse_field_mapping()?);  // Recursive
        }
        if self.check(TokenKind::Dedent) {
            self.advance();
        }
    }

    Ok(FieldMapping {
        field_name,
        field_type,
        source,
        subfields,
        span: self.span(),
    })
}

fn parse_dotted_field_name(&mut self) -> Result<DottedFieldName, ParserError> {
    // .db.host.port -> ["db", "host", "port"]
    let mut parts = Vec::new();

    self.expect(TokenKind::Dot)?;
    parts.push(self.expect_identifier()?);

    while self.check(TokenKind::Dot) && self.peek_ahead_is_identifier() {
        self.advance();  // consume dot
        parts.push(self.expect_identifier()?);
    }

    Ok(DottedFieldName { parts, span: self.span() })
}

fn parse_enum_constant(&mut self) -> Result<EnumConstant, ParserError> {
    // #DB.RDBMS.Posgresql
    self.expect(TokenKind::Hash)?;

    let enum_type = self.expect_identifier()?;
    self.expect(TokenKind::Dot)?;
    let enum_name = self.expect_identifier()?;
    self.expect(TokenKind::Dot)?;
    let value = self.expect_identifier()?;

    Ok(EnumConstant {
        enum_type,
        enum_name,
        value,
        span: self.span(),
    })
}
```

---

## AST Representation

```rust
pub struct FieldMapping {
    pub field_name: DottedFieldName,
    pub field_type: TypeAnnotation,
    pub source: FieldSource,
    pub subfields: Vec<FieldMapping>,  // Recursive nesting
    pub span: Span,
}

pub struct DottedFieldName {
    pub parts: Vec<String>,  // ["db", "host"] for .db.host
    pub span: Span,
}

pub enum FieldSource {
    FieldAccessor(Vec<String>),  // .connection.host
    EnumConstant(EnumConstant),  // #DB.RDBMS.Posgresql
}

pub struct EnumConstant {
    pub enum_type: String,  // "DB"
    pub enum_name: String,  // "RDBMS"
    pub value: String,      // "Posgresql"
    pub span: Span,
}
```

---

## YAML Source File Example

**`config/database.yaml`:**
```yaml
connection:
  host: "localhost"
  port: 5432
  credentials:
    username: "app_user"
    password: "secure_password"
  pool:
    min_connections: 5
    max_connections: 20
    timeout_seconds: 30
```

**`config/cache.yaml`:**
```yaml
cache:
  enabled: true
  ttl: 3600
```

**`config/logging.yaml`:**
```yaml
logging:
  level: "INFO"
  format: "json"
```

---

## Field Accessor Patterns

### Simple Field Accessor
```polyglot
[.] .db.host:pg.string << .connection.host
```
Maps `connection.host` from YAML to `.db.host` in enum.

### Enum Constant Assignment
```polyglot
[.] .db.RDBMS:pg.string << #DB.RDBMS.Posgresql
```
Assigns enum constant value (not from YAML).

### Nested Serial Field
```polyglot
[.] .db.credentials:pg.serial << .connection.credentials
   [.] .username:pg.string << .username
   [.] .password:pg.string << .password
```
Maps nested YAML object to nested serial fields.

### Dotted Nested Paths
```polyglot
[.] .db.pool:pg.serial << .connection.pool
   [.] .connections.min:pg.int << .min_connections
   [.] .connections.max:pg.int << .max_connections
```
Uses dotted notation (`.connections.min`) within nested structure.

---

## Indentation Rules

1. **Field mappings under `[s]`** - Must be indented (typically 3 spaces)
2. **Nested subfields** - Additional indentation level (typically 6 spaces)
3. **Error handler body** - Indented under `[s][!]` if custom handler
4. **Consistency** - All indentation at same level must align

---

## Common Errors (From Initial Example)

### ❌ Using Underscores Instead of Dots
```polyglot
[.] .db_host:pg.string << .connection.host  # WRONG
[.] .db.host:pg.string << .connection.host  # CORRECT
```

### ❌ Missing Enum Constant Pattern
```polyglot
# Missing this critical pattern:
[.] .db.RDBMS:pg.string << #DB.RDBMS.Posgresql
```

### ❌ Incomplete Nested Structures
```polyglot
# Only showing credentials, missing pool settings:
[.] .db.credentials:pg.serial << .connection.credentials
   [.] .username:pg.string << .username
   [.] .password:pg.string << .password
# Should also include:
[.] .db.pool:pg.serial << .connection.pool
   [.] .connections.min:pg.int << .min_connections
   # etc.
```

---

## Related Documentation

### User Documentation
- [Enum Definitions Guide](../../../User/language/types/enum-definitions.md) - Complete user guide with examples
- [Enums Syntax Guide](../../../User/language/types/enums.md) - Basic enum usage
- [Error Handling Basics](../../../User/language/error-handling/basics.md) - Error patterns

### Technical Documentation
- [Parser Architecture - Enum Indentation Parsing](../../../Agile/architecture/parser-architecture-v0.0.4.md#enum-block-indentation-parsing) - Implementation algorithm
- [EBNF Grammar - Serial Load Blocks](../../../User/reference/grammar.md#serial-load-blocks) - Formal grammar rules
- [Enum Syntax Migration Guide](./enum-syntax-migration-v0.0.3-to-v0.0.4.md) - v0.0.3 → v0.0.4 migration
- [v0.0.4 Parsing Rules Assessment](./v0.0.4-parsing-rules-assessment.md) - Documentation completeness analysis

---

**Generated by:** Polly (Polyglot Language Expert)
**Reviewed by:** User (corrected initial errors)
**Date:** 2025-12-31
**Status:** ✅ Canonical reference for v0.0.4 documentation
