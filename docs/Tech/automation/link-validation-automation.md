# Link Validation Automation

**Status:** ✅ ACTIVE
**Created:** 2025-12-24
**Maintainer:** Scribe Documentation System
**Purpose:** Automated link checking to maintain documentation quality

---

## Overview

Comprehensive automated link validation system to protect documentation integrity and prevent broken links from being introduced.

**Coverage:**
- ✅ Pre-commit hooks (local validation)
- ✅ CI/CD pipeline (GitHub Actions)
- ✅ Weekly comprehensive scans
- ✅ Pull request validation

---

## Components

### 1. Link Validation Script

**File:** `/validate_links.sh`

**Purpose:** Lightweight link validator for hooks and CI/CD

**Modes:**
- `staged` - Check only staged files (pre-commit)
- `changed` - Check changed files (CI/CD on PRs)
- `all` - Full documentation scan (weekly)

**Usage:**
```bash
# Check staged files
./validate_links.sh staged

# Check all changed files
./validate_links.sh changed

# Full documentation scan
./validate_links.sh all
```

**Exit Codes:**
- `0` - Success (all links valid or under threshold)
- `1` - Failure (broken links exceed threshold)

**Threshold:** 10 broken links (configurable)

---

### 2. Pre-Commit Hook

**File:** `.git/hooks/pre-commit`

**Purpose:** Prevent broken links from being committed

**Behavior:**
- Runs automatically before every commit
- Checks only staged markdown files
- Allows commit if under threshold
- Can be bypassed with `--no-verify` (not recommended)

**Setup:**
```bash
# Hook is already installed and active
# Test it by staging a file with a broken link
```

**Bypass (emergency only):**
```bash
git commit --no-verify -m "message"
```

---

### 3. GitHub Actions Workflow

**File:** `.github/workflows/validate-docs.yml`

**Triggers:**
- Pull requests (affecting markdown files)
- Pushes to main/dev branches
- Weekly schedule (Sundays at midnight UTC)
- Manual workflow dispatch

**Jobs:**

#### Job 1: `validate-links` (Fast Check)
- Runs on: PRs, pushes
- Checks: Only changed markdown files
- Time: ~30 seconds
- Action: Comments on PR if failures detected

#### Job 2: `comprehensive-validation` (Full Scan)
- Runs on: Weekly schedule, manual trigger
- Checks: All documentation files
- Time: ~2-3 minutes
- Action: Generates comprehensive report

#### Job 3: `link-health-report` (Metrics)
- Runs on: Weekly schedule
- Generates: Health metrics report
- Action: Creates GitHub issue if degradation detected

---

## Configuration

### Adjusting Threshold

**Local (validate_links.sh):**
```bash
# Line 10
THRESHOLD=10  # Change to desired maximum
```

**CI/CD (validate-docs.yml):**
```yaml
# Line 121
threshold=200  # Maximum for comprehensive scan
```

### Changing Schedule

**GitHub Actions:**
```yaml
# Line 24
schedule:
  - cron: '0 0 * * 0'  # Modify cron expression
```

Common schedules:
- Daily: `0 0 * * *`
- Weekly (Sunday): `0 0 * * 0`
- Monthly: `0 0 1 * *`

---

## Usage Guide

### For Developers

#### Local Validation (Before Commit)

```bash
# Check your changes
./validate_links.sh changed

# Fix any broken links, then commit
git add .
git commit -m "docs: update documentation"
# Pre-commit hook runs automatically
```

#### Manual Full Scan

```bash
# Run comprehensive check
./validate_links.sh all

# Or use Python scanner for detailed report
python3 scan_broken_links.py
```

---

### For Repository Maintainers

#### Monitoring Link Health

1. **Check GitHub Actions tab**
   - View weekly validation results
   - Download comprehensive reports

2. **Review auto-created issues**
   - Weekly health degradation alerts
   - Actionable broken link lists

3. **Pull Request Reviews**
   - CI automatically checks PR changes
   - Comments added if validation fails

#### Responding to Failures

**In Pull Requests:**
```bash
# Reviewer: Request changes
"Please fix the broken links detected by CI"

# Contributor: Run locally
./validate_links.sh changed
# Fix links and push update
```

**Weekly Reports:**
```bash
# Download comprehensive report from Actions artifacts
# Review missing files list
# Create issues for high-priority fixes
```

---

## Validation Rules

### What Gets Checked

✅ **Checked:**
- Relative links `[text](../path/to/file.md)`
- Links to markdown files
- Links to directories (need index.md)

❌ **Skipped:**
- External URLs (`http://`, `https://`)
- Anchor links (`#section`)
- mailto links (`mailto:`)
- Empty/whitespace links

### Link Resolution

**Relative paths:**
```markdown
[Link](../User/file.md)
```
- Resolved from source file location
- Must point to existing file

**Directory links:**
```markdown
[Link](./subdirectory/)
```
- Must contain `index.md` file
- Otherwise considered broken

---

## Troubleshooting

### Pre-Commit Hook Not Running

**Check if executable:**
```bash
ls -la .git/hooks/pre-commit
# Should show: -rwxr-xr-x
```

**Fix permissions:**
```bash
chmod +x .git/hooks/pre-commit
```

### False Positives

**Stubs showing as broken:**
- Expected for "in-progress" stubs
- Under threshold, won't block commits

**Recently created files:**
- Ensure file is staged: `git add file.md`
- Hook checks staged state

### CI Workflow Failures

**Check workflow logs:**
1. Go to GitHub Actions tab
2. Click failed workflow
3. Review step outputs

**Common issues:**
- Permissions: Ensure workflow has write access
- Path issues: Verify file paths in workflow

---

## Maintenance

### Weekly Tasks

1. **Review automated reports** (if any issues created)
2. **Check trending metrics** (broken link count)
3. **Update threshold** (if needed)

### Monthly Tasks

1. **Review validation rules** (any adjustments needed?)
2. **Update documentation** (reflect changes)
3. **Check automation health** (workflows running?)

### Quarterly Tasks

1. **Full documentation audit** (manual review)
2. **Update automation scripts** (improvements)
3. **Review thresholds** (adjust based on trends)

---

## Metrics & Reporting

### Available Metrics

**From `validate_links.sh`:**
- Files checked
- Broken links count
- Pass/fail status

**From `scan_broken_links.py`:**
- Total files scanned
- Total links analyzed
- Broken links by category
- Missing files list

### Report Locations

**Local:**
- `docs/Audit/checks/missing-files-list-*.txt`

**CI/CD Artifacts:**
- `link-validation-results` (on failure)
- `comprehensive-validation-report` (weekly)

**GitHub Issues:**
- Auto-created on health degradation
- Labeled: `documentation`, `automated`, `link-health`

---

## Best Practices

### For Contributors

1. **Run validation locally** before pushing
2. **Fix broken links immediately** (don't accumulate)
3. **Create stubs** for planned content (mark as in-progress)
4. **Use relative paths** (not absolute)

### For Maintainers

1. **Monitor weekly reports** (stay ahead of degradation)
2. **Set realistic thresholds** (balance strictness with practicality)
3. **Respond to PR failures** (help contributors fix issues)
4. **Update automation** (keep scripts maintained)

### For Documentation Writers

1. **Verify links** as you write
2. **Use stubs** for future content
3. **Check validation** before submitting PR
4. **Follow link conventions** (relative paths)

---

## Integration with Other Tools

### Scribe Workflows

**Works with:**
- `/scribe doc-validate` - Manual comprehensive validation
- `/scribe doc-sync` - Sync detection includes link checking
- `/scribe session-update` - Can trigger validation

**Coordination:**
- Automated checks complement manual workflows
- Use automation for prevention, Scribe for deep analysis

### Development Workflow

**Standard Flow:**
```bash
# 1. Make documentation changes
vim docs/User/some-file.md

# 2. Check locally
./validate_links.sh changed

# 3. Stage and commit
git add docs/User/some-file.md
git commit -m "docs: update some-file"
# Pre-commit hook validates automatically

# 4. Push
git push origin feature-branch

# 5. Create PR
# CI validates in PR automatically
```

---

## Emergency Procedures

### Temporarily Disable Pre-Commit Hook

**Method 1: Bypass single commit**
```bash
git commit --no-verify -m "emergency fix"
```

**Method 2: Temporarily disable**
```bash
mv .git/hooks/pre-commit .git/hooks/pre-commit.disabled
# Make commits
mv .git/hooks/pre-commit.disabled .git/hooks/pre-commit
```

**⚠️ WARNING:** Re-enable as soon as possible!

### Disable CI Checks

**Temporarily skip workflow:**
Add `[skip ci]` to commit message:
```bash
git commit -m "docs: emergency update [skip ci]"
```

**⚠️ WARNING:** Use only in emergencies!

---

## Future Enhancements

**Planned:**
- [ ] Link checking for external URLs (with caching)
- [ ] Automatic link fixing suggestions
- [ ] Integration with documentation linters
- [ ] Performance optimizations for large repos
- [ ] Dashboard for link health trends

**Under Consideration:**
- [ ] Slack/Discord notifications
- [ ] Auto-fix PRs for simple issues
- [ ] ML-based dead link prediction

---

## Support

**Issues:**
- GitHub Issues: Tag with `documentation`, `automation`
- Scribe: Use `/scribe` for doc-specific help

**Logs:**
- Local: Script output to console
- CI: GitHub Actions workflow logs
- Reports: Artifacts in Actions tab

**Contact:**
- Documentation Team: See CONTRIBUTING.md
- Automation: See .github/CODEOWNERS

---

## References

- [Validation Script](../../validate_links.sh)
- [GitHub Workflow](../../.github/workflows/validate-docs.yml)
- [Scribe Documentation](../README.md)
- [Link Cleanup Campaign Reports](../Audit/history/)

---

**Status:** ✅ PRODUCTION-READY
**Last Updated:** 2025-12-24
**Maintained by:** Documentation Team

---

*This automation system was created during the Phase 2 Link Cleanup Campaign to ensure all improvements are maintained.*
