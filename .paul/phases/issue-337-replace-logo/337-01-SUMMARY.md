---
phase: issue-337-replace-logo
plan: 01
subsystem: brand
tags: [logo, svg, rsvg-convert, imagemagick]

requires:
  - phase: none
    provides: n/a
provides:
  - Regenerated logo exports in all formats (PNG, JPG, PDF, EPS)
  - Cleanup of old terminal renderer
affects: []

tech-stack:
  added: [librsvg2-bin (rsvg-convert 2.58.0)]
  patterns: []

key-files:
  created: []
  modified:
    - Polyglot Logo/PNG/Icon.png
    - Polyglot Logo/PNG/Logo.png
    - Polyglot Logo/JPG/Icon.jpg
    - Polyglot Logo/JPG/Logo.jpg
    - Polyglot Logo/PDF/Icon.pdf
    - Polyglot Logo/PDF/Logo.pdf
    - Polyglot Logo/EPS/Icon.eps
    - Polyglot Logo/EPS/Logo.eps

key-decisions:
  - "Used rsvg-convert instead of broken Inkscape (snap/glibc conflict)"

patterns-established: []

duration: 5min
started: 2026-04-20
completed: 2026-04-20
---

# Issue #337 Plan 01: Replace Logo Format Exports — Summary

**Regenerated 8 logo format exports (PNG/JPG/PDF/EPS) from new octopus SVG using rsvg-convert; deleted old geometric terminal renderer**

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: PNG exports regenerated | Pass | 512x512, Icon 34KB, Logo 72KB |
| AC-2: JPG exports regenerated | Pass | 512x512, Icon 28KB, Logo 62KB |
| AC-3: PDF exports regenerated | Pass | Icon 12KB, Logo 22KB, PDF 1.7 |
| AC-4: EPS exports regenerated | Pass | Icon 489KB, Logo 642KB, DSC 3.0 |
| AC-5: Old terminal renderer removed | Pass | logo.py deleted |
| AC-6: README logo reference valid | Pass | PNG/Logo.png exists at referenced path |

## Accomplishments

- Regenerated all 8 format exports from new octopus SVG sources
- Resolved Inkscape unavailability by using `rsvg-convert` (librsvg2-bin)
- Deleted obsolete `logo.py` geometric terminal renderer

## Files Modified

| File | Change | Purpose |
|------|--------|---------|
| Polyglot Logo/PNG/Icon.png | Regenerated | 512x512 octopus icon PNG |
| Polyglot Logo/PNG/Logo.png | Regenerated | 512x512 octopus full logo PNG |
| Polyglot Logo/JPG/Icon.jpg | Regenerated | JPEG from PNG via ImageMagick |
| Polyglot Logo/JPG/Logo.jpg | Regenerated | JPEG from PNG via ImageMagick |
| Polyglot Logo/PDF/Icon.pdf | Regenerated | Vector PDF from SVG |
| Polyglot Logo/PDF/Logo.pdf | Regenerated | Vector PDF from SVG |
| Polyglot Logo/EPS/Icon.eps | Regenerated | EPS from SVG |
| Polyglot Logo/EPS/Logo.eps | Regenerated | EPS from SVG |
| Polyglot Logo/logo.py | Deleted | Old geometric terminal renderer |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Used rsvg-convert over Inkscape | Inkscape broken (snap/glibc conflict) | librsvg2-bin installed via apt; full PNG/PDF/EPS support |
| JPG via ImageMagick from PNG | rsvg-convert doesn't output JPG directly | Two-step: SVG→PNG→JPG; quality preserved |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 0 | None |
| Scope additions | 0 | None |
| Deferred | 1 | AI/ files need Adobe Illustrator |

### Deferred Items

- AI/Icon.ai and AI/Logo.ai still contain old geometric design — requires Adobe Illustrator (proprietary software, cannot regenerate via CLI)

## Issues Encountered

| Issue | Resolution |
|-------|------------|
| Inkscape broken (snap/glibc library conflict) | Used rsvg-convert from librsvg2-bin package |

## Next Phase Readiness

**Ready:**
- All CLI-exportable formats updated with new octopus design
- Branch ready for merge

**Concerns:**
- AI/ files still show old design (needs manual Adobe update)

**Blockers:**
- None

---
*Phase: issue-337-replace-logo, Plan: 01*
*Completed: 2026-04-20*
