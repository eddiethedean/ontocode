# SHIPPED_CAPABILITIES (engineering snapshot — not product SSOT)

> **Not the public capability matrix.** For adopters and evaluators, use **[`docs/SHIPPED.md`](../../SHIPPED.md)** only.
> This file is a **Protégé-parity engineering baseline** (historically frozen around OntoCode **v0.18.2**). Do not cite it in procurement or Marketplace claims.

# Shipped Capabilities

**Status:** Living Document (engineering)\
**Repository Baseline:** OntoCode v0.18.2 (audit baseline — outdated relative to current tagged release)

------------------------------------------------------------------------

# Purpose

This document catalogs the major capabilities that were already
implemented at the parity-audit baseline. It complements the
parity documents by distinguishing shipped functionality from planned
work **for engineers working in `docs/protege-parity/`**.

A capability may be marked as **shipped** even if additional work is
required to achieve full Protégé parity. Remaining gaps are tracked
separately in `CURRENT_LIMITATIONS.md` and `PARITY_GAP_ANALYSIS.md`.

**Product truth:** [docs/SHIPPED.md](../../SHIPPED.md) (latest tagged release).

------------------------------------------------------------------------

# Status Legend

  Status      Meaning
  ----------- ----------------------------------------------
  SHIPPED     Available in the repository today.
  MATURE      Stable and broadly usable.
  EXPANDING   Implemented but expected to grow before 1.0.

------------------------------------------------------------------------

# Core Platform

  -----------------------------------------------------------------------
  Capability              Status                  Notes
  ----------------------- ----------------------- -----------------------
  Native Rust workspace   MATURE                  Multi-crate
                                                  architecture.

  VS Code / Cursor        MATURE                  Primary user interface.
  extension                                       

  React webviews          MATURE                  Modern UI for ontology
                                                  workflows.

  Language Server         SHIPPED                 Diagnostics,
                                                  navigation, semantic
                                                  tooling.

  CLI tooling             SHIPPED                 Automation and
                                                  scripting support.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Ontology Engineering

  Capability           Status    Notes
  -------------------- --------- --------------------------------------
  Turtle authoring     MATURE    Primary editing workflow.
  OBO authoring        MATURE    Dedicated parser and patch pipeline.
  OWL/XML parsing      SHIPPED   Read/index support.
  RDF/XML parsing      SHIPPED   Read/index support.
  Ontology lifecycle   SHIPPED   Create, open, save, export.
  Prefix management    SHIPPED   Prefix editing support.
  Imports management   SHIPPED   Import browser and workflows.

------------------------------------------------------------------------

# Semantic Features

  -----------------------------------------------------------------------
  Capability              Status                  Notes
  ----------------------- ----------------------- -----------------------
  OWL authoring           EXPANDING               Broad support with
                                                  remaining parity gaps.

  Semantic refactoring    SHIPPED                 Rename, merge,
                                                  namespace migration,
                                                  previews.

  Semantic diff           SHIPPED                 Ontology comparison
                                                  workflows.

  Diagnostics             SHIPPED                 Validation and
                                                  reporting
                                                  infrastructure.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Reasoning & Query

  -----------------------------------------------------------------------
  Capability              Status                  Notes
  ----------------------- ----------------------- -----------------------
  Classification          SHIPPED                 Multiple reasoning
                                                  profiles.

  Consistency checking    SHIPPED                 Current implementation
                                                  emphasizes class-level
                                                  consistency.

  Explanation workflows   EXPANDING               Existing foundation
                                                  with planned
                                                  enhancements.

  SPARQL query            SHIPPED                 Query workbench
                                                  available.

  Search & navigation     MATURE                  Entity search and
                                                  navigation support.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Visualization

  Capability                  Status    Notes
  --------------------------- --------- -------------------------------------------
  Graph views                 SHIPPED   Relationship and hierarchy visualization.
  Import graphs               SHIPPED   Ontology dependency visualization.
  Asserted / inferred views   SHIPPED   Multiple hierarchy modes.

------------------------------------------------------------------------

# Extensibility

  Capability         Status    Notes
  ------------------ --------- -------------------------------------
  Plugin runtime     SHIPPED   Native plugin infrastructure.
  Plugin discovery   SHIPPED   Manifest-based loading.
  Example plugins    SHIPPED   Reference implementations included.

------------------------------------------------------------------------

# Quality Foundation

  -----------------------------------------------------------------------
  Capability              Status                  Notes
  ----------------------- ----------------------- -----------------------
  Automated Rust tests    MATURE                  Extensive repository
                                                  coverage.

  Extension tests         SHIPPED                 UI and integration
                                                  testing.

  Documentation           MATURE                  Broad architectural and
                                                  developer
                                                  documentation.

  Fixtures                SHIPPED                 Ontology fixtures for
                                                  validation and
                                                  regression.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Capabilities Still Advancing

The following areas are intentionally marked as **EXPANDING** because
they exist today but require additional engineering before OntoCode 1.0
can claim complete Protégé Desktop parity:

-   Complete OWL 2 authoring
-   RDF/XML write-back
-   OWL/XML write-back
-   Workspace semantics
-   Full reasoning parity
-   SWRL
-   Stable Plugin SDK
-   Executable parity verification

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   CURRENT_LIMITATIONS.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_GAP_ANALYSIS.md
