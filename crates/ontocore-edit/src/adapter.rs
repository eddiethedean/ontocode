use crate::error::{EditError, Result};
use crate::transaction::Transaction;
use ontocore_core::OntologyFormat;
use ontocore_obo::{self, OboPatchOp};
use ontocore_owl::{self, PatchDiagnostic, PatchOp};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditFormat {
    Turtle,
    Obo,
}

impl EditFormat {
    pub fn from_ontology_format(format: OntologyFormat) -> Result<Self> {
        match format {
            OntologyFormat::Turtle => Ok(Self::Turtle),
            OntologyFormat::Obo => Ok(Self::Obo),
            other => Err(EditError::UnsupportedFormat(other.as_str().to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyTextResult {
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<PatchDiagnostic>,
}

impl From<ontocore_owl::ApplyPatchResult> for ApplyTextResult {
    fn from(value: ontocore_owl::ApplyPatchResult) -> Self {
        Self {
            applied: value.applied,
            preview_text: value.preview_text,
            diagnostics: value.diagnostics,
        }
    }
}

pub fn apply_transaction_to_text(
    transaction: &Transaction,
    source: &str,
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyTextResult> {
    match transaction.format()? {
        EditFormat::Turtle => {
            let patches: Vec<PatchOp> = transaction.turtle_patches()?;
            Ok(ontocore_owl::apply_patches_to_text(source, &patches, preview_only, namespaces)?.into())
        }
        EditFormat::Obo => {
            let patches: Vec<OboPatchOp> = transaction.obo_patches()?;
            let result = ontocore_obo::apply_patches_to_text(source, &patches, preview_only)?;
            Ok(ApplyTextResult {
                applied: result.applied,
                preview_text: result.preview_text,
                diagnostics: result
                    .diagnostics
                    .into_iter()
                    .map(|d| PatchDiagnostic { severity: d.severity, message: d.message })
                    .collect(),
            })
        }
    }
}

pub fn apply_transaction_to_path(
    transaction: &Transaction,
    document_path: &Path,
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ontocore_owl::ApplyPatchResult> {
    match transaction.format()? {
        EditFormat::Turtle => {
            let patches = transaction.turtle_patches()?;
            ontocore_owl::apply_patches(document_path, &patches, preview_only, namespaces)
                .map_err(EditError::from)
        }
        EditFormat::Obo => {
            let patches = transaction.obo_patches()?;
            let result = ontocore_obo::apply_patches(document_path, &patches, preview_only)?;
            Ok(ontocore_owl::ApplyPatchResult {
                applied: result.applied,
                preview_text: result.preview_text,
                diagnostics: result
                    .diagnostics
                    .into_iter()
                    .map(|d| PatchDiagnostic { severity: d.severity, message: d.message })
                    .collect(),
                document_path: result.document_path,
            })
        }
    }
}
