# Tracely — 100+ Pillar Scorecard Audit Report

**Date**: 2026-07-08
**Repo**: `kooshapari/Tracely`
**Type**: Rust library (unified observability)
**Overall Score**: **143/210 = 68% | Grade: C+**

---

## Executive Summary

Tracely is a unified observability library for the Phenotype ecosystem (Rust, wrapping tracing/metrics/opentelemetry crates). It is **well-structured** with strong CI/CD foundations (14 workflows), proper agent context, and mature governance. The main gaps are in **benchmarking, fuzzing breadth, cross-platform CI, release automation, and operational documentation**.

### Scores by Cluster

| Cluster | Pillars | Score | Grade | Notes |
|---------|---------|-------|-------|-------|
| C03 — Onboarding | L30 | 24/30 | **80% B** | Strong agent docs, needs friction-log |
| C04 — Security | L31–L40 | 24/30 | **80% B** | SBOM missing, otherwise excellent |
| C05 — Observability | L41–L50 | 30/30 | **100% A** | This IS the observability lib |
| C06 — Supply Chain | L51–L60 | 18/30 | **60% D** | Missing hermetic build, MCP provenance |
| C07 — Dev Experience | L61–L70 | 20/30 | **67% D** | Benches, fuzz, cross-platform missing |
| C08 — Quality | L71–L80 | 12/30 | **40% D** | No bench harness, mutation, eval |
| C09 — Accessibility | L81–L95 | 0/30 | **0% N/A** | N/A for Rust library |
| C10 — Visual Design | L96–L107 | 0/30 | **0% N/A** | N/A for Rust library |
| C11 — Distribution | L108–L122 | 15/30 | **50% C** | No release workflow, codesign N/A |
| **Total** | **L30–L122** | **143/210** | **68% C+** | (a11y + visual N/A) |

---

## Gap Analysis

### C03 — Onboarding (80% B)
**Strength**: AGENTS.md (514 lines, very thorough), CLAUDE.md, PR template, issue templates, CODEOWNERS, CONTRIBUTING.md, SECURITY.md
**Gaps**: No friction-log, user journeys are TODO-stub, no `.env.example`, no functional requirements doc
**Fixes**: friction-log (S), flesh out journey manifests (M)

### C04 — Security (80% B)
**Strength**: 14 CI workflows — cargo-audit, cargo-deny, codeql-rust, codeql, trufflehog, scorecard, dependabot, renovate
**Gaps**: No SBOM emission, no signed commits enforcement (org-level), no threat model doc
**Fixes**: Add SBOM workflow (S), add threat model ADR (M)

### C05 — Observability (100% A)
**Strength**: This IS the observability library. Full OTel, tracing, metrics, logging wrapped.
**Gaps**: None

### C06 — Supply Chain (60% D)
**Strength**: deny.toml, renovate, pinned toolchain
**Gaps**: No hermetic build, no MCP server provenance, no source provenance
**Fixes**: Library-specific — dependency surface already clean

### C07 — Dev Experience (67% D)
**Strength**: justfile (37 recipes), devcontainer, editorconfig, clippy.toml, rustfmt.toml
**Gaps**: No bench harness, fuzz only for sentinel (not core), no cross-platform CI, no flake detection, no mutation testing
**Fixes**: Add benches (M), add core fuzz target (M), add macOS/windows to CI (S)

### C08 — Quality (40% D)
**Strength**: cargo-semver-checks (API compatibility)
**Gaps**: No bench harness, no mutation testing, no eval corpus
**Fixes**: Add criterion bench (M)

### C09 — Accessibility (N/A)
Library only — no UI.

### C10 — Visual Design (N/A)
Library only — no UI.

### C11 — Distribution (50% C)
**Strength**: 14 CI workflows, cargo-semver-checks, changelog, quality gate
**Gaps**: No release workflow (cargo publish on tag), no provenance attestation
**Fixes**: Add release workflow (M)

---

## DAG Work Plan

```
Phase 0: Quick Wins (S-effort, ~30min each)
├── WP-001: Create docs/friction-log.md (C03)
├── WP-002: Add SBOM CI workflow (C04 L32)
├── WP-003: Add macOS/Windows to CI matrix (C07 L69)

Phase 1: Structural (M-effort, 1-2h each)
├── WP-010: Add criterion bench harness (C07 L64 + C08)
├── WP-011: Add fuzz target for tracely-core (C07 L67)
├── WP-012: Create release workflow (C11)
├── WP-013: Document threat model ADR (C04 L41)

Phase 2: Maturity (L-effort)
├── WP-020: Flesh out user journey manifests (C03)
├── WP-021: Mutation testing setup (C07 L65)
├── WP-022: Cross-platform CI + flake detection (C07 L68-69)
```

---

## Scoring Rubric

| Score | Meaning |
|-------|---------|
| 3/3 | Fully implemented, evidence-backed |
| 2/3 | Implemented, minor gaps |
| 1/3 | Started, major gaps remain |
| 0/3 | Not started or N/A |
