# PARITY_MATRIX

# Protégé Desktop Parity Matrix

**Status:** Living Specification\
**Target Release:** OntoCode 1.0.0

------------------------------------------------------------------------

# Purpose

The parity matrix is the canonical source of truth for tracking
OntoCode's progress toward full Protégé Desktop parity.

Unlike the feature inventory, which lists Protégé capabilities, this
document maps each requirement to its implementation status, evidence,
testing, and acceptance criteria.

Every parity claim must be traceable to this matrix.

------------------------------------------------------------------------

# Status Values

  -----------------------------------------------------------------------
  Status                              Meaning
  ----------------------------------- -----------------------------------
  COMPLETE                            Fully implemented and verified.

  PARTIAL                             Significant implementation exists
                                      but parity is incomplete.

  NOT_IMPLEMENTED                     No meaningful implementation
                                      exists.

  BLOCKED                             Waiting on prerequisite
                                      architectural work.

  VERIFIED                            COMPLETE and all acceptance tests
                                      have passed.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Priority Levels

  Priority   Description
  ---------- -------------------------------------
  P0         Required before 1.0 release.
  P1         Important but not release-blocking.
  P2         Post-1.0 enhancement.

------------------------------------------------------------------------

# Matrix

  ----------------------------------------------------------------------------------------------------------------------------------------------
  ID             Area            Requirement         Priority   Current Status    Evidence                   Tests              Notes
  -------------- --------------- ------------------- ---------- ----------------- -------------------------- ------------------ ----------------
  PAR-LIFE-001   Lifecycle       Create/Open/Save    P0         PARTIAL           IMPLEMENTATION_EVIDENCE    PARITY_TEST_PLAN   Multi-format
                                 ontology                                                                                       save incomplete

  PAR-LIFE-002   Lifecycle       Multi-ontology      P0         PARTIAL           CURRENT_REPOSITORY_AUDIT   PARITY_TEST_PLAN   Session
                                 workspace                                                                                      restoration gap

  PAR-FMT-001    Formats         Turtle semantic     P0         COMPLETE          IMPLEMENTATION_EVIDENCE    Round-trip         Primary workflow
                                 round-trip                                                                                     

  PAR-FMT-002    Formats         OBO semantic        P0         PARTIAL           IMPLEMENTATION_EVIDENCE    Round-trip         Expand corpus
                                 round-trip                                                                                     

  PAR-FMT-003    Formats         RDF/XML write-back  P0         NOT_IMPLEMENTED   Audit                      Missing            Major blocker

  PAR-FMT-004    Formats         OWL/XML write-back  P0         NOT_IMPLEMENTED   Audit                      Missing            Major blocker

  PAR-OWL-001    OWL2            Complete OWL2       P0         PARTIAL           Audit                      OWL fixtures       Structural gaps
                                 authoring                                                                                      remain

  PAR-WS-001     Workspace       Persistent          P0         PARTIAL           Audit                      UI/E2E             Restore live
                                 workspace semantics                                                                            panels

  PAR-RSN-001    Reasoning       Classification      P0         COMPLETE          IMPLEMENTATION_EVIDENCE    Reasoning tests    

  PAR-RSN-002    Reasoning       Full ABox reasoning P0         PARTIAL           Audit                      Conformance        Major gap

  PAR-RSN-003    Reasoning       Native DL           P0         PARTIAL           Audit                      Explanation tests  
                                 explanations                                                                                   

  PAR-QRY-001    Query           SPARQL              P0         COMPLETE          IMPLEMENTATION_EVIDENCE    Query tests        

  PAR-QRY-002    Query           DL Query workflow   P1         PARTIAL           Audit                      UI tests           

  PAR-SWRL-001   SWRL            Rule                P0         NOT_IMPLEMENTED   Audit                      Missing            Major blocker
                                 authoring/editing                                                                              

  PAR-REF-001    Refactoring     Semantic            P0         PARTIAL           IMPLEMENTATION_EVIDENCE    Refactor tests     Turtle-centric
                                 refactoring                                                                                    

  PAR-VIS-001    Visualization   Graph parity        P1         PARTIAL           Audit                      UI tests           

  PAR-PLG-001    Plugins         Stable SDK          P1         PARTIAL           IMPLEMENTATION_EVIDENCE    SDK tests          

  PAR-ACC-001    Accessibility   Keyboard & screen   P1         PARTIAL           Audit                      Accessibility      
                                 reader parity                                                                                  

  PAR-TST-001    Verification    Executable parity   P0         NOT_IMPLEMENTED   Audit                      Missing            Critical release
                                 corpus                                                                                         gate
  ----------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------

# Traceability

Every row should ultimately link to:

-   Source implementation
-   Responsible crate(s)
-   GitHub issue
-   Acceptance criteria
-   Automated tests
-   Documentation

------------------------------------------------------------------------

# Release Rule

OntoCode 1.0 may claim Protégé Desktop parity only when:

-   Every P0 requirement is VERIFIED.
-   Release gates are satisfied.
-   Remaining P1/P2 items are documented and accepted.

------------------------------------------------------------------------

# Maintenance

Whenever implementation changes:

1.  Update this matrix.
2.  Update IMPLEMENTATION_EVIDENCE.md.
3.  Update PARITY_GAP_ANALYSIS.md.
4.  Update automated tests.
5.  Re-run parity validation.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PARITY_SCOPE.md
-   CURRENT_REPOSITORY_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
