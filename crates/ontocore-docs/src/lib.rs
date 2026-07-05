//! Documentation export for indexed ontology workspaces.

use minijinja::{context, Environment};
use ontocore_catalog::OntologyCatalog;
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
            if &doc.id != filter && doc.base_iri.as_deref() != Some(filter.as_str()) {
                continue;
            }
        }
        let mut entities = Vec::new();
        for entity in &catalog.data().entities {
            if entity.ontology_id != doc.id {
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
    let index_body = render_index(&ontologies, options.format)?;
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

fn render_index(ontologies: &[OntologyDoc], format: ExportFormat) -> Result<String> {
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
            Ok(md)
        }
        ExportFormat::Html => {
            let env = html_env()?;
            let tmpl = env.get_template("index.html")?;
            Ok(tmpl.render(context! { ontologies => ontologies })?)
        }
    }
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
}
