# Protégé Parity Program

This directory contains the engineering specifications, audits, and
implementation plans that drive **OntoCode 1.0.0** toward **full
functional parity with Protégé Desktop**.

Unlike earlier planning documents, this documentation set is grounded in
an audit of the current repository. Its purpose is not to imagine a
future architecture, but to identify the remaining engineering work
required to replace Protégé for everyday ontology engineering.

------------------------------------------------------------------------

# Goals

The parity program has four primary goals:

1.  Define the exact scope of Protégé parity.
2.  Measure OntoCode's current capabilities against that scope.
3.  Plan and prioritize the remaining implementation work.
4.  Provide objective evidence that OntoCode 1.0 satisfies its parity
    claims.

------------------------------------------------------------------------

# Documentation Structure

## Foundation

-   `README.md` --- This document.
-   `CURRENT_REPOSITORY_AUDIT.md` --- Evidence-based audit of the
    current repository.
-   `PARITY_SCOPE.md` --- Defines what "Protégé parity" means.
-   `IMPLEMENTATION_EVIDENCE.md` --- Maps parity claims to source code
    and tests.

## Current State

Documents describing the current implementation:

-   `CURRENT_FEATURE_MATRIX.md`
-   `CURRENT_ARCHITECTURE.md`
-   `CURRENT_LIMITATIONS.md`
-   `SHIPPED_CAPABILITIES.md`

## Protégé Audit

Documents describing the Protégé baseline:

-   `PROTEGE_FEATURE_INVENTORY.md`
-   `PROTEGE_MENU_AUDIT.md`
-   `PROTEGE_VIEW_AUDIT.md`
-   `PROTEGE_WORKFLOW_AUDIT.md`
-   `PROTEGE_PLUGIN_AUDIT.md`
-   `PROTEGE_REASONER_AUDIT.md`
-   `PROTEGE_FILE_FORMAT_AUDIT.md`
-   `PROTEGE_UI_AUDIT.md`

## Parity Tracking

-   `PARITY_MATRIX.md`
-   `PARITY_GAP_ANALYSIS.md`
-   `PARITY_STATUS.md`
-   `PARITY_ACCEPTANCE_CRITERIA.md`
-   `PARITY_TEST_PLAN.md`
-   `PARITY_RELEASE_GATE.md`

## Engineering Blockers

The implementation roadmap is organized around blockers rather than
subsystems:

1.  Format-independent editing
2.  Complete OWL 2 authoring
3.  Workspace semantics
4.  Reasoning parity
5.  SWRL support
6.  Advanced ontology operations
7.  Parity verification

## Implementation Planning

-   `IMPLEMENTATION_PLAN.md`
-   `EXECUTION_ORDER.md`
-   `DEPENDENCY_GRAPH.md`
-   `P0_IMPLEMENTATION_PLAN.md`
-   `P1_IMPLEMENTATION_PLAN.md`
-   `RISK_REGISTER.md`

## Subsystem Specifications

Detailed implementation specifications for authoring, formats,
workspace, reasoning, SWRL, refactoring, querying, visualization,
plugins, UI workflows, and testing.

------------------------------------------------------------------------

# Recommended Reading Order

1.  Read [PRE_1_0_PHASES.md](07_BACKLOG/PRE_1_0_PHASES.md) for the versioned release plan (v0.19–1.0).
2.  Read `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md` or `01_CURRENT_STATE/` audit docs.
3.  Read `PARITY_SCOPE.md`.
4.  Review `PARITY_MATRIX.md` and `PARITY_GAP_ANALYSIS.md`.
5.  Read the blocker documents in order.
6.  Execute work from [EXECUTION_ORDER.md](05_IMPLEMENTATION/EXECUTION_ORDER.md) and backlog.
7.  Validate progress using the acceptance criteria and test plan.
8.  Ship only after every release gate has been satisfied.

------------------------------------------------------------------------

# Guiding Principles

-   Functional parity over visual similarity.
-   Semantic correctness over serialization details.
-   Native Rust implementation (no JVM dependency).
-   Evidence-backed engineering decisions.
-   Automated testing before parity claims.
-   Modern UX rather than reproducing Swing.

------------------------------------------------------------------------

# Definition of Success

OntoCode 1.0.0 may be described as a Protégé Desktop replacement only
when:

-   Every P0 parity requirement is complete.
-   All required ontology formats support semantic round-trip.
-   Complete OWL 2 authoring workflows are implemented.
-   Workspace, reasoning, and refactoring parity are achieved.
-   Required automated tests pass.
-   Release gates are satisfied.

------------------------------------------------------------------------

# Maintaining This Directory

Whenever implementation changes:

1.  Update implementation evidence.
2.  Update the parity matrix.
3.  Reassess the gap analysis.
4.  Update the roadmap and backlog.
5.  Expand automated test coverage.
6.  Record any scope changes.

These documents should remain the authoritative engineering source of
truth for the Protégé parity effort.
