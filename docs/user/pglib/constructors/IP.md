---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $IP Constructor

<!-- @u:syntax/constructors -->
<!-- @c:pglib/types/IP -->

The `$IP` constructor produces `#IP` values from IP address strings. The compiler validates that the address is syntactically valid and within range at compile time — no error surface, no `[!]` handling needed.

## String-Parsing Overloads

### IPv4 Address

```aljam3
{$} $IP"{addr}"
   ($) <addr.re << "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+"
   [$] #IP
   [.] .address << <addr
```

Matches IPv4 addresses in dotted-decimal notation (e.g., `"192.168.1.1"`). The `($)` regex captures the format structurally, but the `PgIP` native class performs the actual range validation — each octet must be 0-255.

**Native validation pattern:** Like `$Re`, the `$IP` constructor relies on a native class (`PgIP`) for validation that cannot be expressed as regex constraints. The regex captures the string shape; the native class confirms semantic validity (octet ranges, no leading zeros).

### IPv6 Address

```aljam3
{$} $IP"{addr}"
   ($) <addr.re << "[0-9a-fA-F:.]+"
   [$] #IP
   [.] .address << <addr
```

Matches IPv6 addresses in colon-hex notation (e.g., `"2001:0db8::1"`, `"::1"`, `"fe80::1%25eth0"`). The permissive `($)` regex accepts hex digits, colons, and dots (for IPv4-mapped addresses) — the `PgIP` native class validates proper IPv6 group structure and abbreviation rules.

## Overload Resolution

The two overloads are distinguished by character content:

| Overload | Distinguishing Feature |
|---|---|
| IPv4 `"[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+"` | Digits and dots only, four dotted groups |
| IPv6 `"[0-9a-fA-F:.]+"` | Contains `:` characters |

Resolution order: IPv4 first (more specific regex), IPv6 as fallback (broader character set). The presence of `:` unambiguously selects IPv6.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $localhost4 << $IP"127.0.0.1"
[-] $gateway << $IP"192.168.1.1"
[-] $localhost6 << $IP"::1"
[-] $public6 << $IP"2001:0db8:85a3::8a2e:0370:7334"

[ ] for dynamic strings, use -IP.Parse with error handling
[-] $serverAddr#ip << -IP.Parse
   (<) <raw#string << $hostInput
   [!] !Parse.IP.InvalidFormat
      [-] $serverAddr << $IP"127.0.0.1"
```

## Related

- [[constructors/INDEX|pglib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[pglib/pipelines/IP.Parse|-IP.Parse]] -- runtime IP address string parsing
- [[pglib/types/IP|#IP type]] -- IP address type definition
