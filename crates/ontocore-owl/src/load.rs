use crate::bridge::bridge_ontology;
use crate::error::{OwlError, Result};
use crate::OwlBridgeResult;
use horned_owl::io::rdf::reader::read;
use horned_owl::model::{RcAnnotatedComponent, RcStr};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use horned_owl::ontology::set::SetOntology;
use ontocore_core::OntologyFormat;
use oxigraph::io::{RdfFormat, RdfSerializer};
use oxigraph::model::Quad;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::path::Path;

/// Result of loading a document through the Horned-OWL pipeline.
#[derive(Debug, Clone)]
pub struct OwlLoadResult {
    pub bridge: OwlBridgeResult,
    pub incomplete: bool,
    pub load_warning: Option<String>,
}

/// Load Turtle source via Oxigraph quads → RDF/XML → Horned-OWL.
pub fn load_turtle_text(
    path: &Path,
    ontology_id: &str,
    source_text: &str,
    quads: &[Quad],
    namespaces: &BTreeMap<String, String>,
) -> Result<OwlLoadResult> {
    let _ = path;
    match load_from_quads(quads) {
        Ok((ontology, incomplete)) => {
            let bridge = bridge_ontology(ontology, ontology_id, source_text, namespaces);
            Ok(OwlLoadResult {
                bridge,
                incomplete,
                load_warning: if incomplete {
                    Some("Horned-OWL reported incomplete RDF parse".to_string())
                } else {
                    None
                },
            })
        }
        Err(e) => Err(e),
    }
}

/// Convert Oxigraph quads to Horned-OWL ontology via RDF/XML.
pub fn load_from_quads(
    quads: &[Quad],
) -> Result<(
    horned_owl::io::rdf::reader::ConcreteRDFOntology<
        horned_owl::model::RcStr,
        horned_owl::model::RcAnnotatedComponent,
    >,
    bool,
)> {
    let rdf_xml = quads_to_rdf_xml(quads).map_err(OwlError::LoadFailed)?;
    let mut cursor = Cursor::new(rdf_xml);
    let (ontology, incomplete) =
        read(&mut cursor, Default::default()).map_err(|e| OwlError::LoadFailed(e.to_string()))?;
    Ok((ontology, !incomplete.is_complete()))
}

fn quads_to_rdf_xml(quads: &[Quad]) -> std::result::Result<Vec<u8>, String> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::RdfXml).for_writer(Vec::new());
    for quad in quads {
        serializer.serialize_quad(quad).map_err(|e| e.to_string())?;
    }
    serializer.finish().map_err(|e| e.to_string())
}

/// Load OWL/XML (`.owx`) source via Horned-OWL.
pub fn load_owx_text(
    path: &Path,
    ontology_id: &str,
    source_text: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<OwlLoadResult> {
    let _ = path;
    use horned_owl::io::owx::reader::read as read_owx;
    let mut cursor = Cursor::new(source_text.as_bytes());
    let (set_ont, _mapping): (SetOntology<RcStr>, _) = read_owx(&mut cursor, Default::default())
        .map_err(|e| OwlError::LoadFailed(e.to_string()))?;
    let mapped: ComponentMappedOntology<RcStr, RcAnnotatedComponent> = set_ont.into();
    let bridge = bridge_ontology(mapped, ontology_id, source_text, namespaces);
    Ok(OwlLoadResult { bridge, incomplete: false, load_warning: None })
}

/// Whether Horned-OWL loading is supported for this format.
pub fn supports_horned_load(format: OntologyFormat) -> bool {
    matches!(
        format,
        OntologyFormat::Turtle
            | OntologyFormat::Owl
            | OntologyFormat::RdfXml
            | OntologyFormat::OwlXml
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::io::RdfParser;

    #[test]
    fn loads_fixture_turtle_via_horned() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let parser = RdfParser::from_format(RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces = BTreeMap::from([
            ("ex".to_string(), "http://example.org/people#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ]);
        let result = load_turtle_text(Path::new("example.ttl"), "doc-1", ttl, &quads, &namespaces)
            .expect("load");
        assert!(!result.bridge.entities.is_empty());
        assert!(result.bridge.entities.iter().any(|e| e.short_name == "Person"));
    }
}
