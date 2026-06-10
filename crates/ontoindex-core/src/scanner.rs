use crate::error::{OntoIndexError, Result};
use crate::model::OntologyFormat;
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const ONTOLOGY_EXTENSIONS: &[&str] =
    &["ttl", "rdf", "owl", "jsonld", "json-ld", "nt", "nq", "trig"];

#[derive(Debug, Clone)]
pub struct OntologyFile {
    pub path: PathBuf,
    pub format: OntologyFormat,
    pub content_hash: String,
    pub modified_time: u64,
    pub size_bytes: u64,
}

pub struct WorkspaceScanner {
    root: PathBuf,
}

impl WorkspaceScanner {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn scan(&self) -> Result<Vec<OntologyFile>> {
        if !self.root.exists() {
            return Err(OntoIndexError::Scanner(format!(
                "workspace path does not exist: {}",
                self.root.display()
            )));
        }

        let mut files = Vec::new();
        let walker = WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build();

        for entry in walker {
            let entry = entry.map_err(|e| OntoIndexError::Scanner(e.to_string()))?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ONTOLOGY_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()) {
                    files.push(self.describe_file(path)?);
                }
            }
        }

        files.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(files)
    }

    fn describe_file(&self, path: &Path) -> Result<OntologyFile> {
        let metadata = fs::metadata(path)?;
        let content = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let content_hash = hex::encode(hasher.finalize());

        let modified_time = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or_default();

        Ok(OntologyFile {
            path: path.to_path_buf(),
            format: OntologyFormat::from_extension(ext),
            content_hash,
            modified_time,
            size_bytes: metadata.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn scan_finds_ontology_files() {
        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("example.ttl");
        let mut f = fs::File::create(&ttl).unwrap();
        writeln!(f, "@prefix ex: <http://example.org/> .").unwrap();

        let txt = dir.path().join("readme.txt");
        fs::write(txt, "not ontology").unwrap();

        let scanner = WorkspaceScanner::new(dir.path());
        let files = scanner.scan().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].format, OntologyFormat::Turtle);
        assert!(!files[0].content_hash.is_empty());
    }
}
