//! Minimal OBO Format 1.4 parser → OntoCore catalog model.

use ontocore_core::{
    limits::MAX_TRIPLES_PER_FILE, Annotation, Axiom, Entity, EntityKind, SourceLocation,
    AXIOM_KIND_SUB_CLASS_OF,
};
use std::collections::BTreeMap;
use std::path::Path;

use crate::rdf::{assemble_parsed_ontology, ParseError, ParsedOntology, Result};

pub fn parse_obo_text(path: &Path, ontology_id: &str, source_text: &str) -> Result<ParsedOntology> {
    let mut namespaces = BTreeMap::new();
    let mut base_iri = String::from("http://purl.obolibrary.org/obo/");
    let mut entities = Vec::new();
    let mut annotations = Vec::new();
    let mut axioms = Vec::new();
    let mut axiom_counter = 0usize;

    let mut in_term = false;
    let mut current_id: Option<String> = None;
    let mut current_iri: Option<String> = None;
    let mut labels = Vec::new();
    let mut comments = Vec::new();
    let mut deprecated = false;

    let flush_term = |entities: &mut Vec<Entity>,
                      annotations: &mut Vec<Annotation>,
                      axioms: &mut Vec<Axiom>,
                      axiom_counter: &mut usize,
                      ontology_id: &str,
                      base_iri: &str,
                      current_id: &mut Option<String>,
                      current_iri: &mut Option<String>,
                      labels: &mut Vec<String>,
                      comments: &mut Vec<String>,
                      deprecated: &mut bool| {
        if let (Some(obo_id), Some(iri)) = (current_id.take(), current_iri.take()) {
            let short_name = obo_id.split(':').next_back().unwrap_or(&obo_id).to_string();
            entities.push(Entity {
                iri: iri.clone(),
                short_name,
                kind: EntityKind::Class,
                ontology_id: ontology_id.to_string(),
                source_location: SourceLocation::default(),
                labels: std::mem::take(labels),
                comments: std::mem::take(comments),
                deprecated: *deprecated,
                obo_id: Some(obo_id),
            });
            *deprecated = false;
            let _ = annotations;
            let _ = axioms;
            let _ = axiom_counter;
            let _ = base_iri;
        }
    };

    for line in source_text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('!') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            flush_term(
                &mut entities,
                &mut annotations,
                &mut axioms,
                &mut axiom_counter,
                ontology_id,
                &base_iri,
                &mut current_id,
                &mut current_iri,
                &mut labels,
                &mut comments,
                &mut deprecated,
            );
            in_term = line == "[Term]";
            continue;
        }
        if !in_term {
            if let Some(value) = line.strip_prefix("ontology:") {
                let ont_id = value.trim();
                base_iri = format!("http://purl.obolibrary.org/obo/{ont_id}_");
                namespaces.insert(String::new(), base_iri.clone());
            }
            if let Some(rest) = line.strip_prefix("idspace:") {
                let mut parts = rest.split_whitespace();
                if let (Some(prefix), Some(url)) = (parts.next(), parts.next()) {
                    namespaces.insert(prefix.to_string(), url.to_string());
                }
            }
            continue;
        }

        if let Some(value) = line.strip_prefix("id:") {
            flush_term(
                &mut entities,
                &mut annotations,
                &mut axioms,
                &mut axiom_counter,
                ontology_id,
                &base_iri,
                &mut current_id,
                &mut current_iri,
                &mut labels,
                &mut comments,
                &mut deprecated,
            );
            let obo_id = value.split('!').next().unwrap_or(value).trim().to_string();
            current_iri = Some(obo_id_to_iri(&obo_id, &base_iri));
            current_id = Some(obo_id);
            in_term = true;
            continue;
        }

        let Some(iri) = current_iri.clone() else {
            continue;
        };

        if let Some(value) = line.strip_prefix("name:") {
            labels.push(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("comment:") {
            comments.push(value.trim().to_string());
        } else if line == "is_obsolete: true" {
            deprecated = true;
        } else if let Some(value) = line.strip_prefix("is_a:") {
            let parent_id = value.split('!').next().unwrap_or(value).trim().to_string();
            axiom_counter += 1;
            axioms.push(Axiom {
                id: format!("{ontology_id}#axiom-{axiom_counter}"),
                ontology_id: ontology_id.to_string(),
                subject: iri.clone(),
                predicate: "rdfs:subClassOf".to_string(),
                object: obo_id_to_iri(&parent_id, &base_iri),
                axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
                source_location: SourceLocation::default(),
            });
        } else if let Some(value) = line.strip_prefix("synonym:") {
            annotations.push(Annotation {
                subject: iri.clone(),
                predicate: "obo:hasExactSynonym".to_string(),
                object: value.trim().trim_matches('"').to_string(),
                ontology_id: ontology_id.to_string(),
                source_location: SourceLocation::default(),
            });
        } else if let Some(value) = line.strip_prefix("xref:") {
            annotations.push(Annotation {
                subject: iri.clone(),
                predicate: "obo:hasDbXref".to_string(),
                object: value.trim().to_string(),
                ontology_id: ontology_id.to_string(),
                source_location: SourceLocation::default(),
            });
        } else if let Some(value) = line.strip_prefix("property_value:") {
            let mut parts = value.split_whitespace();
            if let (Some(prop), Some(val)) = (parts.next(), parts.next()) {
                annotations.push(Annotation {
                    subject: iri,
                    predicate: prop.to_string(),
                    object: val.to_string(),
                    ontology_id: ontology_id.to_string(),
                    source_location: SourceLocation::default(),
                });
            }
        }
        if entities.len() + annotations.len() + axioms.len() > MAX_TRIPLES_PER_FILE {
            return Err(ParseError::LimitExceeded(format!(
                "OBO file exceeds entity/axiom limit: {}",
                path.display()
            )));
        }
    }

    flush_term(
        &mut entities,
        &mut annotations,
        &mut axioms,
        &mut axiom_counter,
        ontology_id,
        &base_iri,
        &mut current_id,
        &mut current_iri,
        &mut labels,
        &mut comments,
        &mut deprecated,
    );

    let total = entities.len() + annotations.len() + axioms.len();
    if entities.len() > MAX_TRIPLES_PER_FILE || total > MAX_TRIPLES_PER_FILE {
        return Err(ParseError::LimitExceeded(format!(
            "OBO file exceeds entity/axiom limit: {}",
            path.display()
        )));
    }

    Ok(assemble_parsed_ontology(
        ontology_id,
        Some(base_iri),
        namespaces,
        entities,
        annotations,
        axioms,
    ))
}

fn obo_id_to_iri(obo_id: &str, base: &str) -> String {
    if obo_id.starts_with("http://") || obo_id.starts_with("https://") {
        return obo_id.to_string();
    }
    let normalized = obo_id.replace(':', "_");
    if base.ends_with('_') {
        format!("{base}{normalized}")
    } else {
        format!("{base}_{normalized}")
    }
}

pub fn parse_obo_file(
    path: &Path,
    ontology_id: &str,
    _content_hash: &str,
    _modified_time: u64,
) -> Result<ParsedOntology> {
    let metadata = std::fs::metadata(path).map_err(ParseError::Io)?;
    if metadata.len() > ontocore_core::MAX_FILE_BYTES {
        return Err(ParseError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("file exceeds maximum size of {} bytes", ontocore_core::MAX_FILE_BYTES),
        )));
    }
    let content = std::fs::read_to_string(path).map_err(ParseError::Io)?;
    parse_obo_text(path, ontology_id, &content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_minimal_obo() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            "format-version: 1.2\nontology: test\n\n[Term]\nid: TEST:0000001\nname: example term\nis_a: TEST:0000002 ! parent\n"
        )
        .unwrap();
        let parsed = parse_obo_file(file.path(), "doc-1", "hash", 0).unwrap();
        assert_eq!(parsed.entities.len(), 1);
        assert_eq!(parsed.entities[0].obo_id.as_deref(), Some("TEST:0000001"));
        assert_eq!(parsed.entities[0].labels, vec!["example term"]);
        assert_eq!(parsed.axioms.len(), 1);
        assert!(parsed.triple_count > 0);
    }

    #[test]
    fn rejects_oversized_obo_source_text() {
        let huge = "x".repeat((ontocore_core::MAX_FILE_BYTES as usize) + 1);
        let err = crate::rdf::parse_ontology_text(
            Path::new("big.obo"),
            ontocore_core::OntologyFormat::Obo,
            "doc-1",
            &huge,
            huge.as_bytes(),
        )
        .unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }
}
