# PLAN.md — Tracely

> Unified observability library for Rust

---

## Project Summary

**Type**: Rust Workspace (Library)  
**Purpose**: Distributed tracing, metrics, and structured logging in a single, ergonomic API.

---

## Phase 1: Core Observability Primitives (Week 1-2)

**Duration**: 2 weeks  
**Focus**: tracely-core crate foundation

### Deliverables
- [ ] Span lifecycle management
- [ ] Context propagation (W3C TraceContext)
- [ ] Counter metrics macro
- [ ] Gauge metrics macro
- [ ] Histogram metrics macro
- [ ] Structured JSON logging
- [ ] Log level filtering
- [ ] Zero-allocation log path
- [ ] Configuration builder (TracingConfig)
- [ ] Library initialization
- [ ] 100% rustdoc coverage
- [ ] Unit tests

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Rust Engineer | 40h | 1 |
| QA Engineer | 10h | 1 |

---

## Phase 2: Exporters (Week 3-4)

**Duration**: 2 weeks  
**Focus**: Export capabilities

### Deliverables
- [ ] OTLP exporter (gRPC)
- [ ] OTLP exporter (HTTP)
- [ ] Prometheus metrics endpoint
- [ ] Jaeger exporter
- [ ] Zipkin exporter
- [ ] Exporter trait abstraction
- [ ] Batch export processing
- [ ] Export retry logic
- [ ] Export buffering
- [ ] Exporter configuration

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Rust Engineer | 40h | 1 |
| DevOps Engineer | 10h | 1 |

---

## Phase 3: tracely-sentinel (Week 5-6)

**Duration**: 2 weeks  
**Focus**: Monitoring and alerting crate

### Deliverables
- [ ] Health check framework
- [ ] Custom health indicators
- [ ] Health aggregation
- [ ] Alert rule engine
- [ ] Notification channels (webhook)
- [ ] Metrics threshold alerts
- [ ] Anomaly detection (basic)
- [ ] Sentinel dashboard data API
- [ ] Integration with tracely-core
- [ ] Alert silencing

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Rust Engineer | 40h | 1 |
| Backend Engineer | 15h | 1 |

---

## Phase 4: Performance & Advanced Features (Week 7-8)

**Duration**: 2 weeks  
**Focus**: Production optimization

### Deliverables
- [ ] Async span processing
- [ ] Sampling strategies (head-based, tail-based)
- [ ] Rate limiting
- [ ] Memory pool for spans
- [ ] Lock-free metrics collection
- [ ] Criterion benchmarks
- [ ] Load testing
- [ ] Memory profiling
- [ ] Performance regression tests
- [ ] <1μs log write target verified
- [ ] <5μs trace span target verified

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Rust Engineer | 40h | 1 |
| Performance Engineer | 20h | 1 |

---

## Phase 5: Integration & Documentation (Week 9-10)

**Duration**: 2 weeks  
**Focus**: Ecosystem integration

### Deliverables
- [ ] Axum integration middleware
- [ ] Actix-web integration middleware
- [ ] Tokio tracing integration
- [ ] Serde serialization support
- [ ] Full API documentation
- [ ] Examples repository
- [ ] Migration guide from phenoSentinel
- [ ] crates.io release
- [ ] GitHub Actions CI/CD
- [ ] Benchmark reports
- [ ] Security audit

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Rust Engineer | 40h | 1 |
| Technical Writer | 20h | 1 |
| DevOps Engineer | 10h | 1 |

---

## Summary

| Metric | Value |
|--------|-------|
| **Total Duration** | 10 weeks |
| **Total Engineer Hours** | 325h |
| **Crates Delivered** | 2 (tracely-core, tracely-sentinel) |
| **Milestones** | 5 |

---

## Risk Factors

1. **Zero-allocation constraints** → Mitigation: Extensive benchmarking
2. **opentelemetry crate API changes** → Mitigation: Version pinning, abstraction layer
3. **Performance targets** → Mitigation: Criterion benchmarks from day 1

---

## Dependencies

| Layer | Crate | Version |
|-------|-------|---------|
| Span/event collection | tracing | 0.1 |
| Metrics primitives | metrics | 0.21 |
| OTLP export | opentelemetry | 0.21 |
| Prometheus export | prometheus | 0.13 |
| Serialization | serde_json | 1.0 |
| Benchmarking | criterion | 0.5 |

---

## Quality Gates

- `cargo clippy -- -D warnings` — 0 warnings
- `cargo test` — all pass
- `cargo doc` — 0 missing doc warnings on public items
- Criterion benchmarks pass performance targets

---

*Last updated: 2026-04-04*
