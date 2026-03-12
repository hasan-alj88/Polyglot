# Standard Library Audit: Serial Outputs

**Purpose:** Track stdlib pipelines and wrappers that currently use `:serial` outputs but should use reserved enum schemas for better type safety.

**Date:** 2026-01-01
**Status:** Living document - updated during training

---

## Wrappers with `:serial` Outputs

### Database Wrappers

**W.DB.Connect**
- Current: `>db:serial`
- Should be: `>db-DB-Connection`
- Schema needs: connection handle, connection string, status

**W.DB.Postgresql**
- Current: `>db:serial`
- Should be: `>db-DB-Connection-Postgresql`
- Schema needs: postgresql-specific connection properties

### Runtime Wrappers

**W.RT.Python3.9**
- Current: `>env-python-env:serial`
- Should be: `>env-Python-Environment`
- Schema needs: interpreter path, venv path, installed packages

**W.RT.Node16**
- Current: `>env-node-env:serial`
- Should be: `>env-Node-Environment`
- Schema needs: node path, npm path, installed packages

### File Wrappers

**W.File.Lock**
- Current: `>lock:serial`
- Should be: `>lock-File-Lock`
- Schema needs: lock file path, lock acquired timestamp, process ID

### HTTP Wrappers

**W.HTTP.Client**
- Current: `>client:serial`
- Should be: `>client-HTTP-Client`
- Schema needs: base URL, timeout, connection pool info

---

## Pipelines with `:serial` Outputs

### Database Pipelines

**DB.Query**
- Current: `>results:array.serial`
- Should be: `>results:array-DB-QueryResult`
- Schema needs: row data, column names, row count

### Template Pipelines

**Template.Render**
- Current: Returns `>output:string` (OK - not serial)
- No change needed

### File Operations

**U.File.List**
- Current: `>files:array.path`
- OK - path is primitive type

**U.File.Text.Read**
- Current: `>content:string`
- OK - string is primitive type

### HTTP Operations

**HTTP.Get**
- Current: `>response:serial`
- Should be: `>response-HTTP-Response`
- Schema needs: status code, headers, body, url

**HTTP.Post**
- Current: `>response:serial`
- Should be: `>response-HTTP-Response`
- Same schema as HTTP.Get

---

## Recommended Reserved Enum Schemas

### -DB-Connection
```yaml
name: -DB-Connection
type: schema
fields:
  - connection-string:string
  - database-type:string
  - connected-at:datetime
  - connection-id:uint
```

### -HTTP-Response
```yaml
name: -HTTP-Response
type: schema
fields:
  - status-code:uint
  - headers:serial
  - body:string
  - url:string
  - response-time-ms:uint
```

### -File-Lock
```yaml
name: -File-Lock
type: schema
fields:
  - file-path:path
  - locked-at:datetime
  - process-id:uint
  - lock-type:string  # exclusive, shared
```

### -Python-Environment
```yaml
name: -Python-Environment
type: schema
fields:
  - python-version:string
  - venv-path:path
  - interpreter-path:path
  - installed-packages:array.string
```

### -DB-QueryResult
```yaml
name: -DB-QueryResult
type: schema
fields:
  - columns:array.string
  - rows:array.serial
  - row-count:uint
  - query-time-ms:uint
```

---

## Migration Strategy

### Phase 1: Define Reserved Schemas
1. Create YAML definitions for common types
2. Add to stdlib reserved-enums.yaml
3. Document schema fields

### Phase 2: Update Wrappers
1. Change wrapper signatures
2. Update wrapper implementations
3. Add type validation

### Phase 3: Update Pipelines
1. Change pipeline signatures
2. Update implementations
3. Migrate existing code

### Phase 4: Deprecation
1. Mark old `:serial` signatures as deprecated
2. Provide migration guide
3. Remove in next major version

---

## Benefits of Typed Schemas

### Compile-Time Validation
```polyglot
%% With :serial - no validation
[r] |W.DB.Connect
 |  >db:serial >> $db

[r] $host << $db.hostname  %% Typo! Should be .host
%% Runtime error - field not found

%% With schema - compile-time check
[r] |W.DB.Connect
 |  >db-DB-Connection >> $db

[r] $host << $db.hostname  %% Compile error - field doesn't exist
                            %% Should be .connection-string
```

### Auto-completion Support
IDEs can provide field suggestions based on schema.

### Documentation
Schema serves as self-documentation of expected structure.

### Refactoring Safety
Renaming fields caught at compile time.

---

## Action Items

- [ ] Define all common reserved enum schemas in YAML
- [ ] Update stdlib documentation with schema types
- [ ] Create migration guide for existing code
- [ ] Update wrapper implementations
- [ ] Update pipeline implementations
- [ ] Add compile-time schema validation to parser
- [ ] Create comprehensive test suite for schemas

---

**Note:** This audit should be updated during training sessions as new stdlib components are discovered.

**Last Updated:** 2026-01-01
