# Protégé Desktop Test Port (v0.26)

**Status:** Active — Wave 1 in progress\
**Upstream:** [protegeproject/protege](https://github.com/protegeproject/protege)\
**Machine inventory:** [`parity/protege-test-port.yaml`](../../../parity/protege-test-port.yaml)

------------------------------------------------------------------------

## Purpose

Port **portable Protégé Desktop JUnit behaviors** into OntoCode as
Rust semantic oracle tests. This is **not** a JVM/JUnit runner and does
**not** import Swing/OSGi suites.

OntoCode already validates Protégé-like workflows via
[`tests/protege_roundtrip.rs`](../../../tests/protege_roundtrip.rs) and
catalog oracles. The port adds an explicit upstream → OntoCode map and
fills gaps (hierarchy edit lifecycle, merge/deprecation profiles,
axiom location, reference finder, literal/OBO/ID extract).

------------------------------------------------------------------------

## Approach

1. Enumerate upstream `src/test/java` classes (see inventory YAML).
2. Tag each: `PORT_W1` | `PORT_W2` | `SKIP` | `COVERED`.
3. Rewrite case themes as `#[test]` oracles (patch → reindex → assert
   catalog/Horned semantics). No Turtle byte-identity; XML uses ADR-0021
   semantic compare.
4. Link executables via `ontocode_tests` in the inventory and
   `test_ids` on relevant `PAR-*` rows in
   [`protege-desktop-parity.yaml`](../../../parity/protege-desktop-parity.yaml).

------------------------------------------------------------------------

## Tags

| Tag | Meaning |
|-----|---------|
| `PORT_W1` | High-value OWL/edit behaviors — implement in v0.26 Wave 1 |
| `PORT_W2` | Presentation/helpers (rendering, link extractors, OBO Foundry JSON) — later |
| `SKIP` | Protégé UI/OSGi/prefs/network — do not port |
| `COVERED` | Already adequately covered by existing OntoCode oracles |

------------------------------------------------------------------------

## Wave 1 suites

| Suite | Upstream anchors | OntoCode tests |
|-------|------------------|----------------|
| Hierarchy | `AssertedClassHierarchyTest`, property hierarchy, tabbed parser | `tests/protege_port_hierarchy.rs` |
| Merge | `MergeEntitiesChangeListGenerator_TestCase` | `tests/protege_port_merge.rs` |
| Deprecation | `EntityDeprecator_*`, GO/OBI profiles | `tests/protege_port_deprecation.rs` |
| History / change algebra | `HistoryTest`, `ChangeListMinimizer_TestCase` | `tests/protege_port_history.rs` |
| Axiom location | `*LocationStrategy_TestCase` | `tests/protege_port_axiom_location.rs` |
| Refs / defined-class | `ReferenceFinder_*`, `DefinedClassPredicate_*` | `tests/protege_port_refs.rs` |
| Parsers / IDs | literals, OBO utils, RDF extractor, idranges | `tests/protege_port_parsers.rs` |

Fixtures: [`examples/protege-roundtrip/ported/`](../../../examples/protege-roundtrip/ported/).

------------------------------------------------------------------------

## Explicitly skipped

- `protege-launcher` OSGi bundle tests
- Mac `FileDialog` / Swing (`MacUIUtil_TestCase`)
- Preferences / ORCID prefs managers
- `Folder_IT` / Protégé XML catalog
- Live `IdentifiersDotOrg_IT`
- `NoOpReasoner_TestCase` (use OntoLogos / `tests/reasoner_*.rs`)
- SelectionPlane / PopupMenuId
- Byte-identical XML serialization

------------------------------------------------------------------------

## Validation

```bash
python3 scripts/validate-protege-test-port.py
cargo test -p ontocode --test protege_port_hierarchy --test protege_port_merge \
  --test protege_port_deprecation --test protege_port_history \
  --test protege_port_axiom_location --test protege_port_refs \
  --test protege_port_parsers
```

Every `PORT_W1` inventory row must list `ontocode_tests` paths that exist,
or an explicit `gap` note.

------------------------------------------------------------------------

## Related

- [PARITY_TEST_PLAN.md](PARITY_TEST_PLAN.md)
- [BLOCKER_11_PARITY_VERIFICATION.md](../04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md)
- [examples/protege-roundtrip/README.md](../../../examples/protege-roundtrip/README.md)
