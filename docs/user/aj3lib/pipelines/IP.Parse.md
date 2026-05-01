---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:IP.Parse"
metadata_instance: "%-:IP.Parse:N"
---

# -IP.Parse

Parses a dynamic string into an `#ip` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$IP` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -IP.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "IpParse"
   [%] .description << "Parse dynamic string to IP address"
   (-) <raw#string
   (-) >ip#ip
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as an IP address (IPv4 dotted-decimal or IPv6 colon-hex notation) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>ip` | `#ip` | Parsed IP address value with `.address` field populated |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.IP.InvalidFormat` | Input string does not match IPv4 or IPv6 notation (wrong number of groups, invalid characters) |
| `!Parse.IP.OutOfRange` | Address components exceed valid range (IPv4 octet > 255, invalid IPv6 hex group) |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $addr#ip << -IP.Parse
   (<) <raw#string << $hostInput
   [!] !Parse.IP.InvalidFormat
      [-] $addr << $IP"127.0.0.1"
   [!] !Parse.IP.OutOfRange
      [-] $addr << $IP"127.0.0.1"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:IP.Parse` | Compile-time pipeline template |
| Instance | `%-:IP.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/constructors/IP|$IP constructor]] -- compile-time IP address construction
- [[jm3lib/types/IP|#IP type]] -- IP address type definition
