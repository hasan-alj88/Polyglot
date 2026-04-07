# Rust Integrator SDK

Bidirectional SDK for Rust applications integrating with Polyglot.

## Directions

**Host to Polyglot:** Submit pipeline execution requests to the Polyglot Service and receive results.

**Polyglot to Host:** Register Rust functions callable by `=RT.*` runtime pipelines. Functions receive serialized JSON inputs and return JSON outputs following the native function contract.

## Wire Protocol

Uses the same JSON envelope as all native dispatch. See `docs/technical/spec/native-dispatch.md#Serialization Protocol`.
