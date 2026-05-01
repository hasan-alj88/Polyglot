# Lesson 020: Trigger Payload Mapping

**Date**: 2026-05-01
**Context**: HTTP Triggers and Payload data extraction.

## Lesson Summary

When defining an HTTP trigger `[T] -T.Http`, you can map fields from the incoming request payload directly to pipeline inputs. If a payload field is missing, the trigger fails to fire (failed triggers are logged automatically via Otel).

### Correct Usage
```aljam3
   [T] -T.Http"POST /onboard"
      ( ) if the payload does not contain userId
      ( ) the trigger will fail to trigger
      (-) >Payload.userId#string >> <userId
         (>) >! ""
```
Note the syntax `>Payload.fieldName#type >> <inputVariable`.
