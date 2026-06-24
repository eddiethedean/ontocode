use ontoindex_catalog::ClassHierarchy;
use ontologos_core::{EntityKind, Ontology};

pub fn subclass_edges_from_ontology(
    ontology: &Ontology,
    asserted: &ClassHierarchy,
) -> Vec<(String, String)> {
    let asserted_set: std::collections::BTreeSet<(String, String)> =
        asserted.edges.iter().map(|e| (e.child.clone(), e.parent.clone())).collect();

    let mut edges = Vec::new();
    for (id, entity) in ontology.entities().iter() {
        if entity.kind != EntityKind::Class {
            continue;
        }
        let child = match ontology.resolve_iri(entity.iri) {
            Ok(s) => s.to_string(),
            Err(_) => continue,
        };
        for &parent_id in ontology.direct_superclasses(id) {
            let parent_entity = match ontology.entity(parent_id) {
                Ok(e) => e,
                Err(_) => continue,
            };
            let parent = match ontology.resolve_iri(parent_entity.iri) {
                Ok(s) => s.to_string(),
                Err(_) => continue,
            };
            if !asserted_set.contains(&(child.clone(), parent.clone())) {
                edges.push((child.clone(), parent));
            }
        }
    }
    edges
}
