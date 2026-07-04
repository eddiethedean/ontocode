use crate::model::DiffResult;

pub fn format_diff_text(diff: &DiffResult, breaking_only: bool) -> String {
    let mut out = String::new();
    if !breaking_only {
        out.push_str(&format!(
            "Summary: {} entity, {} axiom, {} annotation, {} import, {} inference, {} breaking\n\n",
            diff.entity_changes.len(),
            diff.axiom_changes.len(),
            diff.annotation_changes.len(),
            diff.import_changes.len(),
            diff.inference_changes.len(),
            diff.breaking_changes.len(),
        ));
    }
    if !diff.breaking_changes.is_empty() {
        out.push_str("Breaking changes:\n");
        for b in &diff.breaking_changes {
            out.push_str(&format!("  - [{}] {}\n", reason_label(b.reason), b.message));
        }
        out.push('\n');
    }
    if breaking_only {
        return out;
    }
    if !diff.entity_changes.is_empty() {
        out.push_str("Entity changes:\n");
        for e in &diff.entity_changes {
            out.push_str(&format!("  - {:?} {}\n", e.kind, e.iri));
        }
        out.push('\n');
    }
    if !diff.axiom_changes.is_empty() {
        out.push_str("Axiom changes:\n");
        for a in &diff.axiom_changes {
            out.push_str(&format!(
                "  - {} {} {} {} {}\n",
                a.change, a.axiom_kind, a.subject, a.predicate, a.object
            ));
        }
        out.push('\n');
    }
    if !diff.annotation_changes.is_empty() {
        out.push_str("Annotation changes:\n");
        for a in &diff.annotation_changes {
            out.push_str(&format!("  - {} {} {} {}\n", a.change, a.subject, a.predicate, a.object));
        }
        out.push('\n');
    }
    if !diff.import_changes.is_empty() {
        out.push_str("Import changes:\n");
        for i in &diff.import_changes {
            out.push_str(&format!("  - {} {}\n", i.change, i.import_iri));
        }
        out.push('\n');
    }
    out
}

pub fn format_diff_markdown(diff: &DiffResult, breaking_only: bool) -> String {
    let mut out = String::from("# Ontology semantic diff\n\n");
    if !diff.breaking_changes.is_empty() {
        out.push_str("## Breaking changes\n\n");
        for b in &diff.breaking_changes {
            out.push_str(&format!("- **{}**: {}\n", reason_label(b.reason), b.message));
        }
        out.push('\n');
    }
    if breaking_only {
        return out;
    }
    out.push_str(&format!(
        "## Summary\n\n| Category | Count |\n|---|---|\n| Entities | {} |\n| Axioms | {} |\n| Annotations | {} |\n| Imports | {} |\n| Inferences | {} |\n| Breaking | {} |\n\n",
        diff.entity_changes.len(),
        diff.axiom_changes.len(),
        diff.annotation_changes.len(),
        diff.import_changes.len(),
        diff.inference_changes.len(),
        diff.breaking_changes.len(),
    ));
    out
}

pub fn format_diff_json(diff: &DiffResult) -> String {
    serde_json::to_string_pretty(diff).unwrap_or_else(|_| "{}".to_string())
}

fn reason_label(reason: crate::model::BreakingReason) -> &'static str {
    use crate::model::BreakingReason::*;
    match reason {
        RemovedEntity => "removed_entity",
        RenamedIri => "renamed_iri",
        RemovedSuperclass => "removed_superclass",
        RemovedImport => "removed_import",
        UnsatisfiableClass => "unsatisfiable_class",
        DomainRangeChange => "domain_range_change",
    }
}
