# Telemetry and Analytics

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

Telemetry helps improve product quality while respecting privacy and enterprise constraints.

## 2. Principles

- Opt-in by default for sensitive environments.
- No ontology content collected unless explicitly enabled.
- Aggregate usage only.
- Clear privacy documentation.
- Enterprise policy control.

## 3. Events

Collect optional anonymous events:

- workspace opened
- command executed
- feature used
- query run
- reasoning run
- refactoring previewed
- refactoring applied
- AI action invoked
- plugin activated
- error occurred

## 4. Performance Metrics

Track:

- startup time
- workspace load time
- search latency
- graph render time
- reasoning duration
- query duration
- plugin activation time

## 5. Error Reporting

Include:

- error kind
- stack trace where safe
- platform
- extension version
- plugin involved

Never include ontology content by default.

## 6. Product Metrics

Useful metrics:

- time to first entity selection
- refactoring acceptance rate
- AI suggestion acceptance rate
- query success rate
- reasoning error frequency
- graph workspace usage

## 7. Local Analytics

Provide local workspace insights even when telemetry is disabled.

## 8. Enterprise

Support:

- disable telemetry
- redirect telemetry endpoint
- local-only analytics
- audit export
