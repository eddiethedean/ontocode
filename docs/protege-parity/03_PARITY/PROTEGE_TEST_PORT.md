# Protégé Desktop Test Port (v0.26)

**Status:** Waves 1–3 shipped on `v0.26`\
**Upstream:** [protegeproject/protege](https://github.com/protegeproject/protege)\
**Machine inventory:** [`parity/protege-test-port.yaml`](../../../parity/protege-test-port.yaml)

------------------------------------------------------------------------

## Purpose

Port **portable Protégé Desktop JUnit behaviors** into OntoCode as
Rust semantic oracle tests (and UI presentation helpers where the
product already surfaces them). This is **not** a JVM/JUnit runner and
does **not** import Swing/OSGi suites.

------------------------------------------------------------------------

## Approach

1. Enumerate upstream `src/test/java` classes (see inventory YAML).
2. Tag each: `PORT_W1` | `PORT_W2` | `PORT_W3` | `SKIP` | `COVERED`.
3. Rewrite case themes as OntoCode tests (catalog oracles or pure
   helpers). No Turtle byte-identity; XML uses ADR-0021 semantic compare.
4. Link executables via `ontocode_tests` in the inventory and
   `test_ids` on relevant `PAR-*` rows in
   [`protege-desktop-parity.yaml`](../../../parity/protege-desktop-parity.yaml).

------------------------------------------------------------------------

## Tags

| Tag | Meaning |
|-----|---------|
| `PORT_W1` | High-value OWL/edit behaviors — Wave 1 |
| `PORT_W2` | Presentation helpers — Wave 2 (render/escape/prefix/links) |
| `PORT_W3` | Deferred portable utils — Wave 3 (Foundry JSON, abbreviate, ISO8601, …) |
| `SKIP` | Protégé UI/OSGi/prefs/network or no OntoCode product surface |
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

## Wave 2 suites

| Suite | Upstream anchors | OntoCode tests |
|-------|------------------|----------------|
| Render / escape / prefix / IRI | `OWLEntityRendererImpl`, `RenderingEscapeUtils`, `Prefix*`, `IRIExpander`, `IriSplitter` | `tests/protege_port_render.rs` + `crates/ontocore-owl/src/render.rs` |
| Annotation link extractors | `*LinkExtractor`, `RegExBasedLinkExtractor` | `tests/protege_port_links.rs` + `crates/ontocore-owl/src/links.rs` + `extension/webview-ui/src/utils/annotationLinks.ts` |

UX wiring: LSP hover linkifies labels/comments; Entity Inspector renders annotation hyperlinks.

------------------------------------------------------------------------

## Wave 3 suites

| Suite | Upstream anchors | OntoCode tests |
|-------|------------------|----------------|
| Utils | `StringAbbreviator`, `ISO8601Formatter`, `LiteralLexicalValueReplacer`, `MarkdownRenderer`, `AnnotationPropertyComparator` | `tests/protege_port_utils.rs` + `crates/ontocore-owl/src/util.rs` + `extension/webview-ui/src/utils/annotationOrder.ts` |
| OBO Foundry registry | `OboFoundry*` (+ vendored JSON fixture, not live HTTP) | `tests/protege_port_obofoundry.rs` + `crates/ontocore-obo/src/obofoundry.rs` |

UX wiring: Entity Inspector sorts annotations by Protégé default property order.

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
- Breadcrumb / assertion-row comparator VOs (no equivalent UI surface)

------------------------------------------------------------------------

## Validation

```bash
python3 scripts/validate-protege-test-port.py
cargo test -p ontocode --test protege_port_hierarchy --test protege_port_merge \
  --test protege_port_deprecation --test protege_port_history \
  --test protege_port_axiom_location --test protege_port_refs \
  --test protege_port_parsers --test protege_port_render \
  --test protege_port_links --test protege_port_utils \
  --test protege_port_obofoundry
cd extension/webview-ui && npm test -- --run src/utils/annotationLinks.test.ts \
  src/utils/annotationOrder.test.ts
```

Every `PORT_W1` / `PORT_W2` / `PORT_W3` inventory row must list `ontocode_tests`
paths that exist, or an explicit `gap` note.

------------------------------------------------------------------------

## Related

- [PARITY_TEST_PLAN.md](PARITY_TEST_PLAN.md)
- [BLOCKER_11_PARITY_VERIFICATION.md](../04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md)
- [examples/protege-roundtrip/README.md](../../../examples/protege-roundtrip/README.md)
