---
phase: 342-aj3lib-constructors
plan: 03
status: complete
completed: 2026-04-22
---

# Plan 342-03 Summary: Tier 2b Constructors

## What Was Done

Defined the 4 remaining Tier 2b {$} constructors ($Ver, $URL, $IP, $Color) with their aj3lib {#} type definitions and companion -Type.Parse runtime pipelines. This completes the full constructor catalog for issue #342.

## Files Created (12)

### Type Definitions (4)
- `docs/user/aj3lib/types/Ver.md` — ##Scalar, ###ScalarValue; .major, .minor, .patch (required) + .prerelease, .build (optional with defaults)
- `docs/user/aj3lib/types/URL.md` — ##Scalar; 6 fields (.scheme, .host, .port, .path, .query, .fragment); PgUrl native class for RFC 3986 parsing
- `docs/user/aj3lib/types/IP.md` — ##Scalar, ###ScalarValue; single .address#RawString field; PgIP native class for range validation
- `docs/user/aj3lib/types/Color.md` — ##Scalar, ###ScalarValue; .r, .g, .b, .a (0-255); PgColor native class for range + named color lookup

### Constructor Docs (4)
- `docs/user/aj3lib/constructors/Ver.md` — 4 overloads: basic, +prerelease, +build, +both; pure string-parsing pattern
- `docs/user/aj3lib/constructors/URL.md` — 1 overload; native pipeline conversion via -URL.Decompose (like $Dur)
- `docs/user/aj3lib/constructors/IP.md` — 2 overloads: IPv4 (dotted-decimal) and IPv6 (colon-hex); native validation (like $Re)
- `docs/user/aj3lib/constructors/Color.md` — 3 overloads: hex RGB (6-digit), hex RGBA (8-digit), named; mixed native conversion + validation

### Parse Pipeline Docs (4)
- `docs/user/aj3lib/pipelines/Ver.Parse.md` — {N} native, !Parse.Ver.InvalidFormat
- `docs/user/aj3lib/pipelines/URL.Parse.md` — {N} native, !Parse.URL.InvalidFormat, !Parse.URL.InvalidScheme
- `docs/user/aj3lib/pipelines/IP.Parse.md` — {N} native, !Parse.IP.InvalidFormat, !Parse.IP.OutOfRange
- `docs/user/aj3lib/pipelines/Color.Parse.md` — {N} native, !Parse.Color.InvalidFormat, !Parse.Color.UnknownName

## Files Modified (1)

- `docs/user/aj3lib/constructors/INDEX.md` — Added 4 new rows (5 → 9 constructors), removed "Tier 2b (planned)" section, added 4 parse pipeline related links

## Constructor Pattern Summary

| Constructor | Pattern | Overloads |
|---|---|---|
| $Ver | Basic string-parsing (like $MIME) | 4 |
| $URL | Native pipeline conversion (like $Dur) | 1 |
| $IP | Native validation (like $Re) | 2 |
| $Color | Mixed: hex native conversion + named native validation | 3 |

## Acceptance Criteria

- [x] AC-1: #Ver type definition — ##Scalar with 5 fields, alias "ver"
- [x] AC-2: #URL type definition — ##Scalar with 6 fields, PgUrl native class, well-known schemes
- [x] AC-3: #IP type definition — ##Scalar with .address, PgIP native class
- [x] AC-4: #Color type definition — ##Scalar with RGBA fields, PgColor native class, named colors table
- [x] AC-5: All 4 constructors defined with ($) regex, [$] binding, [.] mapping, overload tables
- [x] AC-6: All 4 parse pipelines defined with {N}, IO, errors
- [x] AC-7: INDEX.md lists all 9 constructors, no "planned" sections

## What's Next

All 9 aj3lib constructors from issue #342 are now defined. Remaining work:
- Issue #343: Compile error codes for constructors
- Issue #344: Cross-reference updates to existing docs
