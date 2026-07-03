use crate::error::{ReasonerError, Result};
use ontocore_catalog::ClassHierarchy;
use ontocore_core::WorkspaceScanner;
use ontologos_bridge::{core_to_triples_all, merge_triples_into_ontology};
use ontologos_core::Ontology;
use ontologos_parser::load_ontology;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ReasonerInput {
    pub workspace: PathBuf,
    pub content_hash: String,
    pub ontology: Ontology,
    pub asserted_hierarchy: ClassHierarchy,
    pub document_overrides: HashMap<PathBuf, String>,
}

pub struct WorkspaceInputLoader {
    workspace: PathBuf,
    document_overrides: HashMap<PathBuf, String>,
}

impl WorkspaceInputLoader {
    pub fn new(workspace: impl Into<PathBuf>) -> Self {
        Self { workspace: workspace.into(), document_overrides: HashMap::new() }
    }

    pub fn document_overrides(mut self, overrides: HashMap<PathBuf, String>) -> Self {
        self.document_overrides = overrides;
        self
    }

    pub fn load(&self, asserted_hierarchy: ClassHierarchy) -> Result<ReasonerInput> {
        let scanner = WorkspaceScanner::new(&self.workspace);
        let files = scanner.scan()?;

        let mut hasher = Sha256::new();
        let mut ontology = Ontology::new();

        for file in &files {
            hasher.update(file.content_hash.as_bytes());
            let loaded = if let Some(text) = self.document_override_text(&file.path) {
                load_ontology_from_temp(&file.path, text)?
            } else {
                load_ontology(&file.path).map_err(|e| ReasonerError::Load {
                    path: file.path.clone(),
                    message: e.to_string(),
                })?
            };
            merge_ontology(&mut ontology, loaded)?;
        }

        for (path, text) in &self.document_overrides {
            if files.iter().any(|f| paths_equal(&f.path, path)) {
                continue;
            }
            hasher.update(path.to_string_lossy().as_bytes());
            hasher.update(text.as_bytes());
            let loaded = load_ontology_from_temp(path, text)?;
            merge_ontology(&mut ontology, loaded)?;
        }

        Ok(ReasonerInput {
            workspace: self.workspace.clone(),
            content_hash: hex::encode(hasher.finalize()),
            ontology,
            asserted_hierarchy,
            document_overrides: self.document_overrides.clone(),
        })
    }

    fn document_override_text(&self, path: &Path) -> Option<&String> {
        self.document_overrides
            .get(path)
            .or_else(|| path.canonicalize().ok().and_then(|p| self.document_overrides.get(&p)))
    }
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

fn load_ontology_from_temp(path: &Path, text: &str) -> Result<Ontology> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("ttl");
    let tmp = tempfile::Builder::new()
        .suffix(&format!(".{ext}"))
        .tempfile()
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })?;
    std::fs::write(tmp.path(), text)
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })?;
    load_ontology(tmp.path())
        .map_err(|e| ReasonerError::Load { path: path.to_path_buf(), message: e.to_string() })
}

fn merge_ontology(target: &mut Ontology, source: Ontology) -> Result<()> {
    let triples =
        core_to_triples_all(&source).map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    merge_triples_into_ontology(target, &triples, &[])
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    Ok(())
}
