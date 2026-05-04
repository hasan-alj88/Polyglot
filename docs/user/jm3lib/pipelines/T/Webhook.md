---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.T:Webhook"
metadata_instance: "%T:Webhook:N"
---

# -T.Webhook

Fires on an incoming HTTP request to the specified endpoint. Endpoint path provided via inline call: `-T.Webhook"/api/onboarding"`.

## Definition

```aljam3
{N} -T.Webhook
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TWebhook"
   [%] .description << "Fires on an incoming HTTP request to the specified endpoint."
   (-) %InlineString << "{endpoint}"
   (-) <endpoint#string <~ ""
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `endpoint` | `#string` | Endpoint path to bind (e.g. `"/api/onboarding"`). Provided inline: `-T.Webhook"/api/onboarding"`. Defaults to `""`. |

## Outputs

| Name | Type | Description |
|------|------|-------------|

## Errors

None.

## Permissions

Web.Socket

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Webhook` | Compile-time pipeline template |
| Instance | `%T:Webhook:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
