#!/usr/bin/env python3
"""Validate parity/protege-desktop-parity.yaml against schema and matrix coverage."""

from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print("error: PyYAML required (pip install pyyaml)", file=sys.stderr)
    sys.exit(2)

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = ROOT / "parity" / "protege-desktop-parity.yaml"
MATRIX = ROOT / "docs" / "protege-parity" / "03_PARITY" / "PARITY_MATRIX.md"

VALID_PRIORITIES = {"P0", "P1", "P2"}
VALID_STATUSES = {
    "COMPLETE",
    "PARTIAL",
    "NOT_IMPLEMENTED",
    "BLOCKED",
    "VERIFIED",
}
REQUIRED_FIELDS = {
    "id",
    "area",
    "title",
    "priority",
    "status",
    "owner",
    "source_files",
    "test_ids",
    "acceptance_criteria",
    "github_issue",
    "documentation",
}


def matrix_ids() -> set[str]:
    text = MATRIX.read_text(encoding="utf-8")
    return set(re.findall(r"\bPAR-[A-Z]+-\d{3}\b", text))


def load_manifest() -> dict:
    if not MANIFEST.is_file():
        raise FileNotFoundError(f"missing manifest: {MANIFEST}")
    data = yaml.safe_load(MANIFEST.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError("manifest root must be a mapping")
    return data


def validate() -> list[str]:
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
    try:
        errors = validate()
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if errors:
        print("parity manifest validation failed:", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print(f"ok: {MANIFEST.relative_to(ROOT)} ({len(load_manifest()['requirements'])} requirements)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
