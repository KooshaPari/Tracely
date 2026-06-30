# Tracely / phenotype-sentinel — Threat Model

_Addresses audit finding L20 (Security architecture hardening: no trust boundary inventory)._
_Scope: tracely-core and tracely-sentinel as in-process Rust libraries._

---

## Scope

Both crates are **in-process Rust libraries**. They have no network listeners, no
persistent storage, no IPC, and no elevated OS privileges. The primary trust
boundary is between **library code** and **caller-controlled inputs** (config,
request metadata, downstream URLs).

---

## Trust Boundaries

```
┌─────────────────────────────────────────────────────────────────┐
│ Caller process (trusted)                                        │
│                                                                 │
│  ┌─────────────────────┐    ┌──────────────────────────────┐   │
│  │   tracely-core      │    │   tracely-sentinel           │   │
│  │   (tracing/logging) │    │   (rate limiter / CB / BH)   │   │
│  └──────────┬──────────┘    └───────────┬──────────────────┘   │
│             │                           │                       │
│         OTLP/Prometheus              in-memory state            │
│         (optional, external)         (Arc<RwLock<...>>)         │
└─────────────┼───────────────────────────┼───────────────────────┘
              │ [TB-1]                    │ [TB-2]
   ┌──────────▼────────────┐   No external boundary; state
   │ External collector     │   is local to the caller process.
   │ (Jaeger / Prometheus) │
   └───────────────────────┘
```

| Boundary | Label | Description |
|----------|-------|-------------|
| Library API → caller config | TB-0 | Config structs accepted at init; validated internally |
| tracely-core → OTLP collector | TB-1 | gRPC/HTTP to external host; optional feature |
| tracely-sentinel → caller state | TB-2 | In-process Arc<RwLock>; no external egress |

---

## Threat Inventory

### T-01 — Config injection via untrusted strings

| Field | Risk | Mitigation |
|-------|------|------------|
| `CircuitBreakerConfig.failure_threshold` | DoS if set to 0 or u64::MAX | `validation.rs` rejects 0; upper bound check pending (FR-SENT-VAL-001) |
| `RateLimiterConfig.capacity` | Panic if 0 (explicit `panic!` in `TokenBucket::new`) | Convert to `Result` return — tracked in issues |
| OTLP endpoint URL (tracely-core) | SSRF if caller supplies attacker-controlled URL | Caller is trusted; document that OTLP endpoint must be operator-controlled |

**Current posture**: `validation.rs` exists and is exercised, but coverage is
partial. Remaining gaps are tracked as FR-SENT-VAL-002.

### T-02 — Bulkhead shared-state race

**Threat**: Two concurrent tokio tasks acquire the same partition slot
simultaneously, exceeding `capacity_per_partition`.

**Mitigation**: `try_acquire` holds the `partitions` write-lock for the
duration of the check-and-increment. The `RwLock` prevents concurrent
increments. No TOCTOU window exists within a single `try_acquire` call.

**Residual risk**: `Drop`-based release spawns a `tokio::spawn`; if the
runtime shuts down before the spawn runs, the slot leaks until process exit.
This is documented behavior and acceptable for server workloads. CLI/short-lived
processes should call `release()` explicitly.

### T-03 — Denial of service via metric label explosion (future feature)

**Threat**: When first-party metric emission is added (see RUNBOOK.md), an
attacker-controlled label dimension (e.g. per-URL partition ID) could exhaust
memory in the metrics backend.

**Mitigation (pre-emptive)**: When adding metrics, enforce a bounded label
cardinality — use partition index (integer) not free-form strings as label values.

### T-04 — Log injection via unvalidated fields (tracely-core)

**Threat**: Structured log fields emitted via `serde_json` could contain
newlines or control characters that break downstream log parsers or SIEM rules.

**Mitigation**: `serde_json` serialization escapes control characters by
default. Callers should not pass raw user input directly as log field names
(keys), as serde_json does not restrict key content. Document this in the
logging module's rustdoc.

### T-05 — Dependency supply chain

**Current controls**:
- `cargo-deny` with advisory, license, and source checks (`deny.toml`)
- `cargo-audit` in CI
- `trufflehog.yml` for secret scanning
- Renovate for automated dependency updates (`renovate.json5`)

**Gap**: Actions are SHA-pinned in `ci.yml` but not in all supplemental
workflows (e.g. `scorecard.yml`, `fr-coverage.yml`). Full SHA-pinning tracked
as a follow-up hardening item.

---

## Out-of-Scope

The following are explicitly out of scope for this library because they belong
to the caller application:

- Authentication / authorization (no identity primitives in sentinel/core)
- Network TLS (OTLP transport TLS is the collector's responsibility)
- Secrets management (no credentials stored in either crate)
- Multi-tenancy isolation (sentinel provides partition isolation within a single process)

---

## Review Cadence

This threat model should be reviewed:
- On any addition of a network-facing feature (e.g. metrics HTTP exporter)
- On major dependency version bumps (`tokio`, `opentelemetry`)
- Annually as part of the quarterly audit cycle

_Last reviewed: 2026-06-30 (initial creation)._
