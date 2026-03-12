# Migration Guide: v0.0.4 → v0.0.5

**Target Audience:** Developers migrating existing Polyglot code
**Breaking Changes:** Yes - syntax changes required
**Estimated Effort:** Medium (depends on codebase size)
**Last Updated:** 2026-01-02

---

## Overview

v0.0.5 introduces significant syntax changes focused on:
- Improved clarity and consistency
- Better type safety
- Reduced ambiguity
- Schema-based configuration

**Backward Compatibility:** ❌ No - v0.0.5 code will not run on v0.0.4 runtime

---

## Breaking Changes Summary

| Category | v0.0.4 | v0.0.5 | Impact |
|----------|--------|--------|--------|
| Field naming | `total-customers` | `total_customers` | High |
| DateTime type | `:datetime` | `:dt` | Medium |
| Reserved enum prefix | `#` (mixed) | `-` (clear) | High |
| Comments | `//`, `/* */` | `%%`, `%{ }%` | Medium |
| I/O markers | `(|)` | ` | ` (space-wrapped) | Medium |
| Join marker | `[v]` | `[*]` | Low |
| Collection literals | N/A | `( )`, `{ }`, `{:}` | New feature |
| Code blocks | `[+]` only | `[c]` recommended | Low |
| DB wrapper inputs | Individual params | Schema-based | Medium |

---

## Step-by-Step Migration

### Step 1: Update Field Names (CRITICAL)

**Issue:** Dashes in field names create ambiguity with reserved enums.

#### Find and Replace

```bash
# Find all field definitions with dashes
grep -rn '\[.\] \.\w\+-\w\+:' src/

# Example matches:
# .total-customers:uint
# .sent-at:dt
# .error-count:int
```

#### Replacement Pattern

```diff
- [.] .total-customers:uint
+ [.] .total_customers:uint

- [.] .sent-at:dt
+ [.] .sent_at:dt

- [.] .created-at:datetime
+ [.] .created_at:dt
```

#### Automated Migration Script

```bash
#!/bin/bash
# migrate-field-names.sh

find src -name "*.pg" | while read file; do
  # Backup original
  cp "$file" "$file.backup"

  # Replace common patterns
  sed -i 's/\.total-customers/.total_customers/g' "$file"
  sed -i 's/\.sent-at/.sent_at/g' "$file"
  sed -i 's/\.created-at/.created_at/g' "$file"
  sed -i 's/\.updated-at/.updated_at/g' "$file"
  sed -i 's/\.error-count/.error_count/g' "$file"

  # Add more patterns as needed
done
```

**Manual Review Required:** Always review automated changes!

---

### Step 2: Update DateTime Type

**Change:** `:datetime` → `:dt`

#### Find and Replace

```bash
# Find all datetime type annotations
grep -rn ':datetime' src/
```

#### Replacement

```diff
- [.] .created_at:datetime
+ [.] .created_at:dt

- [<] <timestamp:datetime
+ [<] <timestamp:dt

- [>] >result:datetime
+ [>] >result:dt
```

#### Script

```bash
sed -i 's/:datetime/:dt/g' *.pg
```

---

### Step 3: Update Reserved Enum References

**Change:** Mixed `#` prefix → Clear `-` prefix for stdlib enums

#### Identify Reserved Enums

v0.0.4 mixed user and reserved enums with `#`:
```polyglot
%% v0.0.4
[r] $now << #DT.Now
[r] $status << #Email.Status.Success  %% User enum
```

v0.0.5 separates them:
```polyglot
%% v0.0.5
[r] $now << -DT-Now
[r] $status << #Email.Status.Success  %% Still user enum
```

#### Common Reserved Enums to Update

```diff
%% DateTime
- #DT.Now
+ -DT-Now

- #DT.Calendar.Hijri.Standard
+ -DT-Calendar-Hijri-Standard

%% Database
- #DB.RDBMS.Posgresql
+ -DB-RDBMS-Posgresql

%% Email
- #Email.Status.Success
+ -Email-Status-Success

%% Boolean
- #Boolean.True
+ -Boolean-True

%% Input
- #Input.None
+ -Input-None

%% Success
- #Success
+ -Success (in error handling)
```

#### User-Defined Enums (No Change)

Keep `#` prefix for your own enums:
```polyglot
{#} #Campaign.Status
[.] .Success
[.] .Failed
{x}

[r] $status << #Campaign.Status.Success  %% Still correct
```

---

### Step 4: Update Comments

**Change:** `//` and `/* */` → `%%` and `%{ }%`

#### Single-Line Comments

```diff
- // This is a comment
+ %% This is a comment

- [r] $value << 42  // Inline comment
+ [r] $value << 42  %% Inline comment
```

#### Multi-Line Comments

```diff
- /* This is a
-    multi-line comment */
+ %{ This is a
+    multi-line comment }%
```

#### Script

```bash
# Single-line comments
sed -i 's|^//|%%|g' *.pg
sed -i 's| //| %%|g' *.pg

# Multi-line (manual review required)
# Replace /* with %{
# Replace */ with }%
```

---

### Step 5: Update I/O Markers

**Change:** `(|)` → ` | ` (space-wrapped)

#### Pipeline I/O

```diff
- (|) <input:string << "value"
- (|) >output:string >> >result
+ | <input:string << "value"
+ | >output:string >> >result
```

#### Full Example

```diff
- [r] |DB.Query
- (|) <db:serial << $dbConn
- (|) <query:string << $sql
- (|) >results:array.serial >> $data
+ [r] |DB.Query
+  |  <db:serial << $dbConn
+  |  <query:string << $sql
+  |  >results:array.serial >> $data
```

**Note:** Add proper indentation (typically 1 space before `|`).

---

### Step 6: Update Pack Marker

**Change:** `[v]` → `[*]`

```diff
- [v] *Into.Array
+ [*] *Into.Array

- [v] *Aggregate.Sum
+ [*] *Aggregate.Sum
```

#### Script

```bash
sed -i 's/\[v\]/\[*\]/g' *.pg
```

---

### Step 7: Add Collection Literals (Optional)

v0.0.5 introduces inline collection syntax (new feature):

#### Arrays

```diff
%% v0.0.4
- [r] $items:array.string << |Array.Empty
+ [r] $items:array.string << ( )

%% v0.0.4
- [r] $items << |Array.From"a,b,c"
+ [r] $items << ( "a", "b", "c" )
```

#### Sets

```diff
- [r] $unique << |Set.From"x,y,z"
+ [r] $unique << { "x", "y", "z" }
```

#### Serials

```diff
- [r] $config:serial << |Serial.Create
+ [r] $config:serial << {:}  %% Empty serial

%% Inline serial with fields
+ [r] $user << {
+ [+]  .name: "John",
+ [+]  .age: 30
+ [+] }
```

---

### Step 8: Update Code Blocks (Recommended)

**Change:** `[+]` line continuation → `[c]` code blocks

#### SQL Queries

```diff
%% v0.0.4
- [r] |DB.Query
- (|) <query:string << |SQL""
- [+] +"SELECT id, name"
- [+] +"FROM customers"
- [+] +"WHERE active = true"

%% v0.0.5
+ [r] |DB.Query
+  |  <query:string << |SQL""
+ [c] SELECT id, name
+ [c] FROM customers
+ [c] WHERE active = true
```

**Note:** `[+]` still works, but `[c]` is cleaner.

---

### Step 9: Update DB Wrapper Configuration

**Major Change:** Individual parameters → Schema-based

#### v0.0.4 Pattern

```polyglot
[w] |W.DB.Postgresql
 |  <db-host:string << #Config.db_host
 |  <db-port:int << #Config.db_port
 |  <db-user:string << #Config.db_user
 |  <db-password:string << #Config.db_password
 |  <db-name:string << #Config.db_name
 |  >db:serial >> $dbConn
```

#### v0.0.5 Pattern

1. **Define schema implementation:**

```polyglot
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
```

2. **Use schema in wrapper:**

```polyglot
[w] |W.DB.Postgresql
 |  <settings-DB-Settings#MyAppDB
 |  >db-DB-Connection >> $dbConn
```

#### Migration Steps

1. Extract DB configuration into enum
2. Implement `-DB-Settings` schema
3. Update wrapper calls
4. Test connections

---

### Step 10: Update Error Handling Output

**Change:** Better structured error outputs

#### v0.0.4 (if used)

```polyglot
[!] !File.Write.Error
   [>] >result << !File.Write.Error
```

#### v0.0.5 (recommended)

```polyglot
[!] !File.Write.Error
   [>] >report
      [.] .status:error << !File.Write.Error
      [.] .message:string << "Failed to write file"
```

**Benefit:** Type-safe, structured error reporting.

---

## Migration Checklist

### Pre-Migration

- [ ] Backup all source code
- [ ] Document current behavior
- [ ] Create test suite
- [ ] Review v0.0.5 documentation

### Syntax Changes

- [ ] Update field names (dashes → underscores)
- [ ] Update datetime type annotations
- [ ] Update reserved enum prefixes
- [ ] Update comments syntax
- [ ] Update I/O markers
- [ ] Update pack marker `[v]` → `[*]`

### New Features (Optional)

- [ ] Add collection literals where appropriate
- [ ] Convert `[+]` to `[c]` for code blocks
- [ ] Implement schema-based DB configuration
- [ ] Add structured error outputs

### Testing

- [ ] Run syntax validator
- [ ] Execute test suite
- [ ] Verify all pipelines compile
- [ ] Test database connections
- [ ] Validate error handling

### Post-Migration

- [ ] Update documentation
- [ ] Train team on new syntax
- [ ] Update CI/CD pipelines
- [ ] Archive v0.0.4 code

---

## Common Migration Issues

### Issue 1: Field Name Conflicts

**Problem:**
```polyglot
{#} #Report
[.] .total_customers:uint        %% Your field
[.] .connection-DB-Connection    %% Reserved enum reference
%% Compiler confused by mixed patterns
{x}
```

**Solution:** Be consistent - use `_` for all your fields, `-` only for reserved references.

### Issue 2: Missing Exhaustive Error Handling

v0.0.5 requires exhaustive error handling:

```diff
  [r] |File.Write
   |  <path << $file
     [!] !File.Write.Error
        [r] $status << "failed"
+
+    [!] !*
+       [r] $status << "success"
```

### Issue 3: Variable Reassignment

v0.0.5 enforces immutability more strictly:

```diff
- [r] $status << "pending"
- [r] $status << "complete"  %% ERROR in v0.0.5!

+ [r] $status <~ "pending"   %% Default
+ [f] $condition
+    [r] $status << "complete"  %% Final override
```

### Issue 4: Trigger I/O Syntax

```diff
%% v0.0.4
- [t] |T.Folder.NewFiles
- (|) <folder << #Config.folder
- (|) >files >> <newFiles

%% v0.0.5
+ [t] |T.Folder.NewFiles
+  |  <folder:path << #Config.folder
+  |  >files:array.path >> <newFiles
```

---

## Automated Migration Tools

### Recommended Toolchain

1. **Syntax Validator:** `polyglot validate --version 0.0.5`
2. **Auto-formatter:** `polyglot fmt --upgrade-syntax`
3. **Linter:** `polyglot lint --strict`

### Custom Migration Script

```bash
#!/bin/bash
# migrate-to-v0.0.5.sh

echo "=== Polyglot v0.0.4 → v0.0.5 Migration ==="

# Backup
echo "Creating backups..."
tar -czf backup-$(date +%Y%m%d).tar.gz src/

# Field names
echo "Updating field names..."
find src -name "*.pg" -exec sed -i 's/\([.]\..*\)-\([a-z_]*\):/\1_\2:/g' {} \;

# DateTime types
echo "Updating datetime types..."
find src -name "*.pg" -exec sed -i 's/:datetime/:dt/g' {} \;

# Comments
echo "Updating comments..."
find src -name "*.pg" -exec sed -i 's|^//|%%|g' {} \;
find src -name "*.pg" -exec sed -i 's| //| %%|g' {} \;

# Pack marker
echo "Updating pack markers..."
find src -name "*.pg" -exec sed -i 's/\[v\]/\[*\]/g' {} \;

# Validate
echo "Validating syntax..."
polyglot validate --version 0.0.5 src/**/*.pg

echo "=== Migration Complete ==="
echo "Please review changes and run tests!"
```

---

## Testing After Migration

### Unit Tests

```bash
polyglot test --all
```

### Integration Tests

```bash
polyglot test --integration
```

### Syntax Validation

```bash
polyglot validate --strict src/**/*.pg
```

### Type Checking

```bash
polyglot check --type-safety src/**/*.pg
```

---

## Rollback Plan

If migration fails:

1. **Restore from backup:**
   ```bash
   tar -xzf backup-YYYYMMDD.tar.gz
   ```

2. **Stay on v0.0.4 runtime**

3. **Investigate issues**

4. **Retry migration with fixes**

---

## Support Resources

- **Documentation:** `/docs/v0.0.5/`
- **Examples:** `/docs/v0.0.5/training-sessions/session-001-2026-01-02.md`
- **Style Guide:** `/docs/v0.0.5/style-guide/`
- **Training Materials:** All 6 examples from Session 001

---

## Frequently Asked Questions

### Q: Can I run v0.0.4 and v0.0.5 code together?

**A:** No. Pick one version for your project. Migration must be complete.

### Q: How long does migration take?

**A:** Depends on codebase size:
- Small (< 1000 lines): 1-2 hours
- Medium (1000-10000 lines): 1-2 days
- Large (10000+ lines): 1 week+

### Q: Can I automate the entire migration?

**A:** Partially. Automated tools handle ~80% of syntax changes. Manual review required for:
- Reserved vs user enum disambiguation
- Schema implementations
- Error handling restructuring

### Q: What if I find a migration issue not covered here?

**A:** Document it and add to this guide. Contribute back to help others!

---

## Version Comparison Chart

| Feature | v0.0.4 | v0.0.5 |
|---------|--------|--------|
| Field separator | `-` (ambiguous) | `_` (clear) |
| Reserved enums | `#` (mixed) | `-` (distinct) |
| DateTime | `:datetime` | `:dt` |
| Comments | `//`, `/* */` | `%%`, `%{ }%` |
| I/O markers | `(|)` | ` | ` |
| Pack marker | `[v]` | `[*]` |
| Collections | Verbose | Literals `( )`, `{ }` |
| Code blocks | `[+]` only | `[c]` recommended |
| DB config | Individual params | Schema-based |
| Type safety | Good | Better |
| Immutability | Enforced | Stricter |

---

**Migration Status:** ✅ Guide Complete
**Estimated Success Rate:** 95%+ with proper testing
**Last Updated:** 2026-01-02

**Good luck with your migration! 🚀**
