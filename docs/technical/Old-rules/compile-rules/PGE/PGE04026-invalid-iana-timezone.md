---
audience: developer
rule: "4.26"
code: PGE04026
name: Invalid IANA Timezone
severity: error
---

# Rule 4.26 — Invalid IANA Timezone
`PGE04026`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** A string literal assigned to a `#Zone.iana` field or passed as an `<iana#string` input to `-DT.Zone.Set` / `-DT.Zone.Convert` must be a valid IANA timezone identifier. Invalid or unrecognized timezone strings trigger PGE04026. This only applies to string literals — runtime-determined strings are validated at runtime.
**Rationale:** IANA timezone identifiers follow a strict `Area/Location` format. Catching typos and invalid identifiers at compile time prevents silent timezone mismatches that produce incorrect date/time conversions.
**Detection:** When the compiler encounters a string literal in an IANA timezone context, it validates against the IANA timezone database format: must match `Area/Location` pattern (e.g., `America/New_York`, `UTC`). Common aliases (`EST`, `PST`) are not valid IANA identifiers.

**See also:** PGE04001 (type mismatch)

**VALID:**
```aljam3
[-] $zone#Zone
   [.] .iana << "America/New_York"

[ ] ✓ valid IANA timezone identifier
```

```aljam3
[-] $event#dt
   [-] -DT.Zone.Set
      (-) <source << $event
      (-) <iana << "Europe/London"
      (-) >result >> $event

[ ] ✓ valid IANA timezone
```

**INVALID:**
```aljam3
[-] $zone#Zone
   [.] .iana << "EST"              [ ] ✗ PGE04026 — not a valid IANA identifier (use America/New_York)
```

```aljam3
[-] $zone#Zone
   [.] .iana << "NewYork"          [ ] ✗ PGE04026 — missing Area/ prefix
```

## See Also

- [[jm3lib/types/datetime|DateTime Types]] — `#Zone` definition
- [[jm3lib/pipelines/DT/Zone.Set|-DT.Zone.Set]], [[jm3lib/pipelines/DT/Zone.Convert|-DT.Zone.Convert]] — timezone pipelines
