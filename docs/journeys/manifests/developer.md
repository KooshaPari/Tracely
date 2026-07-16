# Developer Journey — Add Tracely Tracing to a Rust App

## Goal

Add OpenTelemetry-powered distributed tracing to a Rust application using
Tracely — wires up the `TracerProvider`, configures an OTLP exporter, and
verifies spans appear.

## Steps

1. **Add dependency**
   ```toml
   [dependencies]
   tracely = { git = "https://github.com/KooshaPari/Tracely" }
   tracing = "0.1"
   tracing-subscriber = "0.3"
   ```

2. **Initialize Tracely in `main.rs`**
   ```rust
   use tracely_core::{init_tracer, TracelyConfig};
   use tracely_core::LayerExt;

   let config = TracelyConfig::builder()
       .service_name("my-app")
       .endpoint("http://localhost:4317")
       .build();

   let (tracer, _guard) = init_tracer(config.clone()).await;
   tracing_subscriber::registry()
       .with(tracer)
       .with(tracing_subscriber::fmt::layer())
       .init();
   ```

3. **Add instrumented functions**
   ```rust
   #[tracing::instrument]
   async fn handle_request(req: Request) -> Response { ... }
   ```

4. **Verify spans in stdout** — run the app and see:
   ```
   INFO handle_request{req_id=123}: tracing_app started
   ```

5. **Connect to OTLP collector** — set `OTEL_EXPORTER_OTLP_ENDPOINT` env var

## Touchpoints

- `tracely-core` crate: `init_tracer`, `TracelyConfig`
- `Cargo.toml` dependency addition
- `main.rs` initialization

## Validation

```bash
cargo run
# stdout should show tracing events with span context
```

## Failure Modes

| Issue | Symptom | Fix |
|-------|---------|-----|
| Missing OTLP endpoint | Spans only in stdout, not in collector | Set `OTEL_EXPORTER_OTLP_ENDPOINT` |
| gRPC connection refused | Tracely logs `connection refused` | Ensure collector is running |
| Version mismatch | Compile error on OTel types | Align `opentelemetry` versions in workspace |
