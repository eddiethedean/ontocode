//! SQL virtual table schema metadata for the query workbench.

use ontocore_catalog::OntologyCatalog;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SqlColumnSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub column_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SqlTableSchema {
    pub name: String,
    pub columns: Vec<SqlColumnSchema>,
}

/// Static table definitions (columns are stable for v0.13).
pub fn list_sql_tables() -> Vec<SqlTableSchema> {
    vec![
        table(
            "ontologies",
            &["id", "path", "format", "base_iri", "parse_status", "content_hash", "modified_time"],
        ),
        table(
            "classes",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "object_properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "data_properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "annotation_properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "individuals",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "entities",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table(
            "properties",
            &[
                "iri",
                "short_name",
                "kind",
                "ontology_id",
                "labels",
                "comments",
                "deprecated",
                "obo_id",
            ],
        ),
        table("annotations", &["subject", "predicate", "object", "ontology_id"]),
        table("axioms", &["id", "ontology_id", "subject", "predicate", "object", "axiom_kind"]),
        table("restrictions", &["class_iri", "property_iri", "restriction_kind", "filler"]),
        table("equivalent_class_axioms", &["class_iri", "expression"]),
        table("disjoint_class_axioms", &["class_iri", "disjoint_with"]),
        table("domain_axioms", &["property_iri", "domain"]),
        table("range_axioms", &["property_iri", "range"]),
        table("namespaces", &["prefix", "iri", "ontology_id"]),
        table("imports", &["ontology_id", "import_iri"]),
        table(
            "diagnostics",
            &["code", "severity", "message", "file", "line", "column", "entity_iri"],
        ),
    ]
}

fn table(name: &str, columns: &[&str]) -> SqlTableSchema {
    SqlTableSchema {
        name: name.to_string(),
        columns: columns
            .iter()
            .map(|c| SqlColumnSchema { name: (*c).to_string(), column_type: "string".to_string() })
            .collect(),
    }
}

/// Returns schema; `catalog` reserved for future dynamic columns.
pub fn list_sql_schema(_catalog: &OntologyCatalog) -> Vec<SqlTableSchema> {
    list_sql_tables()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_axiom_tables() {
        let names: Vec<_> = list_sql_tables().into_iter().map(|t| t.name).collect();
        assert!(names.contains(&"restrictions".to_string()));
        assert!(names.contains(&"equivalent_class_axioms".to_string()));
    }
}
