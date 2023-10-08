# Service to ingest Msgpack metrics over HTTP

## Rust
  - endpoint to consume HTTP metrics
    - Versioning: v1 and v2
    - Messagepack

  - Generate JS and TS SDK

  - Client HTML test page ("/")
    - Usees JS and TS SDK to publish versioned messages

## JS and TS SDK
  - WASM module:
    - serialization
    - deserialization
    - publishing metrics(HTTP POST)

  - Versioned API publishing, matching server handlers

## How to run
  1. Generate JS and TS bindings
  ```
      $ cd hello-wasm
      $ npm install
      $ npm run build
  ```
  2. Run web server
  ```
      $ cd web
      $ cargo run
  ```

  3. Open test page on http://localhost:3000 and send v1 and v2 messages via buttons

  4. To run tests
  ```
      $ cd hello-wasm
      $ wasm-pack test --firefox --headless
  ```