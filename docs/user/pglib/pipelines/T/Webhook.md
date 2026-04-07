---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =T.Webhook

Fires on an incoming HTTP request to the specified endpoint. Endpoint path provided via inline call: `=T.Webhook"/api/onboarding"`.

## Definition

```polyglot
{N} =T.Webhook
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TWebhook"
   [%] .description << "Fires on an incoming HTTP request to the specified endpoint."
   <InlineStringLiteral#string <~ ""
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `InlineStringLiteral` | `#string` | Endpoint path to bind (e.g. `"/api/onboarding"`). Provided inline: `=T.Webhook"/api/onboarding"`. Defaults to `""`. |

## Outputs

| Name | Type | Description |
|------|------|-------------|

## Errors

None.

## Permissions

Web.Socket

## Related

- [[pglib/pipelines/T/INDEX|=T.* Trigger Pipelines]]
