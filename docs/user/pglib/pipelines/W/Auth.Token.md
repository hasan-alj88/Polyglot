---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =W.Auth.Token

Acquires OAuth token on setup, revokes on cleanup.

## Definition

```polyglot
{N} =W.Auth.Token
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WAuthToken"
   [%] .description << "Acquires OAuth token on setup, revokes on cleanup."
   [{] $clientId#string       [ ] OAuth client ID
   [{] $clientSecret#string   [ ] OAuth client secret
   [}] $authToken             [ ] Active OAuth token
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$clientId` | `#string` | OAuth client ID |
| `$clientSecret` | `#string` | OAuth client secret |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$authToken` | token | Active OAuth token |

## Errors

None.

## Permissions

Crypto.Key

## Related

- [[pglib/pipelines/W/INDEX|=W.* Wrappers]]
