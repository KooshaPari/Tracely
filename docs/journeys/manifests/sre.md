# SRE Journey — Debug Production Incident with Tracely

## Goal

Use Tracely-powered distributed traces to debug a production incident —
identify the root cause span, check related traces, and confirm the fix.

## Steps

1. **Identify the affected trace** from the alerting system
   ```bash
   # Extract trace_id from alert payload
   TRACE_ID="abc123def456"
   ```

2. **Query Tracely for the trace details**
   Uses the Tracely query API (if deployed with the server component).

3. **Analyze span timings** — find the span with highest latency

4. **Check downstream dependencies** — find P50/P95/P99 latencies per service

5. **Correlate with logs** — pull logs for the same `trace_id`

6. **Confirm fix** — re-run the query and verify latency regression resolved

## Touchpoints

- Tracely query API (if server deployed)
- OpenTelemetry collector (Grafana Tempo, Jaeger, or Honeycomb)
- Tracely structured log output

## Validation

```bash
# Query spans for a trace
curl "http://tracely:8080/api/v1/trace/$TRACE_ID"
# Expected: list of spans with timing, status, and service metadata
```

## Failure Modes

| Issue | Symptom | Fix |
|-------|---------|-----|
| Trace not found | API returns 404 | Check if trace_id is correct; verify sampling rate |
| Partial trace | Some spans missing | Check if downstream services have Tracely initialized |
| Clock skew | Negative or improbable durations | Verify NTP sync across all nodes |
