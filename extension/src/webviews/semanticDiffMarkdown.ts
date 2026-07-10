import type { DiffPayload } from "./messages";

/** Markdown summary matching the Semantic Diff panel sections (#149). */
export function formatSemanticDiffMarkdown(diff: DiffPayload): string {
  const lines: string[] = ["# Ontology semantic diff", ""];

  const counts = diff.summary_counts ?? {
    entities: diff.entity_changes.length,
    axioms: diff.axiom_changes.length,
    annotations: diff.annotation_changes.length,
    imports: diff.import_changes.length,
    inferences: diff.inference_changes.length,
    breaking: diff.breaking_changes.length,
  };

  lines.push(
    "## Summary",
    "",
    "| Category | Count |",
    "|---|---|",
    `| Entities | ${counts.entities} |`,
    `| Axioms | ${counts.axioms} |`,
    `| Annotations | ${counts.annotations} |`,
    `| Imports | ${counts.imports} |`,
    `| Inferences | ${counts.inferences} |`,
    `| Breaking | ${counts.breaking} |`,
    ""
  );

  if (diff.breaking_changes.length > 0) {
    lines.push("## Breaking changes", "");
    for (const b of diff.breaking_changes) {
      lines.push(`- **${b.reason}**: ${b.message}`);
    }
    lines.push("");
  }

  if (diff.entity_changes.length > 0) {
    lines.push("## Entity changes", "");
    for (const e of diff.entity_changes) {
      const prev = e.previous_iri ? ` ← \`${e.previous_iri}\`` : "";
      lines.push(`- **${e.kind}** \`${e.iri}\`${prev}`);
    }
    lines.push("");
  }

  if (diff.axiom_changes.length > 0) {
    lines.push("## Axiom changes", "");
    for (const a of diff.axiom_changes) {
      lines.push(
        `- **${a.change}** \`${a.axiom_kind}\` — ${a.subject} ${a.predicate} ${a.object}`
      );
    }
    lines.push("");
  }

  if (diff.annotation_changes.length > 0) {
    lines.push("## Annotation changes", "");
    for (const a of diff.annotation_changes) {
      lines.push(`- **${a.change}** \`${a.subject}\` ${a.predicate} ${a.object}`);
    }
    lines.push("");
  }

  if (diff.import_changes.length > 0) {
    lines.push("## Import changes", "");
    for (const i of diff.import_changes) {
      lines.push(`- **${i.change}** ${i.import_iri}`);
    }
    lines.push("");
  }

  if (diff.inference_changes.length > 0) {
    lines.push("## Inference changes", "");
    for (const i of diff.inference_changes) {
      lines.push(`- **${i.change}** \`${i.class_iri}\` — ${i.detail}`);
    }
    lines.push("");
  }

  return lines.join("\n").trimEnd() + "\n";
}
