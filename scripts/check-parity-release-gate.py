#!/usr/bin/env python3
"""Parity release-gate report + metrics (EPIC-011 / BLOCKER_11).

Always evaluates infrastructure (evidence paths) and prints Gate 3 readiness.
Exit codes:
  0 — infra OK (Gate 3 may still be open at v0.25)
  1 — infra failures (missing paths / incomplete evidence) OR --strict-release
      when not all P0 are VERIFIED

Usage:
  python3 scripts/check-parity-release-gate.py
  python3 scripts/check-parity-release-gate.py --strict-release
  python3 scripts/check-parity-release-gate.py --write-metrics
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from parity_common import (  # noqa: E402
    METRICS_JSON,
    ROOT,
    compute_metrics,
    evidence_errors,
    load_manifest,
    requirements,
)


def format_report(metrics: dict, infra_errors: list[str]) -> str:
    lines = [
        "OntoCode parity release-gate report",
        f"  target={metrics.get('target')} baseline={metrics.get('baseline')}",
        "",
        "Gate 3 — P0 VERIFIED (1.0.0 requirement):",
        (
            f"  P0 verified {metrics['P0']['verified']}/{metrics['P0']['total']}"
            f" ({metrics['P0']['verified_pct']}%)"
            f"  complete={metrics['P0']['complete']}"
            f"  partial={metrics['P0']['partial']}"
        ),
        f"  release_ready={metrics['release_ready']}",
        "",
        "P1 snapshot:",
        (
            f"  P1 verified {metrics['P1']['verified']}/{metrics['P1']['total']}"
            f" ({metrics['P1']['verified_pct']}%)"
        ),
        "",
        "Infrastructure (evidence paths / completeness):",
    ]
    if infra_errors:
        lines.append(f"  FAIL ({len(infra_errors)} issue(s))")
        for err in infra_errors:
            lines.append(f"    - {err}")
    else:
        lines.append("  OK")
    lines.append("")
    lines.append(
        "Note: Gate 3 hard-fail is --strict-release (1.0.0-rc)."
        " v0.25 CI requires infrastructure OK only."
    )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--strict-release",
        action="store_true",
        help="Also fail when any P0 is not VERIFIED (1.0.0-rc)",
    )
    parser.add_argument(
        "--write-metrics",
        action="store_true",
        help=f"Write machine-readable metrics to {METRICS_JSON.relative_to(ROOT)}",
    )
    args = parser.parse_args()

    try:
        reqs = requirements()
        metrics = compute_metrics(reqs)
        infra = evidence_errors(reqs)
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    print(format_report(metrics, infra))

    if args.write_metrics:
        METRICS_JSON.parent.mkdir(parents=True, exist_ok=True)
        METRICS_JSON.write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")
        print(f"wrote {METRICS_JSON.relative_to(ROOT)}")

    if infra:
        return 1
    if args.strict_release and not metrics["release_ready"]:
        print(
            "strict-release: Gate 3 failed — not all P0 requirements are VERIFIED",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
