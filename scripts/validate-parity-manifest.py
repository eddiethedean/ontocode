#!/usr/bin/env python3
"""Validate parity/protege-desktop-parity.yaml (schema, matrix IDs, evidence paths).

Usage:
  python3 scripts/validate-parity-manifest.py           # schema + matrix IDs
  python3 scripts/validate-parity-manifest.py --paths   # also evidence/path checks
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

# Allow `python scripts/validate-parity-manifest.py` without installing a package.
sys.path.insert(0, str(Path(__file__).resolve().parent))

from parity_common import (  # noqa: E402
    MANIFEST,
    MATRIX,
    REQUIRED_FIELDS,
    ROOT,
    VALID_PRIORITIES,
    VALID_STATUSES,
    evidence_errors,
    load_manifest,
    matrix_ids,
)


def validate_schema() -> list[str]:
    errors: list[str] = []
    data = load_manifest()

    for key in ("schema_version", "baseline", "target", "requirements"):
        if key not in data:
            errors.append(f"missing top-level field: {key}")

    requirements = data.get("requirements")
    if not isinstance(requirements, list) or not requirements:
        errors.append("requirements must be a non-empty list")
        return errors

    seen: set[str] = set()
    for i, req in enumerate(requirements):
        if not isinstance(req, dict):
            errors.append(f"requirements[{i}] must be a mapping")
            continue

        missing = REQUIRED_FIELDS - req.keys()
        if missing:
            errors.append(f"requirements[{i}] missing fields: {sorted(missing)}")

        req_id = req.get("id")
        if not isinstance(req_id, str) or not re.fullmatch(r"PAR-[A-Z]+-\d{3}", req_id):
            errors.append(f"requirements[{i}] invalid id: {req_id!r}")
            continue

        if req_id in seen:
            errors.append(f"duplicate requirement id: {req_id}")
        seen.add(req_id)

        priority = req.get("priority")
        if priority not in VALID_PRIORITIES:
            errors.append(f"{req_id}: invalid priority {priority!r}")

        status = req.get("status")
        if status not in VALID_STATUSES:
            errors.append(f"{req_id}: invalid status {status!r}")

        for list_field in ("source_files", "test_ids", "acceptance_criteria", "documentation"):
            value = req.get(list_field)
            if value is not None and not isinstance(value, list):
                errors.append(f"{req_id}: {list_field} must be a list")

    expected = matrix_ids()
    if expected:
        missing_in_manifest = expected - seen
        extra_in_manifest = seen - expected
        if missing_in_manifest:
            errors.append(
                "manifest missing matrix ids: " + ", ".join(sorted(missing_in_manifest))
            )
        if extra_in_manifest:
            errors.append(
                "manifest has ids not in matrix: " + ", ".join(sorted(extra_in_manifest))
            )

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--paths",
        action="store_true",
        help="Also validate evidence completeness and path existence",
    )
    args = parser.parse_args()

    try:
        errors = validate_schema()
        if args.paths and not errors:
            errors.extend(evidence_errors())
        elif args.paths and errors:
            # Still report evidence if schema mostly loads.
            try:
                errors.extend(evidence_errors())
            except Exception:
                pass
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if errors:
        print("parity manifest validation failed:", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    n = len(load_manifest()["requirements"])
    mode = "schema+paths" if args.paths else "schema"
    print(f"ok: {MANIFEST.relative_to(ROOT)} ({n} requirements, {mode})")
    if not MATRIX.is_file():
        print(f"warning: matrix missing at {MATRIX}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
