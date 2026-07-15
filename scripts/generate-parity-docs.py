#!/usr/bin/env python3
"""Sync parity matrix status table / metrics markdown from the YAML manifest.

Usage:
  python3 scripts/generate-parity-docs.py --write
  python3 scripts/generate-parity-docs.py --check
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from parity_common import (  # noqa: E402
    MATRIX,
    METRICS_JSON,
    METRICS_MD,
    ROOT,
    compute_metrics,
    requirements,
)

BEGIN = "<!-- BEGIN PARITY_METRICS_GENERATED -->"
END = "<!-- END PARITY_METRICS_GENERATED -->"

# Stable presentation extras for the living matrix (not sourced from YAML).
MATRIX_EXTRAS: dict[str, tuple[str, str, str]] = {
    "PAR-LIFE-001": ("IMPLEMENTATION_EVIDENCE", "dialogs/workflows", "Multi-format save incomplete"),
    "PAR-LIFE-002": ("IMPLEMENTATION_EVIDENCE", "workspace.runtime", "v0.20 workspace runtime"),
    "PAR-FMT-001": ("IMPLEMENTATION_EVIDENCE", "round-trip tests", "Primary Turtle workflow"),
    "PAR-FMT-002": ("IMPLEMENTATION_EVIDENCE", "round-trip tests", "Expand OBO corpus"),
    "PAR-FMT-003": ("ADR-0021", "xml_writeback", "Closed v0.21"),
    "PAR-FMT-004": ("ADR-0021", "xml_writeback", "Closed v0.21"),
    "PAR-OWL-001": ("OWL2_AUTHORING", "owl2_authoring", "Shipped v0.22"),
    "PAR-WS-001": ("BLOCKER_03", "workspace tests", "v0.20 workspace runtime"),
    "PAR-RSN-001": ("IMPLEMENTATION_EVIDENCE", "reasoner_el", "Classification"),
    "PAR-RSN-002": ("BLOCKER_04", "reasoner_abox", "Shipped v0.23"),
    "PAR-RSN-003": ("BLOCKER_04", "explain tests", "Shipped v0.23"),
    "PAR-QRY-001": ("IMPLEMENTATION_EVIDENCE", "query tests", "SPARQL"),
    "PAR-QRY-002": ("BLOCKER_07", "dl_query + UI", "Shipped v0.24"),
    "PAR-SWRL-001": ("BLOCKER_05", "swrl tests", "Shipped v0.23"),
    "PAR-REF-001": ("IMPLEMENTATION_EVIDENCE", "refactor tests", "Multi-format rename/merge/replace"),
    "PAR-VIS-001": ("BLOCKER_08", "GraphPanel tests", "Shipped v0.25 EPIC-008"),
    "PAR-PLG-001": ("BLOCKER_09", "plugin_sdk_compat", "Shipped v0.25 EPIC-009"),
    "PAR-ACC-001": ("ACCESSIBILITY_REPORT", "a11y Vitest", "Shipped v0.25 EPIC-010"),
    "PAR-TST-001": ("BLOCKER_11", "parity CI scripts", "Shipped v0.25 EPIC-011"),
}

MATRIX_ORDER = [
    "PAR-LIFE-001",
    "PAR-LIFE-002",
    "PAR-FMT-001",
    "PAR-FMT-002",
    "PAR-FMT-003",
    "PAR-FMT-004",
    "PAR-OWL-001",
    "PAR-WS-001",
    "PAR-RSN-001",
    "PAR-RSN-002",
    "PAR-RSN-003",
    "PAR-QRY-001",
    "PAR-QRY-002",
    "PAR-SWRL-001",
    "PAR-REF-001",
    "PAR-VIS-001",
    "PAR-PLG-001",
    "PAR-ACC-001",
    "PAR-TST-001",
]


def _pad(s: str, n: int) -> str:
    s = s[:n]
    return s + " " * (n - len(s))


def build_matrix_table(reqs: list[dict]) -> str:
    by_id = {r["id"]: r for r in reqs}
    lines = [
        "# Matrix",
        "",
        "  ----------------------------------------------------------------------------------------------------------------------------------------------",
        "  ID             Area            Requirement         Priority   Current Status    Evidence                   Tests              Notes",
        "  -------------- --------------- ------------------- ---------- ----------------- -------------------------- ------------------ ----------------",
    ]
    for rid in MATRIX_ORDER:
        r = by_id[rid]
        ev, tests, notes = MATRIX_EXTRAS[rid]
        lines.append(
            f"  {_pad(rid, 14)} {_pad(r['area'], 15)} {_pad(r['title'], 19)} {_pad(r['priority'], 10)} "
            f"{_pad(r['status'], 17)} {_pad(ev, 26)} {_pad(tests, 18)} {notes}"
        )
    lines.append(
        "  ----------------------------------------------------------------------------------------------------------------------------------------------"
    )
    lines.append("")
    return "\n".join(lines)


def rewrite_matrix(text: str, reqs: list[dict]) -> str:
    table = build_matrix_table(reqs)
    pat = re.compile(r"# Matrix\n.*?(?=\n# Traceability)", re.S)
    if not pat.search(text):
        raise ValueError("could not find Matrix section before Traceability")
    return pat.sub(table, text)


def matrix_status_errors(text: str, reqs: list[dict]) -> list[str]:
    """Ensure each requirement ID appears with its YAML status."""
    errors: list[str] = []
    for req in reqs:
        req_id = req["id"]
        status = req["status"]
        # Prefer the generated one-line table row.
        m = re.search(rf"(?m)^\s*{re.escape(req_id)}\b.*$", text)
        if not m:
            errors.append(f"matrix missing id row: {req_id}")
            continue
        row = m.group(0)
        if not re.search(rf"\bP[012]\s+{re.escape(status)}\b", row):
            errors.append(f"matrix status mismatch for {req_id}: expected {status} in row")
    return errors


def metrics_block(metrics: dict) -> str:
    p0 = metrics["P0"]
    p1 = metrics["P1"]
    return "\n".join(
        [
            BEGIN,
            "",
            "_Auto-generated by `scripts/generate-parity-docs.py` from_",
            "`parity/protege-desktop-parity.yaml` (do not edit by hand).",
            "",
            "| Metric | Value |",
            "|--------|-------|",
            f"| P0 VERIFIED | {p0['verified']}/{p0['total']} ({p0['verified_pct']}%) |",
            f"| P0 COMPLETE (not yet VERIFIED) | {p0['complete']} |",
            f"| P0 PARTIAL | {p0['partial']} |",
            f"| P1 VERIFIED | {p1['verified']}/{p1['total']} ({p1['verified_pct']}%) |",
            f"| Gate 3 release_ready (all P0 VERIFIED) | `{str(metrics['release_ready']).lower()}` |",
            "",
            "Machine-readable snapshot: [`parity/metrics.json`](../../../parity/metrics.json).",
            "",
            END,
            "",
        ]
    )


def upsert_metrics_md(text: str, block: str) -> str:
    if BEGIN in text and END in text:
        pattern = re.compile(re.escape(BEGIN) + r".*?" + re.escape(END), re.DOTALL)
        return pattern.sub(block.strip(), text, count=1)
    marker = "------------------------------------------------------------------------\n"
    idx = text.find(marker)
    if idx == -1:
        return text.rstrip() + "\n\n" + block
    second = text.find(marker, idx + len(marker))
    insert_at = second + len(marker) if second != -1 else idx + len(marker)
    return text[:insert_at] + "\n" + block + text[insert_at:]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true", help="Update matrix + metrics docs")
    mode.add_argument("--check", action="store_true", help="Fail if docs are stale")
    args = parser.parse_args()

    try:
        reqs = requirements()
        metrics = compute_metrics(reqs)
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    matrix_src = MATRIX.read_text(encoding="utf-8")
    matrix_new = rewrite_matrix(matrix_src, reqs)
    metrics_src = METRICS_MD.read_text(encoding="utf-8") if METRICS_MD.is_file() else ""
    metrics_new = upsert_metrics_md(metrics_src, metrics_block(metrics))
    metrics_json = json.dumps(metrics, indent=2) + "\n"

    if args.write:
        MATRIX.write_text(matrix_new, encoding="utf-8")
        METRICS_MD.write_text(metrics_new, encoding="utf-8")
        METRICS_JSON.write_text(metrics_json, encoding="utf-8")
        print(f"wrote {MATRIX.relative_to(ROOT)}")
        print(f"wrote {METRICS_MD.relative_to(ROOT)}")
        print(f"wrote {METRICS_JSON.relative_to(ROOT)}")
        # Idempotency assert.
        again = rewrite_matrix(MATRIX.read_text(encoding="utf-8"), reqs)
        if again != MATRIX.read_text(encoding="utf-8"):
            print("error: matrix rewrite is not idempotent", file=sys.stderr)
            return 1
        return 0

    stale = []
    if matrix_src != matrix_new:
        stale.append(str(MATRIX.relative_to(ROOT)))
    else:
        for err in matrix_status_errors(matrix_src, reqs):
            stale.append(err)
    if metrics_src != metrics_new:
        stale.append(str(METRICS_MD.relative_to(ROOT)))
    if not METRICS_JSON.is_file() or METRICS_JSON.read_text(encoding="utf-8") != metrics_json:
        stale.append(str(METRICS_JSON.relative_to(ROOT)))

    if stale:
        print(
            "parity docs stale — run: python3 scripts/generate-parity-docs.py --write",
            file=sys.stderr,
        )
        for path in stale:
            print(f"  - {path}", file=sys.stderr)
        return 1

    print("ok: parity docs in sync with manifest")
    return 0


if __name__ == "__main__":
    sys.exit(main())
