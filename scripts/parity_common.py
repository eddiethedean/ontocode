#!/usr/bin/env python3
"""Shared helpers for OntoCode parity manifest tooling (EPIC-011)."""

from __future__ import annotations

import re
import re
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    raise SystemExit("error: PyYAML required (pip install pyyaml)") from exc

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = ROOT / "parity" / "protege-desktop-parity.yaml"
MATRIX = ROOT / "docs" / "protege-parity" / "03_PARITY" / "PARITY_MATRIX.md"
METRICS_MD = ROOT / "docs" / "protege-parity" / "03_PARITY" / "PARITY_METRICS.md"
METRICS_JSON = ROOT / "parity" / "metrics.json"

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

STATUS_RE = re.compile(
    r"\b(COMPLETE|PARTIAL|NOT_IMPLEMENTED|BLOCKED|VERIFIED)\b"
)


def load_manifest() -> dict[str, Any]:
    if not MANIFEST.is_file():
        raise FileNotFoundError(f"missing manifest: {MANIFEST}")
    data = yaml.safe_load(MANIFEST.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError("manifest root must be a mapping")
    return data


def requirements(data: dict[str, Any] | None = None) -> list[dict[str, Any]]:
    payload = data if data is not None else load_manifest()
    reqs = payload.get("requirements")
    if not isinstance(reqs, list):
        raise ValueError("requirements must be a list")
    return [r for r in reqs if isinstance(r, dict)]


def matrix_ids() -> set[str]:
    text = MATRIX.read_text(encoding="utf-8")
    return set(re.findall(r"\bPAR-[A-Z]+-\d{3}\b", text))


def resolve_repo_path(rel: str) -> Path:
    """Map a manifest path or rust module suffix to a repo Path."""
    cleaned = rel.strip()
    # Strip rust module suffixes: path.rs::tests
    cleaned = cleaned.split("::", 1)[0]
    return ROOT / cleaned


def looks_like_path(value: str) -> bool:
    if "/" in value or value.endswith((".rs", ".ts", ".tsx", ".py", ".md", ".toml", ".yaml", ".yml")):
        return True
    if value.endswith("/"):
        return True
    return False


def path_exists(rel: str) -> bool:
    path = resolve_repo_path(rel)
    return path.exists()


def compute_metrics(reqs: list[dict[str, Any]] | None = None) -> dict[str, Any]:
    items = reqs if reqs is not None else requirements()
    by_pri: dict[str, list[dict[str, Any]]] = {"P0": [], "P1": [], "P2": []}
    for req in items:
        pri = req.get("priority")
        if pri in by_pri:
            by_pri[pri].append(req)

    def bucket(rows: list[dict[str, Any]]) -> dict[str, Any]:
        total = len(rows)
        counts = {s: 0 for s in VALID_STATUSES}
        for row in rows:
            st = row.get("status")
            if st in counts:
                counts[st] += 1
        verified = counts["VERIFIED"]
        complete = counts["COMPLETE"]
        return {
            "total": total,
            "verified": verified,
            "complete": complete,
            "partial": counts["PARTIAL"],
            "not_implemented": counts["NOT_IMPLEMENTED"],
            "blocked": counts["BLOCKED"],
            "verified_pct": round(100.0 * verified / total, 1) if total else 0.0,
            "complete_or_verified_pct": round(
                100.0 * (verified + complete) / total, 1
            )
            if total
            else 0.0,
        }

    p0 = bucket(by_pri["P0"])
    p1 = bucket(by_pri["P1"])
    release_ready = p0["total"] > 0 and p0["verified"] == p0["total"]
    return {
        "schema_version": load_manifest().get("schema_version"),
        "baseline": load_manifest().get("baseline"),
        "target": load_manifest().get("target"),
        "P0": p0,
        "P1": p1,
        "P2": bucket(by_pri["P2"]),
        "release_ready": release_ready,
        "gate3_p0_verified": release_ready,
    }


def evidence_errors(reqs: list[dict[str, Any]] | None = None) -> list[str]:
    """Path existence + evidence completeness (Stage 9 CI infra)."""
    errors: list[str] = []
    for req in reqs if reqs is not None else requirements():
        req_id = req.get("id", "?")
        status = req.get("status")
        sources = req.get("source_files") or []
        tests = req.get("test_ids") or []
        docs = req.get("documentation") or []
        criteria = req.get("acceptance_criteria") or []

        if not isinstance(sources, list):
            errors.append(f"{req_id}: source_files must be a list")
            continue
        if not isinstance(tests, list):
            errors.append(f"{req_id}: test_ids must be a list")
            continue

        if status in {"COMPLETE", "VERIFIED"}:
            if not sources:
                errors.append(f"{req_id}: COMPLETE/VERIFIED requires non-empty source_files")
            if not tests:
                errors.append(f"{req_id}: COMPLETE/VERIFIED requires non-empty test_ids")
        elif status == "PARTIAL":
            if not sources and not tests and not docs:
                errors.append(
                    f"{req_id}: PARTIAL requires source_files, test_ids, or documentation"
                )

        for rel in sources:
            if not isinstance(rel, str) or not rel.strip():
                errors.append(f"{req_id}: empty source_files entry")
                continue
            if not path_exists(rel):
                errors.append(f"{req_id}: missing source_files path: {rel}")

        for rel in docs:
            if not isinstance(rel, str):
                continue
            if looks_like_path(rel) and not path_exists(rel):
                errors.append(f"{req_id}: missing documentation path: {rel}")

        for rel in criteria:
            if not isinstance(rel, str):
                continue
            # Only enforce when the entry looks like a repo-relative file path.
            if re.match(r"^(docs|parity|scripts|crates|extension|tests)/", rel) or (
                "/" in rel and rel.endswith((".md", ".yaml", ".yml", ".toml", ".rs", ".ts", ".tsx", ".py"))
            ):
                if not path_exists(rel):
                    errors.append(f"{req_id}: missing acceptance_criteria path: {rel}")

        for tid in tests:
            if not isinstance(tid, str) or not tid.strip():
                errors.append(f"{req_id}: empty test_ids entry")
                continue
            if looks_like_path(tid) and not path_exists(tid):
                errors.append(f"{req_id}: missing test_ids path: {tid}")

    return errors
