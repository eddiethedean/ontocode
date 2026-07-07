#!/usr/bin/env bash
# Download large-ontology benchmark fixtures (gitignored under tests/benchmarks/).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="${ROOT}/tests/benchmarks"
mkdir -p "$DEST"

echo "Benchmark fixtures directory: $DEST"
echo "v0.13 uses repository fixtures for CI smoke benchmarks."
echo "For GO/SNOMED subsets, place pinned OWL/Turtle trees under:"
echo "  $DEST/go-subset/"
echo "  $DEST/snomed-subset/"
echo ""
echo "Suggested sources:"
echo "  - Gene Ontology: https://geneontology.org/docs/download-ontology/"
echo "  - SNOMED CT International RF2 (license required)"
echo ""
echo "Run: cargo test bench_index_smoke -- --nocapture"
