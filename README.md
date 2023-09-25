# Service to ingest Msgpack metrics over HTTP

## Rust
  - endpoint to consume HTTP metrics
    - Versioning
    - Messagepack with custom 8601 time ext format
  - Store in timeseries DB

  - Generate JS SDK

  - Client SDK test page
    - Usees JS SDK to publish versioned messages

  - Summary page
    - Svelte
    - Realtime dashboard(rps, metrics) - websockets
    - Usees JS SDK

  - Architecture diagram D2

  - Tracing?
  - Logging?

## JS SDK
  - WASM module:
    - serialization
    - deserialization
    - publishing metrics(HTTP POST)
    - getting published summary

  - Versioned API publishing, matching server handlers

## Out of scope
  - Auth
