# Old Code Examples - Archive Manifest

**Archived:** 2025-11-25
**Reason:** Non-compliant with v0.0.2 syntax - removed for replacement

---

## Files Archived (6 total)

### Polyglot Code Files (.pg)
1. **variable-states-examples.pg** (7.1K)
   - 9 violations: `#Variables.States` → should be `#PgVar.States`
   
2. **variable-states-advanced-examples.pg** (21K)
   - 46 violations: `#Variables.States` → should be `#PgVar.States`
   - 1 violation: `pg\map` usage (removed in v0.0.2)
   
3. **test-file-processing-pipeline.pg** (14K)
   - 12 violations: `#Variables.States` → should be `#PgVar.States`
   - 1 violation: Missing `|` prefix on trigger (`T.Cron` → `|T.Cron`)
   
4. **test-state-aware-code.pg** (6.7K)
   - 8 violations: `#Variables.States` → should be `#PgVar.States`
   - 1 violation: Missing `|` prefix on trigger

### Related Documentation
5. **test-file-processing-SUMMARY.md**
   - Summary documentation for test-file-processing-pipeline.pg
   
6. **test-state-aware-code-BREAKDOWN.md**
   - Breakdown documentation for test-state-aware-code.pg

---

## Total Violations Removed

- **75+ syntax errors** across 4 .pg files
- All files used old/invalid v0.0.2 syntax
- Replacement files will use correct syntax

---

## Status

✅ **Archived** - Files preserved for reference
🚫 **Do not use** - Syntax is invalid for v0.0.2
📝 **Replacement needed** - New compliant code to be created

See `SYNTAX-COMPLIANCE-AUDIT-2025-11-25.md` for full details.
