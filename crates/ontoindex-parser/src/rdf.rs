use crate::vocab::{Rdf, Rdfs, OWL};
use ontoindex_core::{
    limits::{MAX_FILE_BYTES, MAX_TRIPLES_PER_FILE},
    Annotation, Axiom, Entity, EntityKind, Import, Namespace, OntologyFormat, ParseStatus,
    SourceLocation,
};
use oxigraph::io::{RdfFormat, RdfParseError, RdfParser};
use oxigraph::model::{Quad, Subject, Term};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("RDF parse error: {0}")]
    Rdf(String),

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("limit exceeded: {0}")]
    LimitExceeded(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParsedOntology {
    pub ontology_id: String,
    pub base_iri: Option<String>,
    pub imports: Vec<String>,
    pub namespaces: BTreeMap<String, String>,
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub namespace_rows: Vec<Namespace>,
    pub import_rows: Vec<Import>,
    pub parse_status: ParseStatus,
    pub parse_message: Option<String>,
    pub triple_count: usize,
}

pub fn parse_ontology_file(
    path: &Path,
    format: OntologyFormat,
    ontology_id: &str,
    content_hash: &str,
    modified_time: u64,
) -> Result<ParsedOntology> {
    let _ = (content_hash, modified_time);
    let metadata = fs::metadata(path)?;
    if metadata.len() > MAX_FILE_BYTES {
        return Err(ParseError::LimitExceeded(format!(
            "file exceeds {MAX_FILE_BYTES} bytes: {}",
            path.display()
        )));
    }
    let content = fs::read(path)?;
    let rdf_format = to_rdf_format(format, path)?;

    let mut quads = Vec::new();
    let mut parse_message = None;
    let mut parse_status = ParseStatus::Ok;

    let parser = RdfParser::from_format(rdf_format);
    for quad in parser.for_reader(content.as_slice()) {
        match quad {
            Ok(q) => {
                if quads.len() >= MAX_TRIPLES_PER_FILE {
                    return Err(ParseError::LimitExceeded(format!(
                        "file exceeds {MAX_TRIPLES_PER_FILE} triples: {}",
                        path.display()
                    )));
                }
                quads.push(q);
            }
            Err(e) => {
                parse_status = ParseStatus::Error;
                parse_message = Some(format_parse_error(&e));
                break;
            }
        }
    }

    if parse_status == ParseStatus::Error {
        return Ok(empty_result(ontology_id, parse_status, parse_message, BTreeMap::new()));
    }

    let mut namespaces = extract_prefixes(&quads);
    if namespaces.is_empty() {
        namespaces.insert("".to_string(), default_base_iri(path));
    }

    let mut builder = OntologyBuilder::new(ontology_id.to_string(), namespaces.clone());
    for quad in &quads {
        builder.ingest_quad(quad);
    }
    builder.finish(parse_status, parse_message)
}

fn to_rdf_format(format: OntologyFormat, path: &Path) -> Result<RdfFormat> {
    match format {
        OntologyFormat::Turtle => Ok(RdfFormat::Turtle),
        OntologyFormat::RdfXml | OntologyFormat::Owl => Ok(RdfFormat::RdfXml),
        OntologyFormat::JsonLd => Ok(RdfFormat::JsonLd { profile: Default::default() }),
        OntologyFormat::NTriples => Ok(RdfFormat::NTriples),
        OntologyFormat::NQuads => Ok(RdfFormat::NQuads),
        OntologyFormat::TriG => Ok(RdfFormat::TriG),
        OntologyFormat::Unknown => Err(ParseError::UnsupportedFormat(path.display().to_string())),
    }
}

fn format_parse_error(error: &RdfParseError) -> String {
    error.to_string()
}

fn default_base_iri(path: &Path) -> String {
    format!("file://{}", path.display())
}

fn extract_prefixes(quads: &[Quad]) -> BTreeMap<String, String> {
    let mut prefixes = BTreeMap::new();
    for quad in quads {
        if let oxigraph::model::GraphNameRef::NamedNode(graph) = quad.graph_name.as_ref() {
            let iri = graph.as_str();
            if let Some((prefix, _)) = iri.rsplit_once('#') {
                if let Some((p, _)) = prefix.rsplit_once('/') {
                    prefixes
                        .entry(short_name_from_iri(p))
                        .or_insert_with(|| format!("{}/", prefix.trim_end_matches('#')));
                }
            }
        }
    }
    prefixes
}

fn empty_result(
    ontology_id: &str,
    parse_status: ParseStatus,
    parse_message: Option<String>,
    namespaces: BTreeMap<String, String>,
) -> ParsedOntology {
    ParsedOntology {
        ontology_id: ontology_id.to_string(),
        base_iri: namespaces.values().next().cloned(),
        imports: Vec::new(),
        namespaces: namespaces.clone(),
        entities: Vec::new(),
        annotations: Vec::new(),
        axioms: Vec::new(),
        namespace_rows: namespaces
            .into_iter()
            .map(|(prefix, iri)| Namespace { prefix, iri, ontology_id: ontology_id.to_string() })
            .collect(),
        import_rows: Vec::new(),
        parse_status,
        parse_message,
        triple_count: 0,
    }
}

struct EntityState {
    kind: EntityKind,
    labels: Vec<String>,
    comments: Vec<String>,
    deprecated: bool,
    types: BTreeSet<String>,
}

struct OntologyBuilder {
    ontology_id: String,
    namespaces: BTreeMap<String, String>,
    entities: HashMap<String, EntityState>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
    imports: BTreeSet<String>,
    ontology_iris: BTreeSet<String>,
    triple_count: usize,
    axiom_counter: usize,
}

impl OntologyBuilder {
    fn new(ontology_id: String, namespaces: BTreeMap<String, String>) -> Self {
        Self {
            ontology_id,
            namespaces,
            entities: HashMap::new(),
            annotations: Vec::new(),
            axioms: Vec::new(),
            imports: BTreeSet::new(),
            ontology_iris: BTreeSet::new(),
            triple_count: 0,
            axiom_counter: 0,
        }
    }

    fn ingest_quad(&mut self, quad: &Quad) {
        self.triple_count += 1;
        let subject = subject_to_string(&quad.subject);
        let predicate = quad.predicate.as_str().to_string();
        let object = term_to_string(&quad.object);

        if quad.predicate == Rdf::type_() {
            if let Term::NamedNode(node) = &quad.object {
                let type_iri = node.as_str();
                if type_iri == OWL::ontology().as_str() {
                    self.ontology_iris.insert(subject.clone());
                }
                let kind = entity_kind_for_type(type_iri);
                if kind != EntityKind::Other {
                    let entry =
                        self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                            kind,
                            labels: Vec::new(),
                            comments: Vec::new(),
                            deprecated: false,
                            types: BTreeSet::new(),
                        });
                    entry.types.insert(type_iri.to_string());
                    if entry.kind == EntityKind::Other
                        || kind_priority(kind) > kind_priority(entry.kind)
                    {
                        entry.kind = kind;
                    }
                }
            }
            return;
        }

        if quad.predicate == OWL::imports() {
            self.imports.insert(object.clone());
            return;
        }

        if quad.predicate == Rdfs::label() {
            if let Some(entity) = self.entities.get_mut(&subject) {
                entity.labels.push(object.clone());
            } else {
                self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                    kind: EntityKind::Other,
                    labels: vec![object.clone()],
                    comments: Vec::new(),
                    deprecated: false,
                    types: BTreeSet::new(),
                });
            }
            self.annotations.push(Annotation {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                ontology_id: self.ontology_id.clone(),
            });
            return;
        }

        if quad.predicate == Rdfs::comment() {
            if let Some(entity) = self.entities.get_mut(&subject) {
                entity.comments.push(object.clone());
            } else {
                self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                    kind: EntityKind::Other,
                    labels: Vec::new(),
                    comments: vec![object.clone()],
                    deprecated: false,
                    types: BTreeSet::new(),
                });
            }
            self.annotations.push(Annotation {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                ontology_id: self.ontology_id.clone(),
            });
            return;
        }

        if quad.predicate == OWL::deprecated() {
            if let Some(entity) = self.entities.get_mut(&subject) {
                entity.deprecated = object == "true";
            }
        }

        if quad.predicate == Rdfs::sub_class_of() {
            self.axiom_counter += 1;
            self.axioms.push(Axiom {
                id: format!("{}#axiom-{}", self.ontology_id, self.axiom_counter),
                ontology_id: self.ontology_id.clone(),
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                axiom_kind: "SubClassOf".to_string(),
            });
        }
    }

    fn finish(
        self,
        parse_status: ParseStatus,
        parse_message: Option<String>,
    ) -> Result<ParsedOntology> {
        let base_iri =
            self.namespaces.get("").cloned().or_else(|| self.namespaces.values().next().cloned());

        let ontology_id = if let Some(iri) = self.ontology_iris.iter().next() {
            iri.clone()
        } else {
            self.ontology_id.clone()
        };

        let mut entities = Vec::new();
        for (iri, state) in &self.entities {
            if state.kind == EntityKind::Other {
                continue;
            }
            entities.push(Entity {
                iri: iri.clone(),
                short_name: short_name_from_iri(iri),
                kind: state.kind,
                ontology_id: ontology_id.clone(),
                source_location: SourceLocation::default(),
                labels: state.labels.clone(),
                comments: state.comments.clone(),
                deprecated: state.deprecated,
            });
        }
        entities.sort_by(|a, b| a.iri.cmp(&b.iri));

        let namespace_rows = self
            .namespaces
            .iter()
            .map(|(prefix, iri)| Namespace {
                prefix: prefix.clone(),
                iri: iri.clone(),
                ontology_id: ontology_id.clone(),
            })
            .collect();

        let import_rows = self
            .imports
            .iter()
            .map(|import_iri| Import {
                ontology_id: ontology_id.clone(),
                import_iri: import_iri.clone(),
            })
            .collect();

        Ok(ParsedOntology {
            ontology_id,
            base_iri,
            imports: self.imports.into_iter().collect(),
            namespaces: self.namespaces.clone(),
            entities,
            annotations: self.annotations,
            axioms: self.axioms,
            namespace_rows,
            import_rows,
            parse_status,
            parse_message,
            triple_count: self.triple_count,
        })
    }
}

fn entity_kind_for_type(type_iri: &str) -> EntityKind {
    match type_iri {
        t if t == OWL::class().as_str() || t == Rdfs::class().as_str() => EntityKind::Class,
        t if t == OWL::object_property().as_str() => EntityKind::ObjectProperty,
        t if t == OWL::datatype_property().as_str() => EntityKind::DataProperty,
        t if t == OWL::annotation_property().as_str() => EntityKind::AnnotationProperty,
        t if t == OWL::named_individual().as_str() => EntityKind::Individual,
        t if t == OWL::ontology().as_str() => EntityKind::Ontology,
        _ => EntityKind::Other,
    }
}

fn kind_priority(kind: EntityKind) -> u8 {
    match kind {
        EntityKind::Class => 5,
        EntityKind::ObjectProperty => 5,
        EntityKind::DataProperty => 5,
        EntityKind::AnnotationProperty => 5,
        EntityKind::Individual => 4,
        EntityKind::Ontology => 3,
        EntityKind::Other => 0,
    }
}

fn subject_to_string(subject: &Subject) -> String {
    match subject {
        Subject::NamedNode(node) => node.as_str().to_string(),
        Subject::BlankNode(node) => format!("_:{}", node.as_str()),
        #[allow(unreachable_patterns)]
        _ => subject.to_string(),
    }
}

fn term_to_string(term: &Term) -> String {
    match term {
        Term::NamedNode(node) => node.as_str().to_string(),
        Term::BlankNode(node) => format!("_:{}", node.as_str()),
        Term::Literal(lit) => lit.to_string(),
        #[allow(unreachable_patterns)]
        _ => term.to_string(),
    }
}

fn short_name_from_iri(iri: &str) -> String {
    if let Some((_, name)) = iri.rsplit_once('#') {
        return name.to_string();
    }
    if let Some((_, name)) = iri.rsplit_once('/') {
        return name.to_string();
    }
    iri.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parses_simple_turtle_ontology() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.ttl");
        let mut f = fs::File::create(&path).unwrap();
        writeln!(
            f,
            r#"@prefix ex: <http://example.org/test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<http://example.org/test> a owl:Ontology .

ex:Person a owl:Class ;
    rdfs:label "Person" ;
    rdfs:comment "A human being" .

ex:knows a owl:ObjectProperty ;
    rdfs:label "knows" .
"#
        )
        .unwrap();

        let parsed =
            parse_ontology_file(&path, OntologyFormat::Turtle, "doc-1", "hash", 0).unwrap();

        assert_eq!(parsed.parse_status, ParseStatus::Ok);

        let person = parsed
            .entities
            .iter()
            .find(|e| e.iri == "http://example.org/test#Person")
            .expect("Person entity");
        assert_eq!(person.kind, EntityKind::Class);
        assert_eq!(person.labels, vec!["\"Person\"".to_string()]);

        let knows = parsed
            .entities
            .iter()
            .find(|e| e.iri == "http://example.org/test#knows")
            .expect("knows property");
        assert_eq!(knows.kind, EntityKind::ObjectProperty);
        assert_eq!(knows.labels, vec!["\"knows\"".to_string()]);
    }
}
