//! Documentation export for indexed ontology workspaces.

use minijinja::{context, AutoEscape, Environment};
use ontocore_catalog::OntologyCatalog;
use ontocore_core::{document_matches_entity, document_matches_ontology_id, EntityKind};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Markdown,
    Html,
}

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub output_dir: PathBuf,
    pub format: ExportFormat,
    pub ontology_id: Option<String>,
}

impl ExportOptions {
    pub fn markdown(output_dir: impl Into<PathBuf>) -> Self {
        Self { output_dir: output_dir.into(), format: ExportFormat::Markdown, ontology_id: None }
    }

    pub fn html(output_dir: impl Into<PathBuf>) -> Self {
        Self { output_dir: output_dir.into(), format: ExportFormat::Html, ontology_id: None }
    }

    pub fn with_ontology_id(mut self, id: impl Into<String>) -> Self {
        self.ontology_id = Some(id.into());
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("template error: {0}")]
    Template(#[from] minijinja::Error),
}

pub type Result<T> = std::result::Result<T, ExportError>;

#[derive(Debug, Clone, serde::Serialize)]
struct EntityDoc {
    iri: String,
    short_name: String,
    kind: String,
    labels: Vec<String>,
    comments: Vec<String>,
    parents: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct OntologyDoc {
    id: String,
    slug: String,
    path: String,
    imports: Vec<String>,
    entities: Vec<EntityDoc>,
}

pub fn export_workspace(catalog: &OntologyCatalog, options: ExportOptions) -> Result<()> {
    fs::create_dir_all(&options.output_dir)?;

    let hierarchy = catalog.class_hierarchy();
    let mut ontologies: Vec<OntologyDoc> = Vec::new();

    for doc in &catalog.data().documents {
        if let Some(filter) = &options.ontology_id {
            if !document_matches_ontology_id(filter, doc) {
                continue;
            }
        }
        let mut entities = Vec::new();
        for entity in &catalog.data().entities {
            if !document_matches_entity(entity, doc) {
                continue;
            }
            let parents = hierarchy.parents.get(&entity.iri).cloned().unwrap_or_default();
            entities.push(EntityDoc {
                iri: entity.iri.clone(),
                short_name: entity.short_name.clone(),
                kind: entity.kind.as_str().to_string(),
                labels: entity.labels.clone(),
                comments: entity.comments.clone(),
                parents,
            });
        }
        entities.sort_by(|a, b| a.short_name.cmp(&b.short_name));
        ontologies.push(OntologyDoc {
            id: doc.id.clone(),
            slug: slugify(&doc.id),
            path: doc.path.display().to_string(),
            imports: doc.imports.clone(),
            entities,
        });
    }

    let index_name = match options.format {
        ExportFormat::Markdown => "index.md",
        ExportFormat::Html => "index.html",
    };
    let index_path = options.output_dir.join(index_name);
    let index_body = render_index(&ontologies, &hierarchy, catalog, options.format)?;
    fs::write(index_path, index_body)?;

    for ont in &ontologies {
        let file_name = match options.format {
            ExportFormat::Markdown => format!("{}.md", ont.slug),
            ExportFormat::Html => format!("{}.html", ont.slug),
        };
        let body = render_ontology(ont, options.format)?;
        fs::write(options.output_dir.join(file_name), body)?;
    }

    Ok(())
}

fn slugify(iri: &str) -> String {
    iri.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .chars()
        .take(80)
        .collect()
}

fn render_index(
    ontologies: &[OntologyDoc],
    hierarchy: &ontocore_catalog::ClassHierarchy,
    catalog: &OntologyCatalog,
    format: ExportFormat,
) -> Result<String> {
    match format {
        ExportFormat::Markdown => {
            let mut md = String::from("# Ontology documentation\n\n");
            for ont in ontologies {
                md.push_str(&format!(
                    "- [{}]({}.md) — {} entities, {} imports\n",
                    ont.id,
                    ont.slug,
                    ont.entities.len(),
                    ont.imports.len()
                ));
            }
            md.push_str("\n## Class hierarchy\n\n");
            md.push_str(&render_class_hierarchy(catalog, hierarchy));
            md.push_str("\n## Property index\n\n");
            md.push_str(&render_property_index(catalog));
            Ok(md)
        }
        ExportFormat::Html => {
            let env = html_env()?;
            let tmpl = env.get_template("index.html")?;
            Ok(tmpl.render(context! { ontologies => ontologies })?)
        }
    }
}

pub fn render_class_hierarchy(
    catalog: &OntologyCatalog,
    hierarchy: &ontocore_catalog::ClassHierarchy,
) -> String {
    let mut roots: Vec<&str> = hierarchy
        .parents
        .keys()
        .map(|s| s.as_str())
        .filter(|iri| {
            hierarchy
                .parents
                .get(*iri)
                .map(|p| p.is_empty())
                .unwrap_or(true)
        })
        .collect();
    roots.sort();
    let mut out = String::new();
    for root in roots {
        render_hierarchy_node(catalog, hierarchy, root, 0, &mut out);
    }
    if out.is_empty() {
        out.push_str("_No class hierarchy available._\n");
    }
    out
}

fn render_hierarchy_node(
    catalog: &OntologyCatalog,
    hierarchy: &ontocore_catalog::ClassHierarchy,
    iri: &str,
    depth: usize,
    out: &mut String,
) {
    let label = catalog
        .data()
        .entities
        .iter()
        .find(|e| e.iri == iri)
        .and_then(|e| e.labels.first())
        .map(|s| s.as_str())
        .unwrap_or(iri);
    out.push_str(&format!("{}{} (`{}`)\n", "  ".repeat(depth), label, iri));
    let mut children: Vec<&str> = hierarchy
        .children
        .get(iri)
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();
    children.sort();
    for child in children {
        render_hierarchy_node(catalog, hierarchy, child, depth + 1, out);
    }
}

pub fn render_property_index(catalog: &OntologyCatalog) -> String {
    let mut object_props = Vec::new();
    let mut data_props = Vec::new();
    let mut annotation_props = Vec::new();
    for entity in &catalog.data().entities {
        match entity.kind {
            EntityKind::ObjectProperty => object_props.push(entity),
            EntityKind::DataProperty => data_props.push(entity),
            EntityKind::AnnotationProperty => annotation_props.push(entity),
            _ => {}
        }
    }
    let mut out = String::new();
    for (title, props) in [
        ("Object properties", &object_props),
        ("Data properties", &data_props),
        ("Annotation properties", &annotation_props),
    ] {
        if props.is_empty() {
            continue;
        }
        out.push_str(&format!("### {title}\n\n"));
        let mut sorted = props.clone();
        sorted.sort_by(|a, b| a.short_name.cmp(&b.short_name));
        for prop in sorted {
            let label = prop.labels.first().map(|s| s.as_str()).unwrap_or(&prop.short_name);
            out.push_str(&format!("- **{label}** — `{}`\n", prop.iri));
        }
        out.push('\n');
    }
    if out.is_empty() {
        out.push_str("_No properties indexed._\n");
    }
    out
}

fn render_ontology(ont: &OntologyDoc, format: ExportFormat) -> Result<String> {
    match format {
        ExportFormat::Markdown => {
            let mut md = format!("# {}\n\n", ont.id);
            md.push_str(&format!("Source: `{}`\n\n", ont.path));
            if !ont.imports.is_empty() {
                md.push_str("## Imports\n\n");
                for imp in &ont.imports {
                    md.push_str(&format!("- <{imp}>\n"));
                }
                md.push('\n');
            }
            md.push_str("## Entities\n\n");
            for entity in &ont.entities {
                let label = entity.labels.first().map(|s| s.as_str()).unwrap_or(&entity.short_name);
                md.push_str(&format!("### {label}\n\n"));
                md.push_str(&format!("- IRI: `{}`\n", entity.iri));
                md.push_str(&format!("- Kind: {}\n", entity.kind));
                if !entity.comments.is_empty() {
                    md.push_str(&format!("- Comment: {}\n", entity.comments.join("; ")));
                }
                if !entity.parents.is_empty() {
                    md.push_str(&format!("- Parents: {}\n", entity.parents.join(", ")));
                }
                md.push('\n');
            }
            Ok(md)
        }
        ExportFormat::Html => {
            let env = html_env()?;
            let tmpl = env.get_template("ontology.html")?;
            Ok(tmpl.render(context! { ont => ont })?)
        }
    }
}

fn html_env() -> Result<Environment<'static>> {
    let mut env = Environment::new();
    env.set_auto_escape_callback(|name| {
        if name.ends_with(".html") {
            AutoEscape::Html
        } else {
            AutoEscape::None
        }
    });
    env.add_template(
        "index.html",
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Ontology docs</title></head>
<body><h1>Ontology documentation</h1><ul>
{% for ont in ontologies %}
<li><a href="{{ ont.slug }}.html">{{ ont.id }}</a>
 — {{ ont.entities | length }} entities</li>
{% endfor %}
</ul></body></html>"#,
    )?;
    env.add_template(
        "ontology.html",
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>{{ ont.id }}</title></head>
<body>
<h1>{{ ont.id }}</h1>
<p>Source: <code>{{ ont.path }}</code></p>
{% if ont.imports %}<h2>Imports</h2><ul>{% for imp in ont.imports %}<li>{{ imp }}</li>{% endfor %}</ul>{% endif %}
<h2>Entities</h2>
<table border="1"><tr><th>Name</th><th>Kind</th><th>IRI</th></tr>
{% for e in ont.entities %}
<tr><td>{{ e.labels[0] if e.labels else e.short_name }}</td><td>{{ e.kind }}</td><td>{{ e.iri }}</td></tr>
{% endfor %}
</table>
</body></html>"#,
    )?;
    Ok(env)
}

pub fn entity_kind_counts(catalog: &OntologyCatalog) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for entity in &catalog.data().entities {
        *counts.entry(entity.kind.as_str().to_string()).or_default() += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontocore_catalog::IndexBuilder;
    use std::path::Path;

    #[test]
    fn exports_markdown_for_fixtures() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let dir = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(dir.path())).expect("export");
        assert!(dir.path().join("index.md").exists());
    }

    #[test]
    fn exports_entities_for_owl_ontology_declarations() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let example = catalog
            .data()
            .documents
            .iter()
            .find(|d| d.path.file_name().and_then(|n| n.to_str()) == Some("example.ttl"))
            .expect("example.ttl indexed");
        let entity_count =
            catalog.data().entities.iter().filter(|e| document_matches_entity(e, example)).count();
        assert!(entity_count > 0, "fixture entities should match example.ttl via ontology IRI");

        let dir = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(dir.path())).expect("export");
        let index = fs::read_to_string(dir.path().join("index.md")).expect("index.md");
        let doc_slug = slugify(&example.id);
        let detail_path = dir.path().join(format!("{doc_slug}.md"));
        assert!(detail_path.exists(), "expected per-ontology export file");
        let detail = fs::read_to_string(detail_path).expect("ontology markdown");
        assert!(detail.contains("## Entities"), "ontology page should list entities");
        assert!(!detail.contains("## Entities\n\n\n#"), "entity section should not be empty");
        assert!(
            index.contains(&format!("{entity_count} entities")),
            "index should report exported entity count for example.ttl"
        );
    }

    #[test]
    fn html_export_escapes_entity_labels() {
        let ont = OntologyDoc {
            id: "http://example.org/ex".to_string(),
            slug: "ex".to_string(),
            path: "evil.ttl".to_string(),
            imports: vec![],
            entities: vec![EntityDoc {
                iri: "http://example.org/ex#Evil".to_string(),
                short_name: "Evil".to_string(),
                kind: "class".to_string(),
                labels: vec!["<img src=x onerror=alert(1)>".to_string()],
                comments: vec![],
                parents: vec![],
            }],
        };
        let html = render_ontology(&ont, ExportFormat::Html).expect("render html");
        assert!(html.contains("&lt;img"));
        assert!(!html.contains("<img src=x onerror=alert(1)>"));
    }
}
