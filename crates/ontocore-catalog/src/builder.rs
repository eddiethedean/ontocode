use crate::disk_cache::DiskCache;
use crate::incremental::{
    config_fingerprint, content_hash_text, effective_content_hash, paths_equal, DocumentSnapshot,
    IncrementalStats,
};
use crate::OntologyCatalogData;
use ontocore_core::{
    limits::{MAX_ENTITIES, MAX_TOTAL_TRIPLES, MAX_TRIPLES_PER_FILE},
    read_to_string_capped, Annotation, Axiom, Diagnostic, DiagnosticCode, DiagnosticSeverity,
    Entity, Import, Namespace, OntologyDocument, OntologyFormat, ParseStatus, SourceLocation,
    WorkspaceScanner, MAX_FILE_BYTES,
};
use ontocore_diagnostics::{collect_diagnostics_with_config, find_config, DiagnosticInput};
use ontocore_owl::{load_owx_text, load_turtle_text, supports_horned_load};
use ontocore_parser::{parse_ontology_file, parse_ontology_text, ParsedOntology};
use oxigraph::model::Quad;
use oxigraph::store::Store;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("core error: {0}")]
    Core(#[from] ontocore_core::OntoCoreError),

    #[error("parse error in {path}: {message}")]
    Parse { path: PathBuf, message: String },

    #[error("store error: {0}")]
    Store(String),
}

pub type Result<T> = std::result::Result<T, CatalogError>;

pub struct IndexBuilder {
    workspace: PathBuf,
    scan_roots: Vec<PathBuf>,
    document_overrides: HashMap<PathBuf, String>,
    disk_cache: bool,
    only_paths: Option<Vec<PathBuf>>,
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self {
            workspace: PathBuf::from("."),
            scan_roots: Vec::new(),
            document_overrides: HashMap::new(),
            disk_cache: false,
            only_paths: None,
        }
    }

    pub fn workspace(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace = path.into();
        self
    }

    /// Additional workspace roots to scan and merge into one catalog (multi-root).
    pub fn scan_roots(mut self, roots: Vec<PathBuf>) -> Self {
        self.scan_roots = roots;
        self
    }

    /// Use in-memory text instead of disk for specific paths (LSP open buffers).
    pub fn document_overrides(mut self, overrides: HashMap<PathBuf, String>) -> Self {
        self.document_overrides = overrides;
        self
    }

    /// Enable persistent `.ontocore/cache/` snapshots keyed by content hash.
    pub fn disk_cache(mut self, enabled: bool) -> Self {
        self.disk_cache = enabled;
        self
    }

    /// Restrict scanning to these paths (e.g. git-tracked files for diff worktree side).
    pub fn only_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.only_paths = Some(paths);
        self
    }

    fn document_override_text(&self, path: &Path) -> Option<&String> {
        self.document_overrides
            .get(path)
            .or_else(|| path.canonicalize().ok().and_then(|p| self.document_overrides.get(&p)))
    }

    fn config_fingerprint(&self) -> String {
        let scan_roots = if self.scan_roots.is_empty() {
            vec![self.workspace.clone()]
        } else {
            self.scan_roots.clone()
        };
        config_fingerprint(&scan_roots, self.disk_cache, &self.document_overrides)
    }

    pub fn build(self) -> Result<OntologyCatalog> {
        self.build_with_snapshots(None, false).map(|(catalog, _)| catalog)
    }

    /// Rebuild the catalog reusing unchanged documents from `previous` when content hashes match.
    pub fn build_incremental(self, previous: &OntologyCatalog) -> Result<OntologyCatalog> {
        if !paths_equal(&previous.workspace, &self.workspace) {
            return self.build();
        }
        if previous.config_fingerprint != self.config_fingerprint() {
            return self.build();
        }
        self.build_with_snapshots(Some(&previous.document_snapshots), true)
            .map(|(catalog, _)| catalog)
    }

    fn build_with_snapshots(
        self,
        previous_snapshots: Option<&HashMap<PathBuf, DocumentSnapshot>>,
        incremental: bool,
    ) -> Result<(OntologyCatalog, IncrementalStats)> {
        let scan_roots = if self.scan_roots.is_empty() {
            vec![self.workspace.clone()]
        } else {
            self.scan_roots.clone()
        };
        let mut files = Vec::new();
        let mut seen = std::collections::HashSet::new();
        if let Some(ref only) = self.only_paths {
            for path in only {
                if !path.is_file() {
                    continue;
                }
                let key = path.canonicalize().unwrap_or_else(|_| path.clone());
                if seen.insert(key.clone()) {
                    files.push(WorkspaceScanner::new(&self.workspace).describe_path(&key)?);
                }
            }
        } else {
            for root in scan_roots {
                for file in WorkspaceScanner::new(&root).scan()? {
                    let key = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());
                    if seen.insert(key) {
                        files.push(file);
                    }
                }
            }
        }

        let mut documents = Vec::new();
        let mut entities: Vec<Entity> = Vec::new();
        let mut entity_index: HashMap<String, usize> = HashMap::new();
        let mut entity_to_document: HashMap<String, usize> = HashMap::new();
        let mut document_entity_iris: Vec<Vec<String>> = Vec::new();
        let mut annotations = Vec::new();
        let mut axioms = Vec::new();
        let mut namespaces = Vec::new();
        let mut imports = Vec::new();
        let mut triple_count = 0usize;
        let store = Store::new().map_err(|e| CatalogError::Store(e.to_string()))?;

        let mut bridge_diagnostics = Vec::new();
        let mut document_snapshots = HashMap::new();
        let mut reused = 0usize;
        let mut reparsed = 0usize;
        let mut loaded_content_hashes = HashSet::new();

        let disk_cache = DiskCache::enabled(self.disk_cache, &self.workspace);
        let mut merged_previous = previous_snapshots.cloned().unwrap_or_default();
        if let Some(ref cache) = disk_cache {
            for file in &files {
                let override_text = self.document_override_text(&file.path);
                let effective_hash =
                    effective_content_hash(&file.content_hash, override_text.map(String::as_str));
                let lookup_path = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());
                cache.hydrate_previous(
                    &mut merged_previous,
                    &lookup_path,
                    &effective_hash,
                    file.modified_time,
                );
            }
        }
        let previous_snapshots =
            if merged_previous.is_empty() { None } else { Some(&merged_previous) };

        for (idx, file) in files.iter().enumerate() {
            let doc_id = format!("doc-{}", idx + 1);
            let override_text = self.document_override_text(&file.path);
            let effective_hash =
                effective_content_hash(&file.content_hash, override_text.map(String::as_str));
            let lookup_path = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());

            if let Some(prev) = previous_snapshots.and_then(|m| m.get(&lookup_path)) {
                if prev.content_hash == effective_hash {
                    if let Some((reuse_path, modified_time)) = verified_snapshot_source(
                        &self.workspace,
                        &file.path,
                        override_text.map(String::as_str),
                        &effective_hash,
                    ) {
                        let snap = prev.with_reuse_context(&doc_id, &reuse_path, modified_time);
                        apply_document_snapshot(
                            &snap,
                            &doc_id,
                            &mut documents,
                            &mut entities,
                            &mut entity_index,
                            &mut entity_to_document,
                            &mut document_entity_iris,
                            &mut annotations,
                            &mut axioms,
                            &mut namespaces,
                            &mut imports,
                            &mut triple_count,
                            &store,
                            &mut bridge_diagnostics,
                            &mut loaded_content_hashes,
                        )?;
                        document_snapshots.insert(lookup_path, snap);
                        reused += 1;
                        continue;
                    }
                }
            }

            reparsed += 1;
            let parsed = if let Some(text) = override_text {
                parse_ontology_text(&file.path, file.format, &doc_id, text, text.as_bytes())
                    .map_err(|e| CatalogError::Parse {
                        path: file.path.clone(),
                        message: e.to_string(),
                    })?
            } else {
                parse_ontology_file(
                    &file.path,
                    file.format,
                    &doc_id,
                    &file.content_hash,
                    file.modified_time,
                )
                .map_err(|e| CatalogError::Parse {
                    path: file.path.clone(),
                    message: e.to_string(),
                })?
            };

            // Load quads even when parse_status is Error (partial recovery after a trailing fault).
            if !parsed.quads().is_empty() {
                load_quads_into_store(
                    &store,
                    parsed.quads(),
                    &mut triple_count,
                    &effective_hash,
                    &mut loaded_content_hashes,
                )?;
            }

            documents.push(OntologyDocument {
                id: doc_id.clone(),
                path: file.path.clone(),
                format: file.format,
                base_iri: parsed.base_iri.clone(),
                imports: parsed.imports.clone(),
                namespaces: parsed.namespaces.clone(),
                parse_status: parsed.parse_status,
                content_hash: file.content_hash.clone(),
                modified_time: file.modified_time,
                parse_message: parsed.parse_message.clone(),
                parse_error_location: parsed.parse_error_location.clone(),
            });

            let doc_idx = documents.len() - 1;
            let mut doc_entity_iris = Vec::new();

            let semantics = semantics_for_document(
                &file.path,
                file.format,
                &doc_id,
                &parsed,
                self.document_override_text(&file.path),
            )?;

            let stored_hash = if override_text.is_some() {
                effective_hash.clone()
            } else {
                file.content_hash.clone()
            };
            let snapshot = DocumentSnapshot {
                content_hash: stored_hash.clone(),
                document: documents.last().cloned().expect("document"),
                entities: semantics.entities.clone(),
                annotations: semantics.annotations.clone(),
                axioms: semantics.axioms.clone(),
                namespace_rows: semantics.namespace_rows.clone(),
                imports: semantics.imports.clone(),
                quads: parsed.quads().to_vec(),
                triple_count: parsed.triple_count,
                bridge_warning: semantics.bridge_warning.clone(),
            };

            if let Some(diag) = semantics.bridge_warning {
                bridge_diagnostics.push(diag);
            }

            for entity in semantics.entities {
                if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
                    return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(
                        format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                    )));
                }
                if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
                    if prev_doc_idx != doc_idx {
                        document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
                    }
                }
                entity_to_document.insert(entity.iri.clone(), doc_idx);
                doc_entity_iris.push(entity.iri.clone());
                if let Some(&existing_idx) = entity_index.get(&entity.iri) {
                    merge_entity(&mut entities[existing_idx], &entity);
                } else {
                    let idx = entities.len();
                    entity_index.insert(entity.iri.clone(), idx);
                    entities.push(entity);
                }
            }
            document_entity_iris.push(doc_entity_iris);
            annotations.extend(semantics.annotations);
            axioms.extend(semantics.axioms);
            namespaces.extend(semantics.namespace_rows);
            imports.extend(semantics.imports);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_ENTITIES} entities"
                ))));
            }

            if let Some(doc) = documents.last_mut() {
                doc.content_hash = stored_hash.clone();
            }
            if let Some(ref cache) = disk_cache {
                let _ = cache.store(&snapshot);
            }
            document_snapshots.insert(lookup_path, snapshot);
        }

        let previous_paths: HashMap<PathBuf, ()> = previous_snapshots
            .map(|m| m.keys().cloned().map(|p| (p, ())).collect())
            .unwrap_or_default();
        let mut removed = 0usize;
        for path in previous_paths.keys() {
            if !document_snapshots.contains_key(path) {
                removed += 1;
            }
        }

        for (override_path, override_text) in &self.document_overrides {
            if files.iter().any(|f| paths_equal(&f.path, override_path)) {
                continue;
            }
            let format = OntologyFormat::from_extension(
                override_path.extension().and_then(|e| e.to_str()).unwrap_or("ttl"),
            );
            if matches!(format, OntologyFormat::Unknown) {
                continue;
            }
            let doc_id = format!("doc-{}", documents.len() + 1);
            let lookup_path =
                override_path.canonicalize().unwrap_or_else(|_| override_path.clone());
            let effective_hash = content_hash_text(override_text);

            if let Some(prev) = previous_snapshots.and_then(|m| m.get(&lookup_path)) {
                if prev.content_hash == effective_hash {
                    if let Some((reuse_path, modified_time)) = verified_snapshot_source(
                        &self.workspace,
                        override_path,
                        Some(override_text.as_str()),
                        &effective_hash,
                    ) {
                        let snap = prev.with_reuse_context(&doc_id, &reuse_path, modified_time);
                        apply_document_snapshot(
                            &snap,
                            &doc_id,
                            &mut documents,
                            &mut entities,
                            &mut entity_index,
                            &mut entity_to_document,
                            &mut document_entity_iris,
                            &mut annotations,
                            &mut axioms,
                            &mut namespaces,
                            &mut imports,
                            &mut triple_count,
                            &store,
                            &mut bridge_diagnostics,
                            &mut loaded_content_hashes,
                        )?;
                        document_snapshots.insert(lookup_path, snap);
                        reused += 1;
                        continue;
                    }
                }
            }

            reparsed += 1;
            let parsed = parse_ontology_text(
                override_path,
                format,
                &doc_id,
                override_text,
                override_text.as_bytes(),
            )
            .map_err(|e| CatalogError::Parse {
                path: override_path.clone(),
                message: e.to_string(),
            })?;

            // Load quads even when parse_status is Error (partial recovery after a trailing fault).
            if !parsed.quads().is_empty() {
                load_quads_into_store(
                    &store,
                    parsed.quads(),
                    &mut triple_count,
                    &effective_hash,
                    &mut loaded_content_hashes,
                )?;
            }

            documents.push(OntologyDocument {
                id: doc_id.clone(),
                path: override_path.clone(),
                format,
                base_iri: parsed.base_iri.clone(),
                imports: parsed.imports.clone(),
                namespaces: parsed.namespaces.clone(),
                parse_status: parsed.parse_status,
                content_hash: effective_hash.clone(),
                modified_time: 0,
                parse_message: parsed.parse_message.clone(),
                parse_error_location: parsed.parse_error_location.clone(),
            });

            let doc_idx = documents.len() - 1;
            let mut doc_entity_iris = Vec::new();

            let semantics = semantics_for_document(
                override_path,
                format,
                &doc_id,
                &parsed,
                Some(override_text),
            )?;

            let snapshot = DocumentSnapshot {
                content_hash: effective_hash.clone(),
                document: documents.last().cloned().expect("document"),
                entities: semantics.entities.clone(),
                annotations: semantics.annotations.clone(),
                axioms: semantics.axioms.clone(),
                namespace_rows: semantics.namespace_rows.clone(),
                imports: semantics.imports.clone(),
                quads: parsed.quads().to_vec(),
                triple_count: parsed.triple_count,
                bridge_warning: semantics.bridge_warning.clone(),
            };

            if let Some(diag) = semantics.bridge_warning {
                bridge_diagnostics.push(diag);
            }

            for entity in semantics.entities {
                if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
                    return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(
                        format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                    )));
                }
                if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
                    if prev_doc_idx != doc_idx {
                        document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
                    }
                }
                entity_to_document.insert(entity.iri.clone(), doc_idx);
                doc_entity_iris.push(entity.iri.clone());
                if let Some(&existing_idx) = entity_index.get(&entity.iri) {
                    merge_entity(&mut entities[existing_idx], &entity);
                } else {
                    let idx = entities.len();
                    entity_index.insert(entity.iri.clone(), idx);
                    entities.push(entity);
                }
            }
            document_entity_iris.push(doc_entity_iris);
            annotations.extend(semantics.annotations);
            axioms.extend(semantics.axioms);
            namespaces.extend(semantics.namespace_rows);
            imports.extend(semantics.imports);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                    "workspace exceeds maximum of {MAX_ENTITIES} entities"
                ))));
            }

            document_snapshots.insert(lookup_path.clone(), snapshot);
            if let Some(ref cache) = disk_cache {
                let _ = cache.store(document_snapshots.get(&lookup_path).expect("snapshot"));
            }
        }

        let stats = if incremental || reused > 0 || removed > 0 {
            IncrementalStats::Incremental { reused, reparsed, removed }
        } else {
            IncrementalStats::FullBuild
        };

        let config_fingerprint = self.config_fingerprint();
        let mut data = OntologyCatalogData {
            documents,
            entities,
            annotations,
            axioms,
            namespaces,
            imports,
            triple_count,
            diagnostics: Vec::new(),
        };
        let lint_input = DiagnosticInput {
            documents: &data.documents,
            entities: &data.entities,
            annotations: &data.annotations,
            axioms: &data.axioms,
            namespaces: &data.namespaces,
            imports: &data.imports,
        };
        let diag_config = find_config(&self.workspace);
        data.diagnostics = collect_diagnostics_with_config(
            &lint_input,
            &self.document_overrides,
            diag_config.as_ref(),
        );
        data.diagnostics.extend(bridge_diagnostics);

        Ok((
            OntologyCatalog {
                workspace: self.workspace,
                config_fingerprint,
                data,
                store,
                entity_to_document,
                document_entity_iris,
                document_snapshots,
            },
            stats,
        ))
    }
}

impl Default for IndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OntologyCatalog {
    workspace: PathBuf,
    pub(crate) config_fingerprint: String,
    data: OntologyCatalogData,
    store: Store,
    /// Entity IRI → index in [`OntologyCatalogData::documents`].
    pub(crate) entity_to_document: HashMap<String, usize>,
    /// Entity IRIs declared per document (parallel to `documents`).
    pub(crate) document_entity_iris: Vec<Vec<String>>,
    /// Per-file snapshots for incremental reindex (keyed by canonical path).
    pub(crate) document_snapshots: HashMap<PathBuf, DocumentSnapshot>,
}

impl OntologyCatalog {
    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn data(&self) -> &OntologyCatalogData {
        &self.data
    }

    /// Oxigraph triple store for SPARQL — not a stable public API; use [`ontocore_query::sparql_catalog`].
    #[doc(hidden)]
    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_document_snapshot(
    snap: &DocumentSnapshot,
    doc_id: &str,
    documents: &mut Vec<OntologyDocument>,
    entities: &mut Vec<Entity>,
    entity_index: &mut HashMap<String, usize>,
    entity_to_document: &mut HashMap<String, usize>,
    document_entity_iris: &mut Vec<Vec<String>>,
    annotations: &mut Vec<Annotation>,
    axioms: &mut Vec<Axiom>,
    namespaces: &mut Vec<Namespace>,
    imports: &mut Vec<Import>,
    triple_count: &mut usize,
    store: &Store,
    bridge_diagnostics: &mut Vec<Diagnostic>,
    loaded_content_hashes: &mut HashSet<String>,
) -> Result<()> {
    if snap.triple_count != snap.quads.len() {
        return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(
            "document snapshot triple_count does not match quads length".to_string(),
        )));
    }
    if !snap.quads.is_empty() {
        load_quads_into_store(
            store,
            &snap.quads,
            triple_count,
            &snap.content_hash,
            loaded_content_hashes,
        )?;
    }

    documents.push(snap.document.clone());
    let doc_idx = documents.len() - 1;
    let mut doc_entity_iris = Vec::new();

    if let Some(diag) = &snap.bridge_warning {
        bridge_diagnostics.push(diag.clone());
    }

    for entity in &snap.entities {
        if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
            return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                "workspace exceeds maximum of {MAX_ENTITIES} entities"
            ))));
        }
        if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
            if prev_doc_idx != doc_idx {
                document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
            }
        }
        entity_to_document.insert(entity.iri.clone(), doc_idx);
        doc_entity_iris.push(entity.iri.clone());
        if let Some(&existing_idx) = entity_index.get(&entity.iri) {
            merge_entity(&mut entities[existing_idx], entity);
        } else {
            let idx = entities.len();
            entity_index.insert(entity.iri.clone(), idx);
            entities.push(entity.clone());
        }
    }
    document_entity_iris.push(doc_entity_iris);
    annotations.extend(snap.annotations.clone());
    axioms.extend(snap.axioms.clone());
    namespaces.extend(snap.namespace_rows.clone());
    imports.extend(snap.imports.clone());

    let _ = doc_id;
    Ok(())
}

fn merge_entity(existing: &mut Entity, incoming: &Entity) {
    for label in &incoming.labels {
        if !existing.labels.contains(label) {
            existing.labels.push(label.clone());
        }
    }
    for comment in &incoming.comments {
        if !existing.comments.contains(comment) {
            existing.comments.push(comment.clone());
        }
    }
    existing.deprecated |= incoming.deprecated;
    existing.ontology_id = incoming.ontology_id.clone();
    if existing.short_name.is_empty() {
        existing.short_name = incoming.short_name.clone();
    }
}

struct DocumentSemantics {
    entities: Vec<Entity>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
    namespace_rows: Vec<Namespace>,
    imports: Vec<Import>,
    bridge_warning: Option<Diagnostic>,
}

fn semantics_for_document(
    path: &Path,
    format: OntologyFormat,
    doc_id: &str,
    parsed: &ParsedOntology,
    override_text: Option<&String>,
) -> Result<DocumentSemantics> {
    if parsed.parse_status == ParseStatus::Error || !supports_horned_load(format) {
        return Ok(DocumentSemantics {
            entities: parsed.entities.clone(),
            annotations: parsed.annotations.clone(),
            axioms: parsed.axioms.clone(),
            namespace_rows: parsed.namespace_rows.clone(),
            imports: parsed.import_rows.clone(),
            bridge_warning: None,
        });
    }

    let source_text = if let Some(text) = override_text {
        text.clone()
    } else {
        read_to_string_capped(path, MAX_FILE_BYTES).map_err(CatalogError::Core)?
    };

    if format == OntologyFormat::OwlXml {
        return match load_owx_text(path, doc_id, &source_text, &parsed.namespaces) {
            Ok(owl) => Ok(DocumentSemantics {
                entities: owl.bridge.entities,
                annotations: owl.bridge.annotations,
                axioms: owl.bridge.axioms,
                namespace_rows: owl.bridge.namespace_rows,
                imports: owl.bridge.imports,
                bridge_warning: None,
            }),
            Err(e) => {
                eprintln!(
                    "ontocore-catalog: Horned-OWL OWX load failed for {}: {e}; using parser entities",
                    path.display()
                );
                Ok(DocumentSemantics {
                    entities: parsed.entities.clone(),
                    annotations: parsed.annotations.clone(),
                    axioms: parsed.axioms.clone(),
                    namespace_rows: parsed.namespace_rows.clone(),
                    imports: parsed.import_rows.clone(),
                    bridge_warning: None,
                })
            }
        };
    }

    match load_turtle_text(path, doc_id, &source_text, parsed.quads(), &parsed.namespaces) {
        Ok(owl) => Ok(DocumentSemantics {
            entities: owl.bridge.entities,
            annotations: owl.bridge.annotations,
            axioms: owl.bridge.axioms,
            namespace_rows: owl.bridge.namespace_rows,
            imports: owl.bridge.imports,
            bridge_warning: None,
        }),
        Err(e) => {
            eprintln!(
                "ontocore-catalog: Horned-OWL load failed for {}: {e}; using parser entities",
                path.display()
            );
            Ok(DocumentSemantics {
                entities: parsed.entities.clone(),
                annotations: parsed.annotations.clone(),
                axioms: parsed.axioms.clone(),
                namespace_rows: parsed.namespace_rows.clone(),
                imports: parsed.import_rows.clone(),
                bridge_warning: Some(Diagnostic {
                    code: DiagnosticCode::OwlBridgeFailed,
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "Horned-OWL bridge failed; using parser-only entities and axioms: {e}"
                    ),
                    file: path.to_path_buf(),
                    range: SourceLocation::default(),
                    entity_iri: None,
                    quick_fix: None,
                }),
            })
        }
    }
}

fn load_quads_into_store(
    store: &Store,
    quads: &[Quad],
    triple_count: &mut usize,
    content_hash: &str,
    loaded_hashes: &mut HashSet<String>,
) -> Result<()> {
    if !loaded_hashes.insert(content_hash.to_string()) {
        // Duplicate content (identical file body): quads already in the store.
        return Ok(());
    }

    let mut file_triples = 0usize;
    for quad in quads {
        file_triples += 1;
        if file_triples > MAX_TRIPLES_PER_FILE {
            return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                "file exceeds {MAX_TRIPLES_PER_FILE} triples"
            ))));
        }
        *triple_count += 1;
        if *triple_count > MAX_TOTAL_TRIPLES {
            return Err(CatalogError::Core(ontocore_core::OntoCoreError::Scanner(format!(
                "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
            ))));
        }
        store.insert(quad).map_err(|e| CatalogError::Store(e.to_string()))?;
    }
    Ok(())
}

fn verified_snapshot_source(
    workspace: &Path,
    path: &Path,
    override_text: Option<&str>,
    effective_hash: &str,
) -> Option<(PathBuf, u64)> {
    if let Some(text) = override_text {
        if content_hash_text(text) != effective_hash {
            return None;
        }
        let modified_time = std::fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        return Some((path.to_path_buf(), modified_time));
    }
    if !path.exists() {
        return None;
    }
    let file = WorkspaceScanner::new(workspace).describe_path(path).ok()?;
    if file.content_hash != effective_hash {
        return None;
    }
    // Re-verify immediately before reuse to close TOCTOU between hash check and apply.
    let file = WorkspaceScanner::new(workspace).describe_path(path).ok()?;
    if file.content_hash == effective_hash {
        Some((file.path, file.modified_time))
    } else {
        None
    }
}

#[cfg(test)]
mod incremental_tests {
    use super::*;

    #[test]
    fn incremental_rebuild_preserves_unchanged_documents() {
        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("a.ttl");
        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\n",
        )
        .unwrap();

        let first = IndexBuilder::new().workspace(dir.path()).build().expect("first build");
        let entity_count = first.data().entities.len();
        assert!(entity_count > 0);

        let second = IndexBuilder::new()
            .workspace(dir.path())
            .build_incremental(&first)
            .expect("incremental build");
        assert_eq!(second.data().entities.len(), entity_count);
        assert_eq!(second.data().documents.len(), 1);
    }

    #[test]
    fn incremental_rebuild_picks_up_edited_file() {
        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("a.ttl");
        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\n",
        )
        .unwrap();
        let first = IndexBuilder::new().workspace(dir.path()).build().expect("first build");

        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\nex:B a owl:Class .\n",
        )
        .unwrap();

        let second = IndexBuilder::new()
            .workspace(dir.path())
            .build_incremental(&first)
            .expect("incremental build");
        assert!(
            second.data().entities.len() > first.data().entities.len(),
            "edited ontology should add entities"
        );
    }
}
