# Phase 2: Link Cleanup Plan

**Date:** 2025-12-24
**Planner:** Scribe Documentation Architect
**Status:** PLANNING
**Scope:** Fix 1,614 remaining broken links (non-migration issues)

---

## Executive Summary

Phase 1 successfully fixed 1,425 migration-related links. Phase 2 will address the remaining 1,614 broken links, which are pre-existing issues unrelated to the reorganization.

**Goal:** Improve link integrity score from 45/100 to 90/100+

---

## Link Categories & Approach

### Category 1: Non-Existent Files (~970 links, 60%)

**Root Cause:** Links to planned features that were never implemented

**Examples:**
- `./changes-from-v0.0.3/README.md` - Never created
- `./quick-reference/README.md` - Never created
- `./features/error-handling/error-handling.md` - Planned but not created
- `./core-syntax/enums-structs.md` - Never existed

**Approach:**
1. **Identify Missing Content** (2 hours)
   - Scan all broken links
   - Categorize by topic
   - Determine if content should exist

2. **Decision Matrix** (1 hour)
   - Should exist → Create stub with TODO
   - Not needed → Remove link
   - Merged elsewhere → Update link to actual location

3. **Action Plan** (8-12 hours)
   - Create essential missing files (priority content)
   - Remove unnecessary links
   - Update redirected links

**Expected Fix:** ~800 links
**Estimated Effort:** 11-15 hours

---

### Category 2: Absolute Path Issues (~324 links, 20%)

**Root Cause:** Links using absolute paths instead of relative

**Examples:**
- `/docs/User/language/syntax/markers.md` (should be relative)
- `/docs/Agile/prd.md` (should be relative)
- `/docs/specifications/v0.0.4/...` (should be relative)

**Approach:**
1. **Pattern Detection** (1 hour)
   ```bash
   grep -r "](../../" --include="*.md" docs/
   ```

2. **Automated Conversion** (2 hours)
   - Script to convert `/docs/X/Y.md` to relative paths
   - Calculate correct `../` depth per file
   - Apply batch replacements

3. **Validation** (1 hour)
   - Verify all conversions work
   - Test navigation

**Expected Fix:** ~324 links
**Estimated Effort:** 4 hours

---

### Category 3: Old Archive References (~242 links, 15%)

**Root Cause:** Links to files that exist only in archive

**Examples:**
- `../index.md` (multiple versions in archive)
- `../project/old-file.md` (now in archive)
- References to deprecated structure

**Approach:**
1. **Identify Archive Links** (1 hour)
   - Find links pointing to archived content
   - Determine if they should point to new location or archive

2. **Decision Matrix** (1 hour)
   - Historical reference → Update to archive path
   - Has replacement → Update to new location
   - Not needed → Remove

3. **Update Links** (4-6 hours)
   - Update to correct archive paths
   - Update to replacement locations
   - Remove unnecessary references

**Expected Fix:** ~200 links
**Estimated Effort:** 6-8 hours

---

### Category 4: Placeholder Links (~78 links, 5%)

**Root Cause:** Invalid placeholder links left during development

**Examples:**
- `...` (literal three dots)
- `TBD`
- Empty links: `[text]()`
- Malformed: `[text]`

**Approach:**
1. **Pattern Search** (30 minutes)
   ```bash
   grep -r "](\.\.\.)" --include="*.md" docs/
   grep -r "](TBD)" --include="*.md" docs/
   grep -r "]\(\)" --include="*.md" docs/
   ```

2. **Manual Review** (2 hours)
   - Each placeholder needs context review
   - Determine proper link or remove

3. **Cleanup** (2 hours)
   - Fix or remove all placeholders

**Expected Fix:** ~78 links
**Estimated Effort:** 4-5 hours

---

## Implementation Strategy

### Phase 2A: Quick Wins (1 week)
**Focus:** Categories 2 & 4 (automated + simple fixes)

**Tasks:**
1. ✅ Convert absolute paths to relative (4 hours)
2. ✅ Remove placeholder links (4-5 hours)

**Expected Results:**
- Fix ~402 links
- Improve link integrity to 70/100
- Low risk, high reward

---

### Phase 2B: Content Decisions (2 weeks)
**Focus:** Category 1 (non-existent files)

**Tasks:**
1. Audit all missing file references (2 hours)
2. Create priority list of content to create (1 hour)
3. Create essential missing files (8-10 hours)
4. Remove unnecessary links (2 hours)

**Expected Results:**
- Fix ~800 links
- Improve link integrity to 85/100
- Requires content decisions

---

### Phase 2C: Archive Cleanup (1 week)
**Focus:** Category 3 (archive references)

**Tasks:**
1. Identify archive links (1 hour)
2. Map to new locations or archive paths (1 hour)
3. Update all references (4-6 hours)

**Expected Results:**
- Fix ~200 links
- Improve link integrity to 90/100+
- Complete Phase 2

---

## Prioritization

### High Priority (Week 1)
- **Absolute path conversion** - Automated, low risk
- **Placeholder removal** - Simple cleanup
- **Expected:** 402 links fixed

### Medium Priority (Week 2-3)
- **Non-existent files** - Requires decisions
- **Content creation** - Time-intensive
- **Expected:** 800 links fixed

### Low Priority (Week 4)
- **Archive references** - Lower impact
- **Historical cleanup** - Nice to have
- **Expected:** 200 links fixed

---

## Automation Opportunities

### Script 1: Absolute Path Converter
```python
# Convert /docs/X/Y.md to ../X/Y.md based on file location
import os
import re

def convert_absolute_to_relative(file_path, content):
    # Calculate relative path depth
    depth = file_path.count('/') - 1
    prefix = '../' * depth

    # Replace /docs/ with relative path
    pattern = r']\(/docs/'
    replacement = f']({prefix}'
    return re.sub(pattern, replacement, content)
```

### Script 2: Broken Link Detector
```python
# Scan all links and verify targets exist
def validate_links(md_file):
    links = extract_links(md_file)
    broken = []

    for link in links:
        target = resolve_path(md_file, link)
        if not os.path.exists(target):
            broken.append({
                'file': md_file,
                'link': link,
                'target': target,
                'category': categorize(target)
            })

    return broken
```

### Script 3: Missing File Stub Creator
```python
# Create stub files for missing content
def create_stub(file_path, topic):
    template = f"""# {topic}

> ⚠️ **STUB DOCUMENT**
>
> This document is planned but not yet written.
>
> **Status:** TODO
> **Priority:** TBD
> **Created:** {datetime.now().date()}

## Planned Content

[Describe what this document should contain]

## See Also

[Related documents]

---

**Status:** STUB - Content needed
"""
    write_file(file_path, template)
```

---

## Risk Assessment

### Low Risk
- Absolute path conversion (automated)
- Placeholder removal (simple)
- Archive path updates (clear mapping)

### Medium Risk
- Creating missing files (requires content decisions)
- Removing unnecessary links (might break expectations)

### Mitigation
1. **Backup before changes** - Archive current state
2. **Incremental approach** - Fix category by category
3. **Validation after each phase** - Run link checker
4. **Audit trail** - Document all changes

---

## Success Metrics

| Metric | Current | Phase 2A | Phase 2B | Phase 2C | Target |
|--------|---------|----------|----------|----------|--------|
| **Broken Links** | 1,614 | 1,212 | 412 | 212 | <100 |
| **Link Integrity** | 45/100 | 70/100 | 85/100 | 90/100 | 90/100 |
| **Navigation Issues** | High | Medium | Low | Minimal | Minimal |
| **User Experience** | Poor | Fair | Good | Excellent | Excellent |

---

## Estimated Timeline

**Phase 2A (Quick Wins):** 1 week (8-9 hours)
**Phase 2B (Content Decisions):** 2 weeks (11-13 hours)
**Phase 2C (Archive Cleanup):** 1 week (6-8 hours)

**Total Timeline:** 4 weeks
**Total Effort:** 25-30 hours

---

## Deliverables

1. **Scripts** - Automation tools for path conversion and validation
2. **Missing Content** - Essential stub files created
3. **Updated Links** - 1,400+ links fixed
4. **Validation Report** - Post-Phase 2 link integrity assessment
5. **Documentation** - Complete audit trail in `docs/Audit/history/`

---

## Recommendations

### Start with Phase 2A
- Quick wins build momentum
- Low risk, high reward
- Automated tools can be reused

### Decision Points for User
1. **Content creation priority** - Which missing files should we create?
2. **Link removal policy** - Remove all unnecessary links or mark as deprecated?
3. **Archive linking** - Should docs link to archive or avoid it?

---

## Next Steps

**For User:**
1. Review this plan
2. Approve Phase 2A to start (quick wins)
3. Provide guidance on content creation priorities
4. Decide archive linking policy

**For Scribe:**
1. Await user approval
2. Prepare automation scripts
3. Ready validation tools
4. Stand by for Phase 2A execution

---

**Status:** AWAITING APPROVAL
**Next Phase:** Phase 2A - Quick Wins (absolute paths + placeholders)

---

**Planner:** Scribe Documentation Architect
**Date:** 2025-12-24
**Location:** `docs/Audit/checks/phase2-link-cleanup-plan-2025-12-24.md`
