# Platform Engineer Journey — Deploy Tracely as Org-Wide Observability

## Goal

Deploy and manage Tracely across the organization — configure sampling,
manage OTLP collector endpoints, and enforce tracing standards.

## Steps

1. **Add Tracely as a workspace dependency**
   ```toml
   # workspace Cargo.toml
   [workspace.dependencies]
   tracely = { git = "https://github.com/KooshaPari/Tracely", tag = "v0.2.0" }
   ```

2. **Create shared tracing config crate**
   ```rust
   // shared-tracing/src/lib.rs
   pub fn init_org_tracing() -> (Tracer, impl Drop) {
       let config = TracelyConfig::builder()
           .service_name(std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "unknown".into()))
           .endpoint(std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
               .unwrap_or_else(|_| "http://localhost:4317".into()))
           .sample_rate(0.1) // 10% sampling for high-traffic services
           .build();
       init_tracer(config)
   }
   ```

3. **Add SLO monitoring** — configure alerting on span drop rate

4. **Run cargo-deny in CI** to enforce consistent OTel versions
   ```bash
   cargo deny check
   ```

## Touchpoints

- Workspace `Cargo.toml` manifest
- Shared tracing config crate
- CI workflows (`cargo-audit.yml`, `cargo-deny.yml`)
- OTLP collector infrastructure

## Validation

```bash
cargo build --workspace
cargo test --workspace
cargo deny check
```

## Failure Modes

| Issue | Symptom | Fix |
|-------|---------|-----|
| Leaky spans | `Tracer` dropped before spans complete | Keep the `_guard` alive for app lifetime |
| Over-sampling | High cost on high-traffic services | Set `sample_rate` per service in config |
| Cross-crate version drift | OTel type mismatches in compile | Pin versions in workspace `Cargo.toml` |
