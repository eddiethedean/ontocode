#!/bin/sh
# Thin reference provider entry for SDK 1.0 (reasoner/query/refactor/graph).
set -eu

ACTION="${1:-}"
shift || true

FOCUS=""
while [ $# -gt 0 ]; do
  case "$1" in
    --iri|--root)
      FOCUS="${2:-}"
      shift 2
      ;;
    --workspace|--query|--step|--view)
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

IRI="${FOCUS:-http://example.org/Person}"

case "$ACTION" in
  reasoner.classify)
    printf '{"profile":"demo-stub","unsatisfiable":[],"logs":"demo reasoner ok"}\n'
    ;;
  query.run)
    printf '{"columns":["iri","label"],"rows":[["%s","Person"]],"logs":"demo query ok"}\n' "$IRI"
    ;;
  refactor.preview)
    printf '{"affected_iris":["%s"],"hints":["Rename local name to PascalCase"],"logs":"demo refactor ok"}\n' "$IRI"
    ;;
  graph.build)
    printf '{"graph_kind":"plugin_neighborhood","root_iris":["%s"],"logs":"demo graph ok"}\n' "$IRI"
    ;;
  *)
    printf '{"logs":"unknown action %s","exit_message":"unsupported"}\n' "$ACTION"
    exit 1
    ;;
esac
