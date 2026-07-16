# Threat Model — tracely

## Scope

This threat model covers the tracely Rust library (the `tracely` crate). tracely is a library, not a service — it runs embedded in the host process. Threats are evaluated from the perspective of the host application and its consumers.

## Trust Model

| Trust Boundary | Actors | Data Flow |
|----------------|--------|-----------|
| Library code | Maintainers, contributors | No network access by default |
| Public API | Host application | All inputs are caller-controlled |
| OTLP export (optional) | Host app → OTLP collector | External network (configurable) |
| Prometheus export (optional) | Host app → Prometheus | External network (configurable) |

## Threats (STRIDE per component)

### 1. Public API Surface

| Threat | Type | Risk | Mitigation |
|--------|------|------|------------|
| Malformed config causes panic | DoS | Low | `TracingConfig` / `LoggerConfig` use safe defaults; no unsafe parsing |
| Race condition on init | Tampering | Low | `init_tracing` uses `OnceCell` / `set_global_default` — only first call succeeds |
| Log injection via untrusted input | Spoofing | Low | Structured logs separate message from metadata; `tracing_subscriber` handles escaping |

### 2. OTel Export (optional feature)

| Threat | Type | Risk | Mitigation |
|--------|------|------|------------|
| Unencrypted OTLP connection | Info disclosure | Medium | OTLP default is gRPC+TLS; library doesn't override transport security |
| Exfiltration via span attributes | Info disclosure | Medium | Span attributes are caller-controlled; library doesn't inject env vars |

### 3. Dependency Chain

| Threat | Type | Risk | Mitigation |
|--------|------|------|------------|
| Compromised upstream crate | Supply chain | High | cargo-deny + cargo-audit in CI; pinned lockfile; dependabot + renovate |
| Malicious transitive dep | Supply chain | Medium | `deny.toml` blocks advisories; scorecard automation |

## Summary

tracely's threat surface is limited because it is a **library**, not a network service. The primary risks are:
1. **Supply chain** — mitigated by CI security scanning (cargo-deny + cargo-audit + 12 other workflows)
2. **DoS via malformed input** — mitigated by safe defaults and no unsafe parsing
3. **Data exfiltration via telemetry** — mitigated by caller-controlled export configuration
